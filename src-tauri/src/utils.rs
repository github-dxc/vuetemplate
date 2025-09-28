use crate::enums::*;
use crate::model::*;

use chrono::{NaiveDate, NaiveDateTime, NaiveTime, TimeZone, Utc};
use chrono_tz::Asia::Shanghai;
use lazy_static::lazy_static;
use log::info;
use reqwest::cookie::{CookieStore, Jar};
use reqwest::header::{
    HeaderMap, HeaderValue, ACCEPT, ACCEPT_LANGUAGE, CACHE_CONTROL, CONNECTION, CONTENT_LENGTH,
    CONTENT_TYPE, HOST, ORIGIN, PRAGMA, REFERER, UPGRADE_INSECURE_REQUESTS, USER_AGENT,
};
use reqwest::multipart::{Form, Part};
use reqwest::redirect::Policy;
use reqwest::Client;
use scraper::{Html, Selector};
use serde_html_form;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::Mutex;
use url::Url;

// 定义一个全局锁
lazy_static! {
    static ref GLOBAL_LOCK: Arc<Mutex<i32>> = Arc::new(Mutex::new(0));
}

// login 页面
pub async fn login(
    jar: Arc<Jar>,
    username: &str,
    password: &str,
    host: &str,
) -> Result<String, Box<dyn std::error::Error>> {
    println!("{},{},{}", username, password, host);
    // 地址
    let origin = format!("http://{}", host);
    let url = origin.to_string() + "/login.php";

    // 构建请求体
    let body = format!(
        "return=index.php&username={}&password={}&perm_login=on&secure_session=on",
        username, password
    );

    // 构建请求头
    let mut headers = HeaderMap::new();
    headers.insert(ACCEPT,HeaderValue::from_static("text/html,application/xhtml+xml,application/xml;q=0.9,image/avif,image/webp,image/apng,*/*;q=0.8,application/signed-exchange;v=b3;q=0.7"));
    headers.insert(
        ACCEPT_LANGUAGE,
        HeaderValue::from_static("zh-CN,zh;q=0.9,en;q=0.8,en-GB;q=0.7,en-US;q=0.6"),
    );
    headers.insert(CACHE_CONTROL, HeaderValue::from_static("max-age=0"));
    headers.insert(
        CONTENT_TYPE,
        HeaderValue::from_static("application/x-www-form-urlencoded"),
    );
    headers.insert(
        CONTENT_LENGTH,
        HeaderValue::from_str(&body.len().to_string())?,
    );
    headers.insert(ORIGIN, HeaderValue::from_str(&origin)?);
    headers.insert(
        REFERER,
        HeaderValue::from_str(&(origin.to_string() + "/login_password_page.php"))?,
    );
    headers.insert(UPGRADE_INSECURE_REQUESTS, HeaderValue::from_static("1"));
    headers.insert(USER_AGENT,HeaderValue::from_static("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/137.0.0.0 Safari/537.36 Edg/137.0.0.0"));
    headers.insert(CONNECTION, HeaderValue::from_static("keep-alive"));
    headers.insert(HOST, HeaderValue::from_str(host)?);
    let jar1 = jar.clone();
    let dst = format!("{}/login_cookie_test.php?return=index.php", origin);
    // 创建 reqwest 客户端
    let client = Client::builder()
        .timeout(Duration::from_secs(2))
        .cookie_provider(jar)
        .danger_accept_invalid_certs(true) // --insecure
        .default_headers(headers)
        .redirect(Policy::custom(move |attempt| {
            // 自定义重定向策略
            if attempt.url().as_str().eq(dst.as_str()) {
                attempt.stop()
            } else {
                attempt.error("login err")
            }
        }))
        .build()
        .map_err(|e| e.to_string())?;

    // 发送 POST 请求
    let resp = client
        .post(url)
        .body(body)
        .send()
        .await
        .map_err(|e| e.to_string())?;

    let text = resp.text().await?;
    info!("login resp: {}", text);
    let cookie = jar1
        .cookies(&Url::parse(&origin).unwrap())
        .map(|e| format!("{}", e.to_str().unwrap()))
        .ok_or("".to_string())?;
    info!("login cookie: {}", cookie);
    Ok(cookie)
}

// my_view_page 页面
pub async fn my_view_page(jar: Arc<Jar>, host: &str) -> Result<String, Box<dyn std::error::Error>> {
    let origin = format!("http://{}", host);
    let url = origin.to_string() + "/my_view_page.php?days=0&all=1";

    // 构建请求头
    let mut headers = HeaderMap::new();
    headers.insert(ACCEPT, HeaderValue::from_static("text/html,application/xhtml+xml,application/xml;q=0.9,image/avif,image/webp,image/apng,*/*;q=0.8,application/signed-exchange;v=b3;q=0.7"));
    headers.insert(
        ACCEPT_LANGUAGE,
        HeaderValue::from_static("zh-CN,zh;q=0.9,en;q=0.8,en-GB;q=0.7,en-US;q=0.6"),
    );
    headers.insert(CONNECTION, HeaderValue::from_static("keep-alive"));
    headers.insert(UPGRADE_INSECURE_REQUESTS, HeaderValue::from_static("1"));
    headers.insert(USER_AGENT, HeaderValue::from_static("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/137.0.0.0 Safari/537.36 Edg/137.0.0.0"));

    // 创建 reqwest 客户端
    let client = Client::builder()
        .timeout(Duration::from_secs(2))
        .cookie_provider(jar)
        .danger_accept_invalid_certs(true) // --insecure
        .default_headers(headers)
        .build()
        .map_err(|e| e.to_string())?;

    // 发送 GET 请求
    let resp = client.get(url).send().await.map_err(|e| e.to_string())?;

    let text = resp.text().await.map_err(|e| e.to_string())?;
    Ok(text)
}

