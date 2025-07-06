use serde::Serialize;

// <option value="10">无</option>
// <option value="20">低</option>
// <option value="30" selected="selected">中</option>
// <option value="40">高</option>
// <option value="50">紧急</option>
// <option value="60">非常紧急</option>

#[derive(Debug, Clone, Serialize)]
pub struct KV {
    pub key: i64,
    pub value: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Priority {
    Unknown = 0,
    None = 10,
    Low = 20,
    Middle = 30,
    High = 40,
    Emergency = 50,
    VeryEmergency = 60,
}
impl Priority {
    pub fn as_str(&self) -> &'static str {
        match self {
            Priority::Unknown => "[任意]",
            Priority::None => "无",
            Priority::Low => "低",
            Priority::Middle => "中",
            Priority::High => "高",
            Priority::Emergency => "紧急",
            Priority::VeryEmergency => "非常紧急",
        }
    }
    pub fn as_i64(&self) -> i64 {
        *self as i64
    }
    pub fn kv() -> Vec<KV> {
        vec![
            KV {
                key: Priority::None.as_i64(),
                value: Priority::None.as_str().to_string(),
            },
            KV {
                key: Priority::Low.as_i64(),
                value: Priority::Low.as_str().to_string(),
            },
            KV {
                key: Priority::Middle.as_i64(),
                value: Priority::Middle.as_str().to_string(),
            },
            KV {
                key: Priority::High.as_i64(),
                value: Priority::High.as_str().to_string(),
            },
            KV {
                key: Priority::Emergency.as_i64(),
                value: Priority::Emergency.as_str().to_string(),
            },
            KV {
                key: Priority::VeryEmergency.as_i64(),
                value: Priority::VeryEmergency.as_str().to_string(),
            },
        ]
    }
}
impl std::fmt::Display for Priority {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}
impl From<&str> for Priority {
    fn from(s: &str) -> Self {
        match s {
            "无" => Priority::None,
            "低" => Priority::Low,
            "中" => Priority::Middle,
            "高" => Priority::High,
            "紧急" => Priority::Emergency,
            "非常紧急" => Priority::VeryEmergency,
            _ => Priority::Unknown,
        }
    }
}
impl From<i64> for Priority {
    fn from(n: i64) -> Self {
        match n {
            10 => Priority::None,
            20 => Priority::Low,
            30 => Priority::Middle,
            40 => Priority::High,
            50 => Priority::Emergency,
            60 => Priority::VeryEmergency,
            _ => Priority::Unknown,
        }
    }
}

// <option value="1">General</option>
// <option value="6">UI设计</option>
// <option value="5">产品设计</option>
// <option value="8">前端</option>
// <option value="9">后端</option>
// <option value="10">大数据</option>
// <option value="11">安卓客户端</option>

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Category {
    Unknown = 0,
    General = 1,
    UIDesign = 6,
    ProductDesign = 5,
    Frontend = 8,
    Backend = 9,
    BigData = 10,
    AndroidClient = 11,
}
impl Category {
    pub fn as_str(&self) -> &'static str {
        match self {
            Category::Unknown => "[任意]",
            Category::General => "通用",
            Category::UIDesign => "UI设计",
            Category::ProductDesign => "产品设计",
            Category::Frontend => "前端",
            Category::Backend => "后端",
            Category::BigData => "大数据",
            Category::AndroidClient => "安卓客户端",
        }
    }
    pub fn as_i64(&self) -> i64 {
        *self as i64
    }
    pub fn kv() -> Vec<KV> {
        vec![
            KV {
                key: Category::General.as_i64(),
                value: Category::General.as_str().to_string(),
            },
            KV {
                key: Category::UIDesign.as_i64(),
                value: Category::UIDesign.as_str().to_string(),
            },
            KV {
                key: Category::ProductDesign.as_i64(),
                value: Category::ProductDesign.as_str().to_string(),
            },
            KV {
                key: Category::Frontend.as_i64(),
                value: Category::Frontend.as_str().to_string(),
            },
            KV {
                key: Category::Backend.as_i64(),
                value: Category::Backend.as_str().to_string(),
            },
            KV {
                key: Category::BigData.as_i64(),
                value: Category::BigData.as_str().to_string(),
            },
            KV {
                key: Category::AndroidClient.as_i64(),
                value: Category::AndroidClient.as_str().to_string(),
            },
        ]
    }
}
impl std::fmt::Display for Category {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}
impl From<&str> for Category {
    fn from(s: &str) -> Self {
        match s {
            "通用" => Category::General,
            "UI设计" => Category::UIDesign,
            "产品设计" => Category::ProductDesign,
            "前端" => Category::Frontend,
            "后端" => Category::Backend,
            "大数据" => Category::BigData,
            "安卓客户端" => Category::AndroidClient,
            _ => Category::Unknown,
        }
    }
}
impl From<i64> for Category {
    fn from(n: i64) -> Self {
        match n {
            1 => Category::General,
            6 => Category::UIDesign,
            5 => Category::ProductDesign,
            8 => Category::Frontend,
            9 => Category::Backend,
            10 => Category::BigData,
            11 => Category::AndroidClient,
            _ => Category::Unknown,
        }
    }
}

