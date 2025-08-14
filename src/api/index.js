import { invoke } from "@tauri-apps/api/core";

export const loginInfo = async () => {
  return await invoke("api_login_info");
};

export const login = async (username, password) => {
  return await invoke("api_login",{ username, password });
};

export const logout = async () => {
  return await invoke("api_logout");
};

export const initData = async () => {
  return await invoke("api_init_data");
};

export const initBugs = async () => {
  return await invoke("api_init_bugs");
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
  return await invoke("api_image_bytes",{uri});
};