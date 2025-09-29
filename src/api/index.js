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

export const initMsgs = async () => {
  return await invoke("api_init_msgs");
};

export const bugList = async ({page=1,limit=10,view_state=0,priority=[],severity=[],status=[],resolution=[],project_id='',reporter_id=[],handler_id=[],category_id=[]}) => {
  return await invoke("api_bug_list",{page,limit,view_state,priority,severity,status,resolution,project_id,reporter_id,handler_id,category_id});
};

export const changeHost = async (host) => {
  return await invoke("api_change_host",{host});
};

export const changeSubParams = async (params=[]) => {
  return await invoke("api_change_sub_params",{params});
};

export const readMsg = async (read_msg) => {
  return await invoke("api_read_msg",{read_msg});
};

export const setWindowMessageNotify = async (count) => {
  return await invoke("api_set_message_notify",{count});
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

export const subParamsInfo = async () => {
  return await invoke("api_sub_params_info");
};

export const updateBug = async ({ bug_id, severity=0, status=0, resolution=0, category_id=0, handler_id=0, summary="", description="", steps_to_reproduce="" }) => {
  return await invoke("api_update_bug",{bug_id,severity,status,resolution,category_id,handler_id,summary,description,steps_to_reproduce});
};

export const bugReport = async ({project_id='0',category_id=1,reproducibility=30,severity=50,priority=30,handler_id=0,
  summary='',description='',steps_to_reproduce='',file_path=[],binary_file=[]}) => {
  return await invoke("api_bug_report",{project_id,category_id,reproducibility,severity,priority,handler_id,
  summary,description,steps_to_reproduce,file_path,binary_file});
}

export const bugNoteAdd = async ({bug_id,bugnote_text,file_path=[],binary_file=[]}) => {
  return await invoke("api_bug_note_add",{bug_id,bugnote_text,file_path,binary_file});
}

export const imageBase64 = async (uri) => {
  return await invoke("api_image_bytes",{uri});
};

export const browserOpen = async (url) => {
  return await invoke("api_browser_open",{url});
};