// <option value="10">新功能</option>
// <option value="20">细节</option>
// <option value="30">文字</option>
// <option value="40">小调整</option>
// <option value="50" selected="selected">小错误</option>
// <option value="60">很严重</option>
// <option value="70">崩溃</option>
// <option value="80">宕机</option>

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Severity {
    Unknown = 0,
    NewFeature = 10,
    Detail = 20,
    Text = 30,
    SmallAdjustment = 40,
    SmallError = 50,
    SeriousError = 60,
    Crash = 70,
    Downtime = 80,
}
impl Severity {
    pub fn as_str(&self) -> &'static str {
        match self {
            Severity::Unknown => "[任意]",
            Severity::NewFeature => "新功能",
            Severity::Detail => "细节",
            Severity::Text => "文字",
            Severity::SmallAdjustment => "小调整",
            Severity::SmallError => "小错误",
            Severity::SeriousError => "很严重",
            Severity::Crash => "崩溃",
            Severity::Downtime => "宕机",
        }
    }
    pub fn as_i64(&self) -> i64 {
        *self as i64
    }
    pub fn kv() -> Vec<KV> {
        vec![
            KV {
                key: Severity::NewFeature.as_i64(),
                value: Severity::NewFeature.as_str().to_string(),
            },
            KV {
                key: Severity::Detail.as_i64(),
                value: Severity::Detail.as_str().to_string(),
            },
            KV {
                key: Severity::Text.as_i64(),
                value: Severity::Text.as_str().to_string(),
            },
            KV {
                key: Severity::SmallAdjustment.as_i64(),
                value: Severity::SmallAdjustment.as_str().to_string(),
            },
            KV {
                key: Severity::SmallError.as_i64(),
                value: Severity::SmallError.as_str().to_string(),
            },
            KV {
                key: Severity::SeriousError.as_i64(),
                value: Severity::SeriousError.as_str().to_string(),
            },
            KV {
                key: Severity::Crash.as_i64(),
                value: Severity::Crash.as_str().to_string(),
            },
            KV {
                key: Severity::Downtime.as_i64(),
                value: Severity::Downtime.as_str().to_string(),
            },
        ]
    }
}
impl std::fmt::Display for Severity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}
impl From<&str> for Severity {
    fn from(s: &str) -> Self {
        match s {
            "新功能" => Severity::NewFeature,
            "细节" => Severity::Detail,
            "文字" => Severity::Text,
            "小调整" => Severity::SmallAdjustment,
            "小错误" => Severity::SmallError,
            "很严重" => Severity::SeriousError,
            "崩溃" => Severity::Crash,
            "宕机" => Severity::Downtime,
            _ => Severity::Unknown,
        }
    }
}
impl From<i64> for Severity {
    fn from(n: i64) -> Self {
        match n {
            10 => Severity::NewFeature,
            20 => Severity::Detail,
            30 => Severity::Text,
            40 => Severity::SmallAdjustment,
            50 => Severity::SmallError,
            60 => Severity::SeriousError,
            70 => Severity::Crash,
            80 => Severity::Downtime,
            _ => Severity::Unknown,
        }
    }
}

