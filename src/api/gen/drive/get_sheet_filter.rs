//! doc url: <https://open.feishu.cn/document/ukTMukTMukTM/uUDN04SN0QjL1QDN/sheets-v3/spreadsheet-sheet-filter/get>
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
    /// **api 版本: 2024-07-31T09:17:17+00:00**
    ///
    /// ## 获取筛选
    ///
    /// 获取电子表格中工作表的详细筛选信息，包括筛选的应用范围、筛选条件、被筛选条件过滤掉的行。
    ///

    ///
    /// doc: <https://open.feishu.cn/document/ukTMukTMukTM/uUDN04SN0QjL1QDN/sheets-v3/spreadsheet-sheet-filter/get>
    ///
    /// new doc: <https://open.feishu.cn/document/server-docs/docs/sheets-v3/spreadsheet-sheet-filter/get>
    ///
    /// api url: <https://open.feishu.cn/document_portal/v1/document/get_detail?fullPath=%2Fserver-docs%2Fdocs%2Fsheets-v3%2Fspreadsheet-sheet-filter%2Fget>
    pub async fn get_sheet_filter(
        &self,
        req: GetSheetFilterReq,
    ) -> Result<(GetSheetFilterResp, CommonResponse), Error> {
        self.get_sheet_filter_with_opt(req, Default::default())
            .await
    }

    /// 参见 [get_sheet_filter](#method.get_sheet_filter) 函数
    pub async fn get_sheet_filter_with_opt(
        &self,
        req: GetSheetFilterReq,
        method_option: MethodOption,
    ) -> Result<(GetSheetFilterResp, CommonResponse), Error> {
        #[cfg(feature = "test-util")]
        {
            if let Some(f) = self.mock().get_mock_get_sheet_filter(&req) {
                tracing::info!("[lark] Drive#GetSheetFilter **mocking** api");
                return f(req);
            }
        }

        tracing::info!("[lark] Drive#GetSheetFilter call api");

        let req = ApiRequest {
            scope: "Drive",
            api: "GetSheetFilter",
            method: http::Method::GET,
            url: String::new()
                + self.cli.open_base_url.as_ref()
                + "/open-apis/sheets/v3/spreadsheets/:spreadsheet_token/sheets/:sheet_id/filter",
            param_data: req.gen_param(),
            method_option,
            need_tenant_access_token: true,
            need_user_access_token: true,
            ..Default::default()
        };

        let (resp, common_resp): (GetSheetFilterRespInner, _) = self.cli.do_req(req).await?;
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
pub struct GetSheetFilterReq {
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
struct GetSheetFilterRespInner {
    #[serde(flatten)]
    data: Option<GetSheetFilterResp>,
    #[serde(flatten)]
    base: BaseResp,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct GetSheetFilterResp {
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
    /// 筛选信息
    #[serde(
        rename = "sheet_filter_info",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub sheet_filter_info: SheetFilterInfoSubResp,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(default)]
pub struct SheetFilterInfoSubResp {
    /// 筛选的应用范围
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "8fe9d6!A1:H14"
    #[serde(
        rename = "range",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub range: String,
    /// 被筛选条件过滤掉的行。从 1 开始索引。
    ///
    /// **是否必填**: 是
    #[serde(
        rename = "filtered_out_rows",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub filtered_out_rows: Vec<i64>,
    /// 工作表的筛选条件
    ///
    /// **是否必填**: 是
    #[serde(
        rename = "filter_infos",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub filter_infos: Vec<FilterInfoSubResp>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(default)]
pub struct FilterInfoSubResp {
    /// 设置了筛选条件的列
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "E"
    #[serde(
        rename = "col",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub col: String,
    /// 筛选条件
    ///
    /// **是否必填**: 是
    #[serde(
        rename = "conditions",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub conditions: Vec<ConditionSubResp>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(default)]
pub struct ConditionSubResp {
    /// 筛选类型，枚举值如下所示。了解更多，参考[筛选指南](https://open.feishu.cn/document/ukTMukTMukTM/uUDN04SN0QjL1QDN/sheets-v3/spreadsheet-sheet-filter/filter-user-guide)。
    ///
    /// - multiValue ：多值筛选
    ///
    /// - number ：数字筛选
    ///
    /// - text ：文本筛选
    ///
    /// - color ：颜色筛选
    ///
    /// - clear ：清除筛选
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "number"
    #[serde(
        rename = "filter_type",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub filter_type: String,
    /// 比较类型。不同筛选类型的比较类型的枚举值不同，详情参考[筛选指南](https://open.feishu.cn/document/ukTMukTMukTM/uUDN04SN0QjL1QDN/sheets-v3/spreadsheet-sheet-filter/filter-user-guide)。
    ///
    /// **示例值**: "less"
    #[serde(
        rename = "compare_type",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub compare_type: String,
    /// 筛选参数。不同筛选类型的筛选参数限制不同，详情参考[筛选指南](https://open.feishu.cn/document/ukTMukTMukTM/uUDN04SN0QjL1QDN/sheets-v3/spreadsheet-sheet-filter/filter-user-guide)。
    ///
    /// **是否必填**: 是
    #[serde(
        rename = "expected",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub expected: Vec<String>,
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
        Fn(GetSheetFilterReq) -> Result<(GetSheetFilterResp, CommonResponse), Error>
        + Send
        + Sync
        + 'static
    {
    }
    impl<
            T: Fn(GetSheetFilterReq) -> Result<(GetSheetFilterResp, CommonResponse), Error>
                + Send
                + Sync
                + 'static,
        > MockFunc for T
    {
    }

    impl<'c, IStore: Store, IClient: HttpClient> DriveServiceMocker<'c, IStore, IClient> {
        pub fn mock_get_sheet_filter<F: MockFunc>(
            &self,
            f: F,
        ) -> MockerBuilder<Mocker, GetSheetFilterReq, GetSheetFilterResp, Arc<dyn MockFunc>>
        {
            MockerBuilder::new(self.cli.instance_id, Arc::new(f))
        }

        pub(super) fn get_mock_get_sheet_filter(
            &self,
            req: &GetSheetFilterReq,
        ) -> Option<Arc<dyn MockFunc>> {
            do_mock::<Mocker, GetSheetFilterReq, GetSheetFilterResp, Arc<dyn MockFunc>>(
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
        api::gen::drive::get_sheet_filter::{
            GetSheetFilterReq, GetSheetFilterResp, GetSheetFilterRespInner,
        },
        core::{model::CommonResponse, Lark},
    };

    #[tokio::test]
    async fn test_mock() {
        let lark = Lark::new("".to_owned(), "".into());
        let mocker = lark
            .drive()
            .mock()
            .mock_get_sheet_filter(|_| {
                Ok((GetSheetFilterResp::default(), CommonResponse::default()))
            })
            .build();
        let res = lark
            .drive()
            .get_sheet_filter(GetSheetFilterReq::default())
            .await;
        assert!(res.is_ok());
        mocker.clear();
        let res = lark
            .drive()
            .get_sheet_filter(GetSheetFilterReq::default())
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
        "sheet_filter_info": {
            "range": "8fe9d6!A1:H14",
            "filtered_out_rows": [
                4,
                5,
                6
            ],
            "filter_infos": [
                {
                    "col": "E",
                    "conditions": [
                        {
                            "filter_type": "number",
                            "compare_type": "less",
                            "expected": [
                                "50%"
                            ]
                        }
                    ]
                }
            ]
        }
    }
}"#;
    #[test]
    fn test_resp() {
        let res = serde_json::from_str::<GetSheetFilterRespInner>(RESP);
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
