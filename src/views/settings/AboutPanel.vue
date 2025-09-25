<template>
  <div class="about-content">
    <div class="app-logo">
      <el-icon :size="60"><ChatDotRound /></el-icon>
      <h2>MantisBT-Tool</h2>
    </div>
    <div class="app-info">
      <div v-if="newVersion">
        <el-link type="danger" @click="checkForUpdate">{{ newVersion }} （{{ version.currentVersion }}）</el-link>
      </div>
      <p v-if="!newVersion">版本：{{ version.currentVersion }}</p>
      <p>更新时间：{{ formatDate(version.updateTime) }}</p>
      <p>© 2025 DengXiangCheng. All rights reserved.</p>
    </div>
    <div class="about-actions">
      <!-- 更新组件 - 全局使用 -->
      <el-button @click="checkForUpdate" :loading="isChecking">检查更新</el-button>
    </div>
  </div>
</template>

<script setup vapor>
import { ref, computed } from 'vue'
import { ChatDotRound } from '@element-plus/icons-vue';
import { ElNotification } from 'element-plus'
import { checkUpdate } from '../../api';
import { useUserStore } from '../../store';

const userStore = useUserStore()

const isChecking = ref(false)
const version = computed(() => userStore.versionInfo)
const newVersion = computed(() => {
  if (userStore.versionInfo.currentVersion !== userStore.versionInfo.lastVersion) {
    return `新版本: ${userStore.versionInfo.lastVersion}`;
  }
})

// 检查更新
const checkForUpdate = async () => {
  userStore.updateSetting({ update: {skipVersion: '0.0.0'} })
  if (isChecking.value) return

  try {
    const version = await checkUpdate();
    if (!version) {
      ElNotification({
        title: '无可用更新',
        message: '当前已是最新版本',
        type: 'info',
        duration: 3000
      })
    }
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

const formatDate = function(ts) {
  const date = new Date(ts * 1000) // 秒转毫秒
  const y = date.getFullYear()
  const m = String(date.getMonth() + 1).padStart(2, '0')
  const d = String(date.getDate()).padStart(2, '0')
  return `${y}-${m}-${d}`
}

</script>

<style scoped>
/* About panel styles from original file */
.about-content { text-align: center; padding-top: 50px; }
.app-logo { margin-bottom: 30px; }
.app-logo h2 { margin: 15px 0 0 0; font-size: 28px; color: #333; }
.app-info { margin-bottom: 40px; color: #666; line-height: 1.8; }
.app-info p { margin: 8px 0; }
.about-actions { display: flex; justify-content: center; gap: 15px; }
</style>