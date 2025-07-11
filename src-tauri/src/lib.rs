mod enums;
mod model;
mod utils;

use enums::*;
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
use tauri_plugin_updater::{Update, UpdaterExt};

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
            api_check_update,
            api_download_and_install
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
    last_version: Arc<Option<Update>>,
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
    let bug_update_page_token;
    let bug_info;
    {
        let body = my_view_detail(jar.clone(), bug_id).await?;
        let document = Html::parse_document(body.as_str());
        bug_update_page_token = get_page_token(&document,"bug_update_page_token")?;
        bug_info = my_view_detail_data(&document)?;
    }
    // bug修改页面
    let body = bug_update_page(
        jar.clone(),
        UpdateToken {
            bug_id,
            bug_update_page_token,
        },
    )
    .await?;
    let bug_update_token = get_page_token(&Html::parse_document(body.as_str()),"bug_update_token")?;
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

// 检查更新
#[tauri::command(rename_all = "snake_case")]
async fn api_check_update(app: tauri::AppHandle) -> tauri_plugin_updater::Result<()> {
    check_update(app.clone()).await
}

// 下载更新
#[tauri::command(rename_all = "snake_case")]
async fn api_download_and_install(app: tauri::AppHandle) -> tauri_plugin_updater::Result<()> {
    download_and_install(app.clone()).await
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
            let _ = check_update(app.clone()).await;
        }
    });
}

// 检查更新
async fn check_update(app: tauri::AppHandle) -> tauri_plugin_updater::Result<()> {
    if let Some(update) = app
    .updater()?
    .check()
    .await?
    {
        info!("version_info: old:{} new:{} target:{}", update.current_version,update.version,update.target);

        // 获取更新版本信息
        let version_info = VersionInfo {
            current_version: update.current_version.clone(),
            version: update.version.clone(),
            target: update.target.clone(),
        };

        // 保存到全局状态
        let state = app.state::<Mutex<MyState>>();
        let mut my_state = state.lock().map_err(|_| tauri_plugin_updater::Error::Network("保存到全局状态失败".to_string()))?;
        my_state.last_version = Arc::new(Some(update));

        // 通知前端
        app.emit("app-update", version_info)?;
        Ok(())
    }else {
        // 通知前端
        app.emit("app-update-none", "")?;
        Ok(())
    }
}

// 下载安装
async fn download_and_install(app: tauri::AppHandle) -> tauri_plugin_updater::Result<()> {
    let mut downloaded = 0;

    // 从全局状态拿数据
    let up: Option<Update>;
    {
        let state = app.state::<Mutex<MyState>>();
        let my_state = state.lock().map_err(|_| tauri_plugin_updater::Error::ReleaseNotFound)?;
        let last_version: Arc<Option<Update>> = my_state.last_version.clone();
        up = last_version.as_ref().clone();
    }
    if let Some(update) = up {
        return update.download_and_install(
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
        ).await.or_else(|e|{
            let _ = app.emit("app-update-error", format!("{}",e));
            Err(e)
        })
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
        // println!("body: {}", body);

        let r = view_all_set_data(&Html::parse_document(body.as_str()));
        let data = r.unwrap();
        for a in data.bugs {
            println!("Summary: {:?}", a);
        }
        println!("Total: {}", data.total);
        println!("Page: {}", data.page);
        println!("Limit: {}", data.limit);
    }

    #[tokio::test]
    async fn test_update_bug() {
        let jar = Arc::new(Jar::default());
        let result = login(jar.clone(), "dengxiangcheng", "dxc3434DXC").await;
        println!("cookie: {}",result.unwrap());

        let bug_id = 2448;
        let status = 81;
        let resolution = 20;

        // 查询bug详情
        let bug_update_page_token;
        let bug_info;
        {
            let body = my_view_detail(jar.clone(), bug_id).await.unwrap();
            let document = Html::parse_document(body.as_str());
            bug_update_page_token = get_page_token(&document,"bug_update_page_token").unwrap();
            bug_info = my_view_detail_data(&document).unwrap();
        }
        println!("bug_update_page_token: {}",bug_update_page_token);
        // bug修改页面
        let body = bug_update_page(
            jar.clone(),
            UpdateToken {
                bug_id,
                bug_update_page_token,
            },
        )
        .await.unwrap();
        let bug_update_token = get_page_token(&Html::parse_document(body.as_str()),"bug_update_token").unwrap();
        println!("bug_update_token: {}",bug_update_token);

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
        println!("{}", serde_html_form::to_string(&bug).unwrap());
        // let _ = bug_update(jar.clone(), bug).await.map_or_else(
        //     |d|{println!("ok:{}",d);Ok(d)},
        //     |e|{println!("err:{}",e);Err(e)}
        // );
    }

    #[tokio::test]
    async fn test_view_all_set_data() {
        // 读取view_all_set.html文件内容
        let html_content = include_str!("./example/view_all_set.html");
        let document = Html::parse_document(html_content);
        let r = view_all_set_data(&document);
        assert!(r.is_ok());
        let data = r.unwrap();
        println!("Summary: {:?}", data);
    }

    #[tokio::test]
    async fn test_my_view_detail_data() {
        // 读取view_all_set.html文件内容
        let html_content = include_str!("./example/view.php.html");
        let document = Html::parse_document(html_content);
        let r = my_view_detail_data(&document);
        assert!(r.is_ok());
        let data = r.unwrap();
        println!("Summary: {:?}", data);
    }

    #[tokio::test]
    async fn test_bug_update_token_data() {
        // 读取view_all_set.html文件内容
        let html_content = include_str!("./example/bug_update_page.php.html");
        let document = Html::parse_document(html_content);
        let result = get_page_token(&document,"bug_update_token");
        assert!(result.is_ok());
        let data = result.unwrap();
        println!("Summary: {:?}", data);
    }
}
