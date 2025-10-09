import { createRouter, createWebHistory } from 'vue-router'
import Home from '../views/Home.vue'
import Login from '../views/Login.vue'
import Image from '../components/Image.vue'
import TimeTrans from '../views/TimeTrans.vue'

const routes = [
  {
    path: '/',
    redirect: '/login' // 设置默认路由
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
    path: '/image',
    name: 'Image',
    component: Image
  },
  {
    path: '/time-trans',
    name: 'TimeTrans',
    component: TimeTrans
  }
]

const router = createRouter({
  history: createWebHistory(),
  routes
})

export default router