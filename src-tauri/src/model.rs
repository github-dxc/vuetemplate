use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Serialize, Deserialize, Hash, Clone)]
pub struct Bug {
    pub bug_id: i64,
    pub issue_notes_count: i64, //问题说明数量
    pub project_id: String,        //项目id
    pub project: String,        //项目
    pub category_id: i64,       //类别ID
    pub handler_id: i64,        //处理人
    pub handler: String,        //处理人名称
    pub priority: i64,          //优先级
    pub attachments: i64,       //附件计数
    pub severity: i64,          //严重性
    pub status: i64,            //状态
    pub resolution: i64,        //处理状况
    pub date_submitted: i64,    //报告日期
    pub last_updated: i64,      //最后更新时间
    pub summary: String,        //摘要
}

#[derive(Serialize, Default)]
pub struct UpdateToken {
    pub bug_update_page_token: String,
    pub bug_id: i64,
}

#[derive(Serialize, Default, Debug)]
pub struct UpdateBug {
    pub bug_update_token: String,
    pub bug_id: i64,
    pub last_updated: i64, //更新时间
    pub category_id: i64,
    pub view_state: i64,
    pub handler_id: i64,                //处理人
    pub priority: i64,                  //优先级
    pub severity: i64,                  //严重性
    pub reproducibility: i64,           //出现频率
    pub status: i64,                    //状态
    pub resolution: i64,                //处理状况
    pub summary: String,                //摘要
    pub description: String,            //描述
    pub additional_information: String, //附注
    pub steps_to_reproduce: String,     //问题重现步骤
    pub bugnote_text: String,           //问题注释
}

#[derive(Debug, Default, Serialize, Clone)]
pub struct BugList {
    pub bugs: Vec<Bug>,
    pub total: i64,
    pub page: i64,
    pub limit: i64,
}

#[derive(Debug, Default, Serialize)]
pub struct BugInfo {
    pub bug_id: i64,
    pub project_id: String,                //项目id
    pub project: String,                //项目
    pub category_id: i64,               //类别ID
    pub view_state: i64,                //查看权限
    pub date_submitted: i64,            //报告日期
    pub last_updated: i64,              //最后更新时间
    pub reporter_id: i64,               //报告员id
    pub reporter: String,               //报告员
    pub handler_id: i64,                //处理人
    pub handler: String,                //处理人名称
    pub priority: i64,                  //优先级
    pub severity: i64,                  //严重性
    pub reproducibility: i64,           //出现频率
    pub status: i64,                    //状态
    pub resolution: i64,                //处理状况
    pub summary: String,                //摘要
    pub description: String,            //描述
    pub additional_information: String, //附注
    pub steps_to_reproduce: String,     //问题重现步骤
    pub tags: String,                   //标签
    pub bugnote_notes: Vec<BugNote>,    //注释列表
    pub attachments: Vec<FileInfo>,     //附件
}

#[derive(Debug, Default, Serialize)]
pub struct FileInfo {
    pub size: i64,
    pub url: String,
    pub name: String,
}

#[derive(Debug, Default, Serialize)]
pub struct BugNote {
    pub note_id: i64,
    pub time: i64,
    pub text: String,
    pub attachments: Vec<FileInfo>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct FindBugListParams {
    #[serde(default, rename = "type")]
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
    pub category_id: Vec<String>,
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

#[derive(Debug, Default, Serialize, Clone)]
pub struct VersionInfo {
    pub current_version: String,
    pub version: String,
    pub target: String,
    pub update_time: i64,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct FiltersParams {
    pub view_type: String,
    pub filter_target: String,
    pub data_filter_id: String
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct LoginInfo {
    pub logined: bool,
    pub username: String,
}