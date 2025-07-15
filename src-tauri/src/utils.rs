use crate::enums::*;
use crate::model::*;

use chrono::{NaiveDate, TimeZone};
use chrono_tz::Asia::Shanghai;
use reqwest::cookie::{CookieStore, Jar};
use reqwest::header::{
    HeaderMap, HeaderValue, ACCEPT, ACCEPT_LANGUAGE, CACHE_CONTROL, CONNECTION, CONTENT_LENGTH,
    CONTENT_TYPE, HOST, ORIGIN, REFERER, UPGRADE_INSECURE_REQUESTS, USER_AGENT,
};
use reqwest::redirect::Policy;
use reqwest::Client;
use scraper::{Html, Selector};
use serde_html_form;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::sync::Arc;
use std::time::Duration;
use url::Url;

// login 页面
pub async fn login(
    jar: Arc<Jar>,
    username: &str,
    password: &str,
) -> Result<String, Box<dyn std::error::Error>> {
    // 地址
    let url = "http://bug.test.com/login.php";

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
    headers.insert(ORIGIN, HeaderValue::from_static("http://bug.test.com"));
    headers.insert(
        REFERER,
        HeaderValue::from_static("http://bug.test.com/login_password_page.php"),
    );
    headers.insert(UPGRADE_INSECURE_REQUESTS, HeaderValue::from_static("1"));
    headers.insert(USER_AGENT,HeaderValue::from_static("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/137.0.0.0 Safari/537.36 Edg/137.0.0.0"));
    headers.insert(CONNECTION, HeaderValue::from_static("keep-alive"));
    headers.insert(HOST, HeaderValue::from_static("bug.test.com"));
    let jar1 = jar.clone();
    // 创建 reqwest 客户端
    let client = Client::builder()
        .timeout(Duration::from_secs(2))
        .cookie_provider(jar)
        .danger_accept_invalid_certs(true) // --insecure
        .default_headers(headers)
        .redirect(Policy::custom(|attempt| {
            // 自定义重定向策略
            if attempt.previous().len() > 5 {
                attempt.error("too many redirects")
            } else if attempt
                .url()
                .as_str()
                .eq("http://bug.test.com/login_cookie_test.php?return=index.php")
            {
                attempt.stop()
            } else {
                attempt.stop()
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

    resp.text().await?;
    let cookie = jar1
        .cookies(&Url::parse("http://bug.test.com").unwrap())
        .map(|e| format!("{}", e.to_str().unwrap()))
        .ok_or("".to_string())?;
    Ok(cookie)
}

// my_view_page 页面
pub async fn my_view_page(jar: Arc<Jar>) -> Result<String, Box<dyn std::error::Error>> {
    let url = "http://bug.test.com/my_view_page.php";

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
) -> Result<String, Box<dyn std::error::Error>> {
    // 地址
    let url = "http://bug.test.com/view_all_set.php";

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
    headers.insert(ORIGIN, HeaderValue::from_static("http://bug.test.com"));
    headers.insert(
        REFERER,
        HeaderValue::from_static("http://bug.test.com/view_all_bug_page.php"),
    );
    headers.insert(UPGRADE_INSECURE_REQUESTS, HeaderValue::from_static("1"));
    headers.insert(USER_AGENT,HeaderValue::from_static("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/137.0.0.0 Safari/537.36 Edg/137.0.0.0"));
    headers.insert(CONNECTION, HeaderValue::from_static("keep-alive"));
    headers.insert(HOST, HeaderValue::from_static("bug.test.com"));

    // 创建 reqwest 客户端
    let client = Client::builder()
        .timeout(Duration::from_secs(2))
        .cookie_provider(jar)
        .danger_accept_invalid_certs(true) // --insecure
        .default_headers(headers)
        .redirect(Policy::custom(|attempt| {
            // 自定义重定向策略
            attempt.follow()
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

    let text = resp.text().await.unwrap();
    Ok(text)
}

// my_view_detail 页面
pub async fn my_view_detail(jar: Arc<Jar>, id: i64) -> Result<String, String> {
    let url = format!("http://bug.test.com/view.php?id={}", id);

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
pub async fn bug_update_page(jar: Arc<Jar>, bug: UpdateToken) -> Result<String, String> {
    let url = "http://bug.test.com/bug_update_page.php";

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
pub async fn bug_update(jar: Arc<Jar>, bug: UpdateBug) -> Result<String, String> {
    let url = "http://bug.test.com/bug_update.php";

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

// bug_change_status_page 页面
pub async fn bug_change_status_page(jar: Arc<Jar>, bug: UpdateToken) -> Result<String, String> {
    let url = "http://bug.test.com/bug_change_status_page.php";

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

// 查询 Jar 中的 Cookie
pub fn find_jar_cookies(
    jar: &Jar,
    cookie_name: &str,
) -> Result<String, Box<dyn std::error::Error>> {
    let url = Url::parse("http://bug.test.com/")?;

    let cookie_prefix = format!("{}=", cookie_name);

    let cookies = jar
        .cookies(&url)
        .ok_or_else(|| format!("URL {} 没有关联的 Cookie", url))?;

    let cookie_str = cookies
        .to_str()
        .map_err(|e| format!("Cookie 字符串解析失败: {}", e))?;

    let cookie_value = cookie_str
        .split(';')
        .find(|s| s.trim().starts_with(&cookie_prefix))
        .and_then(|s| s.trim().strip_prefix(&cookie_prefix))
        .ok_or_else(|| format!("未找到名为 {} 的 Cookie", cookie_name))?;

    Ok(cookie_value.to_string())
}

// 查询符合条件的节点
pub fn find_all_tasks(
    html_content: &str,
    selector_text: &str,
) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    // 解析 HTML
    let document = Html::parse_document(html_content);

    // 创建选择器
    let selector =
        Selector::parse(selector_text).map_err(|e| format!("Selector 解析失败: {:?}", e))?;

    // 收集所有匹配元素的文本
    let results: Vec<String> = document
        .select(&selector)
        .map(|element| {
            let text = element.text().collect::<Vec<_>>().join("");
            text
        })
        .collect();

    if results.is_empty() {
        Err("未找到任何匹配的链接元素".into())
    } else {
        Ok(results)
    }
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

// 查询view_all_set数据
pub fn view_all_set_data(document: &Html) -> Result<BugList, Box<dyn std::error::Error>> {
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
                .find_map(|e| Some(Priority::from(e.value().attr("title").unwrap_or("")).as_i64()))
                .unwrap_or(0);

            // category_id
            let category_selector = Selector::parse(".column-category div").unwrap();
            bug.category_id = element
                .select(&category_selector)
                .find_map(|e| Some(Category::from(e.text().last().unwrap_or(&"").trim()).as_i64()))
                .unwrap_or(0);

            // project_id
            let project_selector = Selector::parse(".column-category div span a").unwrap();
            bug.project_id = element
                .select(&project_selector)
                .find_map(|e| {
                    // project
                    bug.project = e.inner_html().trim().to_string();
                    Some(Project::from(bug.project.as_str()).as_i64())
                })
                .unwrap_or(0);

            // severity
            let severity_selector = Selector::parse(".column-severity").unwrap();
            bug.severity = element
                .select(&severity_selector)
                .find_map(|e| Some(Severity::from(e.inner_html().trim()).as_i64()))
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
                        href.split('=').nth(1).and_then(|id| id.parse::<i64>().ok())
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
                        .from_local_datetime(&date.and_hms_opt(0, 0, 0)?)
                        .unwrap();
                    // 转为时间戳（秒）
                    Some(datetime.timestamp())
                })
                .unwrap_or(0);

            // last_updated
            let last_modified_selector = Selector::parse(".column-last-modified").unwrap();
            // <td class="column-last-modified">2025-06-18</td>
            bug.last_updated = element
                .select(&last_modified_selector)
                .find_map(|e| {
                    let date_str = e.inner_html();
                    // 解析为 NaiveDate
                    let date = NaiveDate::parse_from_str(date_str.as_str(), "%Y-%m-%d").ok()?;
                    // 设为上海时区的0点
                    let datetime = Shanghai
                        .from_local_datetime(&date.and_hms_opt(0, 0, 0)?)
                        .unwrap();
                    // 转为时间戳（秒）
                    Some(datetime.timestamp())
                })
                .unwrap_or(0);

            // summary
            let summary_selector = Selector::parse(".column-summary a").unwrap();
            bug.summary = element
                .select(&summary_selector)
                .map(|e| e.inner_html().trim().to_string())
                .collect::<Vec<String>>()
                .join("")
                .trim()
                .to_string();

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
pub fn my_view_detail_data(document: &Html) -> Result<BugInfo, String> {
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
            Some(Project::from(bug.project.as_str()).as_i64())
        })
        .unwrap_or(0);

    // category_id
    let category_selector = Selector::parse(".bug-header-data .bug-category").unwrap();
    bug.category_id = document
        .select(&category_selector)
        .find_map(|e| Some(Category::from(e.inner_html().trim()).as_i64()))
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
            let date = NaiveDate::parse_from_str(date_str.as_str(), "%Y-%m-%d %H:%M").ok()?;
            // 设为上海时区的0点
            let datetime = Shanghai
                .from_local_datetime(&date.and_hms_opt(0, 0, 0)?)
                .unwrap();
            // 转为时间戳（秒）
            Some(datetime.timestamp())
        })
        .unwrap_or(0);

    // last_updated
    let last_modified_selector = Selector::parse("input[name=\"last_updated\"]").unwrap();
    bug.last_updated = document
        .select(&last_modified_selector)
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
                .and_then(|href| href.split('=').nth(1).and_then(|id| id.parse::<i64>().ok()))
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
                .and_then(|href| href.split('=').nth(1).and_then(|id| id.parse::<i64>().ok()))
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
                .unwrap_or("")
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
    let attachments_selector = Selector::parse(".bug-attach-tags .collapse-open.noprint").unwrap();
    bug.attachments = document
        .select(&attachments_selector)
        .map(|e| {
            let mut size = 0;
            let mut url = String::new();
            let mut name = String::new();
            e.select(&Selector::parse("a:nth-of-type(2)").unwrap())
                .for_each(|e| {
                    //<a href="file_download.php?file_id=2365&amp;type=bug">image.png</a>&#32;(179,667&#32;字节)&#32;&nbsp;&nbsp;
                    url = format!(
                        "http://bug.test.com/{}",
                        e.value().attr("href").unwrap_or("")
                    );
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

    // bugnote_notes
    let bugnote_notes_selector = Selector::parse(".bugnote.visible-on-hover-toggle").unwrap();
    bug.bugnote_notes = document
        .select(&bugnote_notes_selector)
        .map(|e| {
            let time = e
                .select(&Selector::parse(".no-margin.small.lighter").unwrap())
                .find_map(|e| {
                    let date_str = e.text().last().unwrap_or(&"").trim();
                    // 解析为 NaiveDate
                    let date = NaiveDate::parse_from_str(date_str, "%Y-%m-%d %H:%M").ok()?;
                    // 设为上海时区的0点
                    let datetime = Shanghai
                        .from_local_datetime(&date.and_hms_opt(0, 0, 0)?)
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
                .find_map(|e| e.text().find(|_| true))
                .unwrap_or("");
            // attachments
            let attachments_selector = Selector::parse(".collapse-open.noprint").unwrap();
            let attachments = document
                .select(&attachments_selector)
                .map(|e| {
                    let mut size = 0;
                    let mut url = String::new();
                    let mut name = String::new();
                    e.select(&Selector::parse("a:nth-of-type(2)").unwrap())
                        .for_each(|e| {
                            //<a href="file_download.php?file_id=2365&amp;type=bug">image.png</a>&#32;(179,667&#32;字节)&#32;&nbsp;&nbsp;
                            url = format!(
                                "http://bug.test.com/{}",
                                e.value().attr("href").unwrap_or("")
                            );
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
            BugNote {
                time,
                note_id,
                text: text.to_string(),
                attachments,
            }
        })
        .collect();

    Ok(bug)
}

pub fn get_hash<T: Hash>(t: &T) -> u64 {
    let mut s = DefaultHasher::new();
    t.hash(&mut s);
    s.finish()
}
