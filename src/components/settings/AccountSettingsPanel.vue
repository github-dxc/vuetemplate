<template>
  <div class="user-profile">
    <el-avatar :size="80" :src="userAvatar" />
    <div class="profile-info">
      <h2>等风来</h2>
      <p class="wechat-id">微信号：dxc3434</p>
    </div>
  </div>
  <div class="login-options">
    <el-button type="primary" size="large">自动登录</el-button>
    <el-button size="large">已开启</el-button>
    <el-button size="large">关闭</el-button>
  </div>
  <div class="login-tip">
    <p>开启后，在本机登录微信时无需手机确认，</p>
    <p>可在手机微信上关闭。</p>
  </div>
  <div class="logout-section">
    <el-button text type="primary" @click="$emit('logout')">退出登录</el-button>
  </div>
</template>

<script setup>
defineProps({
  userAvatar: String
});
defineEmits(['logout']);
</script>

<style scoped>
/* Account settings styles from original file */
.user-profile { display: flex; align-items: center; gap: 20px; margin-bottom: 30px; padding-bottom: 20px; border-bottom: 1px solid #f0f0f0; }
.profile-info h2 { margin: 0 0 8px 0; font-size: 24px; color: #333; }
.wechat-id { margin: 0; color: #666; font-size: 14px; }
.login-options { display: flex; gap: 12px; margin-bottom: 20px; }
.login-tip { color: #666; font-size: 14px; line-height: 1.5; margin-bottom: 40px; }
.login-tip p { margin: 4px 0; }
.logout-section { margin-top: auto; }
</style>