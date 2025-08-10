// api/request.js - HTTP 请求封装
import axios from 'axios'
import { useUserStore } from '../store'

const userStore = useUserStore();

const request = axios.create({
  baseURL: 'http://localhost:8989/',
  timeout: 10000
}) 

request.interceptors.request.use(config => {
  const token = userStore.userToken;
  if (token) {
    // token存放的是cookie的值，请把cookie放进header中
    config.headers['Cookie'] = token;
  }
  return config
})

request.interceptors.response.use(
  response => response.data,
  error => {
    if (error.response?.status === 401) {
      userStore.setToken(null)
      window.location.href = '/login'
    }
    return Promise.reject(error)
  }
)

export default request