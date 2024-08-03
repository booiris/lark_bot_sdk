//! doc url: <https://open.feishu.cn/document/ukTMukTMukTM/uUDN04SN0QjL1QDN/sheets-v3/spreadsheet-sheet/get>
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
    /// **api 版本: 2024-07-31T09:17:15+00:00**
    ///
    /// ## 查询工作表
    ///
    /// 根据工作表 ID 查询工作表属性信息，包括工作表的标题、索引位置、是否被隐藏等。
    ///

    ///
    /// doc: <https://open.feishu.cn/document/ukTMukTMukTM/uUDN04SN0QjL1QDN/sheets-v3/spreadsheet-sheet/get>
    ///
    /// new doc: <https://open.feishu.cn/document/server-docs/docs/sheets-v3/spreadsheet-sheet/get>
    ///
    /// api url: <https://open.feishu.cn/document_portal/v1/document/get_detail?fullPath=%2Fserver-docs%2Fdocs%2Fsheets-v3%2Fspreadsheet-sheet%2Fget>
    pub async fn get_sheet(
        &self,
        req: GetSheetReq,
    ) -> Result<(GetSheetResp, CommonResponse), Error> {
        self.get_sheet_with_opt(req, Default::default()).await
    }

    /// 参见 [get_sheet](#method.get_sheet) 函数
    pub async fn get_sheet_with_opt(
        &self,
        req: GetSheetReq,
        method_option: MethodOption,
    ) -> Result<(GetSheetResp, CommonResponse), Error> {
        #[cfg(feature = "test-util")]
        {
            if let Some(f) = self.mock().get_mock_get_sheet(&req) {
                tracing::info!("[lark] Drive#GetSheet **mocking** api");
                return f(req);
            }
        }

        tracing::info!("[lark] Drive#GetSheet call api");

        let req = ApiRequest {
            scope: "Drive",
            api: "GetSheet",
            method: http::Method::GET,
            url: String::new()
                + self.cli.open_base_url.as_ref()
                + "/open-apis/sheets/v3/spreadsheets/:spreadsheet_token/sheets/:sheet_id",
            param_data: req.gen_param(),
            method_option,
            need_tenant_access_token: true,
            need_user_access_token: true,
            ..Default::default()
        };

        let (resp, common_resp): (GetSheetRespInner, _) = self.cli.do_req(req).await?;
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
pub struct GetSheetReq {
    /// 电子表格的 token。可通过以下两种方式获取。了解更多，参考[电子表格概述](https://open.feishu.cn/document/ukTMukTMukTM/uATMzUjLwEzM14CMxMTN/overview)。
    ///
    /// -  电子表格的 URL：https://sample.feishu.cn/sheets/==Iow7sNNEphp3WbtnbCscPqabcef==
    ///
    /// - 调用[获取文件夹中的文件清单](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/drive-v1/file/list)
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "Iow7sNNEphp3WbtnbCscPqabcef"
    #[api(kind = "path", name = "spreadsheet_token")]
    pub spreadsheet_token: String,
    /// 工作表的 ID。调用[获取工作表](https://open.feishu.cn/document/ukTMukTMukTM/uUDN04SN0QjL1QDN/sheets-v3/spreadsheet-sheet/query)获取 ID。
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "giDk9k"
    #[api(kind = "path", name = "sheet_id")]
    pub sheet_id: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, lark_bot_sdk_macros::ApiBaseResp)]
