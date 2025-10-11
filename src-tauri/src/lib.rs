mod badge;
mod enums;
mod model;
mod utils;

use chrono::{NaiveTime, TimeZone, Utc};
use chrono_tz::Asia::Shanghai;
use enums::*;
use env_logger;
use log::{debug, info, warn};
use model::*;
use rdev::EventType;
use reqwest::cookie::{CookieStore, Jar};
use scraper::Html;
use serde_json::Value;
use std::collections::HashMap;
use std::collections::HashSet;
use std::sync::{Arc, Mutex};
use tauri::async_runtime::block_on;
use tauri::{AppHandle, Emitter, Manager, WindowEvent};
use tauri_plugin_autostart::MacosLauncher;
use tauri_plugin_notification::{NotificationExt, PermissionState};
use tauri_plugin_opener::OpenerExt;
use tauri_plugin_store::StoreExt;
use tauri_plugin_updater::{Update, UpdaterExt};
use tokio::signal;
use tokio::time::{interval, Duration};
use url::Url;
use rdev::listen;
use std::thread;
use utils::*;

use crate::badge::set_message_notify;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    // 初始化日志实现库
    env_logger::Builder::new()
        .filter_level(log::LevelFilter::Info) // 设置日志级别为 Debug
        .init();
    tauri::Builder::default()
        .plugin(tauri_plugin_clipboard_manager::init())
        .plugin(tauri_plugin_autostart::init(
            MacosLauncher::LaunchAgent,
            Some(vec!["--flag1", "--flag2"]),
        ))
        .plugin(tauri_plugin_store::Builder::new().build())
        .plugin(tauri_plugin_updater::Builder::new().build())
        .plugin(tauri_plugin_notification::init())
        .plugin(tauri_plugin_opener::init())
        .manage(MyState::default()) // 注册全局状态
        .invoke_handler(tauri::generate_handler![
            api_browser_open,
            api_init_data,
            api_init_bugs,
            api_init_msgs,
            api_sub_params_info,
            api_change_sub_params,
            api_change_host,
            api_read_msg,
            api_set_message_notify,
            api_login_info,
            api_login,
            api_logout,
            api_bug_info,
            api_bug_list,
            api_image_bytes,
            api_update_bug,
            api_bug_report,
            api_bug_note_add,
            api_check_update,
            api_download_and_install
        ])
        .setup(|app| {
            //清空所有状态
            // let _ = clear_global_state(app.handle().clone());
            //初始化全局状态
            let _ = init_global_state(app.handle().clone())
                .map_err(|e| println!("init_global_state err:{}", e));
            //初始化分组和项目
            let _ = sync_init_project_catgory(app.handle().clone())
                .map_err(|e| println!("sync_init_project_catgory err:{}", e));
            //定时更新
            update_app(app.handle().clone());
            //定时查询订阅数据
            find_sub_data(app.handle().clone());
            //注册关闭回调
            close_app_callback(app.handle().clone());
            //注册快捷键
            listen_keybord_event(app.handle().clone())?;
            Ok(())
        })
        .on_window_event(move |window, event| {
            match event {
                //关闭事件
                WindowEvent::CloseRequested { api, .. } => {
                    let label = window.title().unwrap_or_default();
                    if label == "image" || label == "time-trans" {
                        println!("非main窗口关闭");
                        return;
                    }
                    println!("窗口即将关闭，执行清理操作...");
                    let r = save_global_state(window.app_handle().clone());
                    match r {
                        Ok(_) => println!("handle exec ok"),
                        Err(msg) => println!("handle exec err: {}", msg),
                    }
                    // 如果你想阻止关闭，可以调用：
                    // api.prevent_close();
                }
                WindowEvent::Destroyed => {
                    println!("window closed");
                }
                _ => {}
            }
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

// 全局状态
#[derive(Default)]
struct MyState {
    user: Arc<Mutex<LoginInfo>>, //登录信息
    jar: Mutex<Arc<Jar>>,
    host: Arc<Mutex<String>>,
    sub_params: Arc<Mutex<Vec<String>>>,             //订阅列表
    sub_bugs: Arc<Mutex<Vec<Bug>>>,                  //bugs列表
    last_version: Arc<Mutex<Option<Update>>>,        //系统版本
    change_historys: Arc<Mutex<Vec<ChangeHistory>>>, //操作日志(保存100条)

    catgory_kv: Arc<Mutex<Vec<KV>>>, //分组的kv信息
    project_kv: Arc<Mutex<Vec<KV>>>, //项目的kv信息
    users_kv: Arc<Mutex<Vec<KV>>>,   //用户的kv信息
}

// 打开外部浏览器地址
#[tauri::command(rename_all = "snake_case")]
async fn api_browser_open(app: tauri::AppHandle, url: String) -> Result<(), String> {
    println!("open url:{}", url);
    // 如果url只有路径，则补全
    let url = if url.starts_with("http://") || url.starts_with("https://") {
        url
    } else {
        let state = app.state::<MyState>().clone();
        let host = state
            .host
            .lock()
            .map_err(|e| format!("lock err:{}", e))?
            .clone();
        let uri = if url.starts_with("/") {
            url
        } else {
            format!("/{}", url)
        };
        format!("http://{}{}", host, uri)
    };
    app.opener()
        .open_url(url, None::<&str>)
        .map_err(|e| format!("open_url err:{}", e))
}

// 初始化数据
#[tauri::command(rename_all = "snake_case")]
async fn api_init_data(app: AppHandle) -> Result<String, String> {
    let state = app.state::<MyState>().clone();

    // 查询项目列表
    let projects = state
        .project_kv
        .lock()
        .map_err(|e| format!("lock err:{}", e))?
        .clone();
    if projects.len() == 0 {
        init_project_catgory(app.clone()).await?;
    }
    // 查询项目列表
    let projects = state
        .project_kv
        .lock()
        .map_err(|e| format!("lock err:{}", e))?
        .clone();

    // 查询分组列表
    let category = state
        .catgory_kv
        .lock()
        .map_err(|e| format!("lock err:{}", e))?
        .clone();

    // 查询用户列表
    let users = state
        .users_kv
        .lock()
        .map_err(|e| format!("lock err:{}", e))?
        .clone();

    let mut hm = HashMap::new();
    hm.insert("Priority", Priority::kv());
    hm.insert("Severity", Severity::kv());
    hm.insert("Reproducibility", Reproducibility::kv());
    hm.insert("ViewStatus", ViewStatus::kv());
    hm.insert("Category", category);
    hm.insert("Project", projects);
    hm.insert("Users", users);
    hm.insert("Status", Status::kv());
    hm.insert("Resolution", Resolution::kv());
    serde_json::to_string(&hm).map_err(|e| format!("serde_json err:{}", e))
}

// 修改host地址
#[tauri::command(rename_all = "snake_case")]
async fn api_change_host(app: AppHandle, host: &str) -> Result<String, String> {
    let state = app.state::<MyState>().clone();

    let mut state_host = state.host.lock().map_err(|e| format!("lock err:{}", e))?;
    if host != "" {
        *state_host = host.to_string();
    }
    Ok(state_host.clone())
}

// 设置已读
#[tauri::command(rename_all = "snake_case")]
async fn api_read_msg(app: AppHandle, read_msg: &str) -> Result<(), String> {
    let state = app.state::<MyState>().clone();

    let mut user = state.user.lock().map_err(|e| format!("lock err:{}", e))?;
    user.read_msg = read_msg.to_owned();
    Ok(())
}

#[tauri::command(rename_all = "snake_case")]
async fn api_change_sub_params(app: tauri::AppHandle, params: Vec<String>) -> Result<(), String> {
    if params.len() == 0 {
        return Err("params is empty".to_string());
    }
    for sub_param in params.iter() {
        let _ = serde_html_form::from_str::<FindBugListParams>(sub_param)
            .map_err(|e| format!("serde_sub_params err:{}", e))?;
    }
    let state = app.state::<MyState>().clone();
    let mut sub_params = state
        .sub_params
        .lock()
        .map_err(|e| format!("lock err:{}", e))?;
    *sub_params = params;
    Ok(())
}

// 初始化bugs数据
#[tauri::command(rename_all = "snake_case")]
async fn api_init_bugs(app: AppHandle) -> Result<Vec<Bug>, String> {
    let state = app.state::<MyState>().clone();

    // 查询缓存的bug列表
    let sub_bugs = state
        .sub_bugs
        .lock()
        .map_err(|e| format!("lock err:{}", e))?
        .clone();
    Ok(sub_bugs)
}

// 初始化msgs数据
#[tauri::command(rename_all = "snake_case")]
async fn api_init_msgs(app: AppHandle) -> Result<Vec<ChangeHistory>, String> {
    let state = app.state::<MyState>().clone();

    // 查询缓存的bug列表
    let logs = state
        .change_historys
        .lock()
        .map_err(|e| format!("lock err:{}", e))?
        .clone();
    Ok(logs)
}

// 获取登录信息
#[tauri::command(rename_all = "snake_case")]
async fn api_login_info(app: AppHandle) -> Result<LoginInfo, String> {
    let state = app.state::<MyState>().clone();

    let user = state
        .user
        .lock()
        .map_err(|e| format!("lock err:{}", e))?
        .clone();
    Ok(user)
}

// 获取订阅信息
#[tauri::command(rename_all = "snake_case")]
async fn api_sub_params_info(app: AppHandle) -> Result<Vec<String>, String> {
    let state = app.state::<MyState>().clone();

    let params = state
        .sub_params
        .lock()
        .map_err(|e| format!("lock err:{}", e))?
        .clone();
    Ok(params)
}

// 登录
#[tauri::command(rename_all = "snake_case")]
async fn api_login(app: AppHandle, username: &str, password: &str) -> Result<String, String> {
    let jar = Arc::new(Jar::default());
    let state = app.state::<MyState>().clone();
    let host = state
        .host
        .lock()
        .map_err(|e| format!("lock err:{}", e))?
        .clone();
    let result = login(jar.clone(), username, password, &host)
        .await
        .map_err(|e| format!("login err:{}", e))?;
    let body = my_view_page(jar.clone(), &host)
        .await
        .map_err(|e| format!("my_view_page err:{}", e))?;

    // 保存cookie到全局状态
    {
        let mut user = state.user.lock().map_err(|e| format!("lock err:{}", e))?;
        user.logined = true;
        user.username = username.to_string();
        user.user_id = get_user_id(&Html::parse_document(body.as_str()))?;
        let mut jar_ = state.jar.lock().map_err(|e| format!("lock err:{}", e))?;
        *jar_ = jar;
    }

    // 初始化分组和项目
    init_project_catgory(app).await?;
    return Ok(result);
}

// 退出登录
#[tauri::command(rename_all = "snake_case")]
async fn api_logout(app: AppHandle) -> Result<(), String> {
    clear_global_state(app.clone())?;
    init_global_state(app.clone())?;
    init_project_catgory(app).await?;
    Ok(())
}

// bug列表
#[tauri::command(rename_all = "snake_case")]
async fn api_bug_list(
    app: AppHandle,
    page: i64,
    limit: i64,
    view_state: i64,
    priority: Vec<i64>,
    severity: Vec<i64>,
    status: Vec<i64>,
    resolution: Vec<i64>,
    project_id: &str,
    reporter_id: Vec<i64>,
    handler_id: Vec<i64>,
    category_id: Vec<String>,
) -> Result<BugList, String> {
    let state = app.state::<MyState>().clone();
    let user = state
        .user
        .lock()
        .map_err(|e| format!("lock err:{}", e))?
        .clone();
    let jar = state
        .jar
        .lock()
        .map_err(|e| format!("lock err:{}", e))?
        .clone();
    let host = state
        .host
        .lock()
        .map_err(|e| format!("lock err:{}", e))?
        .clone();
    let project_kv = state
        .project_kv
        .lock()
        .map_err(|e| format!("lock err:{}", e))?
        .clone();
    let catgory_kv = state
        .catgory_kv
        .lock()
        .map_err(|e| format!("lock err:{}", e))?
        .clone();
    if !user.logined {
        return Err("未登录".to_string());
    }

    let param_str = r"type=1&view_type=simple&reporter_id[]=0&handler_id[]=0&monitor_user_id[]=0&note_user_id[]=0&priority[]=0&severity[]=0&view_state=0&sticky=1&category_id[]=0&hide_status[]=90&status[]=0&resolution[]=0&profile_id[]=0&platform[]=0&os[]=0&os_build[]=0&relationship_type=-1&relationship_bug=0&tag_string=&per_page=50&sort[]=date_submitted&dir[]=DESC&sort[]=status&dir[]=ASC&sort[]=last_updated&dir[]=DESC&match_type=0&highlight_changed=6&search=&filter_submit=应用过滤器";
    let mut param = serde_html_form::from_str::<FindBugListParams>(param_str)
        .map_err(|e| format!("serde_html_form err:{}", e))?;
    param.per_page = limit;
    param.view_state = view_state;
    param.priority = priority;
    param.severity = severity;
    param.status = status;
    param.resolution = resolution;
    param.reporter_id = reporter_id;
    param.handler_id = handler_id;
    param.category_id = category_id;
    param.project_id = project_id.to_string();

    // 查询列表
    let body = view_all_set(jar, param, project_id, page, &host)
        .await
        .map_err(|e| format!("view_all_set err:{}", e))?;
    // 解析数据
    let mut data = view_all_set_data(
        &Html::parse_document(body.as_str()),
        &catgory_kv,
        &project_kv,
    )
    .map_err(|e| format!("view_all_set_data err:{}", e))?;

    // 设置当前项目
    if !(project_id == "0" || project_id == "") {
        if let Some(project) = project_kv.find_by_key(project_id) {
            data.bugs.iter_mut().for_each(|b| {
                b.project = project.value.clone();
            });
        };
    }

    Ok(data)
}

// 查询bug详情
#[tauri::command(rename_all = "snake_case")]
async fn api_bug_info(app: AppHandle, bug_id: i64) -> Result<BugInfo, String> {
    let state = app.state::<MyState>().clone();
    let user = state
        .user
        .lock()
        .map_err(|e| format!("lock err:{}", e))?
        .clone();
    let jar = state
        .jar
        .lock()
        .map_err(|e| format!("lock err:{}", e))?
        .clone();
    let host = state
        .host
        .lock()
        .map_err(|e| format!("lock err:{}", e))?
        .clone();
    let project_kv = state
        .project_kv
        .lock()
        .map_err(|e| format!("lock err:{}", e))?
        .clone();
    let catgory_kv = state
        .catgory_kv
        .lock()
        .map_err(|e| format!("lock err:{}", e))?
        .clone();
    if !user.logined {
        return Err("未登录".to_string());
    }
    // 查询bug详情
    let body = my_view_detail(jar.clone(), bug_id, &host).await?;
    let document = Html::parse_document(body.as_str());
    let bug_info = my_view_detail_data(&document, &host, &catgory_kv, &project_kv)?;
    Ok(bug_info)
}

// 修改bug
#[tauri::command(rename_all = "snake_case")]
async fn api_update_bug(
    app: AppHandle,
    bug_id: i64,
    severity: i64,
    status: i64,
    resolution: i64,
    category_id: i64,
    handler_id: i64,
    summary: String,
    description: String,
    steps_to_reproduce: String,
) -> Result<String, String> {
    let state = app.state::<MyState>().clone();
    let user = state
        .user
        .lock()
        .map_err(|e| format!("lock err:{}", e))?
        .clone();
    let jar = state
        .jar
        .lock()
        .map_err(|e| format!("lock err:{}", e))?
        .clone();
    let host = state
        .host
        .lock()
        .map_err(|e| format!("lock err:{}", e))?
        .clone();
    let project_kv = state
        .project_kv
        .lock()
        .map_err(|e| format!("lock err:{}", e))?
        .clone();
    let catgory_kv = state
        .catgory_kv
        .lock()
        .map_err(|e| format!("lock err:{}", e))?
        .clone();
    if !user.logined {
        return Err("未登录".to_string());
    }
    // 查询bug详情
    let bug_update_page_token;
    let bug_info;
    {
        let body = my_view_detail(jar.clone(), bug_id, &host).await?;
        let document = Html::parse_document(body.as_str());
        bug_update_page_token = get_page_token(&document, "bug_update_page_token")?;
        bug_info = my_view_detail_data(&document, &host, &catgory_kv, &project_kv)?;
    }
    // bug修改页面
    let body = bug_update_page(
        jar.clone(),
        UpdateToken {
            bug_id,
            bug_update_page_token,
        },
        &host,
    )
    .await?;
    let bug_update_token =
        get_page_token(&Html::parse_document(body.as_str()), "bug_update_token")?;
    // 提交bug
    let bug = UpdateBug {
        bug_update_token,
        bug_id,
        last_updated: bug_info.last_updated_sec,
        category_id: if category_id == 0 {
            bug_info.category_id
        } else {
            category_id
        },
        view_state: bug_info.view_state,
        handler_id: if handler_id == 0 {
            bug_info.handler_id
        } else {
            handler_id
        },
        priority: bug_info.priority,
        severity: if severity == 0 {
            bug_info.severity
        } else {
            severity
        },
        reproducibility: bug_info.reproducibility,
        status: if status == 0 { bug_info.status } else { status },
        resolution: if resolution == 0 {
            bug_info.resolution
        } else {
            resolution
        },
        summary: if summary == "" {
            bug_info.summary
        } else {
            summary
        },
        description: if description == "" {
            bug_info.description
        } else {
            description
        },
        additional_information: bug_info.additional_information,
        steps_to_reproduce: if steps_to_reproduce == "" {
            bug_info.steps_to_reproduce
        } else {
            steps_to_reproduce
        },
        bugnote_text: "".to_string(),
    };
    println!("{:?}", bug);
    let resp = bug_update(jar.clone(), bug, &host).await?;
    if let Some(s) = get_error_info(&Html::parse_document(resp.as_str())) {
        println_cookies(&jar, &host);
        return Err(s);
    };

    // 更新订阅数据
    update_sub_data(app).await?;
    Ok("更新成功".to_string())
}

// 保存note
#[tauri::command(rename_all = "snake_case")]
async fn api_bug_note_add(
    app: AppHandle,
    bug_id: i64,
    bugnote_text: String,
    file_path: Vec<String>,
    binary_file: Vec<(String, Vec<u8>)>,
) -> Result<(), String> {
    let state = app.state::<MyState>().clone();
    let user = state
        .user
        .lock()
        .map_err(|e| format!("lock err:{}", e))?
        .clone();
    let jar = state
        .jar
        .lock()
        .map_err(|e| format!("lock err:{}", e))?
        .clone();
    let host = state
        .host
        .lock()
        .map_err(|e| format!("lock err:{}", e))?
        .clone();
    if !user.logined {
        return Err("未登录".to_string());
    }

    // 查询bug详情
    let bugnote_add_token;
    {
        let body = my_view_detail(jar.clone(), bug_id, &host).await?;
        let document = Html::parse_document(body.as_str());
        bugnote_add_token = get_page_token(&document, "bugnote_add_token")?;
    }
    // 保存note
    let resp = bug_note_add(
        jar.clone(),
        BugNoteAdd {
            bug_id,
            bugnote_add_token,
            bugnote_text,
            max_file_size: 2097152, // 默认值
            file_path,
            binary_file,
        },
        &host,
    )
    .await?;

    if let Some(s) = get_error_info(&Html::parse_document(resp.as_str())) {
        println_cookies(&jar, &host);
        return Err(s);
    };

    Ok(())
}

// 保存note
#[tauri::command(rename_all = "snake_case")]
async fn api_bug_report(
    app: AppHandle,
    project_id: String,
    category_id: i64,
    reproducibility: i64,
    severity: i64,
    priority: i64,
    handler_id: i64,
    summary: String,
    description: String,
    steps_to_reproduce: String,
    file_path: Vec<String>,
    binary_file: Vec<(String, Vec<u8>)>,
) -> Result<(), String> {
    let state = app.state::<MyState>().clone();
    let user = state
        .user
        .lock()
        .map_err(|e| format!("lock err:{}", e))?
        .clone();
    let jar = state
        .jar
        .lock()
        .map_err(|e| format!("lock err:{}", e))?
        .clone();
    let host = state
        .host
        .lock()
        .map_err(|e| format!("lock err:{}", e))?
        .clone();
    if !user.logined {
        return Err("未登录".to_string());
    }

    // 查询bug详情
    let bug_report_token;
    {
        let body = bug_report_page(jar.clone(), &host, project_id.as_str()).await?;
        let document = Html::parse_document(body.as_str());
        bug_report_token = get_page_token(&document, "bug_report_token")?;
    }

    let pid = project_id
        .split(';')
        .last()
        .unwrap_or("0")
        .parse::<i64>()
        .unwrap_or(0);
    // 保存note
    let resp = bug_report(
        jar.clone(),
        ReportBug {
            bug_report_token,
            project_id: pid,
            category_id,
            reproducibility,
            severity,
            priority,
            handler_id,
            summary,
            description,
            steps_to_reproduce,
            view_state: 10,         // 默认值
            max_file_size: 2097152, // 默认值
            file_path,
            binary_file,
            ..ReportBug::default()
        },
        &host,
    )
    .await?;

    if let Some(s) = get_error_info(&Html::parse_document(resp.as_str())) {
        println_cookies(&jar, &host);
        return Err(s);
    };

    Ok(())
}

// 下载图片
#[tauri::command(rename_all = "snake_case")]
async fn api_image_bytes(app: AppHandle, uri: String) -> Result<Vec<u8>, String> {
    let state = app.state::<MyState>().clone();
    let jar = state
        .jar
        .lock()
        .map_err(|e| format!("lock err:{}", e))?
        .clone();
    let host = state
        .host
        .lock()
        .map_err(|e| format!("lock err:{}", e))?
        .clone();
    let bts = image_bytes(jar, host.as_str(), uri.as_str()).await?;
    Ok(bts)
}

// 检查更新
#[tauri::command(rename_all = "snake_case")]
async fn api_check_update(
    app: tauri::AppHandle,
) -> tauri_plugin_updater::Result<Option<VersionInfo>> {
    check_update(app.clone()).await
}

// 下载更新
#[tauri::command(rename_all = "snake_case")]
async fn api_download_and_install(app: tauri::AppHandle) -> tauri_plugin_updater::Result<()> {
    download_and_install(app.clone()).await
}

#[tauri::command(rename_all = "snake_case")]
async fn api_set_message_notify(window: tauri::Window, count: i64) -> Result<(), String> {
    set_message_notify(window, count).await
}

//初始化全局状态
fn init_global_state(app: AppHandle) -> Result<(), String> {
    let state = app.state::<MyState>().clone();
    let store = app.store("settings.json").map_err(|e| format!("{}", e))?;

    let user_value = store.get("user").unwrap_or_default();
    let cookie_value = store.get("cookies").unwrap_or_default();
    let default_host = Value::from("127.0.0.1:8989");
    let host_value = store
        .get("host")
        .map(|e| {
            if e.as_str().unwrap_or_default() == "" {
                return default_host.clone();
            }
            e
        })
        .unwrap_or(default_host);
    println!(
        "user:{};cookie:{};host:{}",
        user_value, cookie_value, host_value
    );

    let sub_params_value = store.get("sub_param").unwrap_or(Value::from(vec![
        r"type=1&view_type=simple&reporter_id[]=-1&handler_id[]=0&monitor_user_id[]=0&note_user_id[]=0&priority[]=0&severity[]=0&view_state=0&sticky=1&category_id[]=0&hide_status[]=90&status[]=0&resolution[]=0&profile_id[]=0&platform[]=0&os[]=0&os_build[]=0&relationship_type=-1&relationship_bug=0&tag_string=&per_page=999&sort[]=last_updated&dir[]=DESC&sort[]=last_updated&dir[]=DESC&sort[]=status&dir[]=ASC&match_type=0&highlight_changed=0&search=&filter_submit=应用过滤器",
        r"type=1&view_type=simple&reporter_id[]=0&handler_id[]=-1&monitor_user_id[]=0&note_user_id[]=0&priority[]=0&severity[]=0&view_state=0&sticky=1&category_id[]=0&hide_status[]=90&status[]=0&resolution[]=0&profile_id[]=0&platform[]=0&os[]=0&os_build[]=0&relationship_type=-1&relationship_bug=0&tag_string=&per_page=999&sort[]=last_updated&dir[]=DESC&sort[]=last_updated&dir[]=DESC&sort[]=status&dir[]=ASC&match_type=0&highlight_changed=0&search=&filter_submit=应用过滤器",
    ]));
    let sub_bugs_value = store.get("sub_bugs").unwrap_or_default();
    let change_historys_value = store.get("change_historys").unwrap_or_default();

    let mut user = state.user.lock().map_err(|e| format!("lock err:{}", e))?;
    *user = serde_json::from_value(user_value)
        .map_err(|e| println!("serde_json err:{}", e))
        .unwrap_or_default();

    let mut jar_ = state.jar.lock().map_err(|e| format!("lock err:{}", e))?;
    let jar = Jar::default();
    let url = format!("http://{}", host_value.as_str().unwrap_or_default())
        .parse::<Url>()
        .map_err(|e| format!("url parse err:{}", e))?;
    cookie_value.as_str().and_then(|s| {
        s.split(';').for_each(|c| {
            if !c.is_empty() {
                jar.add_cookie_str(c.trim(), &url);
            }
        });
        Some(())
    });

    *jar_ = Arc::new(jar);

    let mut host = state.host.lock().map_err(|e| format!("lock err:{}", e))?;
    *host = host_value.as_str().unwrap_or_default().to_string();

    let mut sub_params = state
        .sub_params
        .lock()
        .map_err(|e| format!("lock err:{}", e))?;
    *sub_params = serde_json::from_value(sub_params_value)
        .map_err(|e| println!("serde_json err:{}", e))
        .unwrap_or_default();

    let mut sub_bugs = state
        .sub_bugs
        .lock()
        .map_err(|e| format!("lock err:{}", e))?;
    *sub_bugs = serde_json::from_value(sub_bugs_value)
        .map_err(|e| println!("serde_json err:{}", e))
        .unwrap_or_default();

    let mut change_historys = state
        .change_historys
        .lock()
        .map_err(|e| format!("lock err:{}", e))?;
    *change_historys = serde_json::from_value(change_historys_value)
        .map_err(|e| println!("serde_json err:{}", e))
        .unwrap_or_default();

    info!("init_global_state success!");
    Ok(())
}

fn sync_init_project_catgory(app: AppHandle) -> Result<(), String> {
    // 开启线程同步执行异步函数
    block_on(init_project_catgory(app))
}

// 初始化分组和项目
async fn init_project_catgory(app: AppHandle) -> Result<(), String> {
    let state = app.state::<MyState>().clone();

    let user = state
        .user
        .lock()
        .map_err(|e| format!("lock err:{}", e))?
        .clone();

    let mut projects = vec![];
    let mut category = vec![];
    let mut users = vec![];
    // 如果当前是已登录，则获取分组和项目
    if user.logined {
        // 查询项目列表
        {
            let jar_ = state
                .jar
                .lock()
                .map_err(|e| format!("lock err:{}", e))?
                .clone();
            let host = state
                .host
                .lock()
                .map_err(|e| format!("lock err:{}", e))?
                .clone();
            let body = my_view_page(jar_, &host)
                .await
                .map_err(|e| format!("my_view_page err:{}", e))?;
            projects = project_data(&Html::parse_document(body.as_str()))?;
            let mut project_kv = state
                .project_kv
                .lock()
                .map_err(|e| format!("lock err:{}", e))?;
            *project_kv = projects.clone();
        }

        // 查询分组列表
        {
            let jar_ = state
                .jar
                .lock()
                .map_err(|e| format!("lock err:{}", e))?
                .clone();
            let host = state
                .host
                .lock()
                .map_err(|e| format!("lock err:{}", e))?
                .clone();
            let project_id = projects
                .last()
                .ok_or("projects error".to_owned())?
                .key
                .clone();
            let body = bug_report_page(jar_, &host, &project_id)
                .await
                .map_err(|e| format!("filters_params err:{}", e))?;
            let dc = Html::parse_document(body.as_str());
            category = category_data(&dc)?;
            let mut catgory_kv = state
                .catgory_kv
                .lock()
                .map_err(|e| format!("lock err:{}", e))?;
            *catgory_kv = category.clone();

            // 查询用户列表
            users = users_data(&dc)?;
            let mut users_kv = state
                .users_kv
                .lock()
                .map_err(|e| format!("lock err:{}", e))?;
            *users_kv = users.clone();
        }
        if category.len() == 0 || projects.len() == 0 || users.len() == 0 {
            return Err("".to_string());
        }
    } else {
        let mut project_kv = state
            .project_kv
            .lock()
            .map_err(|e| format!("lock err:{}", e))?;
        *project_kv = projects;
        let mut catgory_kv = state
            .catgory_kv
            .lock()
            .map_err(|e| format!("lock err:{}", e))?;
        *catgory_kv = category;
        let mut users_kv = state
            .users_kv
            .lock()
            .map_err(|e| format!("lock err:{}", e))?;
        *users_kv = users;
    };
    Ok(())
}

//保存全局状态
fn save_global_state(app: AppHandle) -> Result<(), String> {
    let state = app.state::<MyState>().clone();
    let store = app.store("settings.json").map_err(|e| format!("{}", e))?;

    let user = state.user.lock().map_err(|e| format!("lock err:{}", e))?;
    let json_value = serde_json::to_value(user.clone()).map_err(|e| format!("serde err:{}", e))?;
    store.set("user", json_value);

    let host = state.host.lock().map_err(|e| format!("lock err:{}", e))?;
    store.set("host", Value::from(host.clone()));

    let jar = state.jar.lock().map_err(|e| format!("lock err:{}", e))?;
    let url = format!("http://{}", host.clone())
        .parse::<Url>()
        .map_err(|e| format!("parse err:{}", e))?;
    let cookies = jar
        .cookies(&url)
        .map(|h| h.to_str().unwrap_or("").to_string())
        .unwrap_or_default();
    store.set("cookies", Value::from(cookies));

    let sub_params = state
        .sub_params
        .lock()
        .map_err(|e| format!("lock err:{}", e))?;
    store.set("sub_param", sub_params.clone());

    let sub_bugs = state
        .sub_bugs
        .lock()
        .map_err(|e| format!("lock err:{}", e))?;
    let json_value =
        serde_json::to_value(&sub_bugs.clone()).map_err(|e| format!("to json err:{}", e))?;
    store.set("sub_bugs", json_value);

    let change_historys = state
        .change_historys
        .lock()
        .map_err(|e| format!("lock err:{}", e))?;
    let json_value =
        serde_json::to_value(&change_historys.clone()).map_err(|e| format!("to json err:{}", e))?;
    store.set("change_historys", json_value);

    store.save().map_err(|e| format!("save err:{}", e))?;
    Ok(())
}

// 清空所有状态
fn clear_global_state(app: AppHandle) -> Result<(), String> {
    let state = app.state::<MyState>().clone();
    let store = app.store("settings.json").map_err(|e| format!("{}", e))?;

    let mut user = state.user.lock().map_err(|e| format!("lock err:{}", e))?;
    *user = LoginInfo::default();
    store.delete("user");

    // let mut host = state.host.lock().map_err(|e|format!("lock err:{}",e))?;
    // *host = "".to_string();
    // store.delete("host");

    let mut jar = state.jar.lock().map_err(|e| format!("lock err:{}", e))?;
    *jar = Arc::new(Jar::default());
    store.delete("cookies");

    let mut sub_params = state
        .sub_params
        .lock()
        .map_err(|e| format!("lock err:{}", e))?;
    *sub_params = vec![];
    store.delete("sub_param");

    let mut sub_bugs = state
        .sub_bugs
        .lock()
        .map_err(|e| format!("lock err:{}", e))?;
    *sub_bugs = Vec::default();
    store.delete("sub_bugs");

    let mut change_historys = state
        .change_historys
        .lock()
        .map_err(|e| format!("lock err:{}", e))?;
    *change_historys = Vec::default();
    store.delete("change_historys");

    store.save().map_err(|e| format!("store save err:{}", e))
}

// 定时查询订阅数据
fn find_sub_data(app: AppHandle) {
    // 启动一个异步定时任务
    tauri::async_runtime::spawn(async move {
        let mut ticker = interval(Duration::from_secs(5));
        loop {
            ticker.tick().await;
            let result = update_sub_data(app.clone()).await;
            if let Err(e) = result {
                info!("error:{}", e);
            }
        }
    });
}

// 更新一次用户订阅列表
async fn update_sub_data(app: AppHandle) -> Result<(), String> {
    let state = app.state::<MyState>().clone();
    // 判断是否登录，如果未登录，则跳过
    let user = state
        .user
        .lock()
        .map_err(|e| format!("lock err:{}", e))?
        .clone();
    if !user.logined {
        info!("未登录，跳过定时任务");
        return Ok(());
    };

    let jar = state
        .jar
        .lock()
        .map_err(|e| format!("lock err:{}", e))?
        .clone();
    let host = state
        .host
        .lock()
        .map_err(|e| format!("lock err:{}", e))?
        .clone();
    let project_kv = state
        .project_kv
        .lock()
        .map_err(|e| format!("lock err:{}", e))?
        .clone();
    let category_kv = state
        .catgory_kv
        .lock()
        .map_err(|e| format!("lock err:{}", e))?
        .clone();

    // 循环查询所有订阅的数据并且去重
    let mut bugs = Vec::new();
    let params = state
        .sub_params
        .lock()
        .map_err(|e| format!("lock err:{}", e))?
        .clone();
    for sub_param in params {
        let sub_param = sub_param.clone();
        // 查询列表
        let param = serde_html_form::from_str::<FindBugListParams>(&sub_param)
            .map_err(|e| format!("serde_html_form err:{}", e))?;
        let body = view_all_set(jar.clone(), param.clone(), "0", 1, &host)
            .await
            .map_err(|e| format!("view_all_set err:{}", e))?;
        // 解析数据
        let mut data = view_all_set_data(
            &Html::parse_document(body.as_str()),
            &category_kv,
            &project_kv,
        )
        .map_err(|e| format!("view_all_set_data err:{}", e))?;

        // 设置当前项目
        let project_id = param.project_id.as_str();
        if !(project_id == "0" || project_id == "") {
            if let Some(project) = project_kv.find_by_key(project_id) {
                data.bugs.iter_mut().for_each(|b| {
                    b.project = project.value.clone();
                });
            };
        }

        bugs.append(&mut data.bugs);
    }
    let mut seen = HashSet::new();
    bugs = bugs.into_iter().filter(|b| seen.insert(b.bug_id)).collect();
    // info!("find bugs: {:?}", bugs);

    // 只查询大于缓存时间的数据的详情，获取新增的日志信息
    let change_historys = state
        .change_historys
        .lock()
        .map_err(|e| format!("lock err:{}", e))?
        .clone();
    let last_history_time = change_historys.last().map_or(0, |c| c.updated_at);
    let last_history_begin = Utc
        .timestamp_opt(last_history_time, 0)
        .single()
        .map(|t| {
            t.with_timezone(&Shanghai)
                .with_time(NaiveTime::MIN)
                .unwrap()
                .timestamp()
        })
        .unwrap_or_default();
    let mut add_historys = Vec::new();
    for b in bugs.iter() {
        if b.last_updated >= last_history_begin {
            let text = my_view_detail(jar.clone(), b.bug_id, &host)
                .await
                .map_err(|e| format!("my_view_detail err:{}", e))?;
            let document = Html::parse_document(text.as_str());
            let bug_info = my_view_detail_data(&document, &host, &category_kv, &project_kv)?;
            for ch in bug_info.change_history {
                if ch.updated_at >= last_history_time {
                    if change_historys
                        .iter()
                        .any(|c| get_hash(&c) == get_hash(&ch))
                    {
                        continue;
                    } else {
                        add_historys.push(ch);
                    }
                }
            }
        }
    }
    add_historys.sort_by_key(|c| (c.updated_at, c.bug_id, c.handler_id));

    // bugs有变更则推送
    let mut old_bugs = state
        .sub_bugs
        .lock()
        .map_err(|e| format!("lock err:{}", e))?;
    if get_hash(&*old_bugs) != get_hash(&bugs) {
        let _ = app.emit("sub_bugs", &bugs);
        *old_bugs = bugs;
    }

    // change_historys有数据则推送增量
    let mut change_historys = state
        .change_historys
        .lock()
        .map_err(|e| format!("lock err:{}", e))?;
    if add_historys.len() > 0 {
        let _ = app.emit("sub_msgs", &add_historys);
        let mut notify: HashMap<String, (i64, String, String)> = HashMap::new();
        for c in add_historys.iter_mut() {
            change_historys.push(c.clone());

            let new_key = format!("{}-{}-{}", c.updated_at, c.bug_id, c.handler_id).to_string();
            if let Some((_handler_id, _title, content)) = notify.get_mut(&new_key) {
                let note = format!("\n- {} {}", c.field, c.change.replace("&gt;", ">"));
                content.push_str(&note);
            } else {
                let bug = old_bugs
                    .iter()
                    .find(|b| b.bug_id == c.bug_id)
                    .ok_or(String::from("not found bug"))?;
                let t = Utc
                    .timestamp_opt(c.updated_at, 0)
                    .single()
                    .unwrap_or_default()
                    .with_timezone(&Shanghai)
                    .format("%Y-%m-%d %H:%M");
                let note = format!("\n- {} {}", c.field, c.change.replace("&gt;", ">"));
                let title = format!("{} #{} （{}）", c.handler, c.bug_id, t);
                let content = format!("{} {}", bug.summary, note);
                notify.insert(new_key, (c.handler_id, title, content));
            }
        }

        // 通知
        notify.iter().for_each(|(_title, t)| {
            // 如果是自己操作的记录，则不通知
            if t.0 != user.user_id {
                // 发送通知
                let _ = send_notify(app.clone(), t.1.as_str(), t.2.as_str());
            }
        });
    }

    // 如何数据量过大，则只保留最近200条
    if change_historys.len() > 200 {
        let len = change_historys.len();
        change_historys.drain(0..(len - 200));
    }
    Ok(())
}

// 定时查询更新
fn update_app(app: tauri::AppHandle) {
    tauri::async_runtime::spawn(async move {
        //5分钟查询一次是否可更新
        let mut ticker = interval(Duration::from_secs(3600));
        loop {
            ticker.tick().await;
            let _ = check_update(app.clone()).await;
        }
    });
}

// 检查更新
async fn check_update(app: tauri::AppHandle) -> tauri_plugin_updater::Result<Option<VersionInfo>> {
    if let Some(update) = app.updater()?.check().await? {
        info!(
            "version_info: old:{} new:{} target:{}",
            update.current_version, update.version, update.target
        );

        // 获取更新版本信息
        let version_info = VersionInfo {
            current_version: update.current_version.clone(),
            version: update.version.clone(),
            target: update.target.clone(),
            update_time: update.date.map_or(0, |t| t.unix_timestamp()),
        };

        // 保存到全局状态
        let state = app.state::<MyState>().clone();
        let mut last_version = state
            .last_version
            .lock()
            .map_err(|_| tauri_plugin_updater::Error::ReleaseNotFound)?;
        *last_version = Some(update);

        // 通知前端
        app.emit("app-update", &version_info)?;
        Ok(Some(version_info))
    } else {
        // 通知前端
        app.emit("app-update-none", "")?;
        Ok(None)
    }
}

// 下载安装
async fn download_and_install(app: tauri::AppHandle) -> tauri_plugin_updater::Result<()> {
    let mut downloaded = 0;

    // 从全局状态拿数据
    let up: Option<Update>;
    {
        let state = app.state::<MyState>().clone();
        let last_version = state
            .last_version
            .lock()
            .map_err(|_| tauri_plugin_updater::Error::ReleaseNotFound)?;
        up = last_version.clone();
    }
    if let Some(update) = up {
        return update
            .download_and_install(
                |chunk_length, content_length| {
                    downloaded += chunk_length;
                    info!("downloaded {} from {:?}", downloaded, content_length);

                    // 发送下载进度给前端
                    let progress = if let Some(total) = content_length {
                        (downloaded as f64 / total as f64 * 100.0) as u32
                    } else {
                        0
                    };

                    let _ = app.emit("app-update-progress", progress);
                },
                || {
                    info!("download finished");
                    let _ = app.emit("app-update-download-finished", ());
                },
            )
            .await
            .or_else(|e| {
                let _ = app.emit("app-update-error", format!("{}", e));
                Err(e)
            });
    }
    Err(tauri_plugin_updater::Error::ReleaseNotFound)
}

// 发送通知
fn send_notify(app: AppHandle, title: &str, content: &str) -> Result<(), String> {
    // 检查通知权限
    let permission = app
        .notification()
        .permission_state()
        .map_err(|e| format!("获取权限失败: {}", e))?;
    if permission != PermissionState::Granted {
        return Err("通知权限未授予".to_string());
    }

    // 创建并发送通知
    let notification = app
        .notification()
        .builder()
        .title(title)
        .body(content)
        .icon("StoreLogo.png")
        .show()
        .map_err(|e| format!("发送通知失败: {}", e))?;
    Ok(notification)
}

// 关闭app回调
fn close_app_callback(app: AppHandle) {
    tauri::async_runtime::spawn(async move {
        signal::ctrl_c().await.expect("failed to listen for event");
        println!("收到 Ctrl+C，执行清理操作...");
        let r = save_global_state(app.clone());
        match r {
            Ok(_) => println!("handle exec ok"),
            Err(msg) => println!("handle exec err: {}", msg),
        }
        app.exit(0);
    });
}

// 监听键盘事件
fn listen_keybord_event(app: AppHandle) -> Result<(), String> {
    thread::Builder::new()
        .name("rdev-listener".into())
        .spawn(move || {
            // rdev::listen 是阻塞的，放在独立线程
            if let Err(err) = listen(move |event| {
                match event.name {
                    Some(name) => {
                        // 向前端广播事件
                        let _ = app.emit("global-keyboard-event", name);
                    }
                    None => {
                        // 获取鼠标位置
                        if let EventType::MouseMove{x,y} = event.event_type {
                            // info!("x:{},y:{}", x, y)
                            let _ = app.emit("mouse-move-event", (x, y));
                        }
                    }
                    _ => { /* 其他按键事件 */}
                }
            }) {
                warn!("Keyboard listener error: {:?}", err);
            }
        })
        .map_err(|e| format!("spawn failed: {}", e))?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use scraper::Html;

    #[tokio::test]
    async fn test_login() {
        let host = "bug.test.com";
        let result = login(
            Arc::new(Jar::default()),
            "dengxiangcheng",
            "dxc3434DXC",
            host,
        )
        .await;
        assert!(result.is_ok());
        if let Ok(text) = result {
            info!("text: {}", text);
        }
    }

    #[tokio::test]
    async fn test_serde_html_form() {
        let sub_param = r"type=1&view_type=simple&reporter_id[]=0&handler_id[]=-1&monitor_user_id[]=0&note_user_id[]=0&priority[]=0&severity[]=0&view_state=0&sticky=1&category_id[]=0&hide_status[]=90&status[]=0&resolution[]=0&profile_id[]=0&platform[]=0&os[]=0&os_build[]=0&relationship_type=-1&relationship_bug=0&tag_string=&per_page=999&sort[]=last_updated&dir[]=DESC&sort[]=date_submitted&dir[]=DESC&sort[]=status&dir[]=ASC&match_type=0&highlight_changed=12&search=&filter_submit=应用过滤器";
        let param = serde_html_form::from_str::<FindBugListParams>(&sub_param)
            .map_err(|e| {
                println!("serde_html_form err: {}", e);
                format!("serde_html_form err:{}", e)
            })
            .map(|a| {
                println!("serde_html_form ok: {:?}", a);
                a
            });
        assert!(param.is_ok());
    }

    #[tokio::test]
    async fn test_view_all_set_data() {
        // // 读取view_all_set.html文件内容
        // let html_content = include_str!("./example/view_all_set.html");
        // let document = Html::parse_document(html_content);
        // let r = view_all_set_data(&document);
        // assert!(r.is_ok());
        // let data = r.unwrap();
        // println!("Summary: {:?}", data);
    }

    #[tokio::test]
    async fn test_my_view_detail_data() {
        // let host = "bug.test.com";
        // // 读取view_all_set.html文件内容
        // let html_content = include_str!("./example/view.php.html");
        // let document = Html::parse_document(html_content);
        // let r = my_view_detail_data(&document,host);
        // assert!(r.is_ok());
        // let data = r.unwrap();
        // println!("Summary: {:?}", data);
    }

    #[tokio::test]
    async fn test_bug_update_token_data() {
        // 读取view_all_set.html文件内容
        let html_content = include_str!("./example/bug_update_page.php.html");
        let document = Html::parse_document(html_content);
        let result = get_page_token(&document, "bug_update_token");
        assert!(result.is_ok());
        let data = result.unwrap();
        println!("Summary: {:?}", data);
    }
}
