//! doc url: <https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/ai/document_ai-v1/id_card/recognize>
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

use crate::api::gen::ai::AiService;

impl<'c, IStore: Store, IClient: HttpClient> AiService<'c, IStore, IClient> {
    /// **api 版本: 2023-10-31T16:06:56+00:00**
    ///
    /// ## 识别文件中的身份证
    ///
    /// 身份证识别接口，支持JPG/JPEG/PNG/BMP四种文件类型的一次性的识别。
    ///
    /// 单租户限流：10QPS，同租户下的应用没有限流，共享本租户的 10QPS 限流
    ///
    /// doc: <https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/ai/document_ai-v1/id_card/recognize>
    ///
    /// new doc: <https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/ai/document_ai-v1/id_card/recognize>
    ///
    /// api url: <https://open.feishu.cn/document_portal/v1/document/get_detail?fullPath=%2FuAjLw4CM%2FukTMukTMukTM%2Freference%2Fai%2Fdocument_ai-v1%2Fid_card%2Frecognize>
    pub async fn recognize_aiid_card<Data: StreamReqData>(
        &self,
        req: RecognizeAiidCardReq<Data>,
    ) -> Result<(RecognizeAiidCardResp, CommonResponse), Error> {
        self.recognize_aiid_card_with_opt(req, Default::default())
            .await
    }

    /// 参见 [recognize_aiid_card](#method.recognize_aiid_card) 函数
    pub async fn recognize_aiid_card_with_opt<Data: StreamReqData>(
        &self,
        req: RecognizeAiidCardReq<Data>,
        method_option: MethodOption,
    ) -> Result<(RecognizeAiidCardResp, CommonResponse), Error> {
        #[cfg(feature = "test-util")]
        {
            if let Some(f) = self.mock().get_mock_recognize_aiid_card(&req) {
                tracing::info!("[lark] Ai#RecognizeAiidCard **mocking** api");
                return f(req);
            }
        }

        tracing::info!("[lark] Ai#RecognizeAiidCard call api");

        let req = ApiRequest::<()> {
            scope: "Ai",
            api: "RecognizeAiidCard",
            method: http::Method::POST,
            url: String::new()
                + self.cli.open_base_url.as_ref()
                + "/open-apis/document_ai/v1/id_card/recognize",
            stream_param_data: req.gen_stream_param(),
            method_option,
            need_tenant_access_token: true,
            ..Default::default()
        };

        let (resp, common_resp): (RecognizeAiidCardRespInner, _) = self.cli.do_req(req).await?;
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
pub struct RecognizeAiidCardReq<Data: StreamReqData> {
    #[api(kind = "stream", name = "#data#", option = "false")]
    pub data: Data,
}

#[derive(Debug, Serialize, Deserialize, Clone, lark_bot_sdk_macros::ApiBaseResp)]
struct RecognizeAiidCardRespInner {
    #[serde(flatten)]
    data: Option<RecognizeAiidCardResp>,
    #[serde(flatten)]
    base: BaseResp,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct RecognizeAiidCardResp {
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
    /// 身份证信息
    #[serde(
        rename = "id_card",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub id_card: IdCardSubResp,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(default)]
pub struct IdCardSubResp {
    /// 识别的实体列表
    #[serde(
        rename = "entities",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub entities: Vec<IdEntitySubResp>,
    /// 正反面，1为身份证-姓名页，0为身份证-国徽页
    ///
    /// **示例值**: "0"
    #[serde(
        rename = "side",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub side: i64,
    /// 四角坐标[x0,y0,x1,y1,x2,y2,x3,y3]
    #[serde(
        rename = "conners",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub conners: Vec<i64>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(default)]
pub struct IdEntitySubResp {
    /// 识别的字段种类
    ///
    /// **示例值**: "identity_name"
    ///
    /// **可选值**:
    ///
    /// `IdentityCode`: 公民身份号码
    ///
    /// `IdentityName`: 姓名
    ///
    /// `Address`: 住址
    ///
    /// `ValidDateStart`: 有效期起始时间
    ///
    /// `ValideDateEnd`: 有效期终止时间（“长期”识别为“长期”）
    ///
    /// `Gender`: 性别
    ///
    /// `Race`: 民族
    ///
    /// `IssuedBy`: 签发机关
    ///
    /// `Birth`: 出生日期
    #[serde(
        rename = "type",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub body_type: String,
    /// 识别出字段的文本信息
    ///
    /// **示例值**: "张三"
    #[serde(
        rename = "value",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub value: String,
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

    pub trait MockFunc<D: StreamReqData>:
        Fn(RecognizeAiidCardReq<D>) -> Result<(RecognizeAiidCardResp, CommonResponse), Error>
        + Send
        + Sync
        + 'static
    {
    }
    impl<
            D: StreamReqData,
            T: Fn(
                    RecognizeAiidCardReq<D>,
                ) -> Result<(RecognizeAiidCardResp, CommonResponse), Error>
                + Send
                + Sync
                + 'static,
        > MockFunc<D> for T
    {
    }

    impl<'c, IStore: Store, IClient: HttpClient> AiServiceMocker<'c, IStore, IClient> {
        pub fn mock_recognize_aiid_card<T: StreamReqData, F: MockFunc<T>>(
            &self,
            f: F,
        ) -> MockerBuilder<
            Mocker,
            RecognizeAiidCardReq<T>,
            RecognizeAiidCardResp,
            Arc<dyn MockFunc<T>>,
        > {
            MockerBuilder::new(self.cli.instance_id, Arc::new(f))
        }

        pub(super) fn get_mock_recognize_aiid_card<T: StreamReqData>(
            &self,
            req: &RecognizeAiidCardReq<T>,
        ) -> Option<Arc<dyn MockFunc<T>>> {
            do_mock::<Mocker, RecognizeAiidCardReq<T>, RecognizeAiidCardResp, Arc<dyn MockFunc<T>>>(
                self.cli.instance_id,
                req,
            )
        }
    }
}