// <option value="0" selected="selected">[任意]</option>
// <option value="10">新建</option>
// <option value="20">反馈</option>
// <option value="30">认可</option>
// <option value="40">已确认</option>
// <option value="50">已分配</option>
// <option value="80">已解决</option>
// <option value="81">已发布</option>
// <option value="82">已验证</option>
// <option value="83">不予解决</option>
// <option value="84">延迟修复</option>
// <option value="85">重新打开</option>
// <option value="90">已关闭</option>

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Status {
    Unknown = 0,
    New = 10,
    Feedback = 20,
    Acknowledged = 30,
    Confirmed = 40,
    Assigned = 50,
    Resolved = 80,
    Released = 81,
    Verified = 82,
    NotFixed = 83,
    DelayedFix = 84,
    Reopened = 85,
    Closed = 90,
}
impl Status {
    pub fn as_str(&self) -> &'static str {
        match self {
            Status::Unknown => "[任意]",
            Status::New => "新建",
            Status::Feedback => "反馈",
            Status::Acknowledged => "认可",
            Status::Confirmed => "已确认",
            Status::Assigned => "已分配",
            Status::Resolved => "已解决",
            Status::Released => "已发布",
            Status::Verified => "已验证",
            Status::NotFixed => "不予解决",
            Status::DelayedFix => "延迟修复",
            Status::Reopened => "重新打开",
            Status::Closed => "已关闭",
        }
    }
    pub fn as_i64(&self) -> i64 {
        *self as i64
    }
    pub fn kv() -> Vec<KV> {
        vec![
            KV {
                key: Status::New.as_i64(),
                value: Status::New.as_str().to_string(),
            },
            KV {
                key: Status::Feedback.as_i64(),
                value: Status::Feedback.as_str().to_string(),
            },
            KV {
                key: Status::Acknowledged.as_i64(),
                value: Status::Acknowledged.as_str().to_string(),
            },
            KV {
                key: Status::Confirmed.as_i64(),
                value: Status::Confirmed.as_str().to_string(),
            },
            KV {
                key: Status::Assigned.as_i64(),
                value: Status::Assigned.as_str().to_string(),
            },
            KV {
                key: Status::Resolved.as_i64(),
                value: Status::Resolved.as_str().to_string(),
            },
            KV {
                key: Status::Released.as_i64(),
                value: Status::Released.as_str().to_string(),
            },
            KV {
                key: Status::Verified.as_i64(),
                value: Status::Verified.as_str().to_string(),
            },
            KV {
                key: Status::NotFixed.as_i64(),
                value: Status::NotFixed.as_str().to_string(),
            },
            KV {
                key: Status::DelayedFix.as_i64(),
                value: Status::DelayedFix.as_str().to_string(),
            },
            KV {
                key: Status::Reopened.as_i64(),
                value: Status::Reopened.as_str().to_string(),
            },
            KV {
                key: Status::Closed.as_i64(),
                value: Status::Closed.as_str().to_string(),
            },
        ]
    }
}
impl std::fmt::Display for Status {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}
impl From<&str> for Status {
    fn from(s: &str) -> Self {
        match s {
            "新建" => Status::New,
            "反馈" => Status::Feedback,
            "认可" => Status::Acknowledged,
            "已确认" => Status::Confirmed,
            "已分配" => Status::Assigned,
            "已解决" => Status::Resolved,
            "已发布" => Status::Released,
            "已验证" => Status::Verified,
            "不予解决" => Status::NotFixed,
            "延迟修复" => Status::DelayedFix,
            "重新打开" => Status::Reopened,
            "已关闭" => Status::Closed,
            _ => Status::Unknown,
        }
    }
}
impl From<i64> for Status {
    fn from(n: i64) -> Self {
        match n {
            0 => Status::Unknown,
            10 => Status::New,
            20 => Status::Feedback,
            30 => Status::Acknowledged,
            40 => Status::Confirmed,
            50 => Status::Assigned,
            80 => Status::Resolved,
            81 => Status::Released,
            82 => Status::Verified,
            83 => Status::NotFixed,
            84 => Status::DelayedFix,
            85 => Status::Reopened,
            90 => Status::Closed,
            _ => Status::Unknown,
        }
    }
}