// view_all_set 页面
pub async fn view_all_set(
    jar: Arc<Jar>,
    params: FindBugListParams,
    project_id: &str,
    page: i64,
    host: &str,
) -> Result<String, Box<dyn std::error::Error>> {
    // 保证每次查询是串行的
    let l = GLOBAL_LOCK.clone();
    let _lock = l.lock().await;
    // 地址
    let origin = format!("http://{}", host);
    let mut url = origin.to_string() + "/view_all_set.php";

    // 设置项目id cookie
    let _ = set_project_cookie(jar.clone(), project_id, host);

    // body
    let body = serde_html_form::to_string(&params).unwrap();

    // 构建请求头
    let mut headers = HeaderMap::new();
    headers.insert(ACCEPT,HeaderValue::from_static("text/html,application/xhtml+xml,application/xml;q=0.9,image/avif,image/webp,image/apng,*/*;q=0.8,application/signed-exchange;v=b3;q=0.7"));
    headers.insert(
        ACCEPT_LANGUAGE,
        HeaderValue::from_static("zh-CN,zh;q=0.9,en;q=0.8,en-GB;q=0.7,en-US;q=0.6"),
    );
    headers.insert(CACHE_CONTROL, HeaderValue::from_static("max-age=0"));
    headers.insert(
        CONTENT_TYPE,
        HeaderValue::from_static("application/x-www-form-urlencoded"),
    );
    headers.insert(
        CONTENT_LENGTH,
        HeaderValue::from_str(&body.len().to_string())?,
    );
    headers.insert(ORIGIN, HeaderValue::from_str(&origin)?);
    headers.insert(PRAGMA, HeaderValue::from_str("no-cache")?);
    headers.insert(
        REFERER,
        HeaderValue::from_str(&(origin.to_string() + "/view_all_bug_page.php"))?,
    );
    headers.insert(UPGRADE_INSECURE_REQUESTS, HeaderValue::from_static("1"));
    headers.insert(USER_AGENT,HeaderValue::from_static("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/137.0.0.0 Safari/537.36 Edg/137.0.0.0"));
    headers.insert(CONNECTION, HeaderValue::from_static("keep-alive"));
    headers.insert(HOST, HeaderValue::from_str(host)?);

    // 创建 reqwest 客户端
    let client = Client::builder()
        .timeout(Duration::from_secs(2))
        .cookie_provider(jar.clone())
        .danger_accept_invalid_certs(true) // --insecure
        .default_headers(headers.clone())
        .redirect(Policy::custom(|attempt| {
            // 自定义重定向策略
            attempt.stop()
        }))
        .build()
        .map_err(|e| e.to_string())?;

    // 发送 POST 请求
    let resp = client
        .post(url)
        .body(body)
        .send()
        .await
        .map_err(|e| e.to_string())?;

    if resp.status() != 302 {
        return Err(format!("请求失败，状态码: {}", resp.status()).into());
    }

    // 获取数据
    url = format!("{}/view_all_bug_page.php", origin);
    if page > 1 {
        url = format!("{}?page_number={}", url, page);
    }

    // 创建 reqwest 客户端
    headers.remove(CONTENT_TYPE);
    headers.remove(CONTENT_LENGTH);
    headers.remove(REFERER);
    let client = Client::builder()
        .timeout(Duration::from_secs(2))
        .cookie_provider(jar.clone())
        .danger_accept_invalid_certs(true) // --insecure
        .default_headers(headers)
        .redirect(Policy::custom(|attempt| {
            // 自定义重定向策略
            attempt.stop()
        }))
        .build()
        .map_err(|e| e.to_string())?;
    let resp = client.get(url).send().await.map_err(|e| e.to_string())?;

    // 清空项目id cookie
    let _ = set_project_cookie(jar.clone(), "0", host);

    let text = resp.text().await.unwrap();
    Ok(text)
}

// my_view_detail 页面
pub async fn my_view_detail(jar: Arc<Jar>, id: i64, host: &str) -> Result<String, String> {
    let origin = format!("http://{}", host);
    let url = format!("{}/view.php?id={}", origin, id);

    // 构建请求头
    let mut headers = HeaderMap::new();
    headers.insert(ACCEPT, HeaderValue::from_static("text/html,application/xhtml+xml,application/xml;q=0.9,image/avif,image/webp,image/apng,*/*;q=0.8,application/signed-exchange;v=b3;q=0.7"));
    headers.insert(
        ACCEPT_LANGUAGE,
        HeaderValue::from_static("zh-CN,zh;q=0.9,en;q=0.8,en-GB;q=0.7,en-US;q=0.6"),
    );
    headers.insert(CONNECTION, HeaderValue::from_static("keep-alive"));
    headers.insert(UPGRADE_INSECURE_REQUESTS, HeaderValue::from_static("1"));
    headers.insert(USER_AGENT, HeaderValue::from_static("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/137.0.0.0 Safari/537.36 Edg/137.0.0.0"));

    // 创建 reqwest 客户端
    let client = Client::builder()
        .timeout(Duration::from_secs(2))
        .cookie_provider(jar)
        .danger_accept_invalid_certs(true) // --insecure
        .default_headers(headers)
        .build()
        .map_err(|e| e.to_string())?;

    // 发送 GET 请求
    let resp = client.get(url).send().await.map_err(|e| e.to_string())?;

    let text = resp.text().await.map_err(|e| e.to_string())?;
    Ok(text)
}

// bug_update_page 页面
pub async fn bug_update_page(
    jar: Arc<Jar>,
    bug: UpdateToken,
    host: &str,
) -> Result<String, String> {
    let origin = format!("http://{}", host);
    let url = origin.to_string() + "/bug_update_page.php";

    // body
    let body = serde_html_form::to_string(&bug).unwrap();
    let referer = format!("http://bug.test.com/view.php?id={}", bug.bug_id);

    // 构建请求头
    let mut headers = HeaderMap::new();
    headers.insert(ACCEPT, HeaderValue::from_static("text/html,application/xhtml+xml,application/xml;q=0.9,image/avif,image/webp,image/apng,*/*;q=0.8,application/signed-exchange;v=b3;q=0.7"));
    headers.insert(
        ACCEPT_LANGUAGE,
        HeaderValue::from_static("zh-CN,zh;q=0.9,en;q=0.8,en-GB;q=0.7,en-US;q=0.6"),
    );
    headers.insert(
        CONTENT_TYPE,
        HeaderValue::from_static("application/x-www-form-urlencoded"),
    );
    headers.insert(CONNECTION, HeaderValue::from_static("keep-alive"));
    headers.insert(
        REFERER,
        HeaderValue::from_str(referer.as_str()).map_err(|e| format!("{}", e))?,
    );
    headers.insert(UPGRADE_INSECURE_REQUESTS, HeaderValue::from_static("1"));
    headers.insert(USER_AGENT, HeaderValue::from_static("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/137.0.0.0 Safari/537.36 Edg/137.0.0.0"));

    // 创建 reqwest 客户端
    let client = Client::builder()
        .timeout(Duration::from_secs(2))
        .cookie_provider(jar)
        .danger_accept_invalid_certs(true) // --insecure
        .default_headers(headers)
        .build()
        .map_err(|e| e.to_string())?;

    // 发送 POST 请求
    let resp = client
        .post(url)
        .body(body)
        .send()
        .await
        .map_err(|e| e.to_string())?;

    let text = resp.text().await.map_err(|e| e.to_string())?;
    Ok(text)
}

