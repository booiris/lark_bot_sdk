use thiserror::Error;

use crate::{
    api::BaseResp,
    core::{model::CommonResponse, store},
};

#[derive(Error, Debug)]
pub enum Error {
    #[error("[lark sdk] resp error: {0}, request-id: {}", .1.request_id)]
    /// http 传输过程中出现的 error，或者是返回的结果不合法导致序列化错误
    ErrResponse(anyhow::Error, CommonResponse),
    #[error("[lark sdk] resp error: {0:?}, request-id: {}", .1.request_id)]
    /// api 请求错误，可能为参数错误或者权限不满足
    ErrApiResponse(BaseResp, CommonResponse),
    #[error("[lark sdk] resp http code error: {0:?}, request-id: {}", .1.request_id)]
    /// http code 为错误码，4xx 或 5xx，一般为上传下载的相关接口出现参数错误，此时会返回 http 错误码，**下载** api 使用此错误
    ErrHttpCode(Option<BaseResp>, CommonResponse),
    #[error("[lark sdk] do req Err: {0}")]
    /// 发送请求接收到返回结果前的错误，常见有传入的 header、method 错误，或者是网络不通
    ErrDoReq(anyhow::Error),
    #[error("[lark sdk] build client Err: {0}")]
    ErrBuildClient(String),
    #[error("[lark sdk] store token Err: {0}")]
    ErrStoreToken(store::StoreError),
    #[error("[lark sdk] get ticket func missing")]
    ErrGetTicketFuncMissing,
}
