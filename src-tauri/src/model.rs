use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Serialize, Deserialize, Hash, Clone)]
pub struct Bug {
    pub bug_id: i64,
    pub issue_notes_count: i64, //问题说明数量
    pub project_id: String,     //项目id
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

#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct BugInfo {
    pub bug_id: i64,
    pub project_id: String,                 //项目id
    pub project: String,                    //项目
    pub category_id: i64,                   //类别ID
    pub view_state: i64,                    //查看权限
    pub date_submitted: i64,                //报告日期
    pub last_updated: i64,                  //最后更新时间
    pub last_updated_sec: i64,              //最后更新时间(用于更新的)
    pub reporter_id: i64,                   //报告员id
    pub reporter: String,                   //报告员
    pub handler_id: i64,                    //处理人
    pub handler: String,                    //处理人名称
    pub priority: i64,                      //优先级
    pub severity: i64,                      //严重性
    pub reproducibility: i64,               //出现频率
    pub status: i64,                        //状态
    pub resolution: i64,                    //处理状况
    pub summary: String,                    //摘要
    pub description: String,                //描述
    pub additional_information: String,     //附注
    pub steps_to_reproduce: String,         //问题重现步骤
    pub tags: String,                       //标签
    pub bugnote_notes: Vec<BugNote>,        //注释列表
    pub attachments: Vec<FileInfo>,         //附件
    pub change_history: Vec<ChangeHistory>, //变更历史
}

#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct FileInfo {
    pub size: i64,
    pub url: String,
    pub name: String,
}

#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct BugNote {
    pub note_id: i64,
    pub time: i64,
    pub text: String,
    pub handler_id: i64, //处理人
    pub handler: String, //处理人名称
    pub attachments: Vec<FileInfo>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize, Hash)]
pub struct ChangeHistory {
    pub bug_id: i64,     //bug_id
    pub updated_at: i64, //变更时间
    pub handler_id: i64, //处理人
    pub handler: String, //处理人名称
    pub field: String,   //字段
    pub change: String,  //变更内容
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct FindBugListParams {
    #[serde(skip)]
    pub project_id: String,
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
    pub data_filter_id: String,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct LoginInfo {
    pub logined: bool,
    pub username: String,
    pub user_id: i64,
    pub read_msg: String,
}

#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct BugNoteAdd {
    pub bugnote_add_token: String,
    pub bug_id: i64,
    pub bugnote_text: String,
    pub max_file_size: i64,
    pub file_path: Vec<String>,
    pub binary_file: Vec<(String, Vec<u8>)>, // (filename, filecontent)
}

#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct OperateLogs {
    pub handler: String,
    pub handler_id: i64,
    pub bug_id: i64,
    pub content: String,
    pub last_updated: i64,
}

#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct ReportBug {
    pub bug_report_token: String,   //【必填】
    pub m_id: i64,                  // 0
    pub project_id: i64,            //【必填】项目id
    pub category_id: i64,           //【必填】分类id
    pub summary: String,            //【必填】摘要
    pub description: String,        //【必填】描述
    pub reproducibility: i64,       //出现频率
    pub severity: i64,              //严重性
    pub priority: i64,              //优先级
    pub handler_id: i64,            //处理人
    pub platform: String,           //平台
    pub os: String,                 //操作系统
    pub os_build: String,           //操作系统版本
    pub steps_to_reproduce: String, //重现步骤
    pub additional_info: String,    //附加信息
    pub tag_string: String,         //标签字符串（逗号分隔）
    pub tag_select: String,         //选择已存在标签
    pub view_state: i64,            //查看权限 10=公开 50=私有
    pub report_stay: i64,           //报告后停留在当前页面 1=是 0=否

    pub max_file_size: i64,
    pub file_path: Vec<String>,
    pub binary_file: Vec<(String, Vec<u8>)>, // (filename, filecontent)
}
