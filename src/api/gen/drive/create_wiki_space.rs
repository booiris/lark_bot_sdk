//! doc url: <https://open.feishu.cn/document/ukTMukTMukTM/uUDN04SN0QjL1QDN/wiki-v2/space/create>
// Code generated by gen_api. DO NOT EDIT.

use serde::{Deserialize, Serialize};

use crate::api::BaseResp;
use crate::api::HasBaseResp;
use crate::core::model::{ApiRequest, CommonResponse, MethodOption, ReqParam, StreamReqParam};
use crate::{
    core::{http_client::HttpClient, store::Store},
    error::Error,
};

use crate::api::gen::drive::DriveService;

impl<'c, IStore: Store, IClient: HttpClient> DriveService<'c, IStore, IClient> {
    /// **api 版本: 2024-07-31T11:17:43+00:00**
    ///
    /// ## 创建知识空间
    ///
    /// 此接口用于创建知识空间
    ///
    /// 此接口不支持tenant access token（应用身份访问）
    ///
    /// doc: <https://open.feishu.cn/document/ukTMukTMukTM/uUDN04SN0QjL1QDN/wiki-v2/space/create>
    ///
    /// new doc: <https://open.feishu.cn/document/server-docs/docs/wiki-v2/space/create>
    ///
    /// api url: <https://open.feishu.cn/document_portal/v1/document/get_detail?fullPath=%2Fserver-docs%2Fdocs%2Fwiki-v2%2Fspace%2Fcreate>
    pub async fn create_wiki_space(
        &self,
        req: CreateWikiSpaceReq,
    ) -> Result<(CreateWikiSpaceResp, CommonResponse), Error> {
        self.create_wiki_space_with_opt(req, Default::default())
            .await
    }

    /// 参见 [create_wiki_space](#method.create_wiki_space) 函数
    pub async fn create_wiki_space_with_opt(
        &self,
        req: CreateWikiSpaceReq,
        method_option: MethodOption,
    ) -> Result<(CreateWikiSpaceResp, CommonResponse), Error> {
        #[cfg(feature = "test-util")]
        {
            if let Some(f) = self.mock().get_mock_create_wiki_space(&req) {
                tracing::info!("[lark] Drive#CreateWikiSpace **mocking** api");
                return f(req);
            }
        }

        tracing::info!("[lark] Drive#CreateWikiSpace call api");

        let req = ApiRequest {
            scope: "Drive",
            api: "CreateWikiSpace",
            method: http::Method::POST,
            url: String::new() + self.cli.open_base_url.as_ref() + "/open-apis/wiki/v2/spaces",
            param_data: req.gen_param(),
            method_option,
            need_user_access_token: true,
            ..Default::default()
        };

        let (resp, common_resp): (CreateWikiSpaceRespInner, _) = self.cli.do_req(req).await?;
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
pub struct CreateWikiSpaceReq {
    /// 知识空间名称
    ///
    /// **示例值**: "知识空间"
    #[api(kind = "body", name = "name")]
    pub name: Option<String>,
    /// 知识空间描述
    ///
    /// **示例值**: "知识空间描述"
    #[api(kind = "body", name = "description")]
    pub description: Option<String>,
    /// 表示知识空间的分享状态
    ///
    /// **示例值**: "open"
    ///
    /// **可选值**:
    ///
    /// `Open`: 打开
    ///
    /// `Closed`: 关闭
    ///
    /// **数据校验规则**：
    ///
    /// - **长度范围**: `0` 字符- `99` 字符
    #[api(kind = "body", name = "open_sharing")]
    pub open_sharing: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, lark_bot_sdk_macros::ApiBaseResp)]
struct CreateWikiSpaceRespInner {
    #[serde(flatten)]
    data: Option<CreateWikiSpaceResp>,
    #[serde(flatten)]
    base: BaseResp,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct CreateWikiSpaceResp {
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
    /// 知识空间
    #[serde(
        rename = "space",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub space: SpaceSubResp,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(default)]
pub struct SpaceSubResp {
    /// 知识空间名称
    ///
    /// **示例值**: "知识空间"
    #[serde(
        rename = "name",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub name: String,
    /// 知识空间描述
    ///
    /// **示例值**: "知识空间描述"
    #[serde(
        rename = "description",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub description: String,
    /// 知识空间id
    ///
    /// **示例值**: "123456"
    #[serde(
        rename = "space_id",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub space_id: String,
    /// 表示知识空间类型（团队空间 或 个人空间）
    ///
    /// **示例值**: "team"
    ///
    /// **可选值**:
    ///
    /// `Team`: 团队空间
    ///
    /// `Person`: 个人空间
    #[serde(
        rename = "space_type",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub space_type: String,
    /// 表示知识空间可见性（公开空间 或 私有空间）
    ///
    /// **示例值**: "private"
    ///
    /// **可选值**:
    ///
    /// `Public`: 公开空间
    ///
    /// `Private`: 私有空间
    #[serde(
        rename = "visibility",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub visibility: String,
    /// 表示知识空间的分享状态
    ///
    /// **示例值**: "open"
    ///
    /// **可选值**:
    ///
    /// `Open`: 打开
    ///
    /// `Closed`: 关闭
    ///
    /// **数据校验规则**：
    ///
    /// - **长度范围**: `0` 字符- `99` 字符
    #[serde(
        rename = "open_sharing",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub open_sharing: String,
}

#[cfg(feature = "test-util")]
mod test_utils {
    use std::sync::Arc;

