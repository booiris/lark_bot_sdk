use core::time;
use std::fmt::Debug;
use std::future::Future;

use anyhow::Context;
use bytes::Bytes;
use futures_util::Stream;
use http::{HeaderMap, StatusCode};
use reqwest::{Body, Client, Url};
use serde::{de::DeserializeOwned, Serialize};
use thiserror::Error;

use crate::api::BaseResp;

use super::model::StreamReqParam;

#[derive(Error, Debug)]
pub enum HttpError {
    #[error("trans data error: {1}")]
    /// http 传输过程中出现的 error, 或者是返回的 data 结构不合法
    TransDataErr(HttpCommonResp, anyhow::Error),
    #[error("http code invalid: {1:?}")]
    /// http code 为错误码，4xx 或 5xx
    HttpCodeErr(HttpCommonResp, Option<BaseResp>),
    #[error(transparent)]
    Any(#[from] anyhow::Error),
}

#[derive(Debug)]
pub struct HttpReq<Body: Debug> {
    pub url: String,
    pub params: Option<Vec<(String, String)>>,
    pub method: http::Method,
    pub headers: http::HeaderMap,
    pub body: Option<Body>,
    pub timeout: time::Duration,
    pub bearer_token: Option<String>,
    pub stream_param: Option<StreamReqParam>,
}

pub struct HttpResp<Data> {
    pub common_resp: HttpCommonResp,
    pub data: Data,
}

#[derive(Debug, Clone, Default)]
pub struct HttpCommonResp {
    pub headers: HeaderMap,
    pub status_code: StatusCode,
    pub content_length: Option<u64>,
    pub url: String,
    pub method: http::Method,
}

pub trait HttpClient: Send + Sync + 'static {
    type StreamRespData: Stream + Send + Unpin;

    fn do_req<B: Send + Serialize + Debug, RespData: DeserializeOwned>(
        &self,
        req: HttpReq<B>,
    ) -> impl Future<Output = Result<HttpResp<RespData>, HttpError>> + Send;

    fn do_stream_resp_req<B: Send + Serialize + Debug>(
        &self,
        req: HttpReq<B>,
    ) -> impl Future<Output = Result<HttpResp<Self::StreamRespData>, HttpError>> + Send;
}

pub struct DefaultClient {
    client: Client,
}

impl HttpClient for DefaultClient {
    async fn do_req<B: Send + Serialize + Debug, RespData: DeserializeOwned>(
        &self,
        req: HttpReq<B>,
    ) -> Result<HttpResp<RespData>, HttpError> {
        let req = self.to_req(req)?;
        tracing::trace!("[lark sdk] http req: {:?}", req);
        let method = req.method().to_owned();
        let resp = self
            .client
            .execute(req)
            .await
            .context("client send req err")?;
        let headers = resp.headers().to_owned();
        let status_code = resp.status().to_owned();
        let content_length = resp.content_length().to_owned().or(resp
            .headers()
            .get("Content-Length")
            .and_then(|x| x.to_str().ok().and_then(|x| x.parse().ok())));
        let url = resp.url().to_string();
        let common_resp = HttpCommonResp {
            headers,
            status_code,
            content_length,
            url,
            method,
        };

        let data = resp.text().await;
        tracing::trace!("[lark sdk] resp data: {:?}", data);

        if status_code.is_server_error() || status_code.is_client_error() {
            let base_resp = data
                .map(|x| serde_json::from_str::<BaseResp>(&x).ok())
                .ok()
                .flatten();
            return Err(HttpError::HttpCodeErr(common_resp, base_resp));
        }

        let data = match data {
            Ok(d) => match serde_json::from_str(&d) {
                Ok(d) => d,
                Err(e) => return Err(HttpError::TransDataErr(common_resp, e.into())),
            },
            Err(e) => return Err(HttpError::TransDataErr(common_resp, e.into())),
        };
        Ok(HttpResp { data, common_resp })
    }

    type StreamRespData = Box<dyn Stream<Item = reqwest::Result<Bytes>> + Unpin + Send>;

