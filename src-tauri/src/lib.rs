mod enums;
mod model;
mod utils;

use enums::*;
use url::Url;
use utils::*;
use model::*;
use chrono::Utc;
use env_logger;
use log::{debug, info, warn};
use reqwest::cookie::Jar;
use scraper::Html;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use tauri::{AppHandle, Emitter, Listener, Manager};
use tauri_plugin_notification::{NotificationExt, PermissionState};
use tokio::time::{interval, Duration};
use tauri_plugin_updater::UpdaterExt;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    // 初始化日志实现库
    env_logger::Builder::new()
        .filter_level(log::LevelFilter::Info) // 设置日志级别为 Debug
        .init();
    tauri::Builder::default()
        .plugin(tauri_plugin_updater::Builder::new().build())
        .plugin(tauri_plugin_notification::init())
        .manage(Mutex::new(MyState::default())) // 注册全局状态
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            api_init_data,
            api_login,
            api_logout,
            api_bug_list,
            api_update_bug,
            api_version_update
        ])
        .setup(|app| {
            let handle1 = app.handle().clone();
            let handle2 = app.handle().clone();
            //是否更新
            update_app(handle1);
            //启动定时任务
            start_timer(handle2);
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

// 全局状态
#[derive(Default)]
struct MyState {
    logined: Arc<bool>, // 是否登录
    jar: Arc<Jar>,
    last_bugs: Arc<Mutex<HashMap<i64, Bug>>>, //bugs列表
    data_hash: Arc<u64>,
}

// 初始化数据
#[tauri::command(rename_all = "snake_case")]
fn api_init_data(_app: AppHandle) -> String {
    let mut hm = HashMap::new();
    hm.insert("Priority", Priority::kv());
    hm.insert("Severity", Severity::kv());
    hm.insert("Reproducibility", Reproducibility::kv());
    hm.insert("ViewStatus", ViewStatus::kv());
    hm.insert("Category", Category::kv());
    hm.insert("Project", Project::kv());
    hm.insert("Status", Status::kv());
    hm.insert("Resolution", Resolution::kv());
    serde_json::to_string(&hm).unwrap_or_else(|_| "{}".to_string())
}

// 登录
#[tauri::command(rename_all = "snake_case")]
async fn api_login(app: AppHandle, username: &str, password: &str) -> Result<String, String> {
    let jar = Arc::new(Jar::default());
    let s = login(jar.clone(), username, password).await;
    match s {
        Ok(_) => {
            // 保存cookie到全局状态
            let state = app.state::<Mutex<MyState>>();
            let mut my_state = state.lock().or(Err("获取全局状态失败".to_string()))?;
            my_state.jar = jar.clone();
            my_state.logined = Arc::new(true);
            return Ok("登录成功".to_string());
        }
        Err(e) => {
            return Err(format!("登录失败: {}", e));
        }
    }
}

// 退出登录
#[tauri::command(rename_all = "snake_case")]
async fn api_logout(app: AppHandle) -> Result<(), String> {
    let state = app.state::<Mutex<MyState>>();
    let mut my_state = state.lock().or(Err("获取全局状态失败".to_string()))?;
    // 清除cookie
    my_state.jar = Arc::new(Jar::default());
    my_state.logined = Arc::new(false);
    Ok(())
}

// bug列表
#[tauri::command(rename_all = "snake_case")]
async fn api_bug_list(app: AppHandle) -> Result<BugList, String> {
    let (logined, jar) = {
        let state = app.state::<Mutex<MyState>>();
        let my_state = state.lock().map_err(|e| format!("error:{}", e))?;

        // my_state.logined = Arc::new(true); // 模拟登录状态
        (my_state.logined.clone(), my_state.jar.clone())
    };
    // let body = include_str!("view_all_set.html").to_string(); //模拟查询数据
    if !*logined {
        return Err("未登录".to_string());
    }
    let param_str = r"type=1&view_type=simple&reporter_id[]=0&handler_id[]=-1&monitor_user_id[]=0&note_user_id[]=0&priority[]=0&severity[]=0&view_state=0&sticky=1&category_id[]=0&hide_status[]=90&status[]=0&resolution[]=0&profile_id[]=0&platform[]=0&os[]=0&os_build[]=0&relationship_type=-1&relationship_bug=0&tag_string=&per_page=50&sort[]=date_submitted&dir[]=DESC&sort[]=status&dir[]=ASC&sort[]=last_updated&dir[]=DESC&match_type=0&highlight_changed=6&search=&filter_submit=应用过滤器";
    let param = serde_html_form::from_str::<FindBugListParams>(param_str)
        .map_err(|e| format!("serde_html_form err:{}", e))?;
    // 查询列表
    let body = view_all_set(jar.clone(), param.clone()).await.map_err(|e|format!("view_all_set err:{}",e))?;
    // 解析数据
    let data = view_all_set_data(&Html::parse_document(body.as_str()))
        .map_err(|e| format!("view_all_set_data err:{}", e))?;

    Ok(data)
}

// 修改bug
#[tauri::command(rename_all = "snake_case")]
async fn api_update_bug(
    app: AppHandle,
    bug_id: i64,
    status: i64,
    resolution: i64,
) -> Result<String, String> {
    let (logined, jar) = {
        let state = app.state::<Mutex<MyState>>();
        let my_state = state.lock().map_err(|e| format!("error:{}", e))?;
        (my_state.logined.clone(), my_state.jar.clone())
    };
    if !*logined {
        return Err("未登录".to_string());
    }
    // 查询bug详情
    let bug_update_token;
    let bug_info;
    {
        let body = my_view_detail(jar.clone(), bug_id).await?;
        let document = Html::parse_document(body.as_str());
        println!("body{}",body);
        bug_update_token = bug_update_token_data(&document)?;
        bug_info = my_view_detail_data(&document)?;
    }
    // bug修改页面
    let body = bug_update_page(
        jar.clone(),
        UpdateToken {
            bug_id,
            bug_update_token,
        },
    )
    .await?;
    println!("body:{}",body);
    let bug_update_token = bug_update_token_data(&Html::parse_document(body.as_str()))?;//bug
    println!("bugtoken:{}",bug_update_token);
    // 提交bug
    let now = Utc::now();
    let bug = UpdateBug {
        bug_update_token,
        bug_id,
        last_updated: now.timestamp(),
        category_id: bug_info.category_id,
        view_state: bug_info.view_state,
        handler_id: bug_info.handler_id,
        priority: bug_info.priority,
        severity: bug_info.severity,
        reproducibility: bug_info.reproducibility,
        status: status,
        resolution: resolution,
        summary: bug_info.summary,
        description: bug_info.description,
        additional_information: "".to_string(),
        steps_to_reproduce: "".to_string(),
        bugnote_text: "".to_string(),
    };
    bug_update(jar.clone(), bug).await
}

// 更新应用接口
#[tauri::command(rename_all = "snake_case")]
async fn api_version_update(app: tauri::AppHandle) -> tauri_plugin_updater::Result<String> {
    version_update(app).await
}

// 定时任务
fn start_timer(app: AppHandle) {
    // 启动一个异步定时任务
    tauri::async_runtime::spawn(async move {
        let state = app.state::<Mutex<MyState>>();
        let mut ticker = interval(Duration::from_secs(5));
        // let param = FindBugListParams::default();
        let param_str = r"type=1&view_type=simple&reporter_id[]=0&handler_id[]=-1&monitor_user_id[]=0&note_user_id[]=0&priority[]=0&severity[]=0&view_state=0&sticky=1&category_id[]=0&hide_status[]=90&status[]=0&resolution[]=0&profile_id[]=0&platform[]=0&os[]=0&os_build[]=0&relationship_type=-1&relationship_bug=0&tag_string=&per_page=50&sort[]=date_submitted&dir[]=DESC&sort[]=status&dir[]=ASC&sort[]=last_updated&dir[]=DESC&match_type=0&highlight_changed=6&search=&filter_submit=应用过滤器";
        let result = serde_html_form::from_str::<FindBugListParams>(param_str)
            .map_err(|e| info!("serde_html_form err:{}", e));
        let param = result.unwrap_or_default();
        loop {
            ticker.tick().await;
            // 判断是否登录
            let (logined, jar) = {
                if let Ok(my_state) = state.lock() {
                    (my_state.logined.clone(), my_state.jar.clone())
                } else {
                    continue;
                }
            };
            // 如果未登录，则跳过
            if !*logined {
                info!("未登录，跳过定时任务");
                continue;
            }
            // 查询列表
            let body = match view_all_set(jar.clone(), param.clone()).await {
                Ok(b) => b,
                Err(e) => {
                    info!("查询列表失败: {}", e);
                    continue;
                }
            };
            // let body = include_str!("view_all_set.html").to_string();//test
            // 解析数据
            let data = match view_all_set_data(&Html::parse_document(body.as_str())) {
                Ok(d) => d,
                Err(e) => {
                    info!("解析数据失败: {}", e);
                    continue;
                }
            };
            // 把data转成string
            let resp = match serde_json::to_string(&data) {
                Ok(s) => s,
                Err(e) => {
                    info!("序列化数据失败: {}", e);
                    continue;
                }
            };
            // 把最新的bug放进全局状态
            if let Ok(my_state) = state.lock() {
                // 如果和上条一样则不更新
                if *(my_state.data_hash.clone()) == get_hash(&resp) {
                    info!("相同数据");
                    continue;
                }
                let last_bugs = my_state.last_bugs.clone();
                let result = last_bugs.lock();
                if let Ok(mut old_map) = result {
                    // info!("{:?}",*old_map);
                    let mut new_map = HashMap::new();
                    for b in data.bugs {
                        // 判断旧的列表中是否有这个bug,如果没有则发送通知
                        if old_map.iter().all(|(bgid, bg)| {
                            (*bgid != b.bug_id) || (*bgid == b.bug_id && bg.status != b.status)
                        }) {
                            if matches!(
                                Status::from(b.status),
                                Status::New
                                    | Status::Feedback
                                    | Status::Acknowledged
                                    | Status::Confirmed
                                    | Status::Assigned
                            ) {
                                let _ = send_notify(
                                    app.clone(),
                                    format!(
                                        " {} | {} - {} -> {}",
                                        b.project,
                                        Category::from(b.category_id).as_str(),
                                        Severity::from(b.severity).as_str(),
                                        b.handler
                                    )
                                    .as_str(),
                                    b.summary.as_str(),
                                );
                            }
                        };
                        new_map.insert(b.bug_id, b);
                    }
                    *old_map = new_map;
                }
            } else {
                continue;
            };
            // 向前端所有窗口广播消息
            let _ = app.emit_str("timer-tick", resp);
        }
    });
}

// 定时查询更新
fn update_app(app: tauri::AppHandle) {
    tauri::async_runtime::spawn(async move {
        //5分钟查询一次是否可更新
        let mut ticker = interval(Duration::from_secs(60));
        loop {
            ticker.tick().await;
            let _ = version_update(app.clone()).await;
        }
    });
}

// 检查更新
async fn version_update(app: tauri::AppHandle) -> tauri_plugin_updater::Result<String> {
    if let Some(update) = app
    .updater()?
    .check()
    .await?
    {
        // 获取更新版本信息
        let version_info = VersionInfo {
            current_version: update.current_version.clone(),
            version: update.version.clone(),
            target: update.target.clone(),
        };
        
        let version_info_json = serde_json::to_string(&version_info)
            .unwrap_or_else(|_| "{}".to_string());
        
        info!("version_info: {}", version_info_json);

        // 通知前端
        app.emit("app-update", version_info)?;

        // 克隆必要的数据给闭包使用
        let app_clone = app.clone();
        let update_arc = std::sync::Arc::new(std::sync::Mutex::new(Some(update)));
        
        // 监听前端确认
        app.listen("app-update-result", move |event| {
            let payload = event.payload();
            if payload == "ok" {
                let app_handle = app_clone.clone();
                let update_mutex = update_arc.clone();
                
                // 异步处理下载和安装
                tauri::async_runtime::spawn(async move {
                    // 从Arc<Mutex<Option<Update>>>中取出update
                    let update = {
                        let mut guard = update_mutex.lock().unwrap();
                        guard.take()
                    };
                    
                    if let Some(update) = update {
                        match download_and_install_update(app_handle.clone(), update).await {
                            Ok(_) => {
                                info!("update installed successfully");
                                app_handle.restart();
                            }
                            Err(e) => {
                                warn!("update failed: {}", e);
                                // 通知前端更新失败
                                let _ = app_handle.emit("app-update-error", format!("更新失败: {}", e));
                            }
                        }
                    }
                });
            }
        });
        Ok("updating".to_string())
    }else {
        info!("Update API does not require an update.");
        Ok("latest version".to_string())
    }
}

// 单独的下载和安装函数
async fn download_and_install_update(
    app: tauri::AppHandle,
    update: tauri_plugin_updater::Update,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let mut downloaded = 0;
    
    update.download_and_install(
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
    .map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send + Sync>)
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
        .icon("icons/icon.ico")
        .show()
        .map_err(|e| format!("发送通知失败: {}", e))?;
    Ok(notification)
}

#[cfg(test)]
mod tests {
    use super::*;
    use scraper::Html;

    #[tokio::test]
    async fn test_login() {
        let result = login(Arc::new(Jar::default()), "dengxiangcheng", "dxc3434DXC").await;
        assert!(result.is_ok());
        if let Ok(text) = result {
            info!("text: {}", text);
        }
    }

    #[tokio::test]
    async fn test_my_view_page() {
        let jar = Arc::new(Jar::default());
        let result = login(jar.clone(), "dengxiangcheng", "dxc3434DXC").await;
        assert!(result.is_ok(), "Login failed");
        let ck = find_jar_cookies(&jar, "MANTIS_STRING_COOKIE").unwrap();
        info!("MANTIS_STRING_COOKIE: {}", ck);
        let result = my_view_page(jar.clone()).await;
        assert!(result.is_ok());
        let body = result.unwrap();
        find_all_tasks(body.as_str(), "#resolved .widget-body .my-buglist-bug td a")
            .unwrap()
            .iter()
            .for_each(|task| {
                info!("task: {}", task);
            });
    }

    #[tokio::test]
    async fn test_view_all_set() {
        let jar = Arc::new(Jar::default());
        let result = login(jar.clone(), "dengxiangcheng", "dxc3434DXC").await;
        assert!(result.is_ok(), "Login failed");

        let param_str = r"type=1&view_type=simple&reporter_id[]=0&handler_id[]=29&monitor_user_id[]=0&note_user_id[]=0&priority[]=0&severity[]=0&view_state=0&sticky=1&category_id[]=0&hide_status[]=-2&status[]=0&resolution[]=0&filter_by_last_updated_date=1&last_updated_start_month=6&last_updated_start_day=1&last_updated_start_year=2025&last_updated_end_month=6&last_updated_end_day=20&last_updated_end_year=2025&profile_id[]=0&platform[]=0&os[]=0&os_build[]=0&relationship_type=-1&relationship_bug=0&tag_string=&per_page=50&sort[]=last_updated&dir[]=DESC&match_type=0&highlight_changed=6&search=&filter_submit=应用过滤器";
        let result = serde_html_form::from_str::<FindBugListParams>(param_str);
        assert!(result.is_ok());
        let result = view_all_set(jar, result.unwrap()).await;
        assert!(result.is_ok());
        let body = result.unwrap();
        // info!("body: {}", body);

        let r = view_all_set_data(&Html::parse_document(body.as_str()));
        let data = r.unwrap();
        for a in data.bugs {
            info!("Summary: {:?}", a);
        }
        info!("Total: {}", data.total);
        info!("Page: {}", data.page);
        info!("Limit: {}", data.limit);
    }

    #[tokio::test]
    async fn test_view_all_set_data() {
        // 读取view_all_set.html文件内容
        let html_content = include_str!("view_all_set.html");
        let document = Html::parse_document(html_content);
        let r = view_all_set_data(&document);
        assert!(r.is_ok());
        let data = r.unwrap();
        info!("Summary: {:?}", data);
    }

    #[tokio::test]
    async fn test_my_view_detail_data() {
        // 读取view_all_set.html文件内容
        let html_content = include_str!("view.php.html");
        let document = Html::parse_document(html_content);
        let r = my_view_detail_data(&document);
        assert!(r.is_ok());
        let data = r.unwrap();
        info!("Summary: {:?}", data);
    }
}
