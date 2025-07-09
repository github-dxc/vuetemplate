<script setup>
import { useRouter } from 'vue-router'
import UpdateNotification from './components/UpdateNotification.vue'
// import TopMenu from './components/TopMenu.vue'

const router = useRouter()

function goHome() {
  router.push('/')
}
function goLogin() {
  router.push('/login')
}
</script>

<template>
  <main class="container">
    <div>
      <!-- <TopMenu /> -->
      <button @click="goHome">跳转首页</button>
      <button @click="goLogin">跳转登录页</button>
      <router-view />
      <!-- 更新组件 - 全局使用 -->
      <UpdateNotification />
    </div>
  </main>
</template>

<style scoped>

</style>
<style>

</style>
