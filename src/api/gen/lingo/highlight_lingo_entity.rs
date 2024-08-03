//! doc url: <https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/lingo-v1/entity/highlight>
// Code generated by gen_api. DO NOT EDIT.

use serde::{Deserialize, Serialize};

use crate::api::BaseResp;
use crate::api::HasBaseResp;
use crate::core::model::{ApiRequest, CommonResponse, MethodOption, ReqParam, StreamReqParam};
use crate::{
    core::{http_client::HttpClient, store::Store},
    error::Error,
};

use crate::api::gen::lingo::LingoService;

impl<'c, IStore: Store, IClient: HttpClient> LingoService<'c, IStore, IClient> {
    /// **api 版本: 2023-10-25T08:26:40+00:00**
    ///
    /// ## 词条高亮
    ///
    /// 传入一句话，智能识别句中对应的词条，并返回词条位置和 entity_id，可在外部系统中快速实现词条智能高亮。
    ///

    ///
    /// doc: <https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/lingo-v1/entity/highlight>
    ///
    /// new doc: <https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/lingo-v1/entity/highlight>
    ///
    /// api url: <https://open.feishu.cn/document_portal/v1/document/get_detail?fullPath=%2FuAjLw4CM%2FukTMukTMukTM%2Flingo-v1%2Fentity%2Fhighlight>
    pub async fn highlight_lingo_entity(
        &self,
        req: HighlightLingoEntityReq,
    ) -> Result<(HighlightLingoEntityResp, CommonResponse), Error> {
        self.highlight_lingo_entity_with_opt(req, Default::default())
            .await
    }

    /// 参见 [highlight_lingo_entity](#method.highlight_lingo_entity) 函数
    pub async fn highlight_lingo_entity_with_opt(
        &self,
        req: HighlightLingoEntityReq,
        method_option: MethodOption,
    ) -> Result<(HighlightLingoEntityResp, CommonResponse), Error> {
        #[cfg(feature = "test-util")]
        {
            if let Some(f) = self.mock().get_mock_highlight_lingo_entity(&req) {
                tracing::info!("[lark] Lingo#HighlightLingoEntity **mocking** api");
                return f(req);
            }
        }

        tracing::info!("[lark] Lingo#HighlightLingoEntity call api");

        let req = ApiRequest {
            scope: "Lingo",
            api: "HighlightLingoEntity",
            method: http::Method::POST,
            url: String::new()
                + self.cli.open_base_url.as_ref()
                + "/open-apis/lingo/v1/entities/highlight",
            param_data: req.gen_param(),
            method_option,
            need_tenant_access_token: true,
            need_user_access_token: true,
            ..Default::default()
        };

        let (resp, common_resp): (HighlightLingoEntityRespInner, _) = self.cli.do_req(req).await?;
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
pub struct HighlightLingoEntityReq {
    /// 需要识别词条的内容（不超过1000字）
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "词典是飞书提供的一款知识管理工具"
    ///
    /// **数据校验规则**：
    ///
    /// - **长度范围**: `1` 字符- `1000` 字符
    #[api(kind = "body", name = "text")]
    pub text: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, lark_bot_sdk_macros::ApiBaseResp)]
struct HighlightLingoEntityRespInner {
    #[serde(flatten)]
    data: Option<HighlightLingoEntityResp>,
    #[serde(flatten)]
    base: BaseResp,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct HighlightLingoEntityResp {
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
    /// 识别到的词条信息
    #[serde(
        rename = "phrases",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub phrases: Vec<PhraseSubResp>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(default)]
pub struct PhraseSubResp {
    /// 识别到的关键词
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "词典"
    #[serde(
        rename = "name",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub name: String,
    /// 对应的词条 ID
    ///
    /// **是否必填**: 是
    #[serde(
        rename = "entity_ids",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub entity_ids: Vec<String>,
    /// 词条所在位置
    ///
    /// **是否必填**: 是
    #[serde(
        rename = "span",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub span: SpanSubResp,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(default)]
pub struct SpanSubResp {
    /// 关键词开始位置，从 0 开始计数（编码格式采用 utf-8）
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "0"
    #[serde(
        rename = "start",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub start: i64,
    /// 关键词结束位置，从 0 开始计数（编码格式采用 utf-8）
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "2"
    #[serde(
        rename = "end",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub end: i64,
}

#[cfg(feature = "test-util")]
mod test_utils {
    use std::sync::Arc;

    use self::gen::lingo::LingoServiceMocker;
    use crate::core::mocker::*;

    use super::*;
    use crate::api::*;
    use crate::core::model::*;

    pub struct Mocker;

    pub trait MockFunc:
        Fn(HighlightLingoEntityReq) -> Result<(HighlightLingoEntityResp, CommonResponse), Error>
        + Send
        + Sync
        + 'static
    {
    }
    impl<
            T: Fn(
                    HighlightLingoEntityReq,
                ) -> Result<(HighlightLingoEntityResp, CommonResponse), Error>
                + Send
                + Sync
                + 'static,
        > MockFunc for T
    {
    }

    impl<'c, IStore: Store, IClient: HttpClient> LingoServiceMocker<'c, IStore, IClient> {
        pub fn mock_highlight_lingo_entity<F: MockFunc>(
            &self,
            f: F,
        ) -> MockerBuilder<
            Mocker,
            HighlightLingoEntityReq,
            HighlightLingoEntityResp,
            Arc<dyn MockFunc>,
        > {
            MockerBuilder::new(self.cli.instance_id, Arc::new(f))
        }

        pub(super) fn get_mock_highlight_lingo_entity(
            &self,
            req: &HighlightLingoEntityReq,
        ) -> Option<Arc<dyn MockFunc>> {
            do_mock::<Mocker, HighlightLingoEntityReq, HighlightLingoEntityResp, Arc<dyn MockFunc>>(
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
        api::gen::lingo::highlight_lingo_entity::{
            HighlightLingoEntityReq, HighlightLingoEntityResp, HighlightLingoEntityRespInner,
        },
        core::{model::CommonResponse, Lark},
    };

    #[tokio::test]
    async fn test_mock() {
        let lark = Lark::new("".to_owned(), "".into());
        let mocker = lark
            .lingo()
            .mock()
            .mock_highlight_lingo_entity(|_| {
                Ok((
                    HighlightLingoEntityResp::default(),
                    CommonResponse::default(),
                ))
            })
            .build();
        let res = lark
            .lingo()
            .highlight_lingo_entity(HighlightLingoEntityReq::default())
            .await;
        assert!(res.is_ok());
        mocker.clear();
        let res = lark
            .lingo()
            .highlight_lingo_entity(HighlightLingoEntityReq::default())
            .await;
        assert!(res.is_err());
    }

    const REQ: &str = r#"{
    "text": "词典是飞书提供的一款知识管理工具"
}"#;

    #[test]
    fn test_req() {
        if REQ == "{}" {
            return;
        }
        if let Err(e) = serde_json::from_str::<super::HighlightLingoEntityReqBody>(REQ) {
            panic!("{}", e);
        }
    }

    const RESP: &str = r#"{
    "code": 0,
    "msg": "success",
    "data": {
        "phrases": [
            {
                "name": "词典",
                "entity_ids": [
                    "enterprise_348***84"
                ],
                "span": {
                    "start": 0,
                    "end": 2
                }
            }
        ]
    }
}"#;
    #[test]
    fn test_resp() {
        let res = serde_json::from_str::<HighlightLingoEntityRespInner>(RESP);
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
