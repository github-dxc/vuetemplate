<template>
  <div class="update-container">
    <!-- 主要内容区域 -->
    <div class="main-content">
      <el-button @click="checkForUpdate" :loading="isChecking">
        检查更新
      </el-button>
      
      <div v-if="updateProgress > 0" class="progress-container">
        <el-progress 
          :percentage="updateProgress" 
          :status="updateProgress === 100 ? 'success' : 'primary'"
        />
        <p>正在下载更新: {{ updateProgress }}%</p>
      </div>
    </div>
  </div>
</template>

<script setup>
import { ref, onMounted, onUnmounted } from 'vue'
import { ElNotification, ElButton } from 'element-plus'
import { invoke } from "@tauri-apps/api/core";
import { emit, listen } from '@tauri-apps/api/event';

// 状态管理
const isChecking = ref(false)
const isShowing = ref(false)
const updateProgress = ref(0)
const currentUpdateInfo = ref(null)

// 事件监听器引用
let updateListener = null
let progressListener = null
let errorListener = null
let finishedListener = null
let noUpdateListener = null

// 存储设置
const STORAGE_KEY = 'app_update_settings'

// 获取用户设置
const getUserSettings = () => {
  const settings = localStorage.getItem(STORAGE_KEY)
  return settings ? JSON.parse(settings) : { skipVersion: null, autoUpdate: true }
}

// 保存用户设置
const saveUserSettings = (settings) => {
  localStorage.setItem(STORAGE_KEY, JSON.stringify(settings))
}

// 检查更新
const checkForUpdate = async () => {
  if (isChecking.value) return

  // 清空用户设置
  saveUserSettings({ skipVersion: null, autoUpdate: true });
  
  isChecking.value = true
  updateProgress.value = 0
  
  try {
    await invoke('api_check_update');
  } catch (error) {
    console.error('api_check_update error:', error)
    ElNotification({
      title: '检查更新失败',
      message: `错误: ${error}`,
      type: 'error',
      duration: 5000
    })
  } finally {
    isChecking.value = false
  }
}

// 显示更新通知
const showUpdateNotification = (updateInfo) => {
  if (isShowing.value) return
  isShowing.value = true
  const userSettings = getUserSettings()
  
  // 检查是否跳过此版本
  if (userSettings.skipVersion === updateInfo.version) {
    console.log(`跳过版本 ${updateInfo.version}`)
    isShowing.value = false
    return
  }
  
  const notification = ElNotification({
    title: '应用更新',
    dangerouslyUseHTMLString: true,
    message: `
    <div style="line-height: 1.6;">
      <p><strong>发现新版本!</strong></p>
      <p>当前版本: ${updateInfo.current_version}</p>
      <p>最新版本: ${updateInfo.version}</p>
      <p>目标平台: ${updateInfo.target}</p>
    </div>
  `,
    type: 'info',
    duration: 5000,
    showClose: false, // 隐藏默认关闭按钮
    customClass: 'update-notification',
    onClose: () => {
      // 通知关闭时的处理
      console.log('更新通知已关闭')
      isShowing.value = false
    }
  })
  
  // 添加自定义按钮
  setTimeout(() => {
    const notificationEl = document.querySelector('.update-notification')
    if (notificationEl) {
      addCustomButtons(notificationEl, updateInfo, notification)
    }
  }, 100)
}

// 添加自定义按钮
const addCustomButtons = (notificationEl, updateInfo, notification) => {
  // 创建按钮容器
  const buttonContainer = document.createElement('div')
  buttonContainer.className = 'update-buttons'
  buttonContainer.style.cssText = `
    margin-top: 12px;
    display: flex;
    gap: 8px;
    justify-content: flex-end;
  `
  
  // 确认更新按钮
  const confirmBtn = document.createElement('button')
  confirmBtn.className = 'el-button el-button--primary el-button--small'
  confirmBtn.textContent = '立即更新'
  confirmBtn.onclick = () => {
    confirmUpdate(updateInfo)
    notification.close()
  }
  
  // 不再提醒按钮
  const skipBtn = document.createElement('button')
  skipBtn.className = 'el-button el-button--default el-button--small'
  skipBtn.textContent = '跳过此版本'
  skipBtn.onclick = () => {
    skipVersion(updateInfo.version)
    notification.close()
  }
  
  // 稍后提醒按钮
  const laterBtn = document.createElement('button')
  laterBtn.className = 'el-button el-button--default el-button--small'
  laterBtn.textContent = '稍后提醒'
  laterBtn.onclick = () => {
    notification.close()
  }
  
  // 添加按钮到容器
  buttonContainer.appendChild(confirmBtn)
  buttonContainer.appendChild(skipBtn)
  buttonContainer.appendChild(laterBtn)
  
  // 将按钮容器添加到通知中
  const messageEl = notificationEl.querySelector('.el-notification__content')
  if (messageEl) {
    messageEl.appendChild(buttonContainer)
  }
}

