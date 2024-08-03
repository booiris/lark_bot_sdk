//! doc url: <https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/okr-v1/image/upload>
// Code generated by gen_api. DO NOT EDIT.

use serde::{Deserialize, Serialize};

use crate::api::BaseResp;
use crate::api::HasBaseResp;
use crate::core::model::{
    ApiRequest, CommonResponse, MethodOption, ReqParam, StreamReqData, StreamReqParam,
};
use crate::{
    core::{http_client::HttpClient, store::Store},
    error::Error,
};

use crate::api::gen::okr::OkrService;

impl<'c, IStore: Store, IClient: HttpClient> OkrService<'c, IStore, IClient> {
    /// **api 版本: 2024-01-09T02:16:19+00:00**
    ///
    /// ## 上传进展记录图片
    ///
    /// 上传进展记录图片。
    ///

    ///
    /// doc: <https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/okr-v1/image/upload>
    ///
    /// new doc: <https://open.feishu.cn/document/server-docs/okr-v1/progress_record/upload>
    ///
    /// api url: <https://open.feishu.cn/document_portal/v1/document/get_detail?fullPath=%2Fserver-docs%2Fokr-v1%2Fprogress_record%2Fupload>
    pub async fn upload_okr_image<Data: StreamReqData>(
        &self,
        req: UploadOkrImageReq<Data>,
    ) -> Result<(UploadOkrImageResp, CommonResponse), Error> {
        self.upload_okr_image_with_opt(req, Default::default())
            .await
    }

    /// 参见 [upload_okr_image](#method.upload_okr_image) 函数
    pub async fn upload_okr_image_with_opt<Data: StreamReqData>(
        &self,
        req: UploadOkrImageReq<Data>,
        method_option: MethodOption,
    ) -> Result<(UploadOkrImageResp, CommonResponse), Error> {
        #[cfg(feature = "test-util")]
        {
            if let Some(f) = self.mock().get_mock_upload_okr_image(&req) {
                tracing::info!("[lark] Okr#UploadOkrImage **mocking** api");
                return f(req);
            }
        }

        tracing::info!("[lark] Okr#UploadOkrImage call api");

        let req = ApiRequest::<()> {
            scope: "Okr",
            api: "UploadOkrImage",
            method: http::Method::POST,
            url: String::new()
                + self.cli.open_base_url.as_ref()
                + "/open-apis/okr/v1/images/upload",
            stream_param_data: req.gen_stream_param(),
            method_option,
            need_tenant_access_token: true,
            need_user_access_token: true,
            ..Default::default()
        };

        let (resp, common_resp): (UploadOkrImageRespInner, _) = self.cli.do_req(req).await?;
        let data = match resp.data {
            Some(data) => data,
            None => {
                return Err(Error::ErrResponse(
                    anyhow::anyhow!("missing response data"),
                    common_resp,
                ));
            }
        };
        Ok((data, common_resp))
    }
}

#[derive(Debug, lark_bot_sdk_macros::ApiReqParams)]
pub struct UploadOkrImageReq<Data: StreamReqData> {
    /// 图片的目标ID
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "6974586812998174252"
    #[api(kind = "stream", name = "target_id", option = "false")]
    pub target_id: String,
    /// 图片使用的目标类型
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "1"
    ///
    /// **可选值**:
    ///
    /// `objective`: okr的O
    ///
    /// `key_result`: okr的KR
    #[api(kind = "stream", name = "target_type", option = "false")]
    pub target_type: i64,
    #[api(kind = "stream", name = "#data#", option = "false")]
    pub data: Data,
}

#[derive(Debug, Serialize, Deserialize, Clone, lark_bot_sdk_macros::ApiBaseResp)]
struct UploadOkrImageRespInner {
    #[serde(flatten)]
    data: Option<UploadOkrImageResp>,
    #[serde(flatten)]
    base: BaseResp,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct UploadOkrImageResp {
    /// \-
    #[serde(
        rename = "data",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub data: ImageInfoSubResp,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(default)]
pub struct ImageInfoSubResp {
    /// 图片token
    ///
    /// **示例值**: "boxbcLxEnhUE3REJSAwAbVFZwPf"
    #[serde(
        rename = "file_token",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub file_token: String,
    /// 图片下载链接
    ///
    /// **示例值**: "https://bytedance.feishu.cn/drive/home/"
    #[serde(
        rename = "url",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub url: String,
}

#[cfg(feature = "test-util")]
mod test_utils {
    use std::sync::Arc;

    use self::gen::okr::OkrServiceMocker;
    use crate::core::mocker::*;

    use super::*;
    use crate::api::*;
    use crate::core::model::*;

    pub struct Mocker;

    pub trait MockFunc<D: StreamReqData>:
        Fn(UploadOkrImageReq<D>) -> Result<(UploadOkrImageResp, CommonResponse), Error>
        + Send
        + Sync
        + 'static
    {
    }
    impl<
            D: StreamReqData,
            T: Fn(UploadOkrImageReq<D>) -> Result<(UploadOkrImageResp, CommonResponse), Error>
                + Send
                + Sync
                + 'static,
        > MockFunc<D> for T
    {
    }

    impl<'c, IStore: Store, IClient: HttpClient> OkrServiceMocker<'c, IStore, IClient> {
        pub fn mock_upload_okr_image<T: StreamReqData, F: MockFunc<T>>(
            &self,
            f: F,
        ) -> MockerBuilder<Mocker, UploadOkrImageReq<T>, UploadOkrImageResp, Arc<dyn MockFunc<T>>>
        {
            MockerBuilder::new(self.cli.instance_id, Arc::new(f))
        }

        pub(super) fn get_mock_upload_okr_image<T: StreamReqData>(
            &self,
            req: &UploadOkrImageReq<T>,
        ) -> Option<Arc<dyn MockFunc<T>>> {
            do_mock::<Mocker, UploadOkrImageReq<T>, UploadOkrImageResp, Arc<dyn MockFunc<T>>>(
                self.cli.instance_id,
                req,
            )
        }
    }
}
