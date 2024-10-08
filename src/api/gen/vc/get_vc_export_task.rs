//! doc url: <https://open.larkoffice.com/document/uAjLw4CM/ukTMukTMukTM/reference/vc-v1/export/get>
// Code generated by gen_api. DO NOT EDIT.

use serde::{Deserialize, Serialize};

use crate::api::BaseResp;
use crate::api::HasBaseResp;
use crate::core::model::{ApiRequest, CommonResponse, MethodOption, ReqParam, StreamReqParam};
use crate::{
    core::{http_client::HttpClient, store::Store},
    error::Error,
};

use crate::api::gen::vc::VcService;

impl<'c, IStore: Store, IClient: HttpClient> VcService<'c, IStore, IClient> {
    /// **api 版本: 2024-07-23T07:32:57+00:00**
    ///
    /// ## 查询导出任务结果
    ///
    /// 查看异步导出的进度。
    ///

    ///
    /// doc: <https://open.larkoffice.com/document/uAjLw4CM/ukTMukTMukTM/reference/vc-v1/export/get>
    ///
    /// new doc: <https://open.larkoffice.com/document/server-docs/vc-v1/export/get>
    ///
    /// api url: <https://open.feishu.cn/document_portal/v1/document/get_detail?fullPath=%2Fserver-docs%2Fvc-v1%2Fexport%2Fget>
    pub async fn get_vc_export_task(
        &self,
        req: GetVcExportTaskReq,
    ) -> Result<(GetVcExportTaskResp, CommonResponse), Error> {
        self.get_vc_export_task_with_opt(req, Default::default())
            .await
    }

    /// 参见 [get_vc_export_task](#method.get_vc_export_task) 函数
    pub async fn get_vc_export_task_with_opt(
        &self,
        req: GetVcExportTaskReq,
        method_option: MethodOption,
    ) -> Result<(GetVcExportTaskResp, CommonResponse), Error> {
        #[cfg(feature = "test-util")]
        {
            if let Some(f) = self.mock().get_mock_get_vc_export_task(&req) {
                tracing::info!("[lark] Vc#GetVcExportTask **mocking** api");
                return f(req);
            }
        }

        tracing::info!("[lark] Vc#GetVcExportTask call api");

        let req = ApiRequest {
            scope: "Vc",
            api: "GetVcExportTask",
            method: http::Method::GET,
            url: String::new()
                + self.cli.open_base_url.as_ref()
                + "/open-apis/vc/v1/exports/:task_id",
            param_data: req.gen_param(),
            method_option,
            need_tenant_access_token: true,
            need_user_access_token: true,
            ..Default::default()
        };

        let (resp, common_resp): (GetVcExportTaskRespInner, _) = self.cli.do_req(req).await?;
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
pub struct GetVcExportTaskReq {
    /// 任务id
    ///
    /// **示例值**: "7108646852144136212"
    #[api(kind = "path", name = "task_id")]
    pub task_id: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, lark_bot_sdk_macros::ApiBaseResp)]
struct GetVcExportTaskRespInner {
    #[serde(flatten)]
    data: Option<GetVcExportTaskResp>,
    #[serde(flatten)]
    base: BaseResp,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct GetVcExportTaskResp {
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
    /// 任务状态
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "3"
    ///
    /// **可选值**:
    ///
    /// `in_progress`: 处理中
    ///
    /// `failed`: 失败
    ///
    /// `done`: 完成
    #[serde(
        rename = "status",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub status: i64,
    /// 文件下载地址
    ///
    /// **示例值**: "https://lf1-ttcdn-tos.pstatp.com/obj/xxx"
    #[serde(
        rename = "url",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub url: String,
    /// 文件token
    ///
    /// **示例值**: "6yHu7Igp7Igy62Ez6fLr6IJz7j9i5WMe6fHq5yZeY2Jz6yLqYAMAY46fZfEz64Lr5fYyYQ=="
    #[serde(
        rename = "file_token",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub file_token: String,
    /// 失败信息
    ///
    /// **示例值**: "no permission"
    #[serde(
        rename = "fail_msg",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub fail_msg: String,
}

#[cfg(feature = "test-util")]
mod test_utils {
    use std::sync::Arc;

    use self::gen::vc::VcServiceMocker;
    use crate::core::mocker::*;

    use super::*;
    use crate::api::*;
    use crate::core::model::*;

    pub struct Mocker;

    pub trait MockFunc:
        Fn(GetVcExportTaskReq) -> Result<(GetVcExportTaskResp, CommonResponse), Error>
        + Send
        + Sync
        + 'static
    {
    }
    impl<
            T: Fn(GetVcExportTaskReq) -> Result<(GetVcExportTaskResp, CommonResponse), Error>
                + Send
                + Sync
                + 'static,
        > MockFunc for T
    {
    }

    impl<'c, IStore: Store, IClient: HttpClient> VcServiceMocker<'c, IStore, IClient> {
        pub fn mock_get_vc_export_task<F: MockFunc>(
            &self,
            f: F,
        ) -> MockerBuilder<Mocker, GetVcExportTaskReq, GetVcExportTaskResp, Arc<dyn MockFunc>>
        {
            MockerBuilder::new(self.cli.instance_id, Arc::new(f))
        }

        pub(super) fn get_mock_get_vc_export_task(
            &self,
            req: &GetVcExportTaskReq,
        ) -> Option<Arc<dyn MockFunc>> {
            do_mock::<Mocker, GetVcExportTaskReq, GetVcExportTaskResp, Arc<dyn MockFunc>>(
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
        api::gen::vc::get_vc_export_task::{
            GetVcExportTaskReq, GetVcExportTaskResp, GetVcExportTaskRespInner,
        },
        core::{model::CommonResponse, Lark},
    };

    #[tokio::test]
    async fn test_mock() {
        let lark = Lark::new("".to_owned(), "".into());
        let mocker = lark
            .vc()
            .mock()
            .mock_get_vc_export_task(|_| {
                Ok((GetVcExportTaskResp::default(), CommonResponse::default()))
            })
            .build();
        let res = lark
            .vc()
            .get_vc_export_task(GetVcExportTaskReq::default())
            .await;
        assert!(res.is_ok());
        mocker.clear();
        let res = lark
            .vc()
            .get_vc_export_task(GetVcExportTaskReq::default())
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
        "status": 3,
        "url": "https://lf1-ttcdn-tos.pstatp.com/obj/xxx",
        "file_token": "6yHu7Igp7Igy62Ez6fLr6IJz7j9i5WMe6fHq5yZeY2Jz6yLqYAMAY46fZfEz64Lr5fYyYQ==",
        "fail_msg": "no permission"
    }
}"#;
    #[test]
    fn test_resp() {
        let res = serde_json::from_str::<GetVcExportTaskRespInner>(RESP);
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
