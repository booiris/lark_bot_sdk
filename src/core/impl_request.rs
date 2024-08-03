use base64::{prelude::BASE64_STANDARD, Engine};
use std::{fmt::Debug, str::from_utf8};

use anyhow::anyhow;
use http::HeaderMap;
use serde::{de::DeserializeOwned, Serialize};

use crate::{api::HasBaseResp, core::http_client::HttpError, error::Error};

use super::{
    http_client::{HttpClient, HttpReq, HttpResp},
    model::{ApiRequest, CommonResponse},
    store::Store,
    Lark, LarkInner,
};

impl<B: Debug + Serialize + Send> ApiRequest<B> {
    fn parse_header<IStore: Send, IHttpClient: Send>(
        &self,
        lark: &LarkInner<IStore, IHttpClient>,
    ) -> Result<HeaderMap, Error> {
        let mut headers = HeaderMap::new();
        if self.param_data.body.is_some() {
            headers.insert(
                "Content-Type",
                "application/json"
                    .parse()
                    .expect("parse content type error"),
            );
        }

        if self.need_help_desk_auth {
            let encode = BASE64_STANDARD.encode(
                [
                    lark.help_desk_id.as_bytes(),
                    ":".as_bytes(),
                    lark.help_desk_token.as_bytes(),
                ]
                .concat(),
            );
            headers.append(
                "X-Lark-Helpdesk-Authorization",
                encode
                    .parse()
                    .expect("parse encoded help_desk string error"),
            );
        }

        Ok(headers)
    }

    async fn parse_bearer_token<IStore: Store, IHttpClient: HttpClient>(
        &mut self,
        lark: &LarkInner<IStore, IHttpClient>,
    ) -> Result<Option<String>, Error> {
        let mut res = None;
        if self.need_user_access_token && self.method_option.user_access_token.is_some() {
            res = self.method_option.user_access_token.take();
        } else if self.need_tenant_access_token {
            let (token, _) = lark.auth().get_tenant_access_token().await?;
            res = Some(token.token);
        } else if self.need_app_access_token {
            let (token, _) = lark.auth().get_app_access_token().await?;
            res = Some(token.token);
        }
        Ok(res)
    }

    fn parse_url(&mut self) -> String {
        let mut url = self.url.clone();
        if let Some(path) = &self.param_data.path {
            for (k, v) in path {
                if url.contains(&(":".to_string() + k)) {
                    url = url.replace(&(":".to_string() + k), v);
                } else {
                    url = url.replace(&("{".to_string() + k + "}"), v);
                }
            }
        }
        url
    }
}

impl<IStore: Store, IHttpClient: HttpClient> LarkInner<IStore, IHttpClient> {
    fn parse_request<ReqBody: Debug + Serialize + Send>(
        &self,
        req: &mut ApiRequest<ReqBody>,
    ) -> Result<HttpReq<ReqBody>, Error> {
        let headers = req.parse_header(self)?;
        if req.param_data.body.is_some() && req.stream_param_data.is_some() {
            return Err(Error::ErrDoReq(anyhow!(
                "body and stream_param_data should not have value at the same time"
            )));
        }
        let mut http_req = HttpReq {
            url: req.parse_url(),
            method: req.method.clone(),
            headers,
            body: req.param_data.body.take(),
            params: req.param_data.query.take(),
            timeout: self.timeout,
            bearer_token: None,
            stream_param: req.stream_param_data.take(),
        };
        if let Some(timeout) = req.method_option.timeout {
            http_req.timeout = timeout;
        }
        if let Some(headers) = req.method_option.attach_headers.take() {
            http_req.headers.extend(headers);
        }

        Ok(http_req)
    }

    pub(crate) async fn do_req<
        ReqBody: Debug + Serialize + Send + 'static,
        RespBody: Debug + DeserializeOwned + HasBaseResp + 'static,
    >(
        &self,
        mut req: ApiRequest<ReqBody>,
    ) -> Result<(RespBody, CommonResponse), Error> {
        let bearer_token = req.parse_bearer_token(self).await?;
        let mut http_req = self.parse_request(&mut req)?;
        http_req.bearer_token = bearer_token;
        self.do_inner_req(http_req, req).await
    }

    pub(crate) async fn do_req_without_auth<
        ReqBody: Debug + Serialize + Send,
        RespBody: Debug + DeserializeOwned + HasBaseResp,
    >(
        &self,
        mut req: ApiRequest<ReqBody>,
    ) -> Result<(RespBody, CommonResponse), Error> {
        let http_req = self.parse_request(&mut req)?;
        self.do_inner_req(http_req, req).await
    }

