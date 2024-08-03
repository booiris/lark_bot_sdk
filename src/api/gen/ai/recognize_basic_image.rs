//! doc url: <https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/ai/optical_char_recognition-v1/image/basic_recognize>
// Code generated by gen_api. DO NOT EDIT.

use serde::{Deserialize, Serialize};

use crate::api::BaseResp;
use crate::api::HasBaseResp;
use crate::core::model::{ApiRequest, CommonResponse, MethodOption, ReqParam, StreamReqParam};
use crate::{
    core::{http_client::HttpClient, store::Store},
    error::Error,
};

use crate::api::gen::ai::AiService;

impl<'c, IStore: Store, IClient: HttpClient> AiService<'c, IStore, IClient> {
    /// **api 版本: 2024-07-23T07:32:43+00:00**
    ///
    /// ## 识别图片中的文字
    ///
    /// 可识别图片中的文字，按图片中的区域划分，分段返回文本列表。
    ///

    ///
    /// doc: <https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/ai/optical_char_recognition-v1/image/basic_recognize>
    ///
    /// new doc: <https://open.feishu.cn/document/server-docs/ai/optical_char_recognition-v1/basic_recognize>
    ///
    /// api url: <https://open.feishu.cn/document_portal/v1/document/get_detail?fullPath=%2Fserver-docs%2Fai%2Foptical_char_recognition-v1%2Fbasic_recognize>
    pub async fn recognize_basic_image(
        &self,
        req: RecognizeBasicImageReq,
    ) -> Result<(RecognizeBasicImageResp, CommonResponse), Error> {
        self.recognize_basic_image_with_opt(req, Default::default())
            .await
    }

    /// 参见 [recognize_basic_image](#method.recognize_basic_image) 函数
    pub async fn recognize_basic_image_with_opt(
        &self,
        req: RecognizeBasicImageReq,
        method_option: MethodOption,
    ) -> Result<(RecognizeBasicImageResp, CommonResponse), Error> {
        #[cfg(feature = "test-util")]
        {
            if let Some(f) = self.mock().get_mock_recognize_basic_image(&req) {
                tracing::info!("[lark] Ai#RecognizeBasicImage **mocking** api");
                return f(req);
            }
        }

        tracing::info!("[lark] Ai#RecognizeBasicImage call api");

        let req = ApiRequest {
            scope: "Ai",
            api: "RecognizeBasicImage",
            method: http::Method::POST,
            url: String::new()
                + self.cli.open_base_url.as_ref()
                + "/open-apis/optical_char_recognition/v1/image/basic_recognize",
            param_data: req.gen_param(),
            method_option,
            need_tenant_access_token: true,
            ..Default::default()
        };

        let (resp, common_resp): (RecognizeBasicImageRespInner, _) = self.cli.do_req(req).await?;
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

#[derive(Debug, Serialize, Deserialize, Clone, Default, lark_bot_sdk_macros::ApiReqParams)]
pub struct RecognizeBasicImageReq {
    /// base64 后的图片数据
    ///
    /// **示例值**: "base64后的图片二进制数据"
    #[api(kind = "body", name = "image")]
    pub image: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, lark_bot_sdk_macros::ApiBaseResp)]
struct RecognizeBasicImageRespInner {
    #[serde(flatten)]
    data: Option<RecognizeBasicImageResp>,
    #[serde(flatten)]
    base: BaseResp,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct RecognizeBasicImageResp {
    /// \-
    #[serde(
        rename = "data",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub data: DataSubResp,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(default)]
pub struct DataSubResp {
    /// 按区域识别，返回文本列表
    ///
    /// **是否必填**: 是
    #[serde(
        rename = "text_list",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub text_list: Vec<String>,
}

#[cfg(feature = "test-util")]
mod test_utils {
    use std::sync::Arc;

    use self::gen::ai::AiServiceMocker;
    use crate::core::mocker::*;

    use super::*;
    use crate::api::*;
    use crate::core::model::*;

    pub struct Mocker;

    pub trait MockFunc:
        Fn(RecognizeBasicImageReq) -> Result<(RecognizeBasicImageResp, CommonResponse), Error>
        + Send
        + Sync
        + 'static
    {
    }
    impl<
            T: Fn(
                    RecognizeBasicImageReq,
                ) -> Result<(RecognizeBasicImageResp, CommonResponse), Error>
                + Send
                + Sync
                + 'static,
        > MockFunc for T
    {
    }

    impl<'c, IStore: Store, IClient: HttpClient> AiServiceMocker<'c, IStore, IClient> {
        pub fn mock_recognize_basic_image<F: MockFunc>(
            &self,
            f: F,
        ) -> MockerBuilder<Mocker, RecognizeBasicImageReq, RecognizeBasicImageResp, Arc<dyn MockFunc>>
        {
            MockerBuilder::new(self.cli.instance_id, Arc::new(f))
        }

        pub(super) fn get_mock_recognize_basic_image(
            &self,
            req: &RecognizeBasicImageReq,
        ) -> Option<Arc<dyn MockFunc>> {
            do_mock::<Mocker, RecognizeBasicImageReq, RecognizeBasicImageResp, Arc<dyn MockFunc>>(
                self.cli.instance_id,
                req,
            )
        }
    }
}

#[cfg(feature = "test-util")]
#[cfg(test)]
mod test {
    use crate::{
        api::gen::ai::recognize_basic_image::{
            RecognizeBasicImageReq, RecognizeBasicImageResp, RecognizeBasicImageRespInner,
        },
        core::{model::CommonResponse, Lark},
    };

    #[tokio::test]
    async fn test_mock() {
        let lark = Lark::new("".to_owned(), "".into());
        let mocker = lark
            .ai()
            .mock()
            .mock_recognize_basic_image(|_| {
                Ok((
                    RecognizeBasicImageResp::default(),
                    CommonResponse::default(),
                ))
            })
            .build();
        let res = lark
            .ai()
            .recognize_basic_image(RecognizeBasicImageReq::default())
            .await;
        assert!(res.is_ok());
        mocker.clear();
        let res = lark
            .ai()
            .recognize_basic_image(RecognizeBasicImageReq::default())
            .await;
        assert!(res.is_err());
    }

    const REQ: &str = r#"{
    "image": "base64后的图片二进制数据"
}"#;

    #[test]
    fn test_req() {
        if REQ == "{}" {
            return;
        }
        if let Err(e) = serde_json::from_str::<super::RecognizeBasicImageReqBody>(REQ) {
            panic!("{}", e);
        }
    }

    const RESP: &str = r#"{
    "code": 0,
    "msg": "success",
    "data": {
        "text_list": [
            "你好"
        ]
    }
}"#;
    #[test]
    fn test_resp() {
        let res = serde_json::from_str::<RecognizeBasicImageRespInner>(RESP);
        if let Err(e) = res {
            panic!("{}", e);
        }
        if let Ok(v) = serde_json::from_str::<serde_json::Value>(RESP) {
            if v.get("data").is_some() {
                assert!(res.unwrap().data.is_some());
            }
        }
    }
}
