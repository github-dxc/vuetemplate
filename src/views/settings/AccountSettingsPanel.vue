<template>
  <div class="user-profile">
    <el-avatar :size="80" :src="userAvatar" />
    <div class="profile-info">
      <h2>{{ username }}</h2>
      <p class="wechat-id">帐号：{{ username }}</p>
    </div>
  </div>
  <!-- <div class="login-options">
    <el-button type="primary" size="large">自动登录</el-button>
    <el-button size="large">已开启</el-button>
    <el-button size="large">关闭</el-button>
  </div>
  <div class="login-tip">
    <p>开启后，自动登录之前的用户</p>
  </div> -->
  <div class="logout-section">
    <el-button text type="primary" @click="logout">退出登录</el-button>
  </div>
</template>

<script setup vapor>
import { ref } from 'vue';
import { useUserStore } from '../../store'
import { useRouter } from 'vue-router';
import { ElMessageBox } from 'element-plus';

const router = useRouter()

const userStore = useUserStore()

const userAvatar = userStore.userInfo.avatar;

const username = userStore.userInfo.username;

async function logout() {
  ElMessageBox.alert('确认退出登录吗？', '退出登录', {
    autofocus: true,
    confirmButtonText: '确认',
    callback: async (action) => {
      if (action.toString() === 'confirm') {
        try {
          await userStore.logout();
          router.push("Login");
        } catch (error) {
          console.error('登出失败:', error)
        }
      }
    },
  })
}

</script>

<style scoped>
/* Account settings styles from original file */
.user-profile {
  display: flex;
  align-items: center;
  gap: 20px;
  margin-bottom: 30px;
  padding-bottom: 20px;
  border-bottom: 1px solid #f0f0f0;
}

.profile-info h2 {
  margin: 0 0 8px 0;
  font-size: 24px;
  color: #333;
}

.wechat-id {
  margin: 0;
  color: #666;
  font-size: 14px;
}

.login-options {
  display: flex;
  gap: 12px;
  margin-bottom: 20px;
}

.login-tip {
  color: #666;
  font-size: 14px;
  line-height: 1.5;
  margin-bottom: 40px;
}

.login-tip p {
  margin: 4px 0;
}

.logout-section {
  margin-top: auto;
}

:deep(.el-avatar) {
  background-color: #ffffff;
}
</style>