// bug_update 页面
pub async fn bug_update(jar: Arc<Jar>, bug: UpdateBug, host: &str) -> Result<String, String> {
    let origin = format!("http://{}", host);
    let url = origin.to_string() + "/bug_update.php";

    // body
    let body = serde_html_form::to_string(&bug).unwrap();

    // 构建请求头
    let mut headers = HeaderMap::new();
    headers.insert(ACCEPT, HeaderValue::from_static("text/html,application/xhtml+xml,application/xml;q=0.9,image/avif,image/webp,image/apng,*/*;q=0.8,application/signed-exchange;v=b3;q=0.7"));
    headers.insert(
        ACCEPT_LANGUAGE,
        HeaderValue::from_static("zh-CN,zh;q=0.9,en;q=0.8,en-GB;q=0.7,en-US;q=0.6"),
    );
    headers.insert(CONNECTION, HeaderValue::from_static("keep-alive"));
    headers.insert(
        CONTENT_TYPE,
        HeaderValue::from_static("application/x-www-form-urlencoded"),
    );
    headers.insert(UPGRADE_INSECURE_REQUESTS, HeaderValue::from_static("1"));
    headers.insert(USER_AGENT, HeaderValue::from_static("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/137.0.0.0 Safari/537.36 Edg/137.0.0.0"));

    // 创建 reqwest 客户端
    let client = Client::builder()
        .timeout(Duration::from_secs(2))
        .cookie_provider(jar)
        .danger_accept_invalid_certs(true) // --insecure
        .default_headers(headers)
        .redirect(Policy::custom(|attempt| {
            // 自定义重定向策略
            attempt.stop()
        }))
        .build()
        .map_err(|e| e.to_string())?;

    // 发送 GET 请求
    let resp = client
        .post(url)
        .body(body)
        .send()
        .await
        .map_err(|e| e.to_string())?;

    let text = resp.text().await.map_err(|e| e.to_string())?;

    Ok(text)
}

// bugnote_add 请求
pub async fn bug_note_add(jar: Arc<Jar>, bug: BugNoteAdd, host: &str) -> Result<String, String> {
    let origin = format!("http://{}", host);
    let url = origin.to_string() + "/bugnote_add.php";

    // 构造 multipart/form-data 表单
    let mut form = Form::new()
        .text("bugnote_text", bug.bugnote_text.to_string())
        .text("bug_id", bug.bug_id.to_string())
        .text("max_file_size", bug.max_file_size.to_string())
        .text("bugnote_add_token", bug.bugnote_add_token.to_string());

    // 如果提供了文件路径，则添加文件部分
    let mut i: usize = 0;
    for path in bug.file_path {
        let file_bytes = tokio::fs::read(&path)
            .await
            .map_err(|e| format!("无法读取文件: {}", e))?;
        let file_name = std::path::Path::new(&path)
            .file_name()
            .and_then(|os_str| os_str.to_str())
            .map(|s| s.to_string())
            .unwrap_or_else(|| "unnamed_file".to_string());

        let mime = get_mime_type_from_filename(&file_name);
        let file_part = Part::bytes(file_bytes)
            .file_name(file_name)
            .mime_str(&mime) // 设置MIME类型
            .unwrap();
        form = form.part(format!("ufile[{}]", i), file_part); // 这里的"attachment"是表单中文件字段的名称
        i += 1;
    }
    // 如果是二进制文件，则添加文件部分
    for (file_name, file_bytes) in bug.binary_file {
        let mime = get_mime_type_from_filename(&file_name);
        let file_part = Part::bytes(file_bytes)
            .file_name(file_name)
            .mime_str(&mime) // 设置MIME类型
            .unwrap();
        form = form.part(format!("ufile[{}]", i), file_part); // 这里的"attachment"是表单中文件字段的名称
        i += 1;
    }
    // 构建请求头
    let mut headers = HeaderMap::new();
    headers.insert(ACCEPT, HeaderValue::from_static("text/html,application/xhtml+xml,application/xml;q=0.9,image/avif,image/webp,image/apng,*/*;q=0.8,application/signed-exchange;v=b3;q=0.7"));
    headers.insert(
        ACCEPT_LANGUAGE,
        HeaderValue::from_static("zh-CN,zh;q=0.9,en;q=0.8,en-GB;q=0.7,en-US;q=0.6"),
    );
    headers.insert(CONNECTION, HeaderValue::from_static("keep-alive"));
    headers.insert(UPGRADE_INSECURE_REQUESTS, HeaderValue::from_static("1"));
    headers.insert(USER_AGENT, HeaderValue::from_static("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/137.0.0.0 Safari/537.36 Edg/137.0.0.0"));

    // 创建 reqwest 客户端
    let client = Client::builder()
        .timeout(Duration::from_secs(2))
        .cookie_provider(jar)
        .danger_accept_invalid_certs(true) // --insecure
        .default_headers(headers)
        .redirect(Policy::custom(|attempt| {
            // 自定义重定向策略
            attempt.stop()
        }))
        .build()
        .map_err(|e| e.to_string())?;

    // 发送 GET 请求
    let resp = client
        .post(url)
        .multipart(form)
        .send()
        .await
        .map_err(|e| e.to_string())?;

    let text = resp.text().await.map_err(|e| e.to_string())?;

    Ok(text)
}

// bug_report 请求
pub async fn bug_report(jar: Arc<Jar>, bug: ReportBug, host: &str) -> Result<String, String> {
    let origin = format!("http://{}", host);
    let url = origin.to_string() + "/bug_report.php";

    // 构造 multipart/form-data 表单
    let mut form = Form::new()
        .text("bug_report_token", bug.bug_report_token.to_string())
        .text("m_id", bug.m_id.to_string())
        .text("project_id", bug.project_id.to_string())
        .text("category_id", bug.category_id.to_string())
        .text("summary", bug.summary.to_string())
        .text("description", bug.description.to_string())
        .text("reproducibility", bug.reproducibility.to_string())
        .text("severity", bug.severity.to_string())
        .text("priority", bug.priority.to_string())
        .text("handler_id", bug.handler_id.to_string())
        .text("platform", bug.platform.to_string())
        .text("os", bug.os.to_string())
        .text("os_build", bug.os_build.to_string())
        .text("steps_to_reproduce", bug.steps_to_reproduce.to_string())
        .text("additional_info", bug.additional_info.to_string())
        .text("tag_string", bug.tag_string.to_string())
        .text("tag_select", bug.tag_select.to_string())
        .text("view_state", bug.view_state.to_string())
        .text("report_stay", bug.report_stay.to_string())
        .text("max_file_size", bug.max_file_size.to_string());

    // 如果提供了文件路径，则添加文件部分
    let mut i: usize = 0;
    for path in bug.file_path {
        let file_bytes = tokio::fs::read(&path)
            .await
            .map_err(|e| format!("无法读取文件: {}", e))?;
        let file_name = std::path::Path::new(&path)
            .file_name()
            .and_then(|os_str| os_str.to_str())
            .map(|s| s.to_string())
            .unwrap_or_else(|| "unnamed_file".to_string());

        let mime = get_mime_type_from_filename(&file_name);
        let file_part = Part::bytes(file_bytes)
            .file_name(file_name)
            .mime_str(&mime) // 设置MIME类型
            .unwrap();
        form = form.part(format!("ufile[{}]", i), file_part); // 这里的"attachment"是表单中文件字段的名称
        i += 1;
    }
    // 如果是二进制文件，则添加文件部分
    for (file_name, file_bytes) in bug.binary_file {
        let mime = get_mime_type_from_filename(&file_name);
        let file_part = Part::bytes(file_bytes)
            .file_name(file_name)
            .mime_str(&mime) // 设置MIME类型
            .unwrap();
        form = form.part(format!("ufile[{}]", i), file_part); // 这里的"attachment"是表单中文件字段的名称
        i += 1;
    }
    // 构建请求头
    let mut headers = HeaderMap::new();
    headers.insert(ACCEPT, HeaderValue::from_static("text/html,application/xhtml+xml,application/xml;q=0.9,image/avif,image/webp,image/apng,*/*;q=0.8,application/signed-exchange;v=b3;q=0.7"));
    headers.insert(
        ACCEPT_LANGUAGE,
        HeaderValue::from_static("zh-CN,zh;q=0.9,en;q=0.8,en-GB;q=0.7,en-US;q=0.6"),
    );
    headers.insert(CONNECTION, HeaderValue::from_static("keep-alive"));
    headers.insert(UPGRADE_INSECURE_REQUESTS, HeaderValue::from_static("1"));
    headers.insert(USER_AGENT, HeaderValue::from_static("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/137.0.0.0 Safari/537.36 Edg/137.0.0.0"));

    // 创建 reqwest 客户端
    let client = Client::builder()
        .timeout(Duration::from_secs(2))
        .cookie_provider(jar)
        .danger_accept_invalid_certs(true) // --insecure
        .default_headers(headers)
        .redirect(Policy::custom(|attempt| {
            // 自定义重定向策略
            attempt.stop()
        }))
        .build()
        .map_err(|e| e.to_string())?;

    // 发送 GET 请求
    let resp = client
        .post(url)
        .multipart(form)
        .send()
        .await
        .map_err(|e| e.to_string())?;

    let text = resp.text().await.map_err(|e| e.to_string())?;

    Ok(text)
}