// <option value="0" selected="selected">[任意]</option>
// <option value="10">未处理</option>
// <option value="20">已修正</option>
// <option value="30">重新打开</option>
// <option value="40">无法重现</option>
// <option value="50">无法修复</option>
// <option value="60">重复问题</option>
// <option value="70">不必改</option>
// <option value="80">稍后处理</option>
// <option value="90">不做修改</option>

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Resolution {
    Unknown = 0,
    Unprocessed = 10,
    Fixed = 20,
    Reopened = 30,
    NotReproducible = 40,
    NotFixable = 50,
    Duplicate = 60,
    NotRequired = 70,
    Later = 80,
    NoChange = 90,
}
impl Resolution {
    pub fn as_str(&self) -> &'static str {
        match self {
            Resolution::Unknown => "[任意]",
            Resolution::Unprocessed => "未处理",
            Resolution::Fixed => "已修正",
            Resolution::Reopened => "重新打开",
            Resolution::NotReproducible => "无法重现",
            Resolution::NotFixable => "无法修复",
            Resolution::Duplicate => "重复问题",
            Resolution::NotRequired => "不必改",
            Resolution::Later => "稍后处理",
            Resolution::NoChange => "不做修改",
        }
    }
    pub fn as_i64(&self) -> i64 {
        *self as i64
    }
    pub fn kv() -> Vec<KV> {
        vec![
            KV {
                key: Resolution::Unprocessed.as_i64(),
                value: Resolution::Unprocessed.as_str().to_string(),
            },
            KV {
                key: Resolution::Fixed.as_i64(),
                value: Resolution::Fixed.as_str().to_string(),
            },
            KV {
                key: Resolution::Reopened.as_i64(),
                value: Resolution::Reopened.as_str().to_string(),
            },
            KV {
                key: Resolution::NotReproducible.as_i64(),
                value: Resolution::NotReproducible.as_str().to_string(),
            },
            KV {
                key: Resolution::NotFixable.as_i64(),
                value: Resolution::NotFixable.as_str().to_string(),
            },
            KV {
                key: Resolution::Duplicate.as_i64(),
                value: Resolution::Duplicate.as_str().to_string(),
            },
            KV {
                key: Resolution::NotRequired.as_i64(),
                value: Resolution::NotRequired.as_str().to_string(),
            },
            KV {
                key: Resolution::Later.as_i64(),
                value: Resolution::Later.as_str().to_string(),
            },
            KV {
                key: Resolution::NoChange.as_i64(),
                value: Resolution::NoChange.as_str().to_string(),
            },
        ]
    }
}
impl std::fmt::Display for Resolution {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}
impl From<&str> for Resolution {
    fn from(s: &str) -> Self {
        match s {
            "未处理" => Resolution::Unprocessed,
            "已修正" => Resolution::Fixed,
            "重新打开" => Resolution::Reopened,
            "无法重现" => Resolution::NotReproducible,
            "无法修复" => Resolution::NotFixable,
            "重复问题" => Resolution::Duplicate,
            "不必改" => Resolution::NotRequired,
            "稍后处理" => Resolution::Later,
            "不做修改" => Resolution::NoChange,
            _ => Resolution::Unknown,
        }
    }
}
impl From<i64> for Resolution {
    fn from(n: i64) -> Self {
        match n {
            0 => Resolution::Unknown,
            10 => Resolution::Unprocessed,
            20 => Resolution::Fixed,
            30 => Resolution::Reopened,
            40 => Resolution::NotReproducible,
            50 => Resolution::NotFixable,
            60 => Resolution::Duplicate,
            70 => Resolution::NotRequired,
            80 => Resolution::Later,
            90 => Resolution::NoChange,
            _ => Resolution::Unknown,
        }
    }
}

