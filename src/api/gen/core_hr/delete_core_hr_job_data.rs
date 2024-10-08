//! doc url: <https://open.larkoffice.com/document/uAjLw4CM/ukTMukTMukTM/reference/corehr-v1/job_data/delete>
// Code generated by gen_api. DO NOT EDIT.

use serde::{Deserialize, Serialize};

use crate::api::BaseResp;
use crate::api::HasBaseResp;
use crate::core::model::{ApiRequest, CommonResponse, MethodOption, ReqParam, StreamReqParam};
use crate::{
    core::{http_client::HttpClient, store::Store},
    error::Error,
};

use crate::api::gen::core_hr::CoreHrService;

impl<'c, IStore: Store, IClient: HttpClient> CoreHrService<'c, IStore, IClient> {
    /// **api 版本: 2024-02-06T02:20:58+00:00**
    ///
    /// ## 删除任职信息
    ///
    /// 删除人员的任职信息。
    ///

    ///
    /// doc: <https://open.larkoffice.com/document/uAjLw4CM/ukTMukTMukTM/reference/corehr-v1/job_data/delete>
    ///
    /// new doc: <https://open.larkoffice.com/document/server-docs/corehr-v1/employee/job_data/delete>
    ///
    /// api url: <https://open.feishu.cn/document_portal/v1/document/get_detail?fullPath=%2Fserver-docs%2Fcorehr-v1%2Femployee%2Fjob_data%2Fdelete>
    pub async fn delete_core_hr_job_data(
        &self,
        req: DeleteCoreHrJobDataReq,
    ) -> Result<(DeleteCoreHrJobDataResp, CommonResponse), Error> {
        self.delete_core_hr_job_data_with_opt(req, Default::default())
            .await
    }

    /// 参见 [delete_core_hr_job_data](#method.delete_core_hr_job_data) 函数
    pub async fn delete_core_hr_job_data_with_opt(
        &self,
        req: DeleteCoreHrJobDataReq,
        method_option: MethodOption,
    ) -> Result<(DeleteCoreHrJobDataResp, CommonResponse), Error> {
        #[cfg(feature = "test-util")]
        {
            if let Some(f) = self.mock().get_mock_delete_core_hr_job_data(&req) {
                tracing::info!("[lark] CoreHr#DeleteCoreHrJobData **mocking** api");
                return f(req);
            }
        }

        tracing::info!("[lark] CoreHr#DeleteCoreHrJobData call api");

        let req = ApiRequest {
            scope: "CoreHr",
            api: "DeleteCoreHrJobData",
            method: http::Method::DELETE,
            url: String::new()
                + self.cli.open_base_url.as_ref()
                + "/open-apis/corehr/v1/job_datas/:job_data_id",
            param_data: req.gen_param(),
            method_option,
            need_tenant_access_token: true,
            ..Default::default()
        };

        let (resp, common_resp): (DeleteCoreHrJobDataRespInner, _) = self.cli.do_req(req).await?;
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
pub struct DeleteCoreHrJobDataReq {
    /// 需要删除的任职信息 ID
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "467642764726472"
    #[api(kind = "path", name = "job_data_id")]
    pub job_data_id: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, lark_bot_sdk_macros::ApiBaseResp)]
struct DeleteCoreHrJobDataRespInner {
    #[serde(flatten)]
    data: Option<DeleteCoreHrJobDataResp>,
    #[serde(flatten)]
    base: BaseResp,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct DeleteCoreHrJobDataResp {
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

    use self::gen::core_hr::CoreHrServiceMocker;
    use crate::core::mocker::*;

    use super::*;
    use crate::api::*;
    use crate::core::model::*;

    pub struct Mocker;

    pub trait MockFunc:
        Fn(DeleteCoreHrJobDataReq) -> Result<(DeleteCoreHrJobDataResp, CommonResponse), Error>
        + Send
        + Sync
        + 'static
    {
    }
    impl<
            T: Fn(
                    DeleteCoreHrJobDataReq,
                ) -> Result<(DeleteCoreHrJobDataResp, CommonResponse), Error>
                + Send
                + Sync
                + 'static,
        > MockFunc for T
    {
    }

    impl<'c, IStore: Store, IClient: HttpClient> CoreHrServiceMocker<'c, IStore, IClient> {
        pub fn mock_delete_core_hr_job_data<F: MockFunc>(
            &self,
            f: F,
        ) -> MockerBuilder<Mocker, DeleteCoreHrJobDataReq, DeleteCoreHrJobDataResp, Arc<dyn MockFunc>>
        {
            MockerBuilder::new(self.cli.instance_id, Arc::new(f))
        }

        pub(super) fn get_mock_delete_core_hr_job_data(
            &self,
            req: &DeleteCoreHrJobDataReq,
        ) -> Option<Arc<dyn MockFunc>> {
            do_mock::<Mocker, DeleteCoreHrJobDataReq, DeleteCoreHrJobDataResp, Arc<dyn MockFunc>>(
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
        api::gen::core_hr::delete_core_hr_job_data::{
            DeleteCoreHrJobDataReq, DeleteCoreHrJobDataResp, DeleteCoreHrJobDataRespInner,
        },
        core::{model::CommonResponse, Lark},
    };

    #[tokio::test]
    async fn test_mock() {
        let lark = Lark::new("".to_owned(), "".into());
        let mocker = lark
            .core_hr()
            .mock()
            .mock_delete_core_hr_job_data(|_| {
                Ok((
                    DeleteCoreHrJobDataResp::default(),
                    CommonResponse::default(),
                ))
            })
            .build();
        let res = lark
            .core_hr()
            .delete_core_hr_job_data(DeleteCoreHrJobDataReq::default())
            .await;
        assert!(res.is_ok());
        mocker.clear();
        let res = lark
            .core_hr()
            .delete_core_hr_job_data(DeleteCoreHrJobDataReq::default())
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
        let res = serde_json::from_str::<DeleteCoreHrJobDataRespInner>(RESP);
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
