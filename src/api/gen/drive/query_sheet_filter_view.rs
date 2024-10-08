//! doc url: <https://open.larkoffice.com/document/ukTMukTMukTM/uUDN04SN0QjL1QDN/sheets-v3/spreadsheet-sheet-filter_view/query>
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
    /// **api 版本: 2024-07-31T09:17:18+00:00**
    ///
    /// ## 查询筛选视图
    ///
    /// 查询电子表格指定工作表的所有筛选视图及其基本信息，包括视图 ID、视图名称和筛选范围。
    ///

    ///
    /// doc: <https://open.larkoffice.com/document/ukTMukTMukTM/uUDN04SN0QjL1QDN/sheets-v3/spreadsheet-sheet-filter_view/query>
    ///
    /// new doc: <https://open.larkoffice.com/document/server-docs/docs/sheets-v3/spreadsheet-sheet-filter_view/query>
    ///
    /// api url: <https://open.feishu.cn/document_portal/v1/document/get_detail?fullPath=%2Fserver-docs%2Fdocs%2Fsheets-v3%2Fspreadsheet-sheet-filter_view%2Fquery>
    pub async fn query_sheet_filter_view(
        &self,
        req: QuerySheetFilterViewReq,
    ) -> Result<(QuerySheetFilterViewResp, CommonResponse), Error> {
        self.query_sheet_filter_view_with_opt(req, Default::default())
            .await
    }

    /// 参见 [query_sheet_filter_view](#method.query_sheet_filter_view) 函数
    pub async fn query_sheet_filter_view_with_opt(
        &self,
        req: QuerySheetFilterViewReq,
        method_option: MethodOption,
    ) -> Result<(QuerySheetFilterViewResp, CommonResponse), Error> {
        #[cfg(feature = "test-util")]
        {
            if let Some(f) = self.mock().get_mock_query_sheet_filter_view(&req) {
                tracing::info!("[lark] Drive#QuerySheetFilterView **mocking** api");
                return f(req);
            }
        }

        tracing::info!("[lark] Drive#QuerySheetFilterView call api");

        let req = ApiRequest {
            scope: "Drive",
            api: "QuerySheetFilterView",
            method: http::Method::GET,
            url: String::new() + self.cli.open_base_url.as_ref() + "/open-apis/sheets/v3/spreadsheets/:spreadsheet_token/sheets/:sheet_id/filter_views/query",
            param_data: req.gen_param(),
            method_option,
            need_tenant_access_token: true,
need_user_access_token: true,
            ..Default::default()
        };

        let (resp, common_resp): (QuerySheetFilterViewRespInner, _) = self.cli.do_req(req).await?;
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
pub struct QuerySheetFilterViewReq {
    /// 电子表格的 token。可通过以下两种方式获取。了解更多，参考[电子表格概述](https://open.feishu.cn/document/ukTMukTMukTM/uATMzUjLwEzM14CMxMTN/overview)。
    ///
    /// - 电子表格的 URL：https://sample.feishu.cn/sheets/==Iow7sNNEphp3WbtnbCscPqabcef==
    ///
    /// - 调用[获取文件夹中的文件清单](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/drive-v1/file/list)
    ///
    /// **示例值**: "Iow7sNNEphp3WbtnbCscPqabcef"
    #[api(kind = "path", name = "spreadsheet_token")]
    pub spreadsheet_token: String,
    /// 工作表 ID，通过[获取工作表](https://open.feishu.cn/document/ukTMukTMukTM/uUDN04SN0QjL1QDN/sheets-v3/spreadsheet-sheet/query) 获取。
    ///
    /// **示例值**: "8fe9d6"
    #[api(kind = "path", name = "sheet_id")]
    pub sheet_id: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, lark_bot_sdk_macros::ApiBaseResp)]
struct QuerySheetFilterViewRespInner {
    #[serde(flatten)]
    data: Option<QuerySheetFilterViewResp>,
    #[serde(flatten)]
    base: BaseResp,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct QuerySheetFilterViewResp {
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
    /// 筛选视图及其基本信息
    #[serde(
        rename = "items",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub items: Vec<FilterViewSubResp>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(default)]
pub struct FilterViewSubResp {
    /// 筛选视图 ID
    ///
    /// **示例值**: "pH9hbVcCXA"
    #[serde(
        rename = "filter_view_id",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub filter_view_id: String,
    /// 筛选视图名称
    ///
    /// **示例值**: "筛选视图 1"
    #[serde(
        rename = "filter_view_name",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub filter_view_name: String,
    /// 筛选视图的筛选范围
    ///
    /// **示例值**: "8fe9d6!C1:H14"
    #[serde(
        rename = "range",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub range: String,
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
        Fn(QuerySheetFilterViewReq) -> Result<(QuerySheetFilterViewResp, CommonResponse), Error>
        + Send
        + Sync
        + 'static
    {
    }
    impl<
            T: Fn(
                    QuerySheetFilterViewReq,
                ) -> Result<(QuerySheetFilterViewResp, CommonResponse), Error>
                + Send
                + Sync
                + 'static,
        > MockFunc for T
    {
    }

    impl<'c, IStore: Store, IClient: HttpClient> DriveServiceMocker<'c, IStore, IClient> {
        pub fn mock_query_sheet_filter_view<F: MockFunc>(
            &self,
            f: F,
        ) -> MockerBuilder<
            Mocker,
            QuerySheetFilterViewReq,
            QuerySheetFilterViewResp,
            Arc<dyn MockFunc>,
        > {
            MockerBuilder::new(self.cli.instance_id, Arc::new(f))
        }

        pub(super) fn get_mock_query_sheet_filter_view(
            &self,
            req: &QuerySheetFilterViewReq,
        ) -> Option<Arc<dyn MockFunc>> {
            do_mock::<Mocker, QuerySheetFilterViewReq, QuerySheetFilterViewResp, Arc<dyn MockFunc>>(
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
        api::gen::drive::query_sheet_filter_view::{
            QuerySheetFilterViewReq, QuerySheetFilterViewResp, QuerySheetFilterViewRespInner,
        },
        core::{model::CommonResponse, Lark},
    };

    #[tokio::test]
    async fn test_mock() {
        let lark = Lark::new("".to_owned(), "".into());
        let mocker = lark
            .drive()
            .mock()
            .mock_query_sheet_filter_view(|_| {
                Ok((
                    QuerySheetFilterViewResp::default(),
                    CommonResponse::default(),
                ))
            })
            .build();
        let res = lark
            .drive()
            .query_sheet_filter_view(QuerySheetFilterViewReq::default())
            .await;
        assert!(res.is_ok());
        mocker.clear();
        let res = lark
            .drive()
            .query_sheet_filter_view(QuerySheetFilterViewReq::default())
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
                "filter_view_id": "pH9hbVcCXA",
                "filter_view_name": "筛选视图 1",
                "range": "8fe9d6!C1:H14"
            }
        ]
    }
}"#;
    #[test]
    fn test_resp() {
        let res = serde_json::from_str::<QuerySheetFilterViewRespInner>(RESP);
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