// bug_change_status_page 页面
pub async fn bug_change_status_page(
    jar: Arc<Jar>,
    bug: UpdateToken,
    host: &str,
) -> Result<String, String> {
    let origin = format!("http://{}", host);
    let url = origin.to_string() + "/bug_change_status_page.php";

    // body
    let body = serde_html_form::to_string(&bug).unwrap();

    // 构建请求头
    let mut headers = HeaderMap::new();
    headers.insert(ACCEPT, HeaderValue::from_static("text/html,application/xhtml+xml,application/xml;q=0.9,image/avif,image/webp,image/apng,*/*;q=0.8,application/signed-exchange;v=b3;q=0.7"));
    headers.insert(
        ACCEPT_LANGUAGE,
        HeaderValue::from_static("zh-CN,zh;q=0.9,en;q=0.8,en-GB;q=0.7,en-US;q=0.6"),
    );
    headers.insert(CONNECTION, HeaderValue::from_static("keep-alive"));
    headers.insert(UPGRADE_INSECURE_REQUESTS, HeaderValue::from_static("1"));
    headers.insert(USER_AGENT, HeaderValue::from_static("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/137.0.0.0 Safari/537.36 Edg/137.0.0.0"));

    // 创建 reqwest 客户端
    let client = Client::builder()
        .timeout(Duration::from_secs(2))
        .cookie_provider(jar)
        .danger_accept_invalid_certs(true) // --insecure
        .default_headers(headers)
        .build()
        .map_err(|e| e.to_string())?;

    // 发送 POST 请求
    let resp = client
        .post(url)
        .body(body)
        .send()
        .await
        .map_err(|e| e.to_string())?;

    let text = resp.text().await.map_err(|e| e.to_string())?;
    Ok(text)
}

// bug_report_page 页面
pub async fn bug_report_page(
    jar: Arc<Jar>,
    host: &str,
    project_id: &str,
) -> Result<String, String> {
    let origin = format!("http://{}", host);
    let url = format!("{}/bug_report_page.php", origin);

    // 设置项目id cookie
    let _ = set_project_cookie(jar.clone(), project_id, host);

    // 构建请求头
    let mut headers = HeaderMap::new();
    headers.insert(ACCEPT, HeaderValue::from_static("text/html,application/xhtml+xml,application/xml;q=0.9,image/avif,image/webp,image/apng,*/*;q=0.8,application/signed-exchange;v=b3;q=0.7"));
    headers.insert(
        ACCEPT_LANGUAGE,
        HeaderValue::from_static("zh-CN,zh;q=0.9,en;q=0.8,en-GB;q=0.7,en-US;q=0.6"),
    );
    headers.insert(CONNECTION, HeaderValue::from_static("keep-alive"));
    headers.insert(UPGRADE_INSECURE_REQUESTS, HeaderValue::from_static("1"));
    headers.insert(USER_AGENT, HeaderValue::from_static("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/137.0.0.0 Safari/537.36 Edg/137.0.0.0"));

    // 创建 reqwest 客户端
    let client = Client::builder()
        .timeout(Duration::from_secs(2))
        .cookie_provider(jar.clone())
        .danger_accept_invalid_certs(true) // --insecure
        .default_headers(headers)
        .build()
        .map_err(|e| e.to_string())?;

    // 发送 GET 请求
    let resp = client.get(url).send().await.map_err(|e| e.to_string())?;

    // 清空项目id cookie
    let _ = set_project_cookie(jar.clone(), "0", host);

    let text = resp.text().await.map_err(|e| e.to_string())?;
    Ok(text)
}

// image信息
pub async fn image_bytes(jar: Arc<Jar>, host: &str, uri: &str) -> Result<Vec<u8>, String> {
    let origin = format!("http://{}", host);
    let url = format!("{}/{}", origin, uri);

    // 构建请求头
    let mut headers = HeaderMap::new();
    headers.insert(
        ACCEPT,
        HeaderValue::from_static(
            "image/avif,image/webp,image/apng,image/svg+xml,image/*,*/*;q=0.8",
        ),
    );
    headers.insert(
        ACCEPT_LANGUAGE,
        HeaderValue::from_static("zh-CN,zh;q=0.9,en;q=0.8,en-GB;q=0.7,en-US;q=0.6"),
    );
    headers.insert(CONNECTION, HeaderValue::from_static("keep-alive"));
    headers.insert(USER_AGENT, HeaderValue::from_static("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/137.0.0.0 Safari/537.36 Edg/137.0.0.0"));

    // 创建 reqwest 客户端
    let client = Client::builder()
        .timeout(Duration::from_secs(2))
        .cookie_provider(jar.clone())
        .danger_accept_invalid_certs(true) // --insecure
        .default_headers(headers)
        .build()
        .map_err(|e| e.to_string())?;

    // 发送 GET 请求
    let resp = client.get(url).send().await.map_err(|e| e.to_string())?;

    let bytes = resp.bytes().await.map_err(|e| e.to_string())?;
    Ok(bytes.to_vec())
}