// <a class="project-link" href="/set_project.php?project_id=0">所有项目</a>
// <a class="project-link" href="/set_project.php?project_id=22">EasyLink</a>
// <a class="project-link" href="/set_project.php?project_id=24">EasyLink-用户端</a>
// <a class="project-link" href="/set_project.php?project_id=23">EasyLink-管理端</a>
// <a class="project-link" href="/set_project.php?project_id=9">云计算</a>
// <a class="project-link" href="/set_project.php?project_id=11">云计算-用户端</a>
// <a class="project-link" href="/set_project.php?project_id=10">云计算-管理端</a>
// <a class="project-link" href="/set_project.php?project_id=21">云计算-线上问题</a>
// <a class="project-link" href="/set_project.php?project_id=26">云计算海外版</a>
// <a class="project-link" href="/set_project.php?project_id=28">云计算海外版-用户端</a>
// <a class="project-link" href="/set_project.php?project_id=27">云计算海外版-管理端</a>
// <a class="project-link" href="/set_project.php?project_id=13">云转码</a>
// <a class="project-link" href="/set_project.php?project_id=16">云转码-用户端</a>
// <a class="project-link" href="/set_project.php?project_id=17">云转码-管理端</a>
// <a class="project-link" href="/set_project.php?project_id=12">证书系统</a>
// <a class="project-link" href="/set_project.php?project_id=25">资源管理系统</a>
// <a class="project-link" href="/set_project.php?project_id=18">钱包</a>
// <a class="project-link" href="/set_project.php?project_id=20">钱包-用户端</a>
// <a class="project-link" href="/set_project.php?project_id=19">钱包-管理端</a>

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Project {
    AllProjects = 0,
    EasyLink = 22,
    EasyLinkUser = 24,
    EasyLinkAdmin = 23,
    CloudComputing = 9,
    CloudComputingUser = 11,
    CloudComputingAdmin = 10,
    CloudComputingOnlineIssues = 21,
    CloudComputingOverseas = 26,
    CloudComputingOverseasUser = 28,
    CloudComputingOverseasAdmin = 27,
    CloudTranscoding = 13,
    CloudTranscodingUser = 16,
    CloudTranscodingAdmin = 17,
    CertificateSystem = 12,
    ResourceManagementSystem = 25,
    Wallet = 18,
    WalletUser = 20,
    WalletAdmin = 19,
}
impl Project {
    pub fn as_str(&self) -> &'static str {
        match self {
            Project::AllProjects => "所有项目",
            Project::EasyLink => "EasyLink",
            Project::EasyLinkUser => "EasyLink-用户端",
            Project::EasyLinkAdmin => "EasyLink-管理端",
            Project::CloudComputing => "云计算",
            Project::CloudComputingUser => "云计算-用户端",
            Project::CloudComputingAdmin => "云计算-管理端",
            Project::CloudComputingOnlineIssues => "云计算-线上问题",
            Project::CloudComputingOverseas => "云计算海外版",
            Project::CloudComputingOverseasUser => "云计算海外版-用户端",
            Project::CloudComputingOverseasAdmin => "云计算海外版-管理端",
            Project::CloudTranscoding => "云转码",
            Project::CloudTranscodingUser => "云转码-用户端",
            Project::CloudTranscodingAdmin => "云转码-管理端",
            Project::CertificateSystem => "证书系统",
            Project::ResourceManagementSystem => "资源管理系统",
            Project::Wallet => "钱包",
            Project::WalletUser => "钱包-用户端",
            Project::WalletAdmin => "钱包-管理端",
        }
    }
    pub fn as_i64(&self) -> i64 {
        *self as i64
    }
    pub fn kv() -> Vec<KV> {
        vec![
            KV {
                key: Project::AllProjects.as_i64(),
                value: Project::AllProjects.as_str().to_string(),
            },
            KV {
                key: Project::EasyLink.as_i64(),
                value: Project::EasyLink.as_str().to_string(),
            },
            KV {
                key: Project::EasyLinkUser.as_i64(),
                value: Project::EasyLinkUser.as_str().to_string(),
            },
            KV {
                key: Project::EasyLinkAdmin.as_i64(),
                value: Project::EasyLinkAdmin.as_str().to_string(),
            },
            KV {
                key: Project::CloudComputing.as_i64(),
                value: Project::CloudComputing.as_str().to_string(),
            },
            KV {
                key: Project::CloudComputingUser.as_i64(),
                value: Project::CloudComputingUser.as_str().to_string(),
            },
            KV {
                key: Project::CloudComputingAdmin.as_i64(),
                value: Project::CloudComputingAdmin.as_str().to_string(),
            },
            KV {
                key: Project::CloudComputingOnlineIssues.as_i64(),
                value: Project::CloudComputingOnlineIssues.as_str().to_string(),
            },
            KV {
                key: Project::CloudComputingOverseas.as_i64(),
                value: Project::CloudComputingOverseas.as_str().to_string(),
            },
            KV {
                key: Project::CloudComputingOverseasUser.as_i64(),
                value: Project::CloudComputingOverseasUser.as_str().to_string(),
            },
            KV {
                key: Project::CloudComputingOverseasAdmin.as_i64(),
                value: Project::CloudComputingOverseasAdmin.as_str().to_string(),
            },
            KV {
                key: Project::CloudTranscoding.as_i64(),
                value: Project::CloudTranscoding.as_str().to_string(),
            },
            KV {
                key: Project::CloudTranscodingUser.as_i64(),
                value: Project::CloudTranscodingUser.as_str().to_string(),
            },
            KV {
                key: Project::CloudTranscodingAdmin.as_i64(),
                value: Project::CloudTranscodingAdmin.as_str().to_string(),
            },
            KV {
                key: Project::CertificateSystem.as_i64(),
                value: Project::CertificateSystem.as_str().to_string(),
            },
            KV {
                key: Project::ResourceManagementSystem.as_i64(),
                value: Project::ResourceManagementSystem.as_str().to_string(),
            },
            KV {
                key: Project::Wallet.as_i64(),
                value: Project::Wallet.as_str().to_string(),
            },
            KV {
                key: Project::WalletUser.as_i64(),
                value: Project::WalletUser.as_str().to_string(),
            },
            KV {
                key: Project::WalletAdmin.as_i64(),
                value: Project::WalletAdmin.as_str().to_string(),
            },
        ]
    }
}
impl std::fmt::Display for Project {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}
impl From<&str> for Project {
    fn from(s: &str) -> Self {
        match s {
            "所有项目" => Project::AllProjects,
            "EasyLink" => Project::EasyLink,
            "EasyLink-用户端" => Project::EasyLinkUser,
            "EasyLink-管理端" => Project::EasyLinkAdmin,
            "云计算" => Project::CloudComputing,
            "云计算-用户端" => Project::CloudComputingUser,
            "云计算-管理端" => Project::CloudComputingAdmin,
            "云计算-线上问题" => Project::CloudComputingOnlineIssues,
            "云计算海外版" => Project::CloudComputingOverseas,
            "云计算海外版-用户端" => Project::CloudComputingOverseasUser,
            "云计算海外版-管理端" => Project::CloudComputingOverseasAdmin,
            "云转码" => Project::CloudTranscoding,
            "云转码-用户端" => Project::CloudTranscodingUser,
            "云转码-管理端" => Project::CloudTranscodingAdmin,
            "证书系统" => Project::CertificateSystem,
            "资源管理系统" => Project::ResourceManagementSystem,
            "钱包" => Project::Wallet,
            "钱包-用户端" => Project::WalletUser,
            "钱包-管理端" => Project::WalletAdmin,
            _ => Project::AllProjects, // 默认返回所有项目
        }
    }
}
impl From<i64> for Project {
    fn from(n: i64) -> Self {
        match n {
            0 => Project::AllProjects,
            22 => Project::EasyLink,
            24 => Project::EasyLinkUser,
            23 => Project::EasyLinkAdmin,
            9 => Project::CloudComputing,
            11 => Project::CloudComputingUser,
            10 => Project::CloudComputingAdmin,
            21 => Project::CloudComputingOnlineIssues,
            26 => Project::CloudComputingOverseas,
            28 => Project::CloudComputingOverseasUser,
            27 => Project::CloudComputingOverseasAdmin,
            13 => Project::CloudTranscoding,
            16 => Project::CloudTranscodingUser,
            17 => Project::CloudTranscodingAdmin,
            12 => Project::CertificateSystem,
            25 => Project::ResourceManagementSystem,
            18 => Project::Wallet,
            20 => Project::WalletUser,
            19 => Project::WalletAdmin,
            _ => Project::AllProjects, // 默认返回所有项目
        }
    }
}

