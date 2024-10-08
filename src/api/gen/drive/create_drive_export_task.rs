//! doc url: <https://open.larkoffice.com/document/uAjLw4CM/ukTMukTMukTM/reference/drive-v1/export_task/create>
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
    /// **api 版本: 2024-07-31T09:16:04+00:00**
    ///
    /// ## 创建导出任务
    ///
    /// 该接口用于创建导出文件的任务，并返回导出任务 ID。导出文件指将飞书文档、电子表格、多维表格导出为本地文件，包括 Word、Excel、PDF、CSV 格式。该接口为异步接口，需要继续调用[查询导出任务结果](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/drive-v1/export_task/get)接口获取导出结果。了解完整的导出步骤，参考[导出云文档概述](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/drive-v1/export_task/export-user-guide)。
    ///

    ///
    /// doc: <https://open.larkoffice.com/document/uAjLw4CM/ukTMukTMukTM/reference/drive-v1/export_task/create>
    ///
    /// new doc: <https://open.larkoffice.com/document/server-docs/docs/drive-v1/export_task/create>
    ///
    /// api url: <https://open.feishu.cn/document_portal/v1/document/get_detail?fullPath=%2Fserver-docs%2Fdocs%2Fdrive-v1%2Fexport_task%2Fcreate>
    pub async fn create_drive_export_task(
        &self,
        req: CreateDriveExportTaskReq,
    ) -> Result<(CreateDriveExportTaskResp, CommonResponse), Error> {
        self.create_drive_export_task_with_opt(req, Default::default())
            .await
    }

    /// 参见 [create_drive_export_task](#method.create_drive_export_task) 函数
    pub async fn create_drive_export_task_with_opt(
        &self,
        req: CreateDriveExportTaskReq,
        method_option: MethodOption,
    ) -> Result<(CreateDriveExportTaskResp, CommonResponse), Error> {
        #[cfg(feature = "test-util")]
        {
            if let Some(f) = self.mock().get_mock_create_drive_export_task(&req) {
                tracing::info!("[lark] Drive#CreateDriveExportTask **mocking** api");
                return f(req);
            }
        }

        tracing::info!("[lark] Drive#CreateDriveExportTask call api");

        let req = ApiRequest {
            scope: "Drive",
            api: "CreateDriveExportTask",
            method: http::Method::POST,
            url: String::new()
                + self.cli.open_base_url.as_ref()
                + "/open-apis/drive/v1/export_tasks",
            param_data: req.gen_param(),
            method_option,
            need_tenant_access_token: true,
            need_user_access_token: true,
            ..Default::default()
        };

        let (resp, common_resp): (CreateDriveExportTaskRespInner, _) = self.cli.do_req(req).await?;
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
pub struct CreateDriveExportTaskReq {
    /// 将云文档导出为本地文件后，本地文件的扩展名。了解各类云文档支持导出的文件格式，参考[导出云文档概述](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/drive-v1/export_task/export-user-guide)。
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "csv"
    ///
    /// **可选值**:
    ///
    /// `Docx`: Microsoft Word 格式
    ///
    /// `Pdf`: PDF 格式
    ///
    /// `Xlsx`: Microsoft Excel 格式
    ///
    /// `Csv`: CSV 格式
    #[api(kind = "body", name = "file_extension")]
    pub file_extension: String,
    /// 要导出的云文档的 token。获取方式参考 [如何获取云文档相关 token](https://open.feishu.cn/document/ukTMukTMukTM/uczNzUjL3czM14yN3MTN#08bb5df6)。
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "Fm7osyjtMh5o7Ktrv32c73abcef"
    ///
    /// **数据校验规则**：
    ///
    /// - **最大长度**: `27` 字符
    #[api(kind = "body", name = "token")]
    pub token: String,
    /// 要导出的云文档的类型 。可通过云文档的链接判断。
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "sheet"
    ///
    /// **可选值**:
    ///
    /// `Doc`: 旧版飞书文档。支持导出扩展名为 docx 和 pdf 的文件。已不推荐使用。
    ///
    /// `Sheet`: 飞书电子表格。支持导出扩展名为 xlsx 和 csv 的文件。
    ///
    /// `Bitable`: 飞书多维表格。支持导出扩展名为 xlsx 和 csv 格式的文件。
    ///
    /// `Docx`: 新版飞书文档。支持导出扩展名为 docx 和 pdf 格式的文件。
    #[api(kind = "body", name = "type")]
    pub body_type: String,
    /// 导出飞书电子表格为 CSV 文件时，需传入电子表格中具体工作表的 ID。
    ///
    /// 你需调用
    ///
    /// [获取工作表](https://open.feishu.cn/document/ukTMukTMukTM/uUDN04SN0QjL1QDN/sheets-v3/spreadsheet-sheet/query) 接口获取返回的 `sheet_id` 的值作为该参数的值。
    ///
    /// **示例值**: "6e5ed3"
    #[api(kind = "body", name = "sub_id")]
    pub sub_id: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, lark_bot_sdk_macros::ApiBaseResp)]