// 查询条件列表（单独获取某一项）
pub async fn filters_params(
    jar: Arc<Jar>,
    params: FiltersParams,
    host: &str,
) -> Result<String, String> {
    let origin = format!("http://{}", host);
    let url = format!(
        "{}/return_dynamic_filters?view_type={}&filter_target={}&filter_id=3&_={}",
        origin,
        params.view_type,
        params.filter_target,
        Utc::now().timestamp_millis()
    );

    // 构建请求头
    let mut headers = HeaderMap::new();
    headers.insert(ACCEPT, HeaderValue::from_static("text/html,application/xhtml+xml,application/xml;q=0.9,image/avif,image/webp,image/apng,*/*;q=0.8,application/signed-exchange;v=b3;q=0.7"));
    headers.insert(
        ACCEPT_LANGUAGE,
        HeaderValue::from_static("zh-CN,zh;q=0.9,en;q=0.8,en-GB;q=0.7,en-US;q=0.6"),
    );
    headers.insert(CONNECTION, HeaderValue::from_static("keep-alive"));
    headers.insert(UPGRADE_INSECURE_REQUESTS, HeaderValue::from_static("1"));
    headers.insert(USER_AGENT, HeaderValue::from_static("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/137.0.0.0 Safari/537.36 Edg/137.0.0.0"));

    // 创建 reqwest 客户端
    let client = Client::builder()
        .timeout(Duration::from_secs(2))
        .cookie_provider(jar)
        .danger_accept_invalid_certs(true) // --insecure
        .default_headers(headers)
        .build()
        .map_err(|e| e.to_string())?;

    // 发送 GET 请求
    let resp = client.get(url).send().await.map_err(|e| e.to_string())?;

    let text = resp.text().await.map_err(|e| e.to_string())?;
    Ok(text)
}

// 查询my_view_page_details数据
pub fn my_view_page_details(
    document: &Html,
) -> Result<Vec<OperateLogs>, Box<dyn std::error::Error>> {
    // 创建选择器
    let selector = Selector::parse(".profile-activity.clearfix")
        .map_err(|e| format!("Selector 解析失败: {:?}", e))?;

    // 收集所有匹配元素的文本
    let results: Vec<OperateLogs> = document
        .select(&selector)
        .map(|element| {
            let mut bug = OperateLogs::default();
            // bug_id
            let id_slector = Selector::parse(".issue_id>a").unwrap();
            bug.bug_id = element
                .select(&id_slector)
                .find_map(|e| e.inner_html().parse::<i64>().ok())
                .unwrap_or(0);

            // handler_id
            let handler_selector = Selector::parse(".action .username>a").unwrap();
            bug.handler_id = element
                .select(&handler_selector)
                .find_map(|e| {
                    //  handler
                    bug.handler = e.inner_html();
                    e.value().attr("href").and_then(|href| {
                        href.split('=').last().and_then(|id| id.parse::<i64>().ok())
                    })
                })
                .unwrap_or(0);

            // last_updated
            let last_modified_selector = Selector::parse(".time").unwrap();
            // <td class="column-last-modified">2025-06-18 23:15</td>
            bug.last_updated = element
                .select(&last_modified_selector)
                .find_map(|e| {
                    let date_str = e
                        .text()
                        .last()
                        .unwrap_or_default()
                        .replace("\"", "")
                        .trim()
                        .to_string();
                    // 解析为 NaiveDate
                    let date =
                        NaiveDateTime::parse_from_str(date_str.as_str(), "%Y-%m-%d %H:%M").ok()?;
                    // 设为上海时区的0点
                    let datetime = Shanghai.from_local_datetime(&date).unwrap();
                    // 转为时间戳（秒）
                    Some(datetime.timestamp())
                })
                .unwrap_or(0);

            // content
            let content_selector = Selector::parse(".action").unwrap();
            bug.content = element
                .select(&content_selector)
                .find_map(|e| Some(e.inner_html()))
                .unwrap_or_default();
            bug
        })
        .collect();

    Ok(results)
}

// 查询view_all_set数据
pub fn view_all_set_data(
    document: &Html,
    category_kv: &Vec<KV>,
    project_kv: &Vec<KV>,
) -> Result<BugList, Box<dyn std::error::Error>> {
    // 创建选择器
    let selector =
        Selector::parse("#buglist tbody tr").map_err(|e| format!("Selector 解析失败: {:?}", e))?;

    // 收集所有匹配元素的文本
    let mut bug_list = BugList::default();
    let results: Vec<Bug> = document
        .select(&selector)
        .map(|element| {
            let mut bug = Bug::default();
            // bug_id
            let id_slector = Selector::parse(".column-id a").unwrap();
            bug.bug_id = element
                .select(&id_slector)
                .find_map(|e| e.inner_html().parse::<i64>().ok())
                .unwrap_or(0);

            // issue_notes_count
            let bugnotes_selector = Selector::parse(".column-bugnotes-count a").unwrap();
            bug.issue_notes_count = element
                .select(&bugnotes_selector)
                .find_map(|e| e.inner_html().parse::<i64>().ok())
                .unwrap_or(0);

            // attachments
            let attachments_selector = Selector::parse(".column-attachments a").unwrap();
            bug.attachments = element
                .select(&attachments_selector)
                .find_map(|e| e.inner_html().parse::<i64>().ok())
                .unwrap_or(0);

            // priority
            let priority_selector = Selector::parse(".column-priority i").unwrap();
            bug.priority = element
                .select(&priority_selector)
                .find_map(|e| {
                    Some(Priority::from(e.value().attr("title").unwrap_or_default()).as_i64())
                })
                .unwrap_or(0);

            // category_id
            let category_selector = Selector::parse(".column-category div").unwrap();

            bug.category_id = element
                .select(&category_selector)
                .find_map(|e| {
                    category_kv
                        .find_by_value(e.text().last().unwrap_or_default().trim())
                        .and_then(|kv| kv.key.parse::<i64>().ok())
                })
                .unwrap_or(0);

            // project_id
            let project_selector = Selector::parse(".column-category div span a").unwrap();
            bug.project_id = element
                .select(&project_selector)
                .find_map(|e| {
                    // project
                    bug.project = e.inner_html().trim().to_string();
                    project_kv
                        .find_by_value(bug.project.as_str())
                        .map(|kv| kv.key.clone())
                })
                .unwrap_or("".to_owned());

            // severity
            let severity_selector = Selector::parse(".column-severity").unwrap();
            bug.severity = element
                .select(&severity_selector)
                .find_map(|e| {
                    Some(Severity::from(e.text().last().unwrap_or_default().trim()).as_i64())
                })
                .unwrap_or(0);

            // status
            let status_selector = Selector::parse(".column-status div span").unwrap();
            bug.status = element
                .select(&status_selector)
                .find_map(|e| Some(Status::from(e.inner_html().trim()).as_i64()))
                .unwrap_or(0);

            // resolution
            let resolution_selector = Selector::parse(".column-status div span").unwrap();
            bug.resolution = element
                .select(&resolution_selector)
                .find_map(|e| {
                    Some(Resolution::from(e.value().attr("title").unwrap_or("0")).as_i64())
                })
                .unwrap_or(0);

            // handler_id
            let handler_selector = Selector::parse(".column-status div a").unwrap();
            bug.handler_id = element
                .select(&handler_selector)
                .find_map(|e| {
                    //  handler
                    bug.handler = e.inner_html();
                    e.value().attr("href").and_then(|href| {
                        href.split('=').last().and_then(|id| id.parse::<i64>().ok())
                    })
                })
                .unwrap_or(0);

            // date_submitted
            let date_submitted_selector = Selector::parse(".column-date-submitted").unwrap();
            // <td class="column-date-submitted">2025-06-06</td>
            bug.date_submitted = element
                .select(&date_submitted_selector)
                .find_map(|e| {
                    let date_str = e.inner_html();
                    // 解析为 NaiveDate
                    let date = NaiveDate::parse_from_str(date_str.as_str(), "%Y-%m-%d").ok()?;
                    // 设为上海时区的0点
                    let datetime = Shanghai
                        .from_local_datetime(&date.and_time(NaiveTime::from_hms_opt(0, 0, 0)?))
                        .unwrap();
                    // 转为时间戳（秒）
                    Some(datetime.timestamp())
                })
                .unwrap_or(0);

            // last_updated
            let last_modified_selector =
                Selector::parse(".column-last-modified , .column-last-modified>span").unwrap();
            // <td class="column-last-modified">2025-06-18</td>
            bug.last_updated = element
                .select(&last_modified_selector)
                .find_map(|e| {
                    let date_str = e.inner_html();
                    // 解析为 NaiveDate
                    let date = NaiveDate::parse_from_str(date_str.as_str(), "%Y-%m-%d").ok()?;
                    // 设为上海时区的0点
                    let datetime = Shanghai
                        .from_local_datetime(&date.and_time(NaiveTime::from_hms_opt(0, 0, 0)?))
                        .unwrap();
                    // 转为时间戳（秒）
                    Some(datetime.timestamp())
                })
                .unwrap_or(0);

            // summary
            let summary_selector = Selector::parse(".column-summary").unwrap();
            bug.summary = element
                .select(&summary_selector)
                .map(|e| e.text().collect::<Vec<_>>().join(""))
                .collect::<Vec<String>>()
                .join("")
                .trim()
                .to_owned();
            bug
        })
        .collect();

    // total eg: 1 - 50 / 61
    let total_selector = Selector::parse("#bug_action .badge").unwrap();
    bug_list.total = document
        .select(&total_selector)
        .find_map(|e| {
            e.inner_html()
                .split('/')
                .last()
                .map(|s| s.trim().parse::<i64>().unwrap_or(0))
        })
        .unwrap_or(0);

    // page
    let page_selector = Selector::parse("#bug_action .active a").unwrap();
    bug_list.page = document
        .select(&page_selector)
        .find_map(|e| e.inner_html().parse::<i64>().ok())
        .unwrap_or(0);

    // limit
    let limit_selector = Selector::parse("#per_page_filter_target input").unwrap();
    bug_list.limit = document
        .select(&limit_selector)
        .find_map(|e| e.value().attr("value").and_then(|v| v.parse::<i64>().ok()))
        .unwrap_or(0);

    bug_list.bugs = results;

    Ok(bug_list)
}

