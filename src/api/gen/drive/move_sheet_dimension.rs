//! doc url: <https://open.larkoffice.com/document/ukTMukTMukTM/uUDN04SN0QjL1QDN/sheets-v3/spreadsheet-sheet/move_dimension>
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
    /// ## 移动行列
    ///
    /// 该接口用于移动行或列。行或列被移动到目标位置后，原本在目标位置的行列会对应右移或下移。
    ///

    ///
    /// doc: <https://open.larkoffice.com/document/ukTMukTMukTM/uUDN04SN0QjL1QDN/sheets-v3/spreadsheet-sheet/move_dimension>
    ///
    /// new doc: <https://open.larkoffice.com/document/server-docs/docs/sheets-v3/sheet-rowcol/move_dimension>
    ///
    /// api url: <https://open.feishu.cn/document_portal/v1/document/get_detail?fullPath=%2Fserver-docs%2Fdocs%2Fsheets-v3%2Fsheet-rowcol%2Fmove_dimension>
    pub async fn move_sheet_dimension(
        &self,
        req: MoveSheetDimensionReq,
    ) -> Result<(MoveSheetDimensionResp, CommonResponse), Error> {
        self.move_sheet_dimension_with_opt(req, Default::default())
            .await
    }

    /// 参见 [move_sheet_dimension](#method.move_sheet_dimension) 函数
    pub async fn move_sheet_dimension_with_opt(
        &self,
        req: MoveSheetDimensionReq,
        method_option: MethodOption,
    ) -> Result<(MoveSheetDimensionResp, CommonResponse), Error> {
        #[cfg(feature = "test-util")]
        {
            if let Some(f) = self.mock().get_mock_move_sheet_dimension(&req) {
                tracing::info!("[lark] Drive#MoveSheetDimension **mocking** api");
                return f(req);
            }
        }

        tracing::info!("[lark] Drive#MoveSheetDimension call api");

        let req = ApiRequest {
            scope: "Drive",
            api: "MoveSheetDimension",
            method: http::Method::POST,
            url: String::new() + self.cli.open_base_url.as_ref() + "/open-apis/sheets/v3/spreadsheets/:spreadsheet_token/sheets/:sheet_id/move_dimension",
            param_data: req.gen_param(),
            method_option,
            need_tenant_access_token: true,
need_user_access_token: true,
            ..Default::default()
        };

        let (resp, common_resp): (MoveSheetDimensionRespInner, _) = self.cli.do_req(req).await?;
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
pub struct MoveSheetDimensionReq {
    /// 电子表格的 token。可通过以下两种方式获取。了解更多，参考[电子表格概述](https://open.feishu.cn/document/ukTMukTMukTM/uATMzUjLwEzM14CMxMTN/overview)。
    ///
    /// -  电子表格的 URL：https://sample.feishu.cn/sheets/==Iow7sNNEphp3WbtnbCscPqabcef==
    ///
    /// - 调用[获取文件夹中的文件清单](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/drive-v1/file/list)
    ///
    /// **示例值**: "Iow7sNNEphp3WbtnbCscPqabcef"
    #[api(kind = "path", name = "spreadsheet_token")]
    pub spreadsheet_token: String,
    /// 工作表的 ID。调用[获取工作表](https://open.feishu.cn/document/ukTMukTMukTM/uUDN04SN0QjL1QDN/sheets-v3/spreadsheet-sheet/query)获取 ID
    ///
    /// **示例值**: "2jm6f6"
    #[api(kind = "path", name = "sheet_id")]
    pub sheet_id: String,

    /// 移动源位置信息
    #[api(kind = "body", name = "source")]
    pub source: Option<DimensionSubReq>,
    /// 移动的目标位置行或者列
    ///
    /// **示例值**: "4"
    #[api(kind = "body", name = "destination_index")]
    pub destination_index: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(default)]
pub struct DimensionSubReq {
    /// 移动的维度。可选值：
    ///
    /// - `ROWS`：行
    ///
    /// - `COLUMNS`：列
    ///
    /// **示例值**: "ROWS"
    #[serde(
        rename = "major_dimension",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub major_dimension: Option<String>,
    /// 要移动的行或列的起始位置。从 0 开始计数。若 `startIndex` 为 3，则从第 4 行或列开始移动。包含第 4 行或列。
    ///
    /// **示例值**: "0"
    #[serde(
        rename = "start_index",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub start_index: Option<i64>,
    /// 要移动的行或列结束的位置。从 0 开始计数。若 `endIndex` 为 7，则要移动的范围至第 8 行或列结束。不包含第 8 行或列。
    ///
    /// 示例：当 `majorDimension`为 `ROWS`、 `startIndex` 为 3、`endIndex ` 为 7 时，则移动第 4、5、6、7 行，共 4 行。
    ///
    /// **示例值**: "1"
    #[serde(
        rename = "end_index",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub end_index: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize, Clone, lark_bot_sdk_macros::ApiBaseResp)]
struct MoveSheetDimensionRespInner {
    #[serde(flatten)]
    data: Option<MoveSheetDimensionResp>,
    #[serde(flatten)]
    base: BaseResp,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct MoveSheetDimensionResp {
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
        Fn(MoveSheetDimensionReq) -> Result<(MoveSheetDimensionResp, CommonResponse), Error>
        + Send
        + Sync
        + 'static
    {
    }
    impl<
            T: Fn(MoveSheetDimensionReq) -> Result<(MoveSheetDimensionResp, CommonResponse), Error>
                + Send
                + Sync
                + 'static,
        > MockFunc for T
    {
    }

    impl<'c, IStore: Store, IClient: HttpClient> DriveServiceMocker<'c, IStore, IClient> {
        pub fn mock_move_sheet_dimension<F: MockFunc>(
            &self,
            f: F,
        ) -> MockerBuilder<Mocker, MoveSheetDimensionReq, MoveSheetDimensionResp, Arc<dyn MockFunc>>
        {
            MockerBuilder::new(self.cli.instance_id, Arc::new(f))
        }

        pub(super) fn get_mock_move_sheet_dimension(
            &self,
            req: &MoveSheetDimensionReq,
        ) -> Option<Arc<dyn MockFunc>> {
            do_mock::<Mocker, MoveSheetDimensionReq, MoveSheetDimensionResp, Arc<dyn MockFunc>>(
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
        api::gen::drive::move_sheet_dimension::{
            MoveSheetDimensionReq, MoveSheetDimensionResp, MoveSheetDimensionRespInner,
        },
        core::{model::CommonResponse, Lark},
    };

    #[tokio::test]
    async fn test_mock() {
        let lark = Lark::new("".to_owned(), "".into());
        let mocker = lark
            .drive()
            .mock()
            .mock_move_sheet_dimension(|_| {
                Ok((MoveSheetDimensionResp::default(), CommonResponse::default()))
            })
            .build();
        let res = lark
            .drive()
            .move_sheet_dimension(MoveSheetDimensionReq::default())
            .await;
        assert!(res.is_ok());
        mocker.clear();
        let res = lark
            .drive()
            .move_sheet_dimension(MoveSheetDimensionReq::default())
            .await;
        assert!(res.is_err());
    }

    const REQ: &str = r#"{
    "source": {
        "major_dimension": "ROWS",
        "start_index": 0,
        "end_index": 1
    },
    "destination_index": 4
}"#;

    #[test]
    fn test_req() {
        if REQ == "{}" {
            return;
        }
        if let Err(e) = serde_json::from_str::<super::MoveSheetDimensionReqBody>(REQ) {
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
        let res = serde_json::from_str::<MoveSheetDimensionRespInner>(RESP);
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
