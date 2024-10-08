//! doc url: <https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/ai/document_ai-v1/tw_mainland_travel_permit/recognize>
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
    /// **api 版本: 2023-10-31T16:05:45+00:00**
    ///
    /// ## 识别文件中的台湾居民来往大陆通行证
    ///
    /// 台湾居民来往大陆通行证识别接口，支持JPG/JPEG/PNG/BMP四种文件类型的一次性的识别。
    ///
    /// 单租户限流：10QPS，同租户下的应用没有限流，共享本租户的 10QPS 限流
    ///
    /// doc: <https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/ai/document_ai-v1/tw_mainland_travel_permit/recognize>
    ///
    /// new doc: <https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/ai/document_ai-v1/tw_mainland_travel_permit/recognize>
    ///
    /// api url: <https://open.feishu.cn/document_portal/v1/document/get_detail?fullPath=%2FuAjLw4CM%2FukTMukTMukTM%2Freference%2Fai%2Fdocument_ai-v1%2Ftw_mainland_travel_permit%2Frecognize>
    pub async fn recognize_ai_tw_mainland_travel_permit<Data: StreamReqData>(
        &self,
        req: RecognizeAiTwMainlandTravelPermitReq<Data>,
    ) -> Result<(RecognizeAiTwMainlandTravelPermitResp, CommonResponse), Error> {
        self.recognize_ai_tw_mainland_travel_permit_with_opt(req, Default::default())
            .await
    }

    /// 参见 [recognize_ai_tw_mainland_travel_permit](#method.recognize_ai_tw_mainland_travel_permit) 函数
    pub async fn recognize_ai_tw_mainland_travel_permit_with_opt<Data: StreamReqData>(
        &self,
        req: RecognizeAiTwMainlandTravelPermitReq<Data>,
        method_option: MethodOption,
    ) -> Result<(RecognizeAiTwMainlandTravelPermitResp, CommonResponse), Error> {
        #[cfg(feature = "test-util")]
        {
            if let Some(f) = self
                .mock()
                .get_mock_recognize_ai_tw_mainland_travel_permit(&req)
            {
                tracing::info!("[lark] Ai#RecognizeAiTwMainlandTravelPermit **mocking** api");
                return f(req);
            }
        }

        tracing::info!("[lark] Ai#RecognizeAiTwMainlandTravelPermit call api");

        let req = ApiRequest::<()> {
            scope: "Ai",
            api: "RecognizeAiTwMainlandTravelPermit",
            method: http::Method::POST,
            url: String::new()
                + self.cli.open_base_url.as_ref()
                + "/open-apis/document_ai/v1/tw_mainland_travel_permit/recognize",
            stream_param_data: req.gen_stream_param(),
            method_option,
            need_tenant_access_token: true,
            ..Default::default()
        };

        let (resp, common_resp): (RecognizeAiTwMainlandTravelPermitRespInner, _) =
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
pub struct RecognizeAiTwMainlandTravelPermitReq<Data: StreamReqData> {
    #[api(kind = "stream", name = "#data#", option = "false")]
    pub data: Data,
}

#[derive(Debug, Serialize, Deserialize, Clone, lark_bot_sdk_macros::ApiBaseResp)]
struct RecognizeAiTwMainlandTravelPermitRespInner {
    #[serde(flatten)]
    data: Option<RecognizeAiTwMainlandTravelPermitResp>,
    #[serde(flatten)]
    base: BaseResp,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct RecognizeAiTwMainlandTravelPermitResp {
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
    /// 台湾居民来往大陆通行证信息
    #[serde(
        rename = "tw_mainland_travel_permit",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub tw_mainland_travel_permit: TwMainlandTravelPermitSubResp,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(default)]
pub struct TwMainlandTravelPermitSubResp {
    /// 识别出的实体类型
    #[serde(
        rename = "entities",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub entities: Vec<TwMainlandTravelPermitEntitySubResp>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(default)]
pub struct TwMainlandTravelPermitEntitySubResp {
    /// 识别的字段种类
    ///
    /// **示例值**: "full_name_cn"
    ///
    /// **可选值**:
    ///
    /// `FullNameCN`: 中文姓名
    ///
    /// `FullNameEN`: 英文格式姓名
    ///
    /// `DateOfBirth`: 出生日期
    ///
    /// `DateOfExpiry`: 有效期至
    ///
    /// `CardNumber`: 证件号码
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
            RecognizeAiTwMainlandTravelPermitReq<D>,
        ) -> Result<(RecognizeAiTwMainlandTravelPermitResp, CommonResponse), Error>
        + Send
        + Sync
        + 'static
    {
    }
    impl<
            D: StreamReqData,
            T: Fn(
                    RecognizeAiTwMainlandTravelPermitReq<D>,
                )
                    -> Result<(RecognizeAiTwMainlandTravelPermitResp, CommonResponse), Error>
                + Send
                + Sync
                + 'static,
        > MockFunc<D> for T
    {
    }

    impl<'c, IStore: Store, IClient: HttpClient> AiServiceMocker<'c, IStore, IClient> {
        pub fn mock_recognize_ai_tw_mainland_travel_permit<T: StreamReqData, F: MockFunc<T>>(
            &self,
            f: F,
        ) -> MockerBuilder<
            Mocker,
            RecognizeAiTwMainlandTravelPermitReq<T>,
            RecognizeAiTwMainlandTravelPermitResp,
            Arc<dyn MockFunc<T>>,
        > {
            MockerBuilder::new(self.cli.instance_id, Arc::new(f))
        }

        pub(super) fn get_mock_recognize_ai_tw_mainland_travel_permit<T: StreamReqData>(
            &self,
            req: &RecognizeAiTwMainlandTravelPermitReq<T>,
        ) -> Option<Arc<dyn MockFunc<T>>> {
            do_mock::<
                Mocker,
                RecognizeAiTwMainlandTravelPermitReq<T>,
                RecognizeAiTwMainlandTravelPermitResp,
                Arc<dyn MockFunc<T>>,
            >(self.cli.instance_id, req)
        }
    }
}
