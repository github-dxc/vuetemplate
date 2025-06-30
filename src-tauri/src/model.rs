use serde::{Serialize,Deserialize};

#[derive(Debug,Default,Serialize)]
pub struct Bug {
    pub bug_id: i64,
    pub issue_notes_count: i64,//问题说明数量
    pub project_id: i64,//项目id
    pub project: String,//项目
    pub category_id: i64,//类别ID
    pub handler_id: i64,//处理人
    pub handler: String,//处理人名称
    pub priority: i64,//优先级
    pub attachments: i64,//附件计数
    pub severity: i64,//严重性
    pub status: i64,//状态
    pub resolution: i64,//处理状况
    pub date_submitted: i64,//报告日期
    pub last_updated: i64,//最后更新时间
    pub summary: String,//摘要
}

#[derive(Serialize)]
pub struct UpdateToken {
    pub bug_update_token: String,
    pub bug_id: i64,
}

#[derive(Serialize, Default)]
pub struct UpdateBug {
    pub bug_update_token: String,
    pub bug_id: i64,
    pub last_updated: i64,//更新时间
    pub category_id: i64,
    pub view_state: i64,
    pub handler_id: i64,//处理人
    pub priority: i64,//优先级
    pub severity: i64,//严重性
    pub reproducibility: i64,//出现频率
    pub status: i64,//状态
    pub resolution: i64,//处理状况
    pub summary: String,//摘要
    pub description: String,//描述
    pub additional_information: String,
    pub bugnote_text: String,//问题注释
}

#[derive(Debug, Default, Serialize)]
pub struct BugList {
    pub bugs: Vec<Bug>,
    pub total: i64,
    pub page: i64,
    pub limit: i64,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct FindBugListParams {
    pub r#type: i64,
    pub view_type: String,
    #[serde(default, rename = "reporter_id[]")]
    pub reporter_id: Vec<i64>,
    #[serde(default, rename = "handler_id[]")]
    pub handler_id: Vec<i64>,
    #[serde(default, rename = "monitor_user_id[]")]
    pub monitor_user_id: Vec<i64>,
    #[serde(default, rename = "note_user_id[]")]
    pub note_user_id: Vec<i64>,
    #[serde(default, rename = "priority[]")]
    pub priority: Vec<i64>,
    #[serde(default, rename = "severity[]")]
    pub severity: Vec<i64>,
    pub view_state: i64,
    pub sticky: i64,
    #[serde(default, rename = "category_id[]")]
    pub category_id: Vec<i64>,
    #[serde(default, rename = "hide_status[]")]
    pub hide_status: Vec<i64>,
    #[serde(default, rename = "status[]")]
    pub status: Vec<i64>,
    #[serde(default, rename = "resolution[]")]
    pub resolution: Vec<i64>,
    #[serde(default, rename = "profile_id[]")]
    pub profile_id: Vec<i64>,
    #[serde(default, rename = "platform[]")]
    pub platform: Vec<i64>,
    #[serde(default, rename = "os[]")]
    pub os: Vec<i64>,
    #[serde(default, rename = "os_build[]")]
    pub os_build: Vec<i64>,
    pub relationship_type: i64,
    pub relationship_bug: i64,
    pub tag_string: String,
    pub per_page: i64,
    #[serde(default, rename = "sort[]")]
    pub sort: Vec<String>,
    #[serde(default, rename = "dir[]")]
    pub dir: Vec<String>,
    pub match_type: i64,
    pub highlight_changed: i64,
    pub search: String,
    pub filter_submit: String,
}