// 查询my_view_detail数据
pub fn my_view_detail_data(
    document: &Html,
    host: &str,
    category_kv: &Vec<KV>,
    project_kv: &Vec<KV>,
) -> Result<BugInfo, String> {
    let mut bug = BugInfo::default();
    // bug_id
    let id_slector = Selector::parse(".bug-header-data .bug-id").unwrap();
    bug.bug_id = document
        .select(&id_slector)
        .find_map(|e| e.inner_html().parse::<i64>().ok())
        .unwrap_or(0);

    // project_id
    let project_selector = Selector::parse(".bug-header-data .bug-project").unwrap();
    bug.project_id = document
        .select(&project_selector)
        .find_map(|e| {
            // project
            bug.project = e.inner_html().trim().to_string();
            project_kv
                .find_by_value(bug.project.as_str())
                .map(|kv| kv.key.clone())
        })
        .unwrap_or("".to_owned());

    // category_id
    let category_selector = Selector::parse(".bug-header-data .bug-category").unwrap();
    bug.category_id = document
        .select(&category_selector)
        .find_map(|e| {
            category_kv
                .find_by_value(
                    e.text()
                        .last()
                        .unwrap_or_default()
                        .replace("[所有项目] ", ""),
                )
                .and_then(|kv| kv.key.parse::<i64>().ok())
        })
        .unwrap_or(0);

    // view_status
    let view_status_selector = Selector::parse(".bug-header-data .bug-view-status").unwrap();
    bug.view_state = document
        .select(&view_status_selector)
        .find_map(|e| Some(ViewStatus::from(e.inner_html().trim()).as_i64()))
        .unwrap_or(0);

    // date_submitted 2025-06-17 16:20
    let date_submitted_selector = Selector::parse(".bug-header-data .bug-date-submitted").unwrap();
    bug.date_submitted = document
        .select(&date_submitted_selector)
        .find_map(|e| {
            let date_str = e.inner_html();
            // 解析为 NaiveDate
            let date = NaiveDateTime::parse_from_str(date_str.as_str(), "%Y-%m-%d %H:%M").ok()?;
            // 设为上海时区的0点
            let datetime = Shanghai.from_local_datetime(&date).unwrap();
            // 转为时间戳（秒）
            Some(datetime.timestamp())
        })
        .unwrap_or(0);

    // last_updated
    let last_modified_selector = Selector::parse(".bug-header-data .bug-last-modified").unwrap();
    bug.last_updated = document
        .select(&last_modified_selector)
        .find_map(|e| {
            let date_str = e.inner_html();
            // 解析为 NaiveDate
            let date = NaiveDateTime::parse_from_str(date_str.as_str(), "%Y-%m-%d %H:%M").ok()?;
            // 设为上海时区的0点
            let datetime = Shanghai.from_local_datetime(&date).unwrap();
            // 转为时间戳（秒）
            Some(datetime.timestamp())
        })
        .unwrap_or(0);

    // last_updated_sec
    let last_updated_selector = Selector::parse("input[name=\"last_updated\"]").unwrap();
    bug.last_updated_sec = document
        .select(&last_updated_selector)
        .find_map(|e| {
            e.value()
                .attr("value")
                .map(|e| e.parse::<i64>().unwrap_or(0))
        })
        .unwrap_or(0);

    // reporter_id
    let handler_selector = Selector::parse("tbody .bug-reporter a").unwrap();
    bug.reporter_id = document
        .select(&handler_selector)
        .find_map(|e| {
            //  reporter
            bug.reporter = e.inner_html();
            e.value()
                .attr("href")
                .and_then(|href| href.split('=').last().and_then(|id| id.parse::<i64>().ok()))
        })
        .unwrap_or(0);

    // handler_id
    let handler_selector = Selector::parse("tbody .bug-assigned-to a").unwrap();
    bug.handler_id = document
        .select(&handler_selector)
        .find_map(|e| {
            //  handler
            bug.handler = e.inner_html();
            e.value()
                .attr("href")
                .and_then(|href| href.split('=').last().and_then(|id| id.parse::<i64>().ok()))
        })
        .unwrap_or(0);

    // priority
    let priority_selector = Selector::parse("tbody td.bug-priority").unwrap();
    bug.priority = document
        .select(&priority_selector)
        .find_map(|e| Some(Priority::from(e.inner_html().trim()).as_i64()))
        .unwrap_or(0);

    // severity
    let severity_selector = Selector::parse("tbody td.bug-severity").unwrap();
    bug.severity = document
        .select(&severity_selector)
        .find_map(|e| Some(Severity::from(e.inner_html().trim()).as_i64()))
        .unwrap_or(0);

    // reproducibility
    let reproducibility_selector = Selector::parse("tbody td.bug-reproducibility").unwrap();
    bug.reproducibility = document
        .select(&reproducibility_selector)
        .find_map(|e| Some(Reproducibility::from(e.inner_html().trim()).as_i64()))
        .unwrap_or(0);

    // status
    let status_selector = Selector::parse("tbody td.bug-status").unwrap();
    bug.status = document
        .select(&status_selector)
        .find_map(|e| Some(Status::from(e.text().last().unwrap_or(&"").trim()).as_i64()))
        .unwrap_or(0);

    // resolution
    let resolution_selector = Selector::parse("tbody td.bug-resolution").unwrap();
    bug.resolution = document
        .select(&resolution_selector)
        .find_map(|e| Some(Resolution::from(e.inner_html().trim()).as_i64()))
        .unwrap_or(0);

    // summary
    let summary_selector = Selector::parse("tbody td.bug-summary").unwrap();
    bug.summary = document
        .select(&summary_selector)
        .map(|e| {
            e.inner_html()
                .trim()
                .split(":")
                .nth(1)
                .unwrap_or_default()
                .trim()
                .to_string()
        })
        .collect::<Vec<String>>()
        .join("")
        .trim()
        .to_string();

    // description
    let description_selector = Selector::parse("tbody td.bug-description").unwrap();
    bug.description = document
        .select(&description_selector)
        .map(|e| e.inner_html().trim().to_string())
        .collect::<Vec<String>>()
        .join("")
        .trim()
        .to_string();

    // additional-information
    let additional_information_selector =
        Selector::parse("tbody td.bug-additional-information").unwrap();
    bug.additional_information = document
        .select(&additional_information_selector)
        .map(|e| e.inner_html().trim().to_string())
        .collect::<Vec<String>>()
        .join("")
        .trim()
        .to_string();

    // bug-steps-to-reproduce
    let steps_to_reproduce_selector = Selector::parse("tbody td.bug-steps-to-reproduce").unwrap();
    bug.steps_to_reproduce = document
        .select(&steps_to_reproduce_selector)
        .map(|e| e.inner_html().trim().to_string())
        .collect::<Vec<String>>()
        .join("")
        .trim()
        .to_string();

    // tags
    let tags_selector = Selector::parse("tbody td.bug-tags").unwrap();
    bug.tags = document
        .select(&tags_selector)
        .map(|e| e.inner_html().trim().to_string())
        .collect::<Vec<String>>()
        .join("")
        .trim()
        .to_string();

    // attachments
    let attachments_selector = Selector::parse(".bug-attach-tags .well.well-xs").unwrap();
    bug.attachments = document
        .select(&attachments_selector)
        .map(|e| {
            let mut size = 0;
            let mut url = String::new();
            let mut name = String::new();
            e.select(&Selector::parse("a:nth-of-type(2)").unwrap())
                .for_each(|e| {
                    //<a href="file_download.php?file_id=2365&amp;type=bug">image.png</a>&#32;(179,667&#32;字节)&#32;&nbsp;&nbsp;
                    url = format!("{}", e.value().attr("href").unwrap_or_default());
                    name = e.inner_html().trim().to_string();
                    e.next_sibling().map(|sibling| {
                        if sibling.value().is_text() {
                            size = sibling
                                .value()
                                .as_text()
                                .map(|text| text.trim().to_string())
                                .unwrap_or(String::new())
                                .replace("字节", "")
                                .replace("(", "")
                                .replace(")", "")
                                .replace(",", "")
                                .trim()
                                .parse::<i64>()
                                .unwrap_or(0);
                        }
                    });
                });
            FileInfo { size, url, name }
        })
        .collect();

    // bugnote_notess
    let bugnote_notes_selector = Selector::parse(".bugnote.visible-on-hover-toggle").unwrap();
    bug.bugnote_notes = document
        .select(&bugnote_notes_selector)
        .map(|e| {
            // handle_id
            let (handler_id, handler) = e
                .select(&Selector::parse(".no-margin .fa.fa-user.grey + a").unwrap())
                .find_map(|e|{
                    e.value().attr("href").and_then(|href| {
                        href.split('=').last().and_then(|id| id.parse::<i64>().ok())
                    })
                    .map(|id| (id, e.inner_html()))
                })
                .unwrap_or_default();
            let time = e
                .select(&Selector::parse(".no-margin.small.lighter").unwrap())
                .find_map(|e| {
                    let date_str = e.text().last().unwrap_or(&"").trim();
                    // 解析为 NaiveDate
                    let date = NaiveDateTime::parse_from_str(date_str, "%Y-%m-%d %H:%M").ok()?;
                    // 设为上海时区的0点
                    let datetime = Shanghai
                        .from_local_datetime(&date)
                        .unwrap();
                    // 转为时间戳（秒）
                    Some(datetime.timestamp())
                })
                .unwrap_or(0);
            let note_id = e
                .select(&Selector::parse("a.lighter").unwrap())
                .find_map(|e| e.inner_html().replace("~", "").trim().parse::<i64>().ok())
                .unwrap_or(0);
            let text = e
                .select(&Selector::parse(".bugnote-note.bugnote-public").unwrap())
                .find_map(|e| {
                    let t = e.text().collect::<Vec<_>>().join("").trim().to_string();
                    Some(t)
                })
                .unwrap_or_default();
            // attachments
            let attachments_selector = Selector::parse("td.bugnote-note>a , td.bugnote-note>.well.well-xs>.collapse-open>a , td.bugnote-note>.well.well-xs>a").unwrap();
            let attachments = e
                .select(&attachments_selector)
                .filter_map(|e| {
                    //<a href="file_download.php?file_id=2365&amp;type=bug">image.png</a>&#32;(179,667&#32;字节)&#32;&nbsp;&nbsp;
                    let mut size = 0;
                    let url = format!("{}",e.value().attr("href").unwrap_or_default().replace("&amp;", "&"));
                    let name = e.inner_html().trim().to_string();
                    e.next_sibling().map(|sibling| {
                        if sibling.value().is_text() {
                            size = sibling
                                .value()
                                .as_text()
                                .map(|text| text.trim().to_string())
                                .unwrap_or(String::new())
                                .replace("字节", "")
                                .replace("(", "")
                                .replace(")", "")
                                .replace(",", "")
                                .trim()
                                .parse::<i64>()
                                .unwrap_or(0);
                        }
                    });
                    if url.is_empty() || name.is_empty() || size == 0 {
                        return None;
                    };
                    Some(FileInfo { size, url, name })
                })
                .collect();
            BugNote {
                time,
                note_id,
                text: text.to_string(),
                handler_id,
                handler,
                attachments,
            }
        })
        .collect();

    // history
    let history_selector = Selector::parse("div#history tbody tr").unwrap();
    bug.change_history = document
        .select(&history_selector)
        .map(|e| {
            let hister_field = Selector::parse("td.small-caption").unwrap();
            let mut s = e.select(&hister_field);
            let updated_at = s
                .next()
                .map(|e| {
                    let date_str = e.inner_html().trim().to_string();
                    // 解析为 NaiveDate
                    let date = NaiveDateTime::parse_from_str(&date_str, "%Y-%m-%d %H:%M").ok()?;
                    // 设为上海时区的0点
                    let datetime = Shanghai.from_local_datetime(&date).unwrap();
                    // 转为时间戳（秒）
                    Some(datetime.timestamp())
                })
                .unwrap_or_default()
                .unwrap_or_default();
            let (handler_id, handler) = s
                .next()
                .map(|e| {
                    let handler = e
                        .select(&Selector::parse("a").unwrap())
                        .find_map(|e| {
                            e.value()
                                .attr("href")
                                .and_then(|href| {
                                    href.split('=').last().and_then(|id| id.parse::<i64>().ok())
                                })
                                .map(|id| (id, e.inner_html()))
                        })
                        .unwrap_or((0, "".to_owned()));
                    handler
                })
                .unwrap_or_default();
            let field = s
                .next()
                .map(|e| e.inner_html().trim().to_string())
                .unwrap_or_default();
            let change = s
                .next()
                .map(|e| e.inner_html().trim().to_string())
                .unwrap_or_default();
            ChangeHistory {
                bug_id: bug.bug_id,
                updated_at,
                handler_id,
                handler,
                field,
                change,
            }
        })
        .collect();
    Ok(bug)
}

