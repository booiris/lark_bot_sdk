//! doc url: <https://open.larkoffice.com/document/ukTMukTMukTM/uUDN04SN0QjL1QDN/sheets-v3/spreadsheet-sheet-filter_view-condition/delete>
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
    /// **api 版本: 2024-07-31T09:17:19+00:00**
    ///
    /// ## 删除筛选条件
    ///
    /// 删除筛选视图指定列的所有筛选条件。
    ///

    ///
    /// doc: <https://open.larkoffice.com/document/ukTMukTMukTM/uUDN04SN0QjL1QDN/sheets-v3/spreadsheet-sheet-filter_view-condition/delete>
    ///
    /// new doc: <https://open.larkoffice.com/document/server-docs/docs/sheets-v3/spreadsheet-sheet-filter_view/spreadsheet-sheet-filter_view-condition/delete>
    ///
    /// api url: <https://open.feishu.cn/document_portal/v1/document/get_detail?fullPath=%2Fserver-docs%2Fdocs%2Fsheets-v3%2Fspreadsheet-sheet-filter_view%2Fspreadsheet-sheet-filter_view-condition%2Fdelete>
    pub async fn delete_sheet_filter_view_condition(
        &self,
        req: DeleteSheetFilterViewConditionReq,
    ) -> Result<(DeleteSheetFilterViewConditionResp, CommonResponse), Error> {
        self.delete_sheet_filter_view_condition_with_opt(req, Default::default())
            .await
    }

    /// 参见 [delete_sheet_filter_view_condition](#method.delete_sheet_filter_view_condition) 函数
    pub async fn delete_sheet_filter_view_condition_with_opt(
        &self,
        req: DeleteSheetFilterViewConditionReq,
        method_option: MethodOption,
    ) -> Result<(DeleteSheetFilterViewConditionResp, CommonResponse), Error> {
        #[cfg(feature = "test-util")]
        {
            if let Some(f) = self
                .mock()
                .get_mock_delete_sheet_filter_view_condition(&req)
            {
                tracing::info!("[lark] Drive#DeleteSheetFilterViewCondition **mocking** api");
                return f(req);
            }
        }

        tracing::info!("[lark] Drive#DeleteSheetFilterViewCondition call api");

        let req = ApiRequest {
            scope: "Drive",
            api: "DeleteSheetFilterViewCondition",
            method: http::Method::DELETE,
            url: String::new() + self.cli.open_base_url.as_ref() + "/open-apis/sheets/v3/spreadsheets/:spreadsheet_token/sheets/:sheet_id/filter_views/:filter_view_id/conditions/:condition_id",
            param_data: req.gen_param(),
            method_option,
            need_tenant_access_token: true,
need_user_access_token: true,
            ..Default::default()
        };

        let (resp, common_resp): (DeleteSheetFilterViewConditionRespInner, _) =
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

#[derive(Debug, Serialize, Deserialize, Clone, Default, lark_bot_sdk_macros::ApiReqParams)]
pub struct DeleteSheetFilterViewConditionReq {
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
    /// 筛选视图 ID。通过[查询筛选视图](https://open.feishu.cn/document/ukTMukTMukTM/uUDN04SN0QjL1QDN/sheets-v3/spreadsheet-sheet-filter_view/query)获取。
    ///
    /// **示例值**: "pH9hbVcCXA"
    #[api(kind = "path", name = "filter_view_id")]
    pub filter_view_id: String,
    /// 要删除所有筛选条件的列，用字母表示。
    ///
    /// **示例值**: "E"
    #[api(kind = "path", name = "condition_id")]
    pub condition_id: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, lark_bot_sdk_macros::ApiBaseResp)]
struct DeleteSheetFilterViewConditionRespInner {
    #[serde(flatten)]
    data: Option<DeleteSheetFilterViewConditionResp>,
    #[serde(flatten)]
    base: BaseResp,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct DeleteSheetFilterViewConditionResp {
    /// \-
    #[serde(
        rename = "data",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub data: (),
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
        Fn(
            DeleteSheetFilterViewConditionReq,
        ) -> Result<(DeleteSheetFilterViewConditionResp, CommonResponse), Error>
        + Send
        + Sync
        + 'static
    {
    }
    impl<
            T: Fn(
                    DeleteSheetFilterViewConditionReq,
                )
                    -> Result<(DeleteSheetFilterViewConditionResp, CommonResponse), Error>
                + Send
                + Sync
                + 'static,
        > MockFunc for T
    {
    }

    impl<'c, IStore: Store, IClient: HttpClient> DriveServiceMocker<'c, IStore, IClient> {
        pub fn mock_delete_sheet_filter_view_condition<F: MockFunc>(
            &self,
            f: F,
        ) -> MockerBuilder<
            Mocker,
            DeleteSheetFilterViewConditionReq,
            DeleteSheetFilterViewConditionResp,
            Arc<dyn MockFunc>,
        > {
            MockerBuilder::new(self.cli.instance_id, Arc::new(f))
        }

        pub(super) fn get_mock_delete_sheet_filter_view_condition(
            &self,
            req: &DeleteSheetFilterViewConditionReq,
        ) -> Option<Arc<dyn MockFunc>> {
            do_mock::<
                Mocker,
                DeleteSheetFilterViewConditionReq,
                DeleteSheetFilterViewConditionResp,
                Arc<dyn MockFunc>,
            >(self.cli.instance_id, req)
        }
    }
}

#[cfg(feature = "test-util")]
#[cfg(test)]
mod test {
    use crate::{
        api::gen::drive::delete_sheet_filter_view_condition::{
            DeleteSheetFilterViewConditionReq, DeleteSheetFilterViewConditionResp,
            DeleteSheetFilterViewConditionRespInner,
        },
        core::{model::CommonResponse, Lark},
    };

    #[tokio::test]
    async fn test_mock() {
        let lark = Lark::new("".to_owned(), "".into());
        let mocker = lark
            .drive()
            .mock()
            .mock_delete_sheet_filter_view_condition(|_| {
                Ok((
                    DeleteSheetFilterViewConditionResp::default(),
                    CommonResponse::default(),
                ))
            })
            .build();
        let res = lark
            .drive()
            .delete_sheet_filter_view_condition(DeleteSheetFilterViewConditionReq::default())
            .await;
        assert!(res.is_ok());
        mocker.clear();
        let res = lark
            .drive()
            .delete_sheet_filter_view_condition(DeleteSheetFilterViewConditionReq::default())
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
    "data": {}
}"#;
    #[test]
    fn test_resp() {
        let res = serde_json::from_str::<DeleteSheetFilterViewConditionRespInner>(RESP);
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