// <option value="10" selected="selected">公开</option>
// <option value="50">私有</option>

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ViewStatus {
    Unknown = 0,
    Public = 10,
    Privite = 50,
}
impl ViewStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            ViewStatus::Unknown => "[任意]",
            ViewStatus::Public => "公开",
            ViewStatus::Privite => "私有",
        }
    }
    pub fn as_i64(&self) -> i64 {
        *self as i64
    }
    pub fn kv() -> Vec<KV> {
        vec![
            KV {
                key: ViewStatus::Public.as_i64(),
                value: ViewStatus::Public.as_str().to_string(),
            },
            KV {
                key: ViewStatus::Privite.as_i64(),
                value: ViewStatus::Privite.as_str().to_string(),
            },
        ]
    }
}
impl std::fmt::Display for ViewStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}
impl From<&str> for ViewStatus {
    fn from(s: &str) -> Self {
        match s {
            "公开" => ViewStatus::Public,
            "私有" => ViewStatus::Privite,
            _ => ViewStatus::Unknown,
        }
    }
}
impl From<i64> for ViewStatus {
    fn from(n: i64) -> Self {
        match n {
            10 => ViewStatus::Public,
            50 => ViewStatus::Privite,
            _ => ViewStatus::Unknown,
        }
    }
}

