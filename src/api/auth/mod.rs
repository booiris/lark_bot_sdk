use std::time;

use crate::api::HasBaseResp;
use serde::Deserialize;

use super::BaseResp;
pub mod get_app_access_token;
pub mod get_auth_app_ticket;
pub mod get_tenant_access_token;
pub mod service;
use crate::core::model::{ReqParam, StreamReqParam};

// lark 中有下面这些 token，每个 token 的决定因素不一样，决定了各自的 store key 组成方式
// tenant-token：app_id
// app-token：app_id
// isv-tenant-token：app-id + tenant_key
// isv app-token：app_id
// app-ticket: app_id
// user-token，不需要 store，通过 method option 传入

pub(in super::auth) fn gen_tenant_token_key<S: AsRef<str>>(
    is_isv: bool,
    app_id: &S,
    tenant_key: &S,
) -> String {
    if is_isv {
        return "isv-tenant-token:".to_string() + app_id.as_ref() + "-" + tenant_key.as_ref();
    }
    return "internal-tenant-token:".to_string() + app_id.as_ref();
}

pub(in super::auth) fn gen_app_token_key<S: AsRef<str>>(is_isv: bool, app_id: &S) -> String {
    if is_isv {
        return "isv-app-token:".to_string() + app_id.as_ref();
    }
    return "internal-app-token:".to_string() + app_id.as_ref();
}

pub(in super::auth) fn gen_isv_app_ticket_key<S: AsRef<str>>(app_id: &S) -> String {
    return "isv-app-ticket:".to_string() + app_id.as_ref();
}

#[derive(Debug)]
pub struct TokenExpire {
    pub token: String,
    pub expire: Option<time::Duration>,
}

#[derive(lark_bot_sdk_macros::ApiReqParams, Default)]
struct GetAccessTokenReq {
    #[api(kind = "body", name = "app_id")]
    app_id: String,
    #[api(kind = "body", name = "app_secret")]
    app_secret: String,
    #[api(kind = "body", name = "app_access_token")]
    app_access_token: String,
    #[api(kind = "body", name = "tenant_key")]
    tenant_key: String,
    #[api(kind = "body", name = "app_ticket")]
    app_ticket: String,
}

#[derive(Debug, Deserialize, Default)]
#[serde(default)]
#[derive(lark_bot_sdk_macros::ApiBaseResp)]
pub struct GetAccessTokenResp {
    #[serde(flatten)]
    pub data: Option<Data>,
    #[serde(flatten)]
    pub base: BaseResp,
}

#[derive(Debug, Deserialize, Default)]
#[serde(default)]
pub struct Data {
    #[serde(rename = "tenant_access_token")]
    pub tenant_access_token: String,
    #[serde(rename = "app_access_token")]
    pub app_access_token: String,
    #[serde(rename = "expire")]
    pub expire: u64,
}
