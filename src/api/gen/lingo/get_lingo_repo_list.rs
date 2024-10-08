//! doc url: <https://open.larkoffice.com/document/uAjLw4CM/ukTMukTMukTM/lingo-v1/repo/list>
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
    /// **api 版本: 2023-10-25T08:27:27+00:00**
    ///
    /// ## 获取词库列表
    ///
    /// 获取有权限访问的飞书词典词库列表。
    ///
    /// 如以应用身份获取，需要在“词库设置”页面添加应用；若以用户身份获取，该用户需要拥有对应词库的可见权限。
    ///

    ///
    /// doc: <https://open.larkoffice.com/document/uAjLw4CM/ukTMukTMukTM/lingo-v1/repo/list>
    ///
    /// new doc: <https://open.larkoffice.com/document/uAjLw4CM/ukTMukTMukTM/lingo-v1/repo/list>
    ///
    /// api url: <https://open.feishu.cn/document_portal/v1/document/get_detail?fullPath=%2FuAjLw4CM%2FukTMukTMukTM%2Flingo-v1%2Frepo%2Flist>
    pub async fn get_lingo_repo_list(
        &self,
        req: GetLingoRepoListReq,
    ) -> Result<(GetLingoRepoListResp, CommonResponse), Error> {
        self.get_lingo_repo_list_with_opt(req, Default::default())
            .await
    }

    /// 参见 [get_lingo_repo_list](#method.get_lingo_repo_list) 函数
    pub async fn get_lingo_repo_list_with_opt(
        &self,
        req: GetLingoRepoListReq,
        method_option: MethodOption,
    ) -> Result<(GetLingoRepoListResp, CommonResponse), Error> {
        #[cfg(feature = "test-util")]
        {
            if let Some(f) = self.mock().get_mock_get_lingo_repo_list(&req) {
                tracing::info!("[lark] Lingo#GetLingoRepoList **mocking** api");
                return f(req);
            }
        }

        tracing::info!("[lark] Lingo#GetLingoRepoList call api");

        let req = ApiRequest {
            scope: "Lingo",
            api: "GetLingoRepoList",
            method: http::Method::GET,
            url: String::new() + self.cli.open_base_url.as_ref() + "/open-apis/lingo/v1/repos",
            param_data: req.gen_param(),
            method_option,
            need_tenant_access_token: true,
            need_user_access_token: true,
            ..Default::default()
        };

        let (resp, common_resp): (GetLingoRepoListRespInner, _) = self.cli.do_req(req).await?;
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
pub struct GetLingoRepoListReq {}

#[derive(Debug, Serialize, Deserialize, Clone, lark_bot_sdk_macros::ApiBaseResp)]
struct GetLingoRepoListRespInner {
    #[serde(flatten)]
    data: Option<GetLingoRepoListResp>,
    #[serde(flatten)]
    base: BaseResp,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct GetLingoRepoListResp {
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
    /// 词库列表
    #[serde(
        rename = "items",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub items: Vec<RepoSubResp>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(default)]
pub struct RepoSubResp {
    /// 词库 id
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "71527909***274113"
    #[serde(
        rename = "id",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub id: String,
    /// 词库名
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "全员词库"
    #[serde(
        rename = "name",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub name: String,
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
        Fn(GetLingoRepoListReq) -> Result<(GetLingoRepoListResp, CommonResponse), Error>
        + Send
        + Sync
        + 'static
    {
    }
    impl<
            T: Fn(GetLingoRepoListReq) -> Result<(GetLingoRepoListResp, CommonResponse), Error>
                + Send
                + Sync
                + 'static,
        > MockFunc for T
    {
    }

    impl<'c, IStore: Store, IClient: HttpClient> LingoServiceMocker<'c, IStore, IClient> {
        pub fn mock_get_lingo_repo_list<F: MockFunc>(
            &self,
            f: F,
        ) -> MockerBuilder<Mocker, GetLingoRepoListReq, GetLingoRepoListResp, Arc<dyn MockFunc>>
        {
            MockerBuilder::new(self.cli.instance_id, Arc::new(f))
        }

        pub(super) fn get_mock_get_lingo_repo_list(
            &self,
            req: &GetLingoRepoListReq,
        ) -> Option<Arc<dyn MockFunc>> {
            do_mock::<Mocker, GetLingoRepoListReq, GetLingoRepoListResp, Arc<dyn MockFunc>>(
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
        api::gen::lingo::get_lingo_repo_list::{
            GetLingoRepoListReq, GetLingoRepoListResp, GetLingoRepoListRespInner,
        },
        core::{model::CommonResponse, Lark},
    };

    #[tokio::test]
    async fn test_mock() {
        let lark = Lark::new("".to_owned(), "".into());
        let mocker = lark
            .lingo()
            .mock()
            .mock_get_lingo_repo_list(|_| {
                Ok((GetLingoRepoListResp::default(), CommonResponse::default()))
            })
            .build();
        let res = lark
            .lingo()
            .get_lingo_repo_list(GetLingoRepoListReq::default())
            .await;
        assert!(res.is_ok());
        mocker.clear();
        let res = lark
            .lingo()
            .get_lingo_repo_list(GetLingoRepoListReq::default())
            .await;
        assert!(res.is_err());
    }

    const REQ: &str = "{}";

    #[test]
    fn test_req() {
        if REQ == "{}" {
            return;
        }
        if let Err(e) = serde_json::from_str::<()>(REQ) {
            panic!("{}", e);
        }
    }

    const RESP: &str = r#"{
    "code": 0,
    "msg": "success",
    "data": {
        "items": [
            {
                "id": "71527909***274113",
                "name": "全员词库"
            }
        ]
    }
}"#;
    #[test]
    fn test_resp() {
        let res = serde_json::from_str::<GetLingoRepoListRespInner>(RESP);
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
