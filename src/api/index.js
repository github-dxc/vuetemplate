import request from './request'
import { invoke } from "@tauri-apps/api/core";

export const login = async (username, password) => {
  return await invoke("api_login",{ username, password });
};

export const logout = async () => {
  return await invoke("api_logout");
};

export const initData = async () => {
  return await invoke("api_init_data");
};

export const initBugs = async (params) => {
  return await invoke("api_init_bugs",params);
};

export const changeHost = async (host) => {
  return await invoke("api_change_host",{host});
};

export const checkUpdate = async () => {
  return await invoke("api_check_update");
};

export const downloadAndInstall = async () => {
  return await invoke("api_download_and_install");
};

export const apiBugInfo = async (bug_id) => {
  return await invoke("api_bug_info",{bug_id});
};

export const updateBug = async (params) => {
  return await invoke("api_update_bug",params);
};

export const imageBase64 = async (uri) => {
  const blob = await request.get(uri, { responseType: 'blob' })
  return new Promise((resolve) => {
    const reader = new FileReader()
    reader.onloadend = () => resolve(reader.result) // reader.result 就是 base64
    reader.readAsDataURL(blob)
  })
};