    async fn do_stream_resp_req<B: Send + Serialize + Debug>(
        &self,
        req: HttpReq<B>,
    ) -> Result<HttpResp<Self::StreamRespData>, HttpError> {
        let req = self.to_req(req)?;
        tracing::trace!("[lark sdk] http req: {:?}", req);
        let method = req.method().to_owned();
        let resp = self
            .client
            .execute(req)
            .await
            .context("client send stream req err")?;
        let headers = resp.headers().to_owned();
        let status_code = resp.status().to_owned();
        let content_length = resp.content_length().to_owned().or(resp
            .headers()
            .get("Content-Length")
            .and_then(|x| x.to_str().ok().and_then(|x| x.parse().ok())));
        let url = resp.url().to_string();
        let common_resp = HttpCommonResp {
            headers,
            status_code,
            content_length,
            url,
            method,
        };

        if resp.status().is_server_error() || resp.status().is_client_error() {
            let base_resp = resp.json::<BaseResp>().await.ok();
            return Err(HttpError::HttpCodeErr(common_resp, base_resp));
        }

        let data = Box::new(resp.bytes_stream());
        Ok(HttpResp { data, common_resp })
    }
}

impl DefaultClient {
    pub fn new() -> Self {
        Self {
            client: Client::new(),
        }
    }

    fn to_req<B: Send + Serialize + Debug>(
        &self,
        req: HttpReq<B>,
    ) -> Result<reqwest::Request, HttpError> {
        let url = if let Some(p) = req.params {
            Url::parse_with_params(req.url.as_ref(), p).context("parse req url invalid")?
        } else {
            Url::parse(req.url.as_ref()).context("parse req url invalid")?
        };
        let mut raw_req = self
            .client
            .request(req.method.to_owned(), url)
            .headers(req.headers)
            .header("User-Agent", "rust_lark_sdk")
            .timeout(req.timeout);
        if let Some(body) = req.body {
            raw_req = raw_req.json(&body);
        }
        if let Some(token) = req.bearer_token {
            raw_req = raw_req.bearer_auth(token);
        }
        if let Some(stream_param) = req.stream_param {
            let stream = tokio_util::io::ReaderStream::new(stream_param.data);
            let file =
                reqwest::multipart::Part::stream(Body::wrap_stream(stream)).file_name("file.file");

            let mut form = reqwest::multipart::Form::new().part("file", file);
            for (k, v) in stream_param.field {
                form = form.text(k, v);
            }

            raw_req = raw_req.multipart(form);
        }
        Ok(raw_req.build().context("build http req error")?)
    }
}

impl Default for DefaultClient {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use futures_util::StreamExt;
    use reqwest::StatusCode;
    use serde::{Deserialize, Serialize};
    use std::time::Duration;

    #[derive(Serialize, Deserialize, Debug)]
    pub struct A {}

    #[tokio::test]
    async fn test_do_req() {
        let client = DefaultClient::new();
        let req = HttpReq::<A> {
            url: "https://httpbin.org/get".to_string(),
            method: reqwest::Method::GET,
            headers: reqwest::header::HeaderMap::new(),
            body: Some(A {}),
            params: None,
            timeout: Duration::from_secs(10),
            bearer_token: None,
            stream_param: None,
        };
        let resp = client.do_req(req).await;
        assert!(resp.is_ok());
        let resp: HttpResp<A> = resp.unwrap();
        assert_eq!(resp.common_resp.status_code, StatusCode::OK);
    }

    #[tokio::test]
    async fn test_do_steam_resp_req() {
        let client = DefaultClient::new();
        let req = HttpReq::<String> {
            url: "https://httpbin.org/json".to_string(),
            method: reqwest::Method::GET,
            headers: reqwest::header::HeaderMap::new(),
            body: None,
            params: None,
            timeout: Duration::from_secs(10),
            bearer_token: None,
            stream_param: None,
        };
        let resp = client.do_stream_resp_req(req).await;
        assert!(resp.is_ok());
        let mut resp = resp.unwrap();
        assert_eq!(resp.common_resp.status_code, StatusCode::OK);
        let content_len = resp
            .common_resp
            .content_length
            .expect("content len missing");
        let mut cnt = 0;
        while let Some(result) = resp.data.next().await {
            match result {
                Ok(bytes) => {
                    cnt += bytes.len();
                }
                Err(err) => {
                    panic!("{}", err)
                }
            }
        }
        assert_eq!(content_len, cnt as u64);
    }
}
