//! doc url: <https://open.larkoffice.com/document/ukTMukTMukTM/uUDN04SN0QjL1QDN/sheets-v3/spreadsheet-sheet/find>
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
    /// **api 版本: 2024-07-31T09:17:16+00:00**
    ///
    /// ## 查找单元格
    ///
    /// 在指定范围内查找符合查找条件的单元格。
    ///

    ///
    /// doc: <https://open.larkoffice.com/document/ukTMukTMukTM/uUDN04SN0QjL1QDN/sheets-v3/spreadsheet-sheet/find>
    ///
    /// new doc: <https://open.larkoffice.com/document/server-docs/docs/sheets-v3/data-operation/find>
    ///
    /// api url: <https://open.feishu.cn/document_portal/v1/document/get_detail?fullPath=%2Fserver-docs%2Fdocs%2Fsheets-v3%2Fdata-operation%2Ffind>
    pub async fn find_sheet(
        &self,
        req: FindSheetReq,
    ) -> Result<(FindSheetResp, CommonResponse), Error> {
        self.find_sheet_with_opt(req, Default::default()).await
    }

    /// 参见 [find_sheet](#method.find_sheet) 函数
    pub async fn find_sheet_with_opt(
        &self,
        req: FindSheetReq,
        method_option: MethodOption,
    ) -> Result<(FindSheetResp, CommonResponse), Error> {
        #[cfg(feature = "test-util")]
        {
            if let Some(f) = self.mock().get_mock_find_sheet(&req) {
                tracing::info!("[lark] Drive#FindSheet **mocking** api");
                return f(req);
            }
        }

        tracing::info!("[lark] Drive#FindSheet call api");

        let req = ApiRequest {
            scope: "Drive",
            api: "FindSheet",
            method: http::Method::POST,
            url: String::new()
                + self.cli.open_base_url.as_ref()
                + "/open-apis/sheets/v3/spreadsheets/:spreadsheet_token/sheets/:sheet_id/find",
            param_data: req.gen_param(),
            method_option,
            need_tenant_access_token: true,
            need_user_access_token: true,
            ..Default::default()
        };

        let (resp, common_resp): (FindSheetRespInner, _) = self.cli.do_req(req).await?;
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
pub struct FindSheetReq {
    /// 电子表格的 token。可通过以下两种方式获取。了解更多，参考[电子表格概述](https://open.feishu.cn/document/ukTMukTMukTM/uATMzUjLwEzM14CMxMTN/overview)。
    ///
    /// - 电子表格的 URL：https://sample.feishu.cn/sheets/==Iow7sNNEphp3WbtnbCscPqabcef==
    ///
    /// - 调用[获取文件夹中的文件清单](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/drive-v1/file/list)
    ///
    /// **示例值**: "Iow7sNNEphp3WbtnbCscPqabcef"
    #[api(kind = "path", name = "spreadsheet_token")]
    pub spreadsheet_token: String,
    /// 工作表的 ID，获取方式见[获取工作表](https://open.feishu.cn/document/ukTMukTMukTM/uUDN04SN0QjL1QDN/sheets-v3/spreadsheet-sheet/query)。
    ///
    /// **示例值**: "PNIfrm"
    #[api(kind = "path", name = "sheet_id")]
    pub sheet_id: String,

    /// 指定查找单元格的条件。
    ///
    /// **是否必填**: 是
    #[api(kind = "body", name = "find_condition")]
    pub find_condition: FindConditionSubReq,
    /// 查找的字符串。当`search_by_regex` 字段为 true 时，你需填入正则表达式
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "如下所示：
    ///
    /// - 字符串查找示例： "hello"
    ///
    /// - 正则表达式查找示例："[A-Z]\w+"
    #[api(kind = "body", name = "find")]
    pub find: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(default)]
pub struct FindConditionSubReq {
    /// 查找范围。格式为 `<sheetId>!<开始位置>:<结束位置>`。其中：
    ///
    /// - `sheetId` 为工作表 ID，通过[获取工作表](https://open.feishu.cn/document/ukTMukTMukTM/uUDN04SN0QjL1QDN/sheets-v3/spreadsheet-sheet/query) 获取
    ///
    /// - `<开始位置>:<结束位置>` 为工作表中单元格的范围，数字表示行索引，字母表示列索引。如 `A2:B2` 表示该工作表第 2 行的 A 列到 B 列。`range`支持四种写法，详情参考[电子表格概述](https://open.feishu.cn/document/ukTMukTMukTM/uATMzUjLwEzM14CMxMTN/overview)
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "PNIfrm!A1:C5"
    #[serde(
        rename = "range",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub range: String,
    /// 是否忽略查找字符串的大小写，默认为 false。
    ///
    /// - `true`：忽略字符串中字母大小写差异
    ///
    /// - `false`：区分字符串中字母大小写
    ///
    /// **示例值**: "true"
    #[serde(
        rename = "match_case",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub match_case: Option<bool>,
    /// 字符串是否需要完全匹配整个单元格，默认值为 false。
    ///
    /// - `true`：完全匹配单元格，比如 `find` 参数 取值为 "hello"，则单元格中的内容必须为 "hello" 才会匹配替换
    ///
    /// - `false`：允许部分匹配单元格，比如 `find` 取值为 "hello"，则单元格中的内容包含 "hello" 即可匹配替换
    ///
    /// **示例值**: "false"
    #[serde(
        rename = "match_entire_cell",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub match_entire_cell: Option<bool>,
    /// 是否使用正则表达式查找，默认值为 false。
    ///
    /// - `true`：使用正则表达式
    ///
    /// - `false`：不使用正则表达式
    ///
    /// **示例值**: "false"
    #[serde(
        rename = "search_by_regex",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub search_by_regex: Option<bool>,
    /// 是否仅搜索单元格公式，默认值为 false。
    ///
    /// - `true`：仅搜索单元格公式
    ///
    /// - `false`：仅搜索单元格内容
    ///
    /// **示例值**: "false"
    #[serde(
        rename = "include_formulas",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub include_formulas: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize, Clone, lark_bot_sdk_macros::ApiBaseResp)]
