<template>
  <div class="update-container">
    <div v-if="updateProgress > 0" class="progress-container">
      <el-progress 
        :percentage="updateProgress" 
        :status="updateProgress === 100 ? 'success' : 'primary'"
      />
      <p>正在下载更新: {{ updateProgress }}%</p>
    </div>
  </div>
</template>

<script setup vapor>
import { ref, computed, onMounted, onUnmounted } from 'vue'
import { ElNotification } from 'element-plus'
import { listen } from '@tauri-apps/api/event';
import { useUserStore } from '../store';
import { downloadAndInstall } from '../api';

// 状态管理
const userStore = useUserStore()
const isShowing = ref(false)
const updateProgress = ref(0)
const updateSetting = computed(() => userStore.updateInfo)

// 事件监听器引用
let updateListener = null
let progressListener = null
let errorListener = null
let finishedListener = null
let noUpdateListener = null

// 显示更新通知
const showUpdateNotification = (updateInfo) => {
  if (isShowing.value) return

  // 检查是否已启用自动更新
  isShowing.value = true
  if (updateSetting.autoUpdate) {
    console.log('自动更新已启用，跳过通知')
    return;
  }
  
  // 检查是否跳过此版本
  if (updateSetting.skipVersion === updateInfo.version) {
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
    // 通知后端开始下载
    await downloadAndInstall();

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
  userStore.updateSetting({ update: {skipVersion: version, autoUpdate: false } })
  
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
  console.log('当前已是最新版本');
  // ElNotification({
  //   title: '无可用更新',
  //   message: '当前已是最新版本',
  //   type: 'info',
  //   duration: 3000
  // })
}

// 生命周期钩子
onMounted(async () => {
  try {
    // 监听更新信息
    updateListener = await listen('app-update', (event) => {
      console.log('收到更新信息:', event.payload)

      // 保存最新版本信息到状态管理
      userStore.setVersion({
        currentVersion: event.payload.current_version,
        lastVersion: event.payload.version,
        updateTime: event.payload.update_time,
      })

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
  position: absolute;
}

.progress-container {
  margin-top: 20px;
  padding: 20px;
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