struct CreateDriveExportTaskRespInner {
    #[serde(flatten)]
    data: Option<CreateDriveExportTaskResp>,
    #[serde(flatten)]
    base: BaseResp,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct CreateDriveExportTaskResp {
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
    /// 导出任务的 ID
    ///
    /// **示例值**: "6933093124755423251"
    #[serde(
        rename = "ticket",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub ticket: String,
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
        Fn(CreateDriveExportTaskReq) -> Result<(CreateDriveExportTaskResp, CommonResponse), Error>
        + Send
        + Sync
        + 'static
    {
    }
    impl<
            T: Fn(
                    CreateDriveExportTaskReq,
                ) -> Result<(CreateDriveExportTaskResp, CommonResponse), Error>
                + Send
                + Sync
                + 'static,
        > MockFunc for T
    {
    }

    impl<'c, IStore: Store, IClient: HttpClient> DriveServiceMocker<'c, IStore, IClient> {
        pub fn mock_create_drive_export_task<F: MockFunc>(
            &self,
            f: F,
        ) -> MockerBuilder<
            Mocker,
            CreateDriveExportTaskReq,
            CreateDriveExportTaskResp,
            Arc<dyn MockFunc>,
        > {
            MockerBuilder::new(self.cli.instance_id, Arc::new(f))
        }

        pub(super) fn get_mock_create_drive_export_task(
            &self,
            req: &CreateDriveExportTaskReq,
        ) -> Option<Arc<dyn MockFunc>> {
            do_mock::<Mocker, CreateDriveExportTaskReq, CreateDriveExportTaskResp, Arc<dyn MockFunc>>(
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
        api::gen::drive::create_drive_export_task::{
            CreateDriveExportTaskReq, CreateDriveExportTaskResp, CreateDriveExportTaskRespInner,
        },
        core::{model::CommonResponse, Lark},
    };

    #[tokio::test]
    async fn test_mock() {
        let lark = Lark::new("".to_owned(), "".into());
        let mocker = lark
            .drive()
            .mock()
            .mock_create_drive_export_task(|_| {
                Ok((
                    CreateDriveExportTaskResp::default(),
                    CommonResponse::default(),
                ))
            })
            .build();
        let res = lark
            .drive()
            .create_drive_export_task(CreateDriveExportTaskReq::default())
            .await;
        assert!(res.is_ok());
        mocker.clear();
        let res = lark
            .drive()
            .create_drive_export_task(CreateDriveExportTaskReq::default())
            .await;
        assert!(res.is_err());
    }

    const REQ: &str = r#"{
    "file_extension": "csv",
    "token": "Fm7osyjtMh5o7Ktrv32c73abcef",
    "type": "sheet",
    "sub_id": "6e5ed3"
}"#;

    #[test]
    fn test_req() {
        if REQ == "{}" {
            return;
        }
        if let Err(e) = serde_json::from_str::<super::CreateDriveExportTaskReqBody>(REQ) {
            panic!("{}", e);
        }
    }

    const RESP: &str = r#"{
    "code": 0,
    "msg": "success",
    "data": {
        "ticket": "6933093124755423251"
    }
}"#;
    #[test]
    fn test_resp() {
        let res = serde_json::from_str::<CreateDriveExportTaskRespInner>(RESP);
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
