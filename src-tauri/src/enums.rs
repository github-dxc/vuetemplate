use serde::Serialize;

// <option value="10">无</option>
// <option value="20">低</option>
// <option value="30" selected="selected">中</option>
// <option value="40">高</option>
// <option value="50">紧急</option>
// <option value="60">非常紧急</option>

#[derive(Debug, Clone, Serialize)]
pub struct KV {
    pub key: String,
    pub value: String,
}

// 查找数据的特征
pub trait VecExtKV {
    fn find_by_key<S: AsRef<str>>(&self, key: S) -> Option<&KV>;
    fn find_by_value<S: AsRef<str>>(&self, value: S) -> Option<&KV>;
}

impl VecExtKV for Vec<KV> {
    fn find_by_key<S: AsRef<str>>(&self, key: S) -> Option<&KV> {
        self.iter().find(|kv| kv.key == key.as_ref())
    }
    fn find_by_value<S: AsRef<str>>(&self, value: S) -> Option<&KV> {
        self.iter().find(|kv| kv.value == value.as_ref())
    }
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
    (Priority::Unknown, "[任意]"),
    (Priority::None, "无"),
    (Priority::Low, "低"),
    (Priority::Middle, "中"),
    (Priority::High, "高"),
    (Priority::Emergency, "紧急"),
    (Priority::VeryEmergency, "非常紧急"),
];
impl Priority {
    pub fn as_str(&self) -> &'static str {
        if let Some((_, b)) = PRIORITY_VALUE.iter().find(|d| d.0 == *self) {
            return *b;
        };
        ""
    }
    pub fn as_i64(&self) -> i64 {
        *self as i64
    }
    pub fn kv() -> Vec<KV> {
        PRIORITY_VALUE
            .iter()
            .map(|d| KV {
                key: (d.0 as i64).to_string(),
                value: d.1.into(),
            })
            .collect()
    }
}
impl std::fmt::Display for Priority {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}
impl From<&str> for Priority {
    fn from(s: &str) -> Self {
        if let Some((a, _)) = PRIORITY_VALUE.iter().find(|d| d.1 == s) {
            return *a;
        };
        Priority::Unknown
    }
}
impl From<i64> for Priority {
    fn from(n: i64) -> Self {
        if let Some((a, _)) = PRIORITY_VALUE.iter().find(|d| d.0.as_i64() == n) {
            return *a;
        };
        Priority::Unknown
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
    (Severity::Unknown, "[任意]"),
    (Severity::NewFeature, "新功能"),
    (Severity::Detail, "细节"),
    (Severity::Text, "文字"),
    (Severity::SmallAdjustment, "小调整"),
    (Severity::SmallError, "小错误"),
    (Severity::SeriousError, "很严重"),
    (Severity::Crash, "崩溃"),
    (Severity::Downtime, "宕机"),
];
impl Severity {
    pub fn as_str(&self) -> &'static str {
        if let Some((_, b)) = SERVERITY_VALUE.iter().find(|d| d.0 == *self) {
            return *b;
        };
        ""
    }
    pub fn as_i64(&self) -> i64 {
        *self as i64
    }
    pub fn kv() -> Vec<KV> {
        SERVERITY_VALUE
            .iter()
            .map(|d| KV {
                key: (d.0 as i64).to_string(),
                value: d.1.into(),
            })
            .collect()
    }
}
impl std::fmt::Display for Severity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}
impl From<&str> for Severity {
    fn from(s: &str) -> Self {
        if let Some((a, _)) = SERVERITY_VALUE.iter().find(|d| d.1 == s) {
            return *a;
        };
        Severity::Unknown
    }
}
impl From<i64> for Severity {
    fn from(n: i64) -> Self {
        if let Some((a, _)) = SERVERITY_VALUE.iter().find(|d| d.0.as_i64() == n) {
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
    (Status::Unknown, "[任意]"),
    (Status::New, "新建"),
    (Status::Feedback, "反馈"),
    (Status::Acknowledged, "认可"),
    (Status::Confirmed, "已确认"),
    (Status::Assigned, "已分配"),
    (Status::Resolved, "已解决"),
    (Status::Released, "已发布"),
    (Status::Verified, "已验证"),
    (Status::NotFixed, "不予解决"),
    (Status::DelayedFix, "延迟修复"),
    (Status::Reopened, "重新打开"),
    (Status::Closed, "已关闭"),
];
impl Status {
    pub fn as_str(&self) -> &'static str {
        if let Some((_, b)) = STATUS_VALUE.iter().find(|d| d.0 == *self) {
            return *b;
        };
        ""
    }
    pub fn as_i64(&self) -> i64 {
        *self as i64
    }
    pub fn kv() -> Vec<KV> {
        STATUS_VALUE
            .iter()
            .map(|d| KV {
                key: (d.0 as i64).to_string(),
                value: d.1.into(),
            })
            .collect()
    }
}
impl std::fmt::Display for Status {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}
impl From<&str> for Status {
    fn from(s: &str) -> Self {
        if let Some((a, _)) = STATUS_VALUE.iter().find(|d| d.1 == s) {
            return *a;
        };
        Status::Unknown
    }
}
impl From<i64> for Status {
    fn from(n: i64) -> Self {
        if let Some((a, _)) = STATUS_VALUE.iter().find(|d| d.0.as_i64() == n) {
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
    (Resolution::Unknown, "[任意]"),
    (Resolution::Unprocessed, "未处理"),
    (Resolution::Fixed, "已修正"),
    (Resolution::Reopened, "重新打开"),
    (Resolution::NotReproducible, "无法重现"),
    (Resolution::NotFixable, "无法修复"),
    (Resolution::Duplicate, "重复问题"),
    (Resolution::NotRequired, "不必改"),
    (Resolution::Later, "稍后处理"),
    (Resolution::NoChange, "不做修改"),
];
impl Resolution {
    pub fn as_str(&self) -> &'static str {
        if let Some((_, b)) = RESOLUTION_VALUE.iter().find(|d| d.0 == *self) {
            return *b;
        };
        ""
    }
    pub fn as_i64(&self) -> i64 {
        *self as i64
    }
    pub fn kv() -> Vec<KV> {
        RESOLUTION_VALUE
            .iter()
            .map(|d| KV {
                key: (d.0 as i64).to_string(),
                value: d.1.into(),
            })
            .collect()
    }
}
impl std::fmt::Display for Resolution {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}
impl From<&str> for Resolution {
    fn from(s: &str) -> Self {
        if let Some((a, _)) = RESOLUTION_VALUE.iter().find(|d| d.1 == s) {
            return *a;
        };
        Resolution::Unknown
    }
}
impl From<i64> for Resolution {
    fn from(n: i64) -> Self {
        if let Some((a, _)) = RESOLUTION_VALUE.iter().find(|d| d.0.as_i64() == n) {
            return *a;
        };
        Resolution::Unknown
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
    (ViewStatus::Unknown, "[任意]"),
    (ViewStatus::Public, "公开"),
    (ViewStatus::Privite, "私有"),
];
impl ViewStatus {
    pub fn as_str(&self) -> &'static str {
        if let Some((_, b)) = VIEW_STATUS_VALUE.iter().find(|d| d.0 == *self) {
            return *b;
        };
        ""
    }
    pub fn as_i64(&self) -> i64 {
        *self as i64
    }
    pub fn kv() -> Vec<KV> {
        VIEW_STATUS_VALUE
            .iter()
            .map(|d| KV {
                key: (d.0 as i64).to_string(),
                value: d.1.into(),
            })
            .collect()
    }
}
impl std::fmt::Display for ViewStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}
impl From<&str> for ViewStatus {
    fn from(s: &str) -> Self {
        if let Some((a, _)) = VIEW_STATUS_VALUE.iter().find(|d| d.1 == s) {
            return *a;
        };
        ViewStatus::Unknown
    }
}
impl From<i64> for ViewStatus {
    fn from(n: i64) -> Self {
        if let Some((a, _)) = VIEW_STATUS_VALUE.iter().find(|d| d.0.as_i64() == n) {
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
    (Reproducibility::Unknown, "[任意]"),
    (Reproducibility::Always, "总是"),
    (Reproducibility::Sometimes, "有时"),
    (Reproducibility::Random, "随机"),
    (Reproducibility::NoTest, "没有试验"),
    (Reproducibility::NotReproducible, "无法重现"),
    (Reproducibility::NotApplicable, "不适用"),
];
impl Reproducibility {
    pub fn as_str(&self) -> &'static str {
        if let Some((_, b)) = REPRODUCIBILITY_VALUE.iter().find(|d| d.0 == *self) {
            return *b;
        };
        ""
    }
    pub fn as_i64(&self) -> i64 {
        *self as i64
    }
    pub fn kv() -> Vec<KV> {
        REPRODUCIBILITY_VALUE
            .iter()
            .map(|d| KV {
                key: (d.0 as i64).to_string(),
                value: d.1.into(),
            })
            .collect()
    }
}
impl std::fmt::Display for Reproducibility {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}
impl From<&str> for Reproducibility {
    fn from(s: &str) -> Self {
        if let Some((a, _)) = REPRODUCIBILITY_VALUE.iter().find(|d| d.1 == s) {
            return *a;
        };
        Reproducibility::Unknown
    }
}
impl From<i64> for Reproducibility {
    fn from(n: i64) -> Self {
        if let Some((a, _)) = REPRODUCIBILITY_VALUE.iter().find(|d| d.0.as_i64() == n) {
            return *a;
        };
        Reproducibility::Unknown
    }
}