// 确认更新
const confirmUpdate = async (updateInfo) => {
  try {
    currentUpdateInfo.value = updateInfo
    
    // 通知后端开始下载
    await invoke('api_download_and_install');

    // 显示开始下载的通知
    ElNotification({
      title: '开始下载',
      message: '正在下载更新，请稍候...',
      type: 'info',
      duration: 2000
    })
  } catch (error) {
    console.error('确认更新失败:', error)
    ElNotification({
      title: '更新失败',
      message: `错误: ${error}`,
      type: 'error',
      duration: 2000
    })
  }
}

// 跳过版本
const skipVersion = (version) => {
  const settings = getUserSettings()
  settings.skipVersion = version
  saveUserSettings(settings)
  
  ElNotification({
    title: '已跳过',
    message: `版本 ${version} 将不再提醒`,
    type: 'warning',
    duration: 3000
  })
}

// 处理更新进度
const handleUpdateProgress = (progress) => {
  updateProgress.value = progress
  
  // 可以添加进度通知
  if (progress === 100) {
    ElNotification({
      title: '下载完成',
      message: '更新下载完成，即将安装...',
      type: 'success',
      duration: 3000
    })
  }
}

// 处理更新错误
const handleUpdateError = (error) => {
  console.error('更新错误:', error)
  updateProgress.value = 0
  
  ElNotification({
    title: '更新失败',
    message: `更新过程中发生错误: ${error}`,
    type: 'error',
    duration: 0 // 错误信息不自动关闭
  })
}

// 处理下载完成
const handleDownloadFinished = () => {
  ElNotification({
    title: '准备安装',
    message: '更新下载完成，应用将重启以完成安装',
    type: 'info',
    duration: 5000
  })
}

// 处理无更新
const handleNoUpdate = () => {
  ElNotification({
    title: '无可用更新',
    message: '当前已是最新版本',
    type: 'info',
    duration: 3000
  })
}

// 生命周期钩子
onMounted(async () => {
  try {
    // 监听更新信息
    updateListener = await listen('app-update', (event) => {
      console.log('收到更新信息:', event.payload)
      showUpdateNotification(event.payload)
    })
    
    // 监听更新进度
    progressListener = await listen('app-update-progress', (event) => {
      handleUpdateProgress(event.payload)
    })
    
    // 监听更新错误
    errorListener = await listen('app-update-error', (event) => {
      handleUpdateError(event.payload)
    })
    
    // 监听下载完成
    finishedListener = await listen('app-update-download-finished', () => {
      handleDownloadFinished()
    })
    
    // 监听无更新
    noUpdateListener = await listen('app-update-none', () => {
      handleNoUpdate()
    })
    
  } catch (error) {
    console.error('设置事件监听器失败:', error)
  }
})

onUnmounted(() => {
  // 清理事件监听器
  if (updateListener) {
    updateListener()
    updateListener = null
  }
  if (progressListener) {
    progressListener()
    progressListener = null
  }
  if (errorListener) {
    errorListener()
    errorListener = null
  }
  if (finishedListener) {
    finishedListener()
    finishedListener = null
  }
  if (noUpdateListener) {
    noUpdateListener()
    noUpdateListener = null
  }
})
</script>

<style scoped>
.update-container {
  padding: 20px;
}

.main-content {
  max-width: 800px;
  margin: 0 auto;
}

.progress-container {
  margin-top: 20px;
  padding: 20px;
  background: #f5f5f5;
  border-radius: 8px;
}

.progress-container p {
  margin-top: 10px;
  text-align: center;
  color: #666;
}
</style>

<style>
/* 全局样式 - 自定义通知样式 */
.update-notification {
  min-width: 400px !important;
}

.update-buttons {
  margin-top: 12px;
  display: flex;
  gap: 8px;
  justify-content: flex-end;
}

.update-buttons button {
  border-radius: 4px;
  padding: 6px 12px;
  font-size: 12px;
  border: 1px solid #dcdfe6;
  cursor: pointer;
  transition: all 0.3s;
}

.update-buttons button:hover {
  opacity: 0.8;
}

.update-buttons .el-button--primary {
  background: #409eff;
  color: white;
  border-color: #409eff;
}

.update-buttons .el-button--default {
  background: #fff;
  color: #606266;
}

.update-buttons .el-button--default:hover {
  color: #409eff;
  border-color: #409eff;
}
</style>