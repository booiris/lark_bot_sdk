//! doc url: <https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/ai/document_ai-v1/business_card/recognize>
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
    /// **api 版本: 2023-10-31T16:08:07+00:00**
    ///
    /// ## 识别文件中的名片
    ///
    /// 名片识别接口，通过上传 JPG / PNG / PDF 等文件类型进行一次性的名片识别。接口适用于20MB以下的文件，适用于英文、日语的名片。
    ///
    /// 单租户限流：10QPS，同租户下的应用没有限流，共享本租户的 10QPS 限流
    ///
    /// doc: <https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/ai/document_ai-v1/business_card/recognize>
    ///
    /// new doc: <https://open.feishu.cn/document/server-docs/ai/document_ai-v1/business_card/recognize>
    ///
    /// api url: <https://open.feishu.cn/document_portal/v1/document/get_detail?fullPath=%2Fserver-docs%2Fai%2Fdocument_ai-v1%2Fbusiness_card%2Frecognize>
    pub async fn recognize_ai_business_card<Data: StreamReqData>(
        &self,
        req: RecognizeAiBusinessCardReq<Data>,
    ) -> Result<(RecognizeAiBusinessCardResp, CommonResponse), Error> {
        self.recognize_ai_business_card_with_opt(req, Default::default())
            .await
    }

    /// 参见 [recognize_ai_business_card](#method.recognize_ai_business_card) 函数
    pub async fn recognize_ai_business_card_with_opt<Data: StreamReqData>(
        &self,
        req: RecognizeAiBusinessCardReq<Data>,
        method_option: MethodOption,
    ) -> Result<(RecognizeAiBusinessCardResp, CommonResponse), Error> {
        #[cfg(feature = "test-util")]
        {
            if let Some(f) = self.mock().get_mock_recognize_ai_business_card(&req) {
                tracing::info!("[lark] Ai#RecognizeAiBusinessCard **mocking** api");
                return f(req);
            }
        }

        tracing::info!("[lark] Ai#RecognizeAiBusinessCard call api");

        let req = ApiRequest::<()> {
            scope: "Ai",
            api: "RecognizeAiBusinessCard",
            method: http::Method::POST,
            url: String::new()
                + self.cli.open_base_url.as_ref()
                + "/open-apis/document_ai/v1/business_card/recognize",
            stream_param_data: req.gen_stream_param(),
            method_option,
            need_tenant_access_token: true,
            ..Default::default()
        };

        let (resp, common_resp): (RecognizeAiBusinessCardRespInner, _) =
            self.cli.do_req(req).await?;
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
pub struct RecognizeAiBusinessCardReq<Data: StreamReqData> {
    #[api(kind = "stream", name = "#data#", option = "false")]
    pub data: Data,
}

#[derive(Debug, Serialize, Deserialize, Clone, lark_bot_sdk_macros::ApiBaseResp)]
struct RecognizeAiBusinessCardRespInner {
    #[serde(flatten)]
    data: Option<RecognizeAiBusinessCardResp>,
    #[serde(flatten)]
    base: BaseResp,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct RecognizeAiBusinessCardResp {
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
    /// 名片信息
    #[serde(
        rename = "business_cards",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub business_cards: Vec<RecognizedEntitiesSubResp>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(default)]
pub struct RecognizedEntitiesSubResp {
    /// 识别的实体列表
    #[serde(
        rename = "entities",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub entities: Vec<RecognizedEntitySubResp>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(default)]
pub struct RecognizedEntitySubResp {
    /// 识别的字段种类
    ///
    /// **示例值**: "contact_names"
    ///
    /// **可选值**:
    ///
    /// `ContactNames`: 联系人名
    ///
    /// `CompanyNames`: 公司名
    ///
    /// `Departments`: 部门
    ///
    /// `JobTitles`: 职位
    ///
    /// `Emails`: 邮箱
    ///
    /// `Websites`: 网站
    ///
    /// `Addresses`: 地址
    ///
    /// `MobilePhones`: 移动电话
    ///
    /// `WorkPhones`: 工作电话
    ///
    /// `OtherPhones`: 其他电话
    ///
    /// `Faxes`: 传真
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
        Fn(
            RecognizeAiBusinessCardReq<D>,
        ) -> Result<(RecognizeAiBusinessCardResp, CommonResponse), Error>
        + Send
        + Sync
        + 'static
    {
    }
    impl<
            D: StreamReqData,
            T: Fn(
                    RecognizeAiBusinessCardReq<D>,
                ) -> Result<(RecognizeAiBusinessCardResp, CommonResponse), Error>
                + Send
                + Sync
                + 'static,
        > MockFunc<D> for T
    {
    }

    impl<'c, IStore: Store, IClient: HttpClient> AiServiceMocker<'c, IStore, IClient> {
        pub fn mock_recognize_ai_business_card<T: StreamReqData, F: MockFunc<T>>(
            &self,
            f: F,
        ) -> MockerBuilder<
            Mocker,
            RecognizeAiBusinessCardReq<T>,
            RecognizeAiBusinessCardResp,
            Arc<dyn MockFunc<T>>,
        > {
            MockerBuilder::new(self.cli.instance_id, Arc::new(f))
        }

        pub(super) fn get_mock_recognize_ai_business_card<T: StreamReqData>(
            &self,
            req: &RecognizeAiBusinessCardReq<T>,
        ) -> Option<Arc<dyn MockFunc<T>>> {
            do_mock::<
                Mocker,
                RecognizeAiBusinessCardReq<T>,
                RecognizeAiBusinessCardResp,
                Arc<dyn MockFunc<T>>,
            >(self.cli.instance_id, req)
        }
    }
}
