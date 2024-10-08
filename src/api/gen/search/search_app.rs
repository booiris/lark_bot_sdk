//! doc url: <https://open.larkoffice.com/document/uAjLw4CM/ukTMukTMukTM/search-v2/app/create>
// Code generated by gen_api. DO NOT EDIT.

use serde::{Deserialize, Serialize};

use crate::api::BaseResp;
use crate::api::HasBaseResp;
use crate::core::model::{ApiRequest, CommonResponse, MethodOption, ReqParam, StreamReqParam};
use crate::{
    core::{http_client::HttpClient, store::Store},
    error::Error,
};

use crate::api::gen::search::SearchService;

impl<'c, IStore: Store, IClient: HttpClient> SearchService<'c, IStore, IClient> {
    /// **api 版本: 2023-11-27T02:54:35+00:00**
    ///
    /// ## 搜索应用
    ///
    /// 用户可以通过关键字搜索到可见应用，应用可见性与套件内搜索一致。
    ///

    ///
    /// doc: <https://open.larkoffice.com/document/uAjLw4CM/ukTMukTMukTM/search-v2/app/create>
    ///
    /// new doc: <https://open.larkoffice.com/document/server-docs/search-v2/suite-search/create-2>
    ///
    /// api url: <https://open.feishu.cn/document_portal/v1/document/get_detail?fullPath=%2Fserver-docs%2Fsearch-v2%2Fsuite-search%2Fcreate-2>
    pub async fn search_app(
        &self,
        req: SearchAppReq,
    ) -> Result<(SearchAppResp, CommonResponse), Error> {
        self.search_app_with_opt(req, Default::default()).await
    }

    /// 参见 [search_app](#method.search_app) 函数
    pub async fn search_app_with_opt(
        &self,
        req: SearchAppReq,
        method_option: MethodOption,
    ) -> Result<(SearchAppResp, CommonResponse), Error> {
        #[cfg(feature = "test-util")]
        {
            if let Some(f) = self.mock().get_mock_search_app(&req) {
                tracing::info!("[lark] Search#SearchApp **mocking** api");
                return f(req);
            }
        }

        tracing::info!("[lark] Search#SearchApp call api");

        let req = ApiRequest {
            scope: "Search",
            api: "SearchApp",
            method: http::Method::POST,
            url: String::new() + self.cli.open_base_url.as_ref() + "/open-apis/search/v2/app",
            param_data: req.gen_param(),
            method_option,
            need_user_access_token: true,
            ..Default::default()
        };

        let (resp, common_resp): (SearchAppRespInner, _) = self.cli.do_req(req).await?;
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
pub struct SearchAppReq {
    /// 用户 ID 类型
    ///
    /// **示例值**: "open_id"
    ///
    /// **可选值**:
    ///
    /// `open_id`: 标识一个用户在某个应用中的身份。同一个用户在不同应用中的 Open ID 不同。[了解更多：如何获取 Open ID](https://open.feishu.cn/document/uAjLw4CM/ugTN1YjL4UTN24CO1UjN/trouble-shooting/how-to-obtain-openid)
    ///
    /// `union_id`: 标识一个用户在某个应用开发商下的身份。同一用户在同一开发商下的应用中的 Union ID 是相同的，在不同开发商下的应用中的 Union ID 是不同的。通过 Union ID，应用开发商可以把同个用户在多个应用中的身份关联起来。[了解更多：如何获取 Union ID？](https://open.feishu.cn/document/uAjLw4CM/ugTN1YjL4UTN24CO1UjN/trouble-shooting/how-to-obtain-union-id)
    ///
    /// `user_id`: 标识一个用户在某个租户内的身份。同一个用户在租户 A 和租户 B 内的 User ID 是不同的。在同一个租户内，一个用户的 User ID 在所有应用（包括商店应用）中都保持一致。User ID 主要用于在不同的应用间打通用户数据。[了解更多：如何获取 User ID？](https://open.feishu.cn/document/uAjLw4CM/ugTN1YjL4UTN24CO1UjN/trouble-shooting/how-to-obtain-user-id)
    #[api(
        kind = "query",
        name = "user_id_type",
        v_type = "var",
        option = "false"
    )]
    pub user_id_type: String,
    /// 分页大小
    ///
    /// **示例值**: "20"
    #[api(kind = "query", name = "page_size", v_type = "var", option = "false")]
    pub page_size: i64,
    /// 分页标记，第一次请求不填，表示从头开始遍历；分页查询结果还有更多项时会同时返回新的 page_token，下次遍历可采用该 page_token 获取查询结果
    ///
    /// **示例值**: "9e91187f9107ef4d43cd71c3722cd97665e6cec51bf30a06328839bc9867"
    #[api(kind = "query", name = "page_token", v_type = "var", option = "false")]
    pub page_token: String,
    /// 搜索关键词
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "测试应用"
    #[api(kind = "body", name = "query")]
    pub query: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, lark_bot_sdk_macros::ApiBaseResp)]
