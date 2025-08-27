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

export const updateBug = async ({ bug_id, severity=0, status=0, resolution=0, category_id=0, handler_id=0, summary="", description="", steps_to_reproduce="" }) => {
  return await invoke("api_update_bug",{bug_id,severity,status,resolution,category_id,handler_id,summary,description,steps_to_reproduce});
};

export const bugNoteAdd = async ({bug_id,bugnote_text,file_path=[],binary_file=[]}) => {
  return await invoke("api_bug_note_add",{bug_id,bugnote_text,file_path,binary_file});
}

export const imageBase64 = async (uri) => {
  return await invoke("api_image_bytes",{uri});
};