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
const PRIORITY_VALUE: [(Priority, &str); 7] = [
    (Priority::Unknown , "[任意]"),
    (Priority::None , "无"),
    (Priority::Low , "低"),
    (Priority::Middle , "中"),
    (Priority::High , "高"),
    (Priority::Emergency , "紧急"),
    (Priority::VeryEmergency , "非常紧急"),
];
impl Priority {
    pub fn as_str(&self) -> &'static str {
        if let Some((_,b)) = PRIORITY_VALUE.iter().find(|d|d.0 == *self) {
            return *b;
        };
        ""
    }
    pub fn as_i64(&self) -> i64 {
        *self as i64
    }
    pub fn kv() -> Vec<KV> {
        PRIORITY_VALUE.iter().map(|d|KV{key:d.0.as_i64(),value:d.1.into()}).collect()
    }
}
impl std::fmt::Display for Priority {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}
impl From<&str> for Priority {
    fn from(s: &str) -> Self {
        if let Some((a,_)) = PRIORITY_VALUE.iter().find(|d|d.1 == s) {
            return *a;
        };
        Priority::Unknown
    }
}
impl From<i64> for Priority {
    fn from(n: i64) -> Self {
        if let Some((a,_)) = PRIORITY_VALUE.iter().find(|d|d.0.as_i64() == n) {
            return *a;
        };
        Priority::Unknown
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
const CATEGORY_VALUE: [(Category, &str); 8] = [
    (Category::Unknown , "[任意]"),
    (Category::General , "General"),
    (Category::UIDesign , "UI设计"),
    (Category::ProductDesign , "产品设计"),
    (Category::Frontend , "前端"),
    (Category::Backend , "后端"),
    (Category::BigData , "大数据"),
    (Category::AndroidClient , "安卓客户端"),
];
impl Category {
    pub fn as_str(&self) -> &'static str {
        if let Some((_,b)) = CATEGORY_VALUE.iter().find(|d|d.0 == *self) {
            return *b;
        };
        ""
    }
    pub fn as_i64(&self) -> i64 {
        *self as i64
    }
    pub fn kv() -> Vec<KV> {
        CATEGORY_VALUE.iter().map(|d|KV{key:d.0.as_i64(),value:d.1.into()}).collect()
    }
}
impl std::fmt::Display for Category {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}
impl From<&str> for Category {
    fn from(s: &str) -> Self {
        if let Some((a,_)) = CATEGORY_VALUE.iter().find(|d|d.1 == s) {
            return *a;
        };
        Category::Unknown
    }
}
impl From<i64> for Category {
    fn from(n: i64) -> Self {
        if let Some((a,_)) = CATEGORY_VALUE.iter().find(|d|d.0.as_i64() == n) {
            return *a;
        };
        Category::Unknown 
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
const SERVERITY_VALUE: [(Severity, &str); 9] = [
    (Severity::Unknown , "[任意]"),
    (Severity::NewFeature , "新功能"),
    (Severity::Detail , "细节"),
    (Severity::Text , "文字"),
    (Severity::SmallAdjustment , "小调整"),
    (Severity::SmallError , "小错误"),
    (Severity::SeriousError , "很严重"),
    (Severity::Crash , "崩溃"),
    (Severity::Downtime , "宕机"),
];
impl Severity {
    pub fn as_str(&self) -> &'static str {
        if let Some((_,b)) = SERVERITY_VALUE.iter().find(|d|d.0 == *self) {
            return *b;
        };
        ""
    }
    pub fn as_i64(&self) -> i64 {
        *self as i64
    }
    pub fn kv() -> Vec<KV> {
        SERVERITY_VALUE.iter().map(|d|KV{key:d.0.as_i64(),value:d.1.into()}).collect()
    }
}
impl std::fmt::Display for Severity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}
impl From<&str> for Severity {
    fn from(s: &str) -> Self {
        if let Some((a,_)) = SERVERITY_VALUE.iter().find(|d|d.1 == s) {
            return *a;
        };
        Severity::Unknown
    }
}
impl From<i64> for Severity {
    fn from(n: i64) -> Self {
        if let Some((a,_)) = SERVERITY_VALUE.iter().find(|d|d.0.as_i64() == n) {
            return *a;
        };
        Severity::Unknown 
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
const STATUS_VALUE: [(Status, &str); 13] = [
    (Status::Unknown , "[任意]"),
    (Status::New , "新建"),
    (Status::Feedback , "反馈"),
    (Status::Acknowledged , "认可"),
    (Status::Confirmed , "已确认"),
    (Status::Assigned , "已分配"),
    (Status::Resolved , "已解决"),
    (Status::Released , "已发布"),
    (Status::Verified , "已验证"),
    (Status::NotFixed , "不予解决"),
    (Status::DelayedFix , "延迟修复"),
    (Status::Reopened , "重新打开"),
    (Status::Closed , "已关闭"),
];
impl Status {
    pub fn as_str(&self) -> &'static str {
        if let Some((_,b)) = STATUS_VALUE.iter().find(|d|d.0 == *self) {
            return *b;
        };
        ""
    }
    pub fn as_i64(&self) -> i64 {
        *self as i64
    }
    pub fn kv() -> Vec<KV> {
        STATUS_VALUE.iter().map(|d|KV{key:d.0.as_i64(),value:d.1.into()}).collect()
    }
}
impl std::fmt::Display for Status {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}
impl From<&str> for Status {
    fn from(s: &str) -> Self {
        if let Some((a,_)) = STATUS_VALUE.iter().find(|d|d.1 == s) {
            return *a;
        };
        Status::Unknown
    }
}
impl From<i64> for Status {
    fn from(n: i64) -> Self {
        if let Some((a,_)) = STATUS_VALUE.iter().find(|d|d.0.as_i64() == n) {
            return *a;
        };
        Status::Unknown 
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
const RESOLUTION_VALUE: [(Resolution, &str); 10] = [
    (Resolution::Unknown , "[任意]"),
    (Resolution::Unprocessed , "未处理"),
    (Resolution::Fixed , "已修正"),
    (Resolution::Reopened , "重新打开"),
    (Resolution::NotReproducible , "无法重现"),
    (Resolution::NotFixable , "无法修复"),
    (Resolution::Duplicate , "重复问题"),
    (Resolution::NotRequired , "不必改"),
    (Resolution::Later , "稍后处理"),
    (Resolution::NoChange , "不做修改"),
];
impl Resolution {
    pub fn as_str(&self) -> &'static str {
        if let Some((_,b)) = RESOLUTION_VALUE.iter().find(|d|d.0 == *self) {
            return *b;
        };
        ""
    }
    pub fn as_i64(&self) -> i64 {
        *self as i64
    }
    pub fn kv() -> Vec<KV> {
        RESOLUTION_VALUE.iter().map(|d|KV{key:d.0.as_i64(),value:d.1.into()}).collect()
    }
}
impl std::fmt::Display for Resolution {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}
impl From<&str> for Resolution {
    fn from(s: &str) -> Self {
        if let Some((a,_)) = RESOLUTION_VALUE.iter().find(|d|d.1 == s) {
            return *a;
        };
        Resolution::Unknown
    }
}
impl From<i64> for Resolution {
    fn from(n: i64) -> Self {
        if let Some((a,_)) = RESOLUTION_VALUE.iter().find(|d|d.0.as_i64() == n) {
            return *a;
        };
        Resolution::Unknown 
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
    Unknown = 0,
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
const PROJECT_VALUE: [(Project, &str); 19] = [
    (Project::Unknown , "所有项目"),
    (Project::EasyLink , "EasyLink"),
    (Project::EasyLinkUser , "EasyLink-用户端"),
    (Project::EasyLinkAdmin , "EasyLink-管理端"),
    (Project::CloudComputing , "云计算"),
    (Project::CloudComputingUser , "云计算-用户端"),
    (Project::CloudComputingAdmin , "云计算-管理端"),
    (Project::CloudComputingOnlineIssues , "云计算-线上问题"),
    (Project::CloudComputingOverseas , "云计算海外版"),
    (Project::CloudComputingOverseasUser , "云计算海外版-用户端"),
    (Project::CloudComputingOverseasAdmin , "云计算海外版-管理端"),
    (Project::CloudTranscoding , "云转码"),
    (Project::CloudTranscodingUser , "云转码-用户端"),
    (Project::CloudTranscodingAdmin , "云转码-管理端"),
    (Project::CertificateSystem , "证书系统"),
    (Project::ResourceManagementSystem , "资源管理系统"),
    (Project::Wallet , "钱包"),
    (Project::WalletUser , "钱包-用户端"),
    (Project::WalletAdmin , "钱包-管理端"),
];
impl Project {
    pub fn as_str(&self) -> &'static str {
        if let Some((_,b)) = PROJECT_VALUE.iter().find(|d|d.0 == *self) {
            return *b;
        };
        ""
    }
    pub fn as_i64(&self) -> i64 {
        *self as i64
    }
    pub fn kv() -> Vec<KV> {
        PROJECT_VALUE.iter().map(|d|KV{key:d.0.as_i64(),value:d.1.into()}).collect()
    }
}
impl std::fmt::Display for Project {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}
impl From<&str> for Project {
    fn from(s: &str) -> Self {
        if let Some((a,_)) = PROJECT_VALUE.iter().find(|d|d.1 == s) {
            return *a;
        };
        Project::Unknown
    }
}
impl From<i64> for Project {
    fn from(n: i64) -> Self {
        if let Some((a,_)) = PROJECT_VALUE.iter().find(|d|d.0.as_i64() == n) {
            return *a;
        };
        Project::Unknown 
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
const VIEW_STATUS_VALUE: [(ViewStatus, &str); 3] = [
    (ViewStatus::Unknown , "[任意]"),
    (ViewStatus::Public , "公开"),
    (ViewStatus::Privite , "私有"),
];
impl ViewStatus {
    pub fn as_str(&self) -> &'static str {
        if let Some((_,b)) = VIEW_STATUS_VALUE.iter().find(|d|d.0 == *self) {
            return *b;
        };
        ""
    }
    pub fn as_i64(&self) -> i64 {
        *self as i64
    }
    pub fn kv() -> Vec<KV> {
        VIEW_STATUS_VALUE.iter().map(|d|KV{key:d.0.as_i64(),value:d.1.into()}).collect()
    }
}
impl std::fmt::Display for ViewStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}
impl From<&str> for ViewStatus {
    fn from(s: &str) -> Self {
        if let Some((a,_)) = VIEW_STATUS_VALUE.iter().find(|d|d.1 == s) {
            return *a;
        };
        ViewStatus::Unknown
    }
}
impl From<i64> for ViewStatus {
    fn from(n: i64) -> Self {
        if let Some((a,_)) = VIEW_STATUS_VALUE.iter().find(|d|d.0.as_i64() == n) {
            return *a;
        };
        ViewStatus::Unknown 
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
const REPRODUCIBILITY_VALUE: [(Reproducibility, &str); 7] = [
    (Reproducibility::Unknown , "[任意]"),
    (Reproducibility::Always , "总是"),
    (Reproducibility::Sometimes , "有时"),
    (Reproducibility::Random , "随机"),
    (Reproducibility::NoTest , "没有试验"),
    (Reproducibility::NotReproducible , "无法重现"),
    (Reproducibility::NotApplicable , "不适用"),
];
impl Reproducibility {
    pub fn as_str(&self) -> &'static str {
        if let Some((_,b)) = REPRODUCIBILITY_VALUE.iter().find(|d|d.0 == *self) {
            return *b;
        };
        ""
    }
    pub fn as_i64(&self) -> i64 {
        *self as i64
    }
    pub fn kv() -> Vec<KV> {
        REPRODUCIBILITY_VALUE.iter().map(|d|KV{key:d.0.as_i64(),value:d.1.into()}).collect()
    }
}
impl std::fmt::Display for Reproducibility {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}
impl From<&str> for Reproducibility {
    fn from(s: &str) -> Self {
        if let Some((a,_)) = REPRODUCIBILITY_VALUE.iter().find(|d|d.1 == s) {
            return *a;
        };
        Reproducibility::Unknown
    }
}
impl From<i64> for Reproducibility {
    fn from(n: i64) -> Self {
        if let Some((a,_)) = REPRODUCIBILITY_VALUE.iter().find(|d|d.0.as_i64() == n) {
            return *a;
        };
        Reproducibility::Unknown 
    }
}