// <option value="10" selected="selected">总是</option>
// <option value="30">有时</option>
// <option value="50">随机</option>
// <option value="70">没有试验</option>
// <option value="90">无法重现</option>
// <option value="100">不适用</option>

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Reproducibility {
    Unknown = 0,
    Always = 10,
    Sometimes = 30,
    Random = 50,
    NoTest = 70,
    NotReproducible = 90,
    NotApplicable = 100,
}
impl Reproducibility {
    pub fn as_str(&self) -> &'static str {
        match self {
            Reproducibility::Unknown => "[任意]",
            Reproducibility::Always => "总是",
            Reproducibility::Sometimes => "有时",
            Reproducibility::Random => "随机",
            Reproducibility::NoTest => "没有试验",
            Reproducibility::NotReproducible => "无法重现",
            Reproducibility::NotApplicable => "不适用",
        }
    }
    pub fn as_i64(&self) -> i64 {
        *self as i64
    }
    pub fn kv() -> Vec<KV> {
        vec![
            KV {
                key: Reproducibility::Always.as_i64(),
                value: Reproducibility::Always.as_str().to_string(),
            },
            KV {
                key: Reproducibility::Sometimes.as_i64(),
                value: Reproducibility::Sometimes.as_str().to_string(),
            },
            KV {
                key: Reproducibility::Random.as_i64(),
                value: Reproducibility::Random.as_str().to_string(),
            },
            KV {
                key: Reproducibility::NoTest.as_i64(),
                value: Reproducibility::NoTest.as_str().to_string(),
            },
            KV {
                key: Reproducibility::NotReproducible.as_i64(),
                value: Reproducibility::NotReproducible.as_str().to_string(),
            },
            KV {
                key: Reproducibility::NotApplicable.as_i64(),
                value: Reproducibility::NotApplicable.as_str().to_string(),
            },
        ]
    }
}
impl std::fmt::Display for Reproducibility {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}
impl From<&str> for Reproducibility {
    fn from(s: &str) -> Self {
        match s {
            "总是" => Reproducibility::Always,
            "有时" => Reproducibility::Sometimes,
            "随机" => Reproducibility::Random,
            "没有试验" => Reproducibility::NoTest,
            "无法重现" => Reproducibility::NotReproducible,
            "不适用" => Reproducibility::NotApplicable,
            _ => Reproducibility::Unknown,
        }
    }
}
impl From<i64> for Reproducibility {
    fn from(n: i64) -> Self {
        match n {
            10 => Reproducibility::Always,
            30 => Reproducibility::Sometimes,
            50 => Reproducibility::Random,
            70 => Reproducibility::NoTest,
            90 => Reproducibility::NotReproducible,
            100 => Reproducibility::NotApplicable,
            _ => Reproducibility::Unknown,
        }
    }
}
