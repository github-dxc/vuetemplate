mod enums;
mod model;
mod utils;

use enums::*;
use env_logger;
use log::{debug, info, warn};
use model::*;
use reqwest::cookie::{CookieStore, Jar};
use scraper::Html;
use serde::Serialize;
use tauri_plugin_store::StoreExt;
use url::Url;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use tauri::{AppHandle, Emitter, Manager, Window, WindowEvent};
use tauri_plugin_notification::{NotificationExt, PermissionState};
use tauri_plugin_updater::{Update, UpdaterExt};
use tokio::time::{interval, Duration};
use tokio::signal;
use serde_json::{Value};
use std::collections::HashSet;
use utils::*;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    // 初始化日志实现库
    env_logger::Builder::new()
        .filter_level(log::LevelFilter::Info) // 设置日志级别为 Debug
        .init();
    tauri::Builder::default()
        .plugin(tauri_plugin_store::Builder::new().build())
        .plugin(tauri_plugin_updater::Builder::new().build())
        .plugin(tauri_plugin_notification::init())
        .plugin(tauri_plugin_opener::init())
        .manage(MyState::default()) // 注册全局状态
        .invoke_handler(tauri::generate_handler![
            api_init_data,
            api_init_bugs,
            api_change_host,
            api_login,
            api_logout,
            api_bug_info,
            api_bug_list,
            api_image_bytes,
            api_update_bug,
            api_check_update,
            api_download_and_install
        ])
        .setup(|app| {
            //清空所有状态
            // let _ = clear_global_state(app.handle().clone());
            //初始化全局状态
            let _ = init_global_state(app.handle().clone());
            //初始化分组和项目
            let _ = init_project_catgory(app.handle().clone());
            //定时更新
            update_app(app.handle().clone());
            //定时查询订阅数据
            find_sub_data(app.handle().clone());
            //注册关闭回调
            close_app_callback(app.handle().clone());
            Ok(())
        })
        .on_window_event(move |window,event| {
            match event {
                //关闭事件
                WindowEvent::CloseRequested { api, .. } => {
                    if window.title().unwrap().as_str() == "image" {
                        println!("非main窗口关闭");
                        return
                    }
                    println!("窗口即将关闭，执行清理操作...");
                    let r = save_global_state(window.app_handle().clone());
                    match r {
                        Ok(_) => println!("handle exec ok"),
                        Err(msg) => println!("handle exec err: {}",msg)
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
    logined: Arc<Mutex<bool>>, // 是否登录
    jar: Mutex<Arc<Jar>>,
    host: Arc<Mutex<String>>,
    sub_params: Arc<Mutex<Vec<String>>>,//订阅列表
    sub_bugs: Arc<Mutex<HashMap<i64, Bug>>>, //bugs列表
    sub_bugs_hash: Arc<Mutex<HashMap<i64, u64>>>,
    last_version: Arc<Mutex<Option<Update>>>,//系统版本

    catgory_kv: Arc<Mutex<Vec<KV>>>,//分组的kv信息
    project_kv: Arc<Mutex<Vec<KV>>>,//项目的kv信息
}

// 初始化数据
#[tauri::command(rename_all = "snake_case")]
async fn api_init_data(app: AppHandle) -> Result<String, String> {
    let state = app.state::<MyState>().clone();

    // 查询项目列表
    let projects = state.project_kv.lock().map_err(|e|format!("lock err:{}",e))?.clone();

    // 查询分组列表
    let category = state.catgory_kv.lock().map_err(|e|format!("lock err:{}",e))?.clone();

    let mut hm = HashMap::new();
    hm.insert("Priority", Priority::kv());
    hm.insert("Severity", Severity::kv());
    hm.insert("Reproducibility", Reproducibility::kv());
    hm.insert("ViewStatus", ViewStatus::kv());
    hm.insert("Category", category);
    hm.insert("Project", projects);
    hm.insert("Status", Status::kv());
    hm.insert("Resolution", Resolution::kv());
    serde_json::to_string(&hm).map_err(|e|format!("serde_json err:{}",e))
}

// 修改host地址
#[tauri::command(rename_all = "snake_case")]
async fn api_change_host(app: AppHandle, host: &str) -> Result<String, String> {
    let state = app.state::<MyState>().clone();

    let mut state_host = state.host.lock().map_err(|e|format!("lock err:{}",e))?;
    if host != "" {
        *state_host = host.to_string();
    }
    Ok(state_host.clone())
}

// 初始化bugs数据
#[tauri::command(rename_all = "snake_case")]
async fn api_init_bugs(app: AppHandle) -> Result<Vec<Bug>, String> {
    let state = app.state::<MyState>().clone();
    
    // 查询缓存的bug列表
    let sub_bugs = state.sub_bugs.lock().map_err(|e|format!("lock err:{}",e))?.clone();
    let mut bugs:Vec<Bug> = sub_bugs.into_iter().map(|(_,bug)|bug).collect();
    bugs.sort_by_key(|b|std::cmp::Reverse(b.last_updated));
    Ok(bugs)
}
    
// 登录
#[tauri::command(rename_all = "snake_case")]
async fn api_login(app: AppHandle, username: &str, password: &str) -> Result<String, String> {
    let jar = Arc::new(Jar::default());
    let state = app.state::<MyState>().clone();
    let host = state.host.lock().map_err(|e|format!("lock err:{}",e))?.clone();
    let result = login(jar.clone(), username, password, &host).await.map_err(|e|format!("login err:{}",e))?;

    // 保存cookie到全局状态
    let mut logined = state.logined.lock().map_err(|e|format!("lock err:{}",e))?;
    *logined = true;
    let mut jar_ = state.jar.lock().map_err(|e|format!("lock err:{}",e))?;
    *jar_ = jar;

    // 初始化分组和项目
    let _ = init_project_catgory(app.clone());
    return Ok(result);
}

// 退出登录
#[tauri::command(rename_all = "snake_case")]
async fn api_logout(app: AppHandle) -> Result<(), String> {
    clear_global_state(app.clone())?;
    init_global_state(app.clone())?;
    init_project_catgory(app)
}

// bug列表
#[tauri::command(rename_all = "snake_case")]
async fn api_bug_list(app: AppHandle) -> Result<BugList, String> {
    let state = app.state::<MyState>().clone();
    let logined = state.logined.lock().map_err(|e|format!("lock err:{}",e))?.clone();
    let jar = state.jar.lock().map_err(|e|format!("lock err:{}",e))?.clone();
    let host = state.host.lock().map_err(|e|format!("lock err:{}",e))?.clone();
    let project_kv = state.project_kv.lock().map_err(|e|format!("lock err:{}",e))?.clone();
    let catgory_kv = state.catgory_kv.lock().map_err(|e|format!("lock err:{}",e))?.clone();
    if !logined {
        return Err("未登录".to_string());
    }
    let param_str = r"type=1&view_type=simple&reporter_id[]=0&handler_id[]=-1&monitor_user_id[]=0&note_user_id[]=0&priority[]=0&severity[]=0&view_state=0&sticky=1&category_id[]=0&hide_status[]=90&status[]=0&resolution[]=0&profile_id[]=0&platform[]=0&os[]=0&os_build[]=0&relationship_type=-1&relationship_bug=0&tag_string=&per_page=50&sort[]=date_submitted&dir[]=DESC&sort[]=status&dir[]=ASC&sort[]=last_updated&dir[]=DESC&match_type=0&highlight_changed=6&search=&filter_submit=应用过滤器";
    let param = serde_html_form::from_str::<FindBugListParams>(param_str)
        .map_err(|e| format!("serde_html_form err:{}", e))?;
    // 查询列表
    let body = view_all_set(jar, param, &host)
        .await
        .map_err(|e| format!("view_all_set err:{}", e))?;
    // 解析数据s
    let data = view_all_set_data(&Html::parse_document(body.as_str()),&catgory_kv,&project_kv)
        .map_err(|e| format!("view_all_set_data err:{}", e))?;

    Ok(data)
}

// 查询bug详情
#[tauri::command(rename_all = "snake_case")]
async fn api_bug_info(
    app: AppHandle,
    bug_id: i64
) -> Result<BugInfo, String> {
    let state = app.state::<MyState>().clone();
    let logined = state.logined.lock().map_err(|e|format!("lock err:{}",e))?.clone();
    let jar = state.jar.lock().map_err(|e|format!("lock err:{}",e))?.clone();
    let host = state.host.lock().map_err(|e|format!("lock err:{}",e))?.clone();
    let project_kv = state.project_kv.lock().map_err(|e|format!("lock err:{}",e))?.clone();
    let catgory_kv = state.catgory_kv.lock().map_err(|e|format!("lock err:{}",e))?.clone();
    if !logined {
        return Err("未登录".to_string());
    }
    // 查询bug详情
    let body = my_view_detail(jar.clone(), bug_id, &host).await?;
    let document = Html::parse_document(body.as_str());
    let bug_info = my_view_detail_data(&document, &host,&catgory_kv,&project_kv)?;
    Ok(bug_info)
}

// 修改bug
#[tauri::command(rename_all = "snake_case")]
async fn api_update_bug(
    app: AppHandle,
    bug_id: i64,
    status: i64,
    resolution: i64,
) -> Result<String, String> {
    let state = app.state::<MyState>().clone();
    let logined = state.logined.lock().map_err(|e|format!("lock err:{}",e))?.clone();
    let jar = state.jar.lock().map_err(|e|format!("lock err:{}",e))?.clone();
    let host = state.host.lock().map_err(|e|format!("lock err:{}",e))?.clone();
    let project_kv = state.project_kv.lock().map_err(|e|format!("lock err:{}",e))?.clone();
    let catgory_kv = state.catgory_kv.lock().map_err(|e|format!("lock err:{}",e))?.clone();
    if !logined {
        return Err("未登录".to_string());
    }
    // 查询bug详情
    let bug_update_page_token;
    let bug_info;
    {
        let body = my_view_detail(jar.clone(), bug_id, &host).await?;
        let document = Html::parse_document(body.as_str());
        bug_update_page_token = get_page_token(&document, "bug_update_page_token")?;
        bug_info = my_view_detail_data(&document, &host,&catgory_kv,&project_kv)?;
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
        last_updated: bug_info.last_updated,
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
        additional_information: bug_info.additional_information,
        steps_to_reproduce: bug_info.steps_to_reproduce,
        bugnote_text: "".to_string(),
    };
    println!("{:?}",bug);
    bug_update(jar.clone(), bug, &host).await
}

// 下载图片
#[tauri::command(rename_all = "snake_case")]
async fn api_image_bytes(app: AppHandle, uri: String) -> Result<Vec<u8>, String> {
    let state = app.state::<MyState>().clone();
    let jar = state.jar.lock().map_err(|e|format!("lock err:{}",e))?.clone();
    let host = state.host.lock().map_err(|e|format!("lock err:{}",e))?.clone();
    let bts = image_bytes(jar, host.as_str(), uri.as_str()).await?;
    Ok(bts)
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

//初始化全局状态
fn init_global_state(app: AppHandle) -> Result<(),String> {
    let state = app.state::<MyState>().clone();
    let store = app.store("settings.json").map_err(|e|format!("{}",e))?;

    let logined_value = store.get("logined").unwrap_or(Value::from(false));
    let cookie_value = store.get("cookies").unwrap_or(Value::from(""));
    println!("init:{};{}",logined_value,cookie_value);
    let default_host: &'static str = "localhost:8989";
    // let default_host: &'static str = "bug.test.com";
    let host_value = store.get("host").unwrap_or(Value::from(default_host));
    let hv = host_value.as_str().and_then(|v|{
        if v == "" {
            return Some(default_host);
        }
        Some(v)
    }).ok_or("host err".to_string())?;
    let sub_params_value = store.get("sub_param").unwrap_or(Value::from(vec![
        r"type=1&view_type=simple&reporter_id[]=0&handler_id[]=-1&monitor_user_id[]=0&note_user_id[]=0&priority[]=0&severity[]=0&view_state=0&sticky=1&category_id[]=0&hide_status[]=90&status[]=0&resolution[]=0&profile_id[]=0&platform[]=0&os[]=0&os_build[]=0&relationship_type=-1&relationship_bug=0&tag_string=&per_page=999&sort[]=last_updated&dir[]=DESC&sort[]=last_updated&dir[]=DESC&sort[]=status&dir[]=ASC&match_type=0&highlight_changed=12&search=&filter_submit=应用过滤器",
        r"type=1&view_type=simple&reporter_id[]=-1&handler_id[]=0&monitor_user_id[]=0&note_user_id[]=0&priority[]=0&severity[]=0&view_state=0&sticky=1&category_id[]=0&hide_status[]=90&status[]=0&resolution[]=0&profile_id[]=0&platform[]=0&os[]=0&os_build[]=0&relationship_type=-1&relationship_bug=0&tag_string=&per_page=999&sort[]=last_updated&dir[]=DESC&sort[]=last_updated&dir[]=DESC&sort[]=status&dir[]=ASC&match_type=0&highlight_changed=12&search=&filter_submit=应用过滤器",
    ]));
    let sub_bugs_value = store.get("sub_bugs").unwrap_or(Value::from(""));
    let sub_bugs_hash_value = store.get("sub_bugs_hash").unwrap_or(Value::from(""));

    let mut logined = state.logined.lock().map_err(|e|format!("lock err:{}",e))?;
    *logined = logined_value.as_bool().unwrap_or(false);

    let mut jar_ = state.jar.lock().map_err(|e|format!("lock err:{}",e))?;
    let jar = Jar::default();
    let url = ("http://".to_string()+hv).parse::<Url>().map_err(|e|format!("url parse err:{}",e))?;
    jar.add_cookie_str(cookie_value.as_str().unwrap_or(""), &url);
    *jar_ = Arc::new(jar);
    
    let mut host = state.host.lock().map_err(|e|format!("lock err:{}",e))?;
    *host = hv.to_string();

    let mut sub_params = state.sub_params.lock().map_err(|e|format!("lock err:{}",e))?;
    *sub_params = sub_params_value.as_array().ok_or("None".to_string())?.iter().map(|v|v.as_str().unwrap_or("").to_owned()).collect();

    let mut sub_bugs = state.sub_bugs.lock().map_err(|e|format!("lock err:{}",e))?;
    *sub_bugs = serde_json::from_str(sub_bugs_value.as_str().unwrap_or("")).map_err(|e|format!("lock err:{}",e))?;

    let mut sub_bugs_hash = state.sub_bugs_hash.lock().map_err(|e|format!("lock err:{}",e))?;
    *sub_bugs_hash = serde_json::from_str(sub_bugs_hash_value.as_str().unwrap_or("")).map_err(|e|format!("lock err:{}",e))?;
    Ok(())
}

// 初始化分组和项目
fn init_project_catgory(app: AppHandle) -> Result<(),String> {
    // 开启线程执行异步函数
    tauri::async_runtime::spawn(async move {
        let state = app.state::<MyState>().clone();
        
        let logined = state.logined.lock().map_err(|e|format!("lock err:{}",e))?.clone();

        let mut projects = vec![];
        let mut category = vec![];
        // 如果当前是已登录，则获取分组和项目
        if logined {
            {
                let jar_ = state.jar.lock().map_err(|e|format!("lock err:{}",e))?.clone();
                let host = state.host.lock().map_err(|e|format!("lock err:{}",e))?.clone();
                let body = my_view_page(jar_, &host).await.map_err(|e|format!("my_view_page err:{}",e))?;
                projects = project_data(&Html::parse_document(body.as_str()))?;
                let mut project_kv = state.project_kv.lock().map_err(|e|format!("lock err:{}",e))?;
                *project_kv = projects.clone();
            }

            // 查询分组列表
            {
                let jar_ = state.jar.lock().map_err(|e|format!("lock err:{}",e))?.clone();
                let host = state.host.lock().map_err(|e|format!("lock err:{}",e))?.clone();
                let project_id = projects.last().ok_or("projects error".to_owned())?.key.clone();
                let body = bug_report_page(jar_, &host, &project_id).await.map_err(|e|format!("filters_params err:{}",e))?;
                category = category_data(&Html::parse_document(body.as_str()))?;
                let mut catgory_kv = state.catgory_kv.lock().map_err(|e|format!("lock err:{}",e))?;
                *catgory_kv = category.clone();
            }
            if category.len() == 0|| projects.len() == 0 {
                return Err("".to_string());
            }
        } else {
            let mut project_kv = state.project_kv.lock().map_err(|e|format!("lock err:{}",e))?;
            *project_kv = projects;
            let mut catgory_kv = state.catgory_kv.lock().map_err(|e|format!("lock err:{}",e))?;
            *catgory_kv = category;
        };
        Ok(())
    });
    Ok(())
}

//保存全局状态
fn save_global_state(app: AppHandle) -> Result<(),String> {
    let state = app.state::<MyState>().clone();
    let store = app.store("settings.json").map_err(|e|format!("{}",e))?;

    let logined = state.logined.lock().map_err(|e|format!("lock err:{}",e))?;
    store.set("logined", logined.clone());

    
    let host = state.host.lock().map_err(|e|format!("lock err:{}",e))?;
    store.set("host", host.clone());

    let jar = state.jar.lock().map_err(|e|format!("lock err:{}",e))?;
    let url = format!("http://{}",host).parse::<Url>().map_err(|e|format!("parse err:{}",e))?;
    let hv = jar.cookies(&url).ok_or("cookies err".to_string())?;
    let cookies = hv.to_str().map_err(|e|format!("cookies err:{}",e))?;
    store.set("cookies", cookies);

    let sub_params = state.sub_params.lock().map_err(|e|format!("lock err:{}",e))?;
    store.set("sub_param", sub_params.clone());

    let sub_bugs = state.sub_bugs.lock().map_err(|e|format!("lock err:{}",e))?;
    let json = serde_json::to_string_pretty(&sub_bugs.clone()).map_err(|e|format!("to json err:{}",e))?;
    store.set("sub_bugs", json);

    let sub_bugs_hash = state.sub_bugs_hash.lock().map_err(|e|format!("lock err:{}",e))?;
    let json = serde_json::to_string_pretty(&sub_bugs_hash.clone()).map_err(|e|format!("to json err:{}",e))?;
    store.set("sub_bugs_hash", json);
    store.save().map_err(|e|format!("save err:{}",e))?;
    Ok(())
}

// 清空所有状态
fn clear_global_state(app: AppHandle) -> Result<(),String> {
    let state = app.state::<MyState>().clone();
    let store = app.store("settings.json").map_err(|e|format!("{}",e))?;

    let mut logined = state.logined.lock().map_err(|e|format!("lock err:{}",e))?;
    *logined = false;
    store.delete("logined");
    
    // let mut host = state.host.lock().map_err(|e|format!("lock err:{}",e))?;
    // *host = "".to_string();
    // store.delete("host");

    let mut jar = state.jar.lock().map_err(|e|format!("lock err:{}",e))?;
    *jar = Arc::new(Jar::default());
    store.delete("cookies");

    let mut sub_params = state.sub_params.lock().map_err(|e|format!("lock err:{}",e))?;
    *sub_params = vec![];
    store.delete("sub_param");

    let mut sub_bugs = state.sub_bugs.lock().map_err(|e|format!("lock err:{}",e))?;
    *sub_bugs = HashMap::default();
    store.delete("sub_bugs");

    let mut sub_bugs_hash = state.sub_bugs_hash.lock().map_err(|e|format!("lock err:{}",e))?;
    *sub_bugs_hash = HashMap::default();
    store.delete("sub_bugs_hash");
    store.save().map_err(|e|format!("store save err:{}",e))
}

// 定时查询订阅数据
fn find_sub_data(app: AppHandle) {
    // 启动一个异步定时任务
    tauri::async_runtime::spawn(async move {
        let state = app.state::<MyState>().clone();
        let mut ticker = interval(Duration::from_secs(5));
        loop {
            let result: Result<(), String> = async {
                ticker.tick().await;

                // 判断是否登录，如果未登录，则跳过
                let logined = state.logined.lock().map_err(|e|format!("lock err:{}",e))?.clone();
                if !logined {
                    info!("未登录，跳过定时任务");
                    return Ok(());
                };

                // 循环查询所有订阅的数据并且去重
                let mut bugs = Vec::new();
                let params = state.sub_params.lock().map_err(|e|format!("lock err:{}",e))?.clone();
                for sub_param in params {
                    let sub_param = sub_param.clone();
                    let jar = state.jar.lock().map_err(|e|format!("lock err:{}",e))?.clone();
                    let host = state.host.lock().map_err(|e|format!("lock err:{}",e))?.clone();
                    let project_kv = state.project_kv.lock().map_err(|e|format!("lock err:{}",e))?.clone();
                    let catgory_kv = state.catgory_kv.lock().map_err(|e|format!("lock err:{}",e))?.clone();
                    // 查询列表
                    let param = serde_html_form::from_str::<FindBugListParams>(&sub_param)
                        .map_err(|e| format!("serde_html_form err:{}", e))?;
                    let body = view_all_set(jar, param, &host).await.map_err(|e|format!("view_all_set err:{}",e))?;
                    // 解析数据
                    let mut data = view_all_set_data(&Html::parse_document(body.as_str()),&catgory_kv,&project_kv).map_err(|e|format!("view_all_set_data err:{}",e))?;
                    bugs.append(&mut data.bugs);
                }
                let mut seen = HashSet::new();
                bugs = bugs.into_iter().filter(|b| seen.insert(b.bug_id)).collect();
                info!("find bugs: {:?}", bugs);

                // 上次的bugs、hash
                let mut old_map = state.sub_bugs.lock().map_err(|e|format!("lock err:{}",e))?;
                let mut old_hash = state.sub_bugs_hash.lock().map_err(|e|format!("lock err:{}",e))?;
                
                // 构建新bugs、hash
                let mut new_map = HashMap::new();
                let mut new_hash = HashMap::new();

                let mut notice_msgs = Vec::new();

                //获取分组kv
                let category_kv = state.catgory_kv.lock().map_err(|e|format!("lock err:{}",e))?;
                
                for b in bugs.clone() {
                    let bgid = b.bug_id;
                    let hash = get_hash(&b);
                    let cg_kv = category_kv.find_by_key(b.category_id.to_string()).ok_or("not find kv".to_owned())?;
                    new_map.insert(bgid, b.clone());
                    new_hash.insert(bgid, hash);
                    //判断hash值是否一致，不一致则通知,没有也通知
                    if let None = old_hash.iter().find(|(&k,&v)| k == bgid && v == hash){
                        //通知
                        let notice = format!(
                                " {} | {} - {} -> {}",
                                b.project,
                                cg_kv.value,
                                Severity::from(b.severity).as_str(),
                                b.handler
                            );
                        notice_msgs.push(notice.clone());
                        let _ = send_notify(
                            app.clone(),
                            notice.as_str(),
                            b.summary.as_str(),
                        );
                    };
                }
                // 向前端所有窗口广播消息
                if notice_msgs.len() > 0 || old_map.len() != new_map.len() {
                    *old_map = new_map;
                    *old_hash = new_hash;
                    bugs.sort_by_key(|b| std::cmp::Reverse(b.last_updated));
                    let json = serde_json::to_string_pretty(&bugs).map_err(|e|format!("to json err:{}",e))?;
                    let _ = app.emit("sub_bugs", bugs);
                    let _ = app.emit("sub_msgs", notice_msgs);
                    // 保存订阅数据到本地
                    let store = app.store("settings.json").map_err(|e|format!("{}",e))?;
                    store.set("sub_bugs", json);
                    store.save().map_err(|e|format!("store save err:{}",e))?;
                }
                Ok(())
            }.await;
            if let Err(e) = result {
                info!("error:{}",e);
            }
        }
    });
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
async fn check_update(app: tauri::AppHandle) -> tauri_plugin_updater::Result<()> {
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
            update_time: update.date.map_or(0,|t| t.unix_timestamp())
        };

        // 保存到全局状态
        let state = app.state::<MyState>().clone();
        let mut last_version = state.last_version.lock().map_err(|_|tauri_plugin_updater::Error::ReleaseNotFound)?;
        *last_version = Some(update);

        // 通知前端
        app.emit("app-update", version_info)?;
        Ok(())
    } else {
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
        let state = app.state::<MyState>().clone();
        let last_version = state.last_version.lock().map_err(|_|tauri_plugin_updater::Error::ReleaseNotFound)?;
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
        .icon("asset://StoreLogo.png")
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
            Err(msg) => println!("handle exec err: {}",msg)
        }
        app.exit(0);
    });
    
}

#[cfg(test)]
mod tests {
    use super::*;
    use scraper::Html;

    #[tokio::test]
    async fn test_login() {
        let host = "bug.test.com";
        let result = login(Arc::new(Jar::default()), "dengxiangcheng", "dxc3434DXC", host).await;
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
