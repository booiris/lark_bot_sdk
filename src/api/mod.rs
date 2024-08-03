use serde::{Deserialize, Serialize};

use crate::core::http_client::HttpClient;

pub mod auth;
mod gen;

#[derive(Debug, Deserialize, Default, Clone, Serialize)]
#[serde(default)]
pub struct BaseResp {
    /// 错误码, 非 0 表示失败，具体错误码参考: [服务端通用错误码](https://open.feishu.cn/document/ukTMukTMukTM/ugjM14COyUjL4ITN)。
    pub code: i32,
    /// 错误描述
    pub msg: String,
    pub err: String,
}

pub trait HasBaseResp {
    fn base_resp(&self) -> &BaseResp;
    fn take_base_resp(self) -> BaseResp;
}

pub struct DownloadResp<IHttpClient: HttpClient> {
    pub data: <IHttpClient as HttpClient>::StreamRespData,
    pub name: Option<String>,
}

impl<IHttpClient: HttpClient> std::fmt::Debug for DownloadResp<IHttpClient> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Ok(writeln!(f, "DownloadFileResp {{ name: {:?} }}", self.name)?)
    }
}

#[cfg_attr(docsrs, doc(cfg(feature = "acs")))]
#[cfg(feature = "acs")]
pub mod acs {
    pub use super::gen::acs::*;
}

#[cfg_attr(docsrs, doc(cfg(feature = "ai")))]
#[cfg(feature = "ai")]
pub mod ai {
    pub use super::gen::ai::*;
}

#[cfg_attr(docsrs, doc(cfg(feature = "admin")))]
#[cfg(feature = "admin")]
pub mod admin {
    pub use super::gen::admin::*;
}

#[cfg_attr(docsrs, doc(cfg(feature = "application")))]
#[cfg(feature = "application")]
pub mod application {
    pub use super::gen::application::*;
}

#[cfg_attr(docsrs, doc(cfg(feature = "approval")))]
#[cfg(feature = "approval")]
pub mod approval {
    pub use super::gen::approval::*;
}

#[cfg_attr(docsrs, doc(cfg(feature = "attendance")))]
#[cfg(feature = "attendance")]
pub mod attendance {
    pub use super::gen::attendance::*;
}

#[cfg_attr(docsrs, doc(cfg(feature = "baike")))]
#[cfg(feature = "baike")]
pub mod baike {
    pub use super::gen::baike::*;
}

#[cfg_attr(docsrs, doc(cfg(feature = "bitable")))]
#[cfg(feature = "bitable")]
pub mod bitable {
    pub use super::gen::bitable::*;
}

#[cfg_attr(docsrs, doc(cfg(feature = "bot")))]
#[cfg(feature = "bot")]
pub mod bot {
    pub use super::gen::bot::*;
}

#[cfg_attr(docsrs, doc(cfg(feature = "calendar")))]
#[cfg(feature = "calendar")]
pub mod calendar {
    pub use super::gen::calendar::*;
}

#[cfg_attr(docsrs, doc(cfg(feature = "chat")))]
#[cfg(feature = "chat")]
pub mod chat {
    pub use super::gen::chat::*;
}

#[cfg_attr(docsrs, doc(cfg(feature = "contact")))]
#[cfg(feature = "contact")]
pub mod contact {
    pub use super::gen::contact::*;
}

#[cfg_attr(docsrs, doc(cfg(feature = "core_hr")))]
#[cfg(feature = "core_hr")]
pub mod core_hr {
    pub use super::gen::core_hr::*;
}

#[cfg_attr(docsrs, doc(cfg(feature = "drive")))]
#[cfg(feature = "drive")]
pub mod drive {
    pub use super::gen::drive::*;
}

#[cfg_attr(docsrs, doc(cfg(feature = "ehr")))]
#[cfg(feature = "ehr")]
pub mod ehr {
    pub use super::gen::ehr::*;
}

#[cfg_attr(docsrs, doc(cfg(feature = "event")))]
#[cfg(feature = "event")]
pub mod event {
    pub use super::gen::event::*;
}

#[cfg_attr(docsrs, doc(cfg(feature = "file")))]
#[cfg(feature = "file")]
pub mod file {
    pub use super::gen::file::*;
}

#[cfg_attr(docsrs, doc(cfg(feature = "helpdesk")))]
#[cfg(feature = "helpdesk")]
pub mod helpdesk {
    pub use super::gen::helpdesk::*;
}

#[cfg_attr(docsrs, doc(cfg(feature = "hire")))]
#[cfg(feature = "hire")]
pub mod hire {
    pub use super::gen::hire::*;
}

#[cfg_attr(docsrs, doc(cfg(feature = "human_auth")))]
#[cfg(feature = "human_auth")]
pub mod human_auth {
    pub use super::gen::human_auth::*;
}

#[cfg_attr(docsrs, doc(cfg(feature = "jssdk")))]
#[cfg(feature = "jssdk")]
pub mod jssdk {
    pub use super::gen::jssdk::*;
}

#[cfg_attr(docsrs, doc(cfg(feature = "lingo")))]
#[cfg(feature = "lingo")]
pub mod lingo {
    pub use super::gen::lingo::*;
}

#[cfg_attr(docsrs, doc(cfg(feature = "mail")))]
#[cfg(feature = "mail")]
pub mod mail {
    pub use super::gen::mail::*;
}

#[cfg_attr(docsrs, doc(cfg(feature = "message")))]
#[cfg(feature = "message")]
pub mod message;

#[cfg_attr(docsrs, doc(cfg(feature = "mina")))]
#[cfg(feature = "mina")]
pub mod mina {
    pub use super::gen::mina::*;
}

#[cfg_attr(docsrs, doc(cfg(feature = "minutes")))]
#[cfg(feature = "minutes")]
pub mod minutes {
    pub use super::gen::minutes::*;
}

#[cfg_attr(docsrs, doc(cfg(feature = "okr")))]
#[cfg(feature = "okr")]
pub mod okr {
    pub use super::gen::okr::*;
}

#[cfg_attr(docsrs, doc(cfg(feature = "passport")))]
#[cfg(feature = "passport")]
pub mod passport {
    pub use super::gen::passport::*;
}

#[cfg_attr(docsrs, doc(cfg(feature = "performance")))]
#[cfg(feature = "performance")]
pub mod performance {
    pub use super::gen::performance::*;
}

#[cfg_attr(docsrs, doc(cfg(feature = "personal_settings")))]
#[cfg(feature = "personal_settings")]
pub mod personal_settings {
    pub use super::gen::personal_settings::*;
}

#[cfg_attr(docsrs, doc(cfg(feature = "search")))]
#[cfg(feature = "search")]
pub mod search {
    pub use super::gen::search::*;
}

#[cfg_attr(docsrs, doc(cfg(feature = "task")))]
#[cfg(feature = "task")]
pub mod task {
    pub use super::gen::task::*;
}

#[cfg_attr(docsrs, doc(cfg(feature = "tenant")))]
#[cfg(feature = "tenant")]
pub mod tenant {
    pub use super::gen::tenant::*;
}

#[cfg_attr(docsrs, doc(cfg(feature = "vc")))]
#[cfg(feature = "vc")]
pub mod vc {
    pub use super::gen::vc::*;
}

#[cfg_attr(docsrs, doc(cfg(feature = "verification")))]
#[cfg(feature = "verification")]
pub mod verification {
    pub use super::gen::verification::*;
}
