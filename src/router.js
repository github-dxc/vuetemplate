import { createRouter, createWebHistory } from 'vue-router'
import Home from './view/Home.vue'
import Login from './view/Login.vue'
import Demo from './view/Demo.vue'

const routes = [
  {
    path: '/',
    redirect: '/home' // 设置默认路由
  },
  {
    path: '/home',
    name: 'Home',
    component: Home
  },
  {
    path: '/login',
    name: 'Login',
    component: Login
  },
  {
    path: '/demo',
    name: 'Demo',
    component: Demo
  }
]

const router = createRouter({
  history: createWebHistory(),
  routes
})

export default router