struct GetSheetRespInner {
    #[serde(flatten)]
    data: Option<GetSheetResp>,
    #[serde(flatten)]
    base: BaseResp,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct GetSheetResp {
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
    /// 工作表
    #[serde(
        rename = "sheet",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub sheet: SheetSubResp,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(default)]
pub struct SheetSubResp {
    /// 工作表 ID
    ///
    /// **示例值**: "sxj5ws"
    #[serde(
        rename = "sheet_id",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub sheet_id: String,
    /// 工作表标题
    ///
    /// **示例值**: "Sheet1"
    #[serde(
        rename = "title",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub title: String,
    /// 工作表索引位置，索引从 0 开始计数。
    ///
    /// **示例值**: "0"
    #[serde(
        rename = "index",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub index: i64,
    /// 工作表是否被隐藏
    ///
    /// - `true`：被隐藏
    ///
    /// - `false`：未被隐藏
    ///
    /// **示例值**: "false"
    #[serde(
        rename = "hidden",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub hidden: bool,
    /// 单元格属性，仅当 `resource_type` 为 `sheet` 即工作表类型为电子表格时返回。
    #[serde(
        rename = "grid_properties",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub grid_properties: GridPropertiesSubResp,
    /// 工作表类型
    ///
    /// - `sheet`：工作表
    ///
    /// - `bitable`：多维表格。详情参考[多维表格概述](https://open.feishu.cn/document/ukTMukTMukTM/uUDN04SN0QjL1QDN/bitable-overview)
    ///
    /// - `#UNSUPPORTED_TYPE`：不支持的类型
    ///
    /// **示例值**: "sheet"
    #[serde(
        rename = "resource_type",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub resource_type: String,
    /// 合并单元格的相关信息。没有合并单元格则不返回。
    #[serde(
        rename = "merges",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub merges: Vec<MergeRangeSubResp>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(default)]
pub struct GridPropertiesSubResp {
    /// 冻结的行数量
    ///
    /// **示例值**: "0"
    #[serde(
        rename = "frozen_row_count",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub frozen_row_count: i64,
    /// 冻结的列数量
    ///
    /// **示例值**: "0"
    #[serde(
        rename = "frozen_column_count",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub frozen_column_count: i64,
    /// 工作表的行数
    ///
    /// **示例值**: "200"
    #[serde(
        rename = "row_count",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub row_count: i64,
    /// 工作表的列数量
    ///
    /// **示例值**: "20"
    #[serde(
        rename = "column_count",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub column_count: i64,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(default)]
pub struct MergeRangeSubResp {
    /// 起始行，从 0 开始计数。
    ///
    /// **示例值**: "0"
    #[serde(
        rename = "start_row_index",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub start_row_index: i64,
    /// 结束行，从 0 开始计数。
    ///
    /// **示例值**: "-"
    #[serde(
        rename = "end_row_index",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub end_row_index: i64,
    /// 起始列，从 0 开始计数。
    ///
    /// **示例值**: "0"
    #[serde(
        rename = "start_column_index",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub start_column_index: i64,
    /// 结束列，从 0 开始计数。
    ///
    /// **示例值**: "0"
    #[serde(
        rename = "end_column_index",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub end_column_index: i64,
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
        Fn(GetSheetReq) -> Result<(GetSheetResp, CommonResponse), Error> + Send + Sync + 'static
    {
    }
    impl<
            T: Fn(GetSheetReq) -> Result<(GetSheetResp, CommonResponse), Error>
                + Send
                + Sync
                + 'static,
        > MockFunc for T
    {
    }

    impl<'c, IStore: Store, IClient: HttpClient> DriveServiceMocker<'c, IStore, IClient> {
        pub fn mock_get_sheet<F: MockFunc>(
            &self,
            f: F,
        ) -> MockerBuilder<Mocker, GetSheetReq, GetSheetResp, Arc<dyn MockFunc>> {
            MockerBuilder::new(self.cli.instance_id, Arc::new(f))
        }

        pub(super) fn get_mock_get_sheet(&self, req: &GetSheetReq) -> Option<Arc<dyn MockFunc>> {
            do_mock::<Mocker, GetSheetReq, GetSheetResp, Arc<dyn MockFunc>>(
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
        api::gen::drive::get_sheet::{GetSheetReq, GetSheetResp, GetSheetRespInner},
        core::{model::CommonResponse, Lark},
    };

    #[tokio::test]
    async fn test_mock() {
        let lark = Lark::new("".to_owned(), "".into());
        let mocker = lark
            .drive()
            .mock()
            .mock_get_sheet(|_| Ok((GetSheetResp::default(), CommonResponse::default())))
            .build();
        let res = lark.drive().get_sheet(GetSheetReq::default()).await;
        assert!(res.is_ok());
        mocker.clear();
        let res = lark.drive().get_sheet(GetSheetReq::default()).await;
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
    "msg": "",
    "data": {
        "sheet": {
            "sheet_id": "sxj5ws",
            "title": "sheet1",
            "index": 0,
            "hidden": false,
            "grid_properties": {
                "frozen_row_count": 0,
                "frozen_column_count": 0,
                "row_count": 200,
                "column_count": 20
            },
            "resource_type": "sheet",
            "merges": [
                {
                    "start_row_index": 0,
                    "end_row_index": 0,
                    "start_column_index": 0,
                    "end_column_index": 0
                }
            ]
        }
    }
}"#;
    #[test]
    fn test_resp() {
        let res = serde_json::from_str::<GetSheetRespInner>(RESP);
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
