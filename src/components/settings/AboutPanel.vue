<template>
  <div class="about-content">
    <div class="app-logo">
      <el-icon :size="60"><ChatDotRound /></el-icon>
      <h2>微信</h2>
    </div>
    <div class="app-info">
      <p>版本：3.9.2 (1.0.0)</p>
      <p>更新时间：2024-01-15</p>
      <p>© 2024 Tencent. All rights reserved.</p>
    </div>
    <div class="about-actions">
      <!-- 更新组件 - 全局使用 -->
      <UpdateNotification />
      <el-button>用户协议</el-button>
      <el-button>隐私政策</el-button>
    </div>
  </div>
</template>

<script setup>
import { ChatDotRound } from '@element-plus/icons-vue';
import UpdateNotification from './UpdateNotification.vue'
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