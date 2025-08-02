import request from './request'
import { invoke } from "@tauri-apps/api/core";

// export const login = (data) => request.post('/user/login', data)
// export const getUserInfo = () => request.get('/user/info')
// export const updateUser = (data) => request.put('/user/update', data)
// export const register = (data) => request.post('/user/register', data)

export const logout = async () => {
  return await invoke("api_logout");
};

export const changeHost = async (host) => {
  return await invoke("api_change_host",host);
};

export const checkUpdate = async () => {
  return await invoke("api_check_update");
};

export const downloadAndInstall = async () => {
  return await invoke("api_download_and_install");
};