// 查询项目列表数据
pub fn project_data(document: &Html) -> Result<Vec<KV>, String> {
    let slector = Selector::parse("#projects-list .project-link").unwrap();
    let r: Vec<KV> = document
        .select(&slector)
        .filter_map(|e| {
            let value = e.inner_html().replace("&nbsp;", "").trim().to_string();
            let key = e
                .value()
                .attr("href")
                .and_then(|href| href.split('=').last().and_then(|ids| Some(ids.to_owned())))?;
            Some(KV { key, value })
        })
        .collect();
    Ok(r)
}
// 查询分类列表数据
pub fn category_data(document: &Html) -> Result<Vec<KV>, String> {
    let slector = Selector::parse("#category_id option").unwrap();
    let r: Vec<KV> = document
        .select(&slector)
        .filter_map(|e| {
            let value = e.inner_html().replace("[所有项目]", "").trim().to_string();
            let key = e.value().attr("value").map(|id| id.to_owned())?;
            if key == "" {
                return None;
            }
            Some(KV { key, value })
        })
        .collect();
    Ok(r)
}
// 查询用户列表数据
pub fn users_data(document: &Html) -> Result<Vec<KV>, String> {
    let slector = Selector::parse("#handler_id option").unwrap();
    let r: Vec<KV> = document
        .select(&slector)
        .filter_map(|e| {
            let mut value = e
                .inner_html()
                .trim()
                .split('(')
                .next()
                .unwrap_or_default()
                .trim()
                .to_string();
            let key = e.value().attr("value").map(|id| id.to_owned())?;
            if key == "" {
                return None;
            }
            if key == "0" {
                value = "[任意]".to_string();
            }
            Some(KV { key, value })
        })
        .collect();
    Ok(r)
}

