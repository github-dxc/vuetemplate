import request from './request'
import { useRouter } from 'vue-router'
import { invoke } from "@tauri-apps/api/core";
const router = useRouter()

// export const login = (data) => request.post('/user/login', data)
// export const getUserInfo = () => request.get('/user/info')
// export const updateUser = (data) => request.put('/user/update', data)
// export const register = (data) => request.post('/user/register', data)

export const logout = async () => {
  console.log('退出登录');
  return await invoke("api_logout");
};

