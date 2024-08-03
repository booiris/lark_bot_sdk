//! # req 定义
//!
//! 用于生成的 api 接口的输入和输出结构

use std::{collections::HashMap, fmt::Debug, time::Duration};

use derive_builder::Builder;
use http::{HeaderMap, Method, StatusCode};
use serde::Serialize;

use super::http_client::HttpCommonResp;

#[derive(Debug, Default, Builder, Clone)]
#[builder(setter(into))]
pub struct MethodOption {
    #[builder(setter(strip_option), default)]
    pub user_access_token: Option<String>,
    #[builder(setter(strip_option), default)]
    pub timeout: Option<Duration>,
    #[builder(default)]
    pub log_req_and_resp: bool,
    #[builder(setter(strip_option), default)]
    pub attach_headers: Option<HeaderMap>,
}

#[derive(Debug, Default, Clone)]
pub struct ReqParam<Body> {
    pub query: Option<Vec<(String, String)>>,
    pub body: Option<Body>,
    pub path: Option<HashMap<String, String>>,
}

#[derive(Debug, Default)]
pub struct ApiRequest<Body: Debug + Serialize> {
    /// api domain, such as Auth, Chat, Mail
    pub scope: &'static str,
    /// api name, as in CreateMailGroup
    pub api: &'static str,
    /// http request method, such as GET, POST
    pub method: Method,
    /// http request url
    pub url: String,
    /// http request body, query, path and other parameter information
    pub param_data: ReqParam<Body>,
    /// do you need TenantAccessToken
    pub need_tenant_access_token: bool,
    /// do you need AppAccessToken
    pub need_app_access_token: bool,
    /// do you need UserAccessToken
    pub need_user_access_token: bool,
    /// do you need Help Desk AccessToken
    pub need_help_desk_auth: bool,
    /// method option, such as userAccessToken
    pub method_option: MethodOption,

    pub stream_param_data: Option<StreamReqParam>,
}

impl<Body: Debug + Serialize + Clone> Clone for ApiRequest<Body> {
    fn clone(&self) -> Self {
        Self {
            scope: self.scope,
            api: self.api,
            method: self.method.clone(),
            url: self.url.clone(),
            param_data: self.param_data.clone(),
            need_tenant_access_token: self.need_tenant_access_token,
            need_app_access_token: self.need_app_access_token,
            need_user_access_token: self.need_user_access_token,
            need_help_desk_auth: self.need_help_desk_auth,
            method_option: self.method_option.clone(),
            stream_param_data: None,
        }
    }
}

pub struct StreamReqParam {
    pub field: HashMap<String, String>,
    // 使用 future 的 AsyncBufRead 得各种转换，太麻烦了，钦点一下 tokio 吧
    pub data: Box<dyn StreamReqData>,
}

pub trait StreamReqData: tokio::io::AsyncBufRead + Send + Unpin + Sync + 'static {}
impl<T: tokio::io::AsyncBufRead + Send + Unpin + Sync + 'static> StreamReqData for T {}

impl Debug for StreamReqParam {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("StreamReqParam")
            .field("field", &self.field)
            .finish()
    }
}

impl Default for StreamReqParam {
    fn default() -> Self {
        Self {
            field: Default::default(),
            data: Box::new(tokio::io::empty()),
        }
    }
}

#[derive(Default, Debug)]
pub struct CommonResponse {
    /// request method
    pub method: Method,
    /// request url
    pub url: String,
    /// request id, if you got some error and oncall lark/feishu team, please with this request id
    pub request_id: String,
    /// http response status code
    pub status_code: StatusCode,
    /// http response header
    pub headers: HeaderMap,
    /// http response content length
    pub content_length: Option<u64>,
}

impl From<HttpCommonResp> for CommonResponse {
    fn from(resp: HttpCommonResp) -> Self {
        let request_id = resp
            .headers
            .get("X-Request-Id")
            .and_then(|x| x.to_str().ok().map(|x| x.to_string()))
            .unwrap_or_default();
        Self {
            method: resp.method,
            url: resp.url,
            status_code: resp.status_code,
            headers: resp.headers,
            content_length: resp.content_length,
            request_id,
        }
    }
}