    async fn do_inner_req<
        ReqBody: Debug + Serialize + Send,
        RespBody: Debug + DeserializeOwned + HasBaseResp,
    >(
        &self,
        http_req: HttpReq<ReqBody>,
        api_req: ApiRequest<ReqBody>,
    ) -> Result<(RespBody, CommonResponse), Error> {
        let is_upload = if http_req.stream_param.is_some() {
            " upload "
        } else {
            " "
        };
        if api_req.method_option.log_req_and_resp {
            tracing::info!(
                "[lark sdk]{}request {}#{}, {} {}, header={:?}, body={:?}",
                is_upload,
                api_req.scope,
                api_req.api,
                api_req.method,
                api_req.url,
                http_req.headers,
                http_req.body
            )
        } else {
            tracing::debug!(
                "[lark sdk]{}request {}#{}, {} {}, header={:?}, body={:?}",
                is_upload,
                api_req.scope,
                api_req.api,
                api_req.method,
                api_req.url,
                http_req.headers,
                http_req.body
            );
        }

        let resp: HttpResp<RespBody> = match self.http_client.do_req(http_req).await {
            Ok(r) => r,
            Err(e) => match e {
                HttpError::TransDataErr(common_resp, e) => {
                    tracing::error!(
                        "[lark sdk] response error {}#{}, {} {}, header={:?}, status_code={:?}, error={:?}",
                        api_req.scope,
                        api_req.api,
                        common_resp.method,
                        common_resp.url,
                        common_resp.headers,
                        common_resp.status_code,
                        e
                    );

                    return Err(Error::ErrResponse(e, common_resp.into()));
                }
                HttpError::HttpCodeErr(common_resp, base_resp) => {
                    tracing::error!(
                        "[lark sdk] response error {}#{}, {} {}, header={:?}, status_code={:?}",
                        api_req.scope,
                        api_req.api,
                        common_resp.method,
                        common_resp.url,
                        common_resp.headers,
                        common_resp.status_code,
                    );

                    return Err(Error::ErrHttpCode(base_resp, common_resp.into()));
                }
                HttpError::Any(e) => {
                    tracing::error!("[lark sdk] send request error {}", e);
                    return Err(Error::ErrDoReq(e));
                }
            },
        };

        let common_resp: CommonResponse = resp.common_resp.into();

        if api_req.method_option.log_req_and_resp {
            tracing::info!(
                "[lark sdk] response {}#{}, {} {}, header={:?}, body={:?}",
                api_req.scope,
                api_req.api,
                common_resp.method,
                common_resp.url,
                common_resp.headers,
                resp.data
            );
        } else {
            tracing::debug!(
                "[lark sdk] response {}#{}, {} {}, header={:?}, body={:?}",
                api_req.scope,
                api_req.api,
                common_resp.method,
                common_resp.url,
                common_resp.headers,
                resp.data
            );
        }

        if resp.data.base_resp().code != 0 {
            tracing::error!(
                "[lark sdk] api response error {}#{}, {} {}, code={:?}, msg={:?}, err={:?}",
                api_req.scope,
                api_req.api,
                common_resp.method,
                common_resp.url,
                resp.data.base_resp().code,
                resp.data.base_resp().msg,
                resp.data.base_resp().err,
            );
            let err = Error::ErrApiResponse(resp.data.take_base_resp(), common_resp);
            return Err(err);
        }

        Ok((resp.data, common_resp))
    }