struct SearchAppRespInner {
    #[serde(flatten)]
    data: Option<SearchAppResp>,
    #[serde(flatten)]
    base: BaseResp,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct SearchAppResp {
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
    /// app_id列表
    #[serde(
        rename = "items",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub items: Vec<String>,
    /// 分页标记，当 has_more 为 true 时，会同时返回新的 page_token，否则不返回 page_token
    ///
    /// **示例值**: "GxmvlNRvP0NdQZpa7yIqf_Lv_QuBwTQ8tXkX7w-irAghVD_TvuYd1aoJ1LQph86O-XImC4X9j9FhUPhXQDvtrQ=="
    #[serde(
        rename = "page_token",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub page_token: String,
    /// 是否还有更多项
    ///
    /// **示例值**: "true"
    #[serde(
        rename = "has_more",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub has_more: bool,
}

#[cfg(feature = "test-util")]
mod test_utils {
    use std::sync::Arc;

    use self::gen::search::SearchServiceMocker;
    use crate::core::mocker::*;

    use super::*;
    use crate::api::*;
    use crate::core::model::*;

    pub struct Mocker;

    pub trait MockFunc:
        Fn(SearchAppReq) -> Result<(SearchAppResp, CommonResponse), Error> + Send + Sync + 'static
    {
    }
    impl<
            T: Fn(SearchAppReq) -> Result<(SearchAppResp, CommonResponse), Error>
                + Send
                + Sync
                + 'static,
        > MockFunc for T
    {
    }

    impl<'c, IStore: Store, IClient: HttpClient> SearchServiceMocker<'c, IStore, IClient> {
        pub fn mock_search_app<F: MockFunc>(
            &self,
            f: F,
        ) -> MockerBuilder<Mocker, SearchAppReq, SearchAppResp, Arc<dyn MockFunc>> {
            MockerBuilder::new(self.cli.instance_id, Arc::new(f))
        }

        pub(super) fn get_mock_search_app(&self, req: &SearchAppReq) -> Option<Arc<dyn MockFunc>> {
            do_mock::<Mocker, SearchAppReq, SearchAppResp, Arc<dyn MockFunc>>(
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
        api::gen::search::search_app::{SearchAppReq, SearchAppResp, SearchAppRespInner},
        core::{model::CommonResponse, Lark},
    };

    #[tokio::test]
    async fn test_mock() {
        let lark = Lark::new("".to_owned(), "".into());
        let mocker = lark
            .search()
            .mock()
            .mock_search_app(|_| Ok((SearchAppResp::default(), CommonResponse::default())))
            .build();
        let res = lark.search().search_app(SearchAppReq::default()).await;
        assert!(res.is_ok());
        mocker.clear();
        let res = lark.search().search_app(SearchAppReq::default()).await;
        assert!(res.is_err());
    }

    const REQ: &str = r#"{
    "query": "测试应用"
}"#;

    #[test]
    fn test_req() {
        if REQ == "{}" {
            return;
        }
        if let Err(e) = serde_json::from_str::<super::SearchAppReqBody>(REQ) {
            panic!("{}", e);
        }
    }

    const RESP: &str = r#"{
    "code": 0,
    "msg": "success",
    "data": {
        "items": [
            "cli_9b445f5258795107"
        ],
        "page_token": "GxmvlNRvP0NdQZpa7yIqf_Lv_QuBwTQ8tXkX7w-irAghVD_TvuYd1aoJ1LQph86O-XImC4X9j9FhUPhXQDvtrQ==",
        "has_more": true
    }
}"#;
    #[test]
    fn test_resp() {
        let res = serde_json::from_str::<SearchAppRespInner>(RESP);
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