// 查询过滤参数
pub fn filters_data(document: &Html) -> Result<Vec<KV>, String> {
    let slector = Selector::parse(".input-xs option").unwrap();
    let r: Vec<KV> = document
        .select(&slector)
        .filter_map(|e| {
            let value = e.inner_html().trim().to_string();
            let key = e.value().attr("value").and_then(|id| Some(id.to_owned()))?;
            Some(KV { key, value })
        })
        .collect();
    Ok(r)
}

// 查询bug_update_token
pub fn get_page_token(document: &Html, token: &str) -> Result<String, String> {
    // 创建选择器
    let selector = Selector::parse(format!("input[name=\"{}\"]", token).as_str())
        .map_err(|e| format!("Selector 解析失败: {:?}", e))?;
    document
        .select(&selector)
        .find_map(|e| e.value().attr("value").map(|s| s.to_string()))
        .ok_or("not found".to_string())
}

// 查询user_id
pub fn get_user_id(document: &Html) -> Result<i64, String> {
    // 创建选择器
    let selector = Selector::parse(".widget-header>.widget-title:first-of-type>.white")
        .map_err(|e| format!("Selector 解析失败: {:?}", e))?;
    document
        .select(&selector)
        .find_map(|e| {
            e.value().attr("href").and_then(|herf| {
                let url = format!("http://{}", herf);
                let a = Url::parse(&url).ok()?;
                a.query_pairs().find_map(|(k, v)| {
                    if k == "handler_id" {
                        return v.parse::<i64>().ok();
                    }
                    None
                })
            })
        })
        .ok_or("not found".to_string())
}

// 查询错误
pub fn get_error_info(document: &Html) -> Option<String> {
    // 创建选择器
    let selector = Selector::parse(".alert.alert-danger").ok()?;
    document.select(&selector).find_map(|e| {
        e.text()
            .collect::<Vec<_>>()
            .join("")
            .trim()
            .to_string()
            .into()
    })
}

pub fn get_hash<T: Hash>(t: &T) -> u64 {
    let mut s = DefaultHasher::new();
    t.hash(&mut s);
    s.finish()
}

pub fn set_project_cookie(jar: Arc<Jar>, project_id: &str, host: &str) -> Result<(), String> {
    let cookie = format!("MANTIS_PROJECT_COOKIE={}", project_id).replace(";", "%3B");
    let origin = format!("http://{}", host);
    let url = &Url::parse(&origin).unwrap();
    jar.add_cookie_str(&cookie, url);
    Ok(())
}

pub fn println_cookies(jar: &Jar, host: &str) {
    let origin = format!("http://{}", host);
    let url = Url::parse(&origin).unwrap();
    jar.cookies(&url).and_then(|c| {
        let s = c.to_str().ok()?;
        println!("cookies:{}", s);
        Some(())
    });
}

fn get_mime_type_from_filename(filename: &str) -> String {
    let mime_type = mime_guess::from_path(filename).first_or_octet_stream();

    // Convert the Mime object to a String
    mime_type.to_string()
}