    #[allow(dead_code)]
    pub(crate) async fn do_download_req<ReqBody: Debug + Serialize + Send>(
        &self,
        mut api_req: ApiRequest<ReqBody>,
    ) -> Result<
        (
            <IHttpClient as HttpClient>::StreamRespData,
            Option<String>,
            CommonResponse,
        ),
        Error,
    > {
        let is_upload = if api_req.stream_param_data.is_some() {
            " upload "
        } else {
            " "
        };
        let bearer_token = api_req.parse_bearer_token(self).await?;
        let mut http_req = self.parse_request(&mut api_req)?;
        http_req.bearer_token = bearer_token;

        if api_req.method_option.log_req_and_resp {
            tracing::info!(
                "[lark sdk]{}request {}#{}, {} {}, header={:?}, body={:?}",
                is_upload,
                api_req.scope,
                api_req.api,
                api_req.method,
                api_req.url,
                http_req.headers,
                http_req.body
            )
        } else {
            tracing::debug!(
                "[lark sdk]{}request {}#{}, {} {}, header={:?}, body={:?}",
                is_upload,
                api_req.scope,
                api_req.api,
                api_req.method,
                api_req.url,
                http_req.headers,
                http_req.body
            );
        }

        let method = http_req.method.clone();
        let resp: HttpResp<_> = match self.http_client.do_stream_resp_req(http_req).await {
            Ok(r) => r,
            Err(e) => match e {
                HttpError::TransDataErr(common_resp, e) => {
                    tracing::error!(
                        "[lark sdk] download response error {}#{}, {} {}, header={:?}, status_code={:?}, error={:?}",
                        api_req.scope,
                        api_req.api,
                        method,
                        common_resp.url,
                        common_resp.headers,
                        common_resp.status_code,
                        e
                    );

                    return Err(Error::ErrResponse(e, common_resp.into()));
                }
                HttpError::HttpCodeErr(common_resp, base_resp) => {
                    tracing::error!(
                        "[lark sdk] download response error {}#{}, {} {}, header={:?}, status_code={:?}",
                        api_req.scope,
                        api_req.api,
                        method,
                        common_resp.url,
                        common_resp.headers,
                        common_resp.status_code,
                    );

                    return Err(Error::ErrHttpCode(base_resp, common_resp.into()));
                }
                HttpError::Any(e) => {
                    tracing::error!("[lark sdk] download send request error {}", e);
                    return Err(Error::ErrDoReq(e));
                }
            },
        };

        let common_resp: CommonResponse = resp.common_resp.into();
        let bin_name = common_resp
            .headers
            .get("Content-Disposition")
            .and_then(|v| {
                from_utf8(v.as_bytes()).ok().and_then(|v| {
                    v.split("filename=")
                        .nth(1)
                        .map(|v| v.trim_matches('"').to_string())
                })
            });

        if api_req.method_option.log_req_and_resp {
            tracing::info!(
                "[lark sdk] download response {}#{}, {} {}, header={:?}, bin_name={:?}",
                api_req.scope,
                api_req.api,
                method,
                common_resp.url,
                common_resp.headers,
                bin_name
            );
        } else {
            tracing::debug!(
                "[lark sdk] download response {}#{}, {} {}, header={:?}, bin_name={:?}",
                api_req.scope,
                api_req.api,
                method,
                common_resp.url,
                common_resp.headers,
                bin_name
            );
        }

        Ok((resp.data, bin_name, common_resp))
    }
}

#[allow(dead_code)]
pub struct DoReqMarker;

impl<IStore: Store, IHttpClient: HttpClient> Lark<IStore, IHttpClient> {
    pub async fn do_req<
        ReqBody: Debug + Serialize + Send + 'static,
        RespBody: Debug + DeserializeOwned + HasBaseResp + 'static,
    >(
        &self,
        req: ApiRequest<ReqBody>,
    ) -> Result<(RespBody, CommonResponse), Error> {
        #[cfg(feature = "test-util")]
        {
            if let Some(f) = self.mock().get_mock_do_req::<DoReqMarker, _, _>(&req) {
                tracing::info!("[lark] do req **mocking** api");
                return f(req);
            }
        }
        self.inner.do_req(req).await
    }

    pub async fn do_marker_req<
        Marker: 'static,
        ReqBody: Debug + Serialize + Send + 'static,
        RespBody: Debug + DeserializeOwned + HasBaseResp + 'static,
    >(
        &self,
        req: ApiRequest<ReqBody>,
    ) -> Result<(RespBody, CommonResponse), Error> {
        #[cfg(feature = "test-util")]
        {
            if let Some(f) = self.mock().get_mock_do_req::<Marker, _, _>(&req) {
                tracing::info!("[lark] do req **mocking** api");
                return f(req);
            }
        }
        self.inner.do_req(req).await
    }
}

#[cfg(test)]
mod tests {
    use http::Method;

    use crate::core::model::{MethodOption, ReqParam};

    use super::*;

    #[test]
    fn test_parse_url() {
        let mut req = ApiRequest::<()> {
            scope: "Auth",
            api: "GetAppAccessToken",
            method: Method::POST,
            url: "https://open.feishu.cn/open-apis/auth/v3/app_access_token/internal".to_string(),
            param_data: ReqParam {
                query: None,
                body: None,
                path: Some(
                    vec![("app_id".to_string(), "123".to_string())]
                        .into_iter()
                        .collect(),
                ),
            },
            need_tenant_access_token: false,
            need_app_access_token: false,
            need_user_access_token: false,
            need_help_desk_auth: false,
            method_option: MethodOption {
                user_access_token: None,
                timeout: None,
                ..Default::default()
            },
            ..Default::default()
        };
        assert_eq!(
            req.parse_url(),
            "https://open.feishu.cn/open-apis/auth/v3/app_access_token/internal"
        );

        req.url = "https://open.feishu.cn/open-apis/{app_secret}/v3/:app_id/internal".to_string();
        req.param_data.path = Some(
            vec![
                ("app_id".to_string(), "123".to_string()),
                ("app_secret".to_string(), "456".to_string()),
            ]
            .into_iter()
            .collect(),
        );
        assert_eq!(
            req.parse_url(),
            "https://open.feishu.cn/open-apis/456/v3/123/internal"
        );
    }
}
