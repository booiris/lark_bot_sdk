//! doc url: <https://open.larkoffice.com/document/uAjLw4CM/ukTMukTMukTM/reference/baike-v1/entity/extract>
// Code generated by gen_api. DO NOT EDIT.

use serde::{Deserialize, Serialize};

use crate::api::BaseResp;
use crate::api::HasBaseResp;
use crate::core::model::{ApiRequest, CommonResponse, MethodOption, ReqParam, StreamReqParam};
use crate::{
    core::{http_client::HttpClient, store::Store},
    error::Error,
};

use crate::api::gen::baike::BaikeService;

impl<'c, IStore: Store, IClient: HttpClient> BaikeService<'c, IStore, IClient> {
    /// **api 版本: 2023-10-13T02:23:10+00:00**
    ///
    /// ## 提取潜在的词条
    ///
    /// 提取文本中可能成为词条的词语，且不会过滤已经成为词条的词语。同时返回推荐的别名。
    ///
    /// 为了更好地提升接口文档的的易理解性，我们对文档进行了升级，请尽快迁移至[新版本>>](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/lingo-v1/entity/extract)
    ///
    /// doc: <https://open.larkoffice.com/document/uAjLw4CM/ukTMukTMukTM/reference/baike-v1/entity/extract>
    ///
    /// new doc: <https://open.larkoffice.com/document/server-docs/baike-v1/entity/extract>
    ///
    /// api url: <https://open.feishu.cn/document_portal/v1/document/get_detail?fullPath=%2Fserver-docs%2Fbaike-v1%2Fentity%2Fextract>
    pub async fn extract_baike_entity(
        &self,
        req: ExtractBaikeEntityReq,
    ) -> Result<(ExtractBaikeEntityResp, CommonResponse), Error> {
        self.extract_baike_entity_with_opt(req, Default::default())
            .await
    }

    /// 参见 [extract_baike_entity](#method.extract_baike_entity) 函数
    pub async fn extract_baike_entity_with_opt(
        &self,
        req: ExtractBaikeEntityReq,
        method_option: MethodOption,
    ) -> Result<(ExtractBaikeEntityResp, CommonResponse), Error> {
        #[cfg(feature = "test-util")]
        {
            if let Some(f) = self.mock().get_mock_extract_baike_entity(&req) {
                tracing::info!("[lark] Baike#ExtractBaikeEntity **mocking** api");
                return f(req);
            }
        }

        tracing::info!("[lark] Baike#ExtractBaikeEntity call api");

        let req = ApiRequest {
            scope: "Baike",
            api: "ExtractBaikeEntity",
            method: http::Method::POST,
            url: String::new()
                + self.cli.open_base_url.as_ref()
                + "/open-apis/baike/v1/entities/extract",
            param_data: req.gen_param(),
            method_option,
            need_tenant_access_token: true,
            need_user_access_token: true,
            ..Default::default()
        };

        let (resp, common_resp): (ExtractBaikeEntityRespInner, _) = self.cli.do_req(req).await?;
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
pub struct ExtractBaikeEntityReq {
    /// 需要被提取词条的文本（不会过滤租户中已成为词条的内容）
    ///
    /// **示例值**: "飞书词典是一部高效汇聚企业内各类信息，并可由企业成员参与编辑的在线词典"
    ///
    /// **数据校验规则**：
    ///
    /// - **最大长度**: `128` 字符
    #[api(kind = "body", name = "text")]
    pub text: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, lark_bot_sdk_macros::ApiBaseResp)]
struct ExtractBaikeEntityRespInner {
    #[serde(flatten)]
    data: Option<ExtractBaikeEntityResp>,
    #[serde(flatten)]
    base: BaseResp,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct ExtractBaikeEntityResp {
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
    /// 文本中可能的成为词条的词汇
    ///
    /// **是否必填**: 是
    #[serde(
        rename = "entity_word",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub entity_word: Vec<EntityWordSubResp>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(default)]
pub struct EntityWordSubResp {
    /// 抽取出的词条名
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "飞书词典"
    #[serde(
        rename = "name",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub name: String,
    /// 词条可能的别名
    #[serde(
        rename = "aliases",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub aliases: Vec<String>,
}

#[cfg(feature = "test-util")]
mod test_utils {
    use std::sync::Arc;

    use self::gen::baike::BaikeServiceMocker;
    use crate::core::mocker::*;

    use super::*;
    use crate::api::*;
    use crate::core::model::*;

    pub struct Mocker;

    pub trait MockFunc:
        Fn(ExtractBaikeEntityReq) -> Result<(ExtractBaikeEntityResp, CommonResponse), Error>
        + Send
        + Sync
        + 'static
    {
    }
    impl<
            T: Fn(ExtractBaikeEntityReq) -> Result<(ExtractBaikeEntityResp, CommonResponse), Error>
                + Send
                + Sync
                + 'static,
        > MockFunc for T
    {
    }

    impl<'c, IStore: Store, IClient: HttpClient> BaikeServiceMocker<'c, IStore, IClient> {
        pub fn mock_extract_baike_entity<F: MockFunc>(
            &self,
            f: F,
        ) -> MockerBuilder<Mocker, ExtractBaikeEntityReq, ExtractBaikeEntityResp, Arc<dyn MockFunc>>
        {
            MockerBuilder::new(self.cli.instance_id, Arc::new(f))
        }

        pub(super) fn get_mock_extract_baike_entity(
            &self,
            req: &ExtractBaikeEntityReq,
        ) -> Option<Arc<dyn MockFunc>> {
            do_mock::<Mocker, ExtractBaikeEntityReq, ExtractBaikeEntityResp, Arc<dyn MockFunc>>(
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
        api::gen::baike::extract_baike_entity::{
            ExtractBaikeEntityReq, ExtractBaikeEntityResp, ExtractBaikeEntityRespInner,
        },
        core::{model::CommonResponse, Lark},
    };

    #[tokio::test]
    async fn test_mock() {
        let lark = Lark::new("".to_owned(), "".into());
        let mocker = lark
            .baike()
            .mock()
            .mock_extract_baike_entity(|_| {
                Ok((ExtractBaikeEntityResp::default(), CommonResponse::default()))
            })
            .build();
        let res = lark
            .baike()
            .extract_baike_entity(ExtractBaikeEntityReq::default())
            .await;
        assert!(res.is_ok());
        mocker.clear();
        let res = lark
            .baike()
            .extract_baike_entity(ExtractBaikeEntityReq::default())
            .await;
        assert!(res.is_err());
    }

    const REQ: &str = r#"{
    "text": "飞书词典是一部高效汇聚企业内各类信息，并可由企业成员参与编辑的在线词典"
}"#;

    #[test]
    fn test_req() {
        if REQ == "{}" {
            return;
        }
        if let Err(e) = serde_json::from_str::<super::ExtractBaikeEntityReqBody>(REQ) {
            panic!("{}", e);
        }
    }

    const RESP: &str = r#"{
    "code": 0,
    "msg": "success",
    "data": {
        "entity_word": [
            {
                "name": "飞书词典",
                "aliases": [
                    "Lingo"
                ]
            }
        ]
    }
}"#;
    #[test]
    fn test_resp() {
        let res = serde_json::from_str::<ExtractBaikeEntityRespInner>(RESP);
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