struct FindSheetRespInner {
    #[serde(flatten)]
    data: Option<FindSheetResp>,
    #[serde(flatten)]
    base: BaseResp,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct FindSheetResp {
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
    /// 符合条件的信息
    #[serde(
        rename = "find_result",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub find_result: FindReplaceResultSubResp,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(default)]
pub struct FindReplaceResultSubResp {
    /// 符合查找条件的单元格数组，不包含公式
    #[serde(
        rename = "matched_cells",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub matched_cells: Vec<String>,
    /// 符合查找条件的含有公式的单元格数组
    #[serde(
        rename = "matched_formula_cells",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub matched_formula_cells: Vec<String>,
    /// 符合查找条件的总行数
    ///
    /// **示例值**: "2"
    #[serde(
        rename = "rows_count",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub rows_count: i64,
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
        Fn(FindSheetReq) -> Result<(FindSheetResp, CommonResponse), Error> + Send + Sync + 'static
    {
    }
    impl<
            T: Fn(FindSheetReq) -> Result<(FindSheetResp, CommonResponse), Error>
                + Send
                + Sync
                + 'static,
        > MockFunc for T
    {
    }

    impl<'c, IStore: Store, IClient: HttpClient> DriveServiceMocker<'c, IStore, IClient> {
        pub fn mock_find_sheet<F: MockFunc>(
            &self,
            f: F,
        ) -> MockerBuilder<Mocker, FindSheetReq, FindSheetResp, Arc<dyn MockFunc>> {
            MockerBuilder::new(self.cli.instance_id, Arc::new(f))
        }

        pub(super) fn get_mock_find_sheet(&self, req: &FindSheetReq) -> Option<Arc<dyn MockFunc>> {
            do_mock::<Mocker, FindSheetReq, FindSheetResp, Arc<dyn MockFunc>>(
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
        api::gen::drive::find_sheet::{FindSheetReq, FindSheetResp, FindSheetRespInner},
        core::{model::CommonResponse, Lark},
    };

    #[tokio::test]
    async fn test_mock() {
        let lark = Lark::new("".to_owned(), "".into());
        let mocker = lark
            .drive()
            .mock()
            .mock_find_sheet(|_| Ok((FindSheetResp::default(), CommonResponse::default())))
            .build();
        let res = lark.drive().find_sheet(FindSheetReq::default()).await;
        assert!(res.is_ok());
        mocker.clear();
        let res = lark.drive().find_sheet(FindSheetReq::default()).await;
        assert!(res.is_err());
    }

    const REQ: &str = r#"{
    "find_condition": {
        "range": "PNIfrm!A1:C5",
        "match_case": true,
        "match_entire_cell": false,
        "search_by_regex": false,
        "include_formulas": false
    },
    "find": "hello"
}"#;

    #[test]
    fn test_req() {
        if REQ == "{}" {
            return;
        }
        if let Err(e) = serde_json::from_str::<super::FindSheetReqBody>(REQ) {
            panic!("{}", e);
        }
    }

    const RESP: &str = r#"{
    "code": 0,
    "msg": "success",
    "data": {
        "find_result": {
            "matched_cells": [
                "A1"
            ],
            "matched_formula_cells": [
                "B3"
            ],
            "rows_count": 2
        }
    }
}"#;
    #[test]
    fn test_resp() {
        let res = serde_json::from_str::<FindSheetRespInner>(RESP);
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
