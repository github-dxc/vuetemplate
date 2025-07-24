<script setup vapor>
import { useRouter } from 'vue-router'

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
    </div>
  </main>
</template>

<style scoped>

</style>
<style>

</style>