    use self::gen::drive::DriveServiceMocker;
    use crate::core::mocker::*;

    use super::*;
    use crate::api::*;
    use crate::core::model::*;

    pub struct Mocker;

    pub trait MockFunc:
        Fn(CreateWikiSpaceReq) -> Result<(CreateWikiSpaceResp, CommonResponse), Error>
        + Send
        + Sync
        + 'static
    {
    }
    impl<
            T: Fn(CreateWikiSpaceReq) -> Result<(CreateWikiSpaceResp, CommonResponse), Error>
                + Send
                + Sync
                + 'static,
        > MockFunc for T
    {
    }

    impl<'c, IStore: Store, IClient: HttpClient> DriveServiceMocker<'c, IStore, IClient> {
        pub fn mock_create_wiki_space<F: MockFunc>(
            &self,
            f: F,
        ) -> MockerBuilder<Mocker, CreateWikiSpaceReq, CreateWikiSpaceResp, Arc<dyn MockFunc>>
        {
            MockerBuilder::new(self.cli.instance_id, Arc::new(f))
        }

        pub(super) fn get_mock_create_wiki_space(
            &self,
            req: &CreateWikiSpaceReq,
        ) -> Option<Arc<dyn MockFunc>> {
            do_mock::<Mocker, CreateWikiSpaceReq, CreateWikiSpaceResp, Arc<dyn MockFunc>>(
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
        api::gen::drive::create_wiki_space::{
            CreateWikiSpaceReq, CreateWikiSpaceResp, CreateWikiSpaceRespInner,
        },
        core::{model::CommonResponse, Lark},
    };

    #[tokio::test]
    async fn test_mock() {
        let lark = Lark::new("".to_owned(), "".into());
        let mocker = lark
            .drive()
            .mock()
            .mock_create_wiki_space(|_| {
                Ok((CreateWikiSpaceResp::default(), CommonResponse::default()))
            })
            .build();
        let res = lark
            .drive()
            .create_wiki_space(CreateWikiSpaceReq::default())
            .await;
        assert!(res.is_ok());
        mocker.clear();
        let res = lark
            .drive()
            .create_wiki_space(CreateWikiSpaceReq::default())
            .await;
        assert!(res.is_err());
    }

    const REQ: &str = r#"{
    "name": "知识空间",
    "description": "知识空间描述"
}"#;

    #[test]
    fn test_req() {
        if REQ == "{}" {
            return;
        }
        if let Err(e) = serde_json::from_str::<super::CreateWikiSpaceReqBody>(REQ) {
            panic!("{}", e);
        }
    }

    const RESP: &str = r#"{
    "code": 0,
    "msg": "success",
    "data": {
        "space": {
            "name": "知识空间",
            "description": "知识空间描述",
            "space_id": "1565676577122621"
        }
    }
}"#;
    #[test]
    fn test_resp() {
        let res = serde_json::from_str::<CreateWikiSpaceRespInner>(RESP);
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
