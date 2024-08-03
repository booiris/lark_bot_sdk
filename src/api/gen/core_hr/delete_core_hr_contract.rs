//! doc url: <https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/corehr-v1/contract/delete>
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
    /// **api 版本: 2024-07-23T06:24:19+00:00**
    ///
    /// ## 删除合同
    ///
    /// 通过本接口可以删除合同数据，删除后即无法查询到。
    ///

    ///
    /// doc: <https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/corehr-v1/contract/delete>
    ///
    /// new doc: <https://open.feishu.cn/document/server-docs/corehr-v1/contract/delete>
    ///
    /// api url: <https://open.feishu.cn/document_portal/v1/document/get_detail?fullPath=%2Fserver-docs%2Fcorehr-v1%2Fcontract%2Fdelete>
    pub async fn delete_core_hr_contract(
        &self,
        req: DeleteCoreHrContractReq,
    ) -> Result<(DeleteCoreHrContractResp, CommonResponse), Error> {
        self.delete_core_hr_contract_with_opt(req, Default::default())
            .await
    }

    /// 参见 [delete_core_hr_contract](#method.delete_core_hr_contract) 函数
    pub async fn delete_core_hr_contract_with_opt(
        &self,
        req: DeleteCoreHrContractReq,
        method_option: MethodOption,
    ) -> Result<(DeleteCoreHrContractResp, CommonResponse), Error> {
        #[cfg(feature = "test-util")]
        {
            if let Some(f) = self.mock().get_mock_delete_core_hr_contract(&req) {
                tracing::info!("[lark] CoreHr#DeleteCoreHrContract **mocking** api");
                return f(req);
            }
        }

        tracing::info!("[lark] CoreHr#DeleteCoreHrContract call api");

        let req = ApiRequest {
            scope: "CoreHr",
            api: "DeleteCoreHrContract",
            method: http::Method::DELETE,
            url: String::new()
                + self.cli.open_base_url.as_ref()
                + "/open-apis/corehr/v1/contracts/:contract_id",
            param_data: req.gen_param(),
            method_option,
            need_tenant_access_token: true,
            ..Default::default()
        };

        let (resp, common_resp): (DeleteCoreHrContractRespInner, _) = self.cli.do_req(req).await?;
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
pub struct DeleteCoreHrContractReq {
    /// 需要删除的合同 ID，该ID可以通过[【批量查询合同】](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/corehr-v1/contract/list)接口获取
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "7091849027838838316"
    #[api(kind = "path", name = "contract_id")]
    pub contract_id: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, lark_bot_sdk_macros::ApiBaseResp)]
struct DeleteCoreHrContractRespInner {
    #[serde(flatten)]
    data: Option<DeleteCoreHrContractResp>,
    #[serde(flatten)]
    base: BaseResp,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct DeleteCoreHrContractResp {
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
        Fn(DeleteCoreHrContractReq) -> Result<(DeleteCoreHrContractResp, CommonResponse), Error>
        + Send
        + Sync
        + 'static
    {
    }
    impl<
            T: Fn(
                    DeleteCoreHrContractReq,
                ) -> Result<(DeleteCoreHrContractResp, CommonResponse), Error>
                + Send
                + Sync
                + 'static,
        > MockFunc for T
    {
    }

    impl<'c, IStore: Store, IClient: HttpClient> CoreHrServiceMocker<'c, IStore, IClient> {
        pub fn mock_delete_core_hr_contract<F: MockFunc>(
            &self,
            f: F,
        ) -> MockerBuilder<
            Mocker,
            DeleteCoreHrContractReq,
            DeleteCoreHrContractResp,
            Arc<dyn MockFunc>,
        > {
            MockerBuilder::new(self.cli.instance_id, Arc::new(f))
        }

        pub(super) fn get_mock_delete_core_hr_contract(
            &self,
            req: &DeleteCoreHrContractReq,
        ) -> Option<Arc<dyn MockFunc>> {
            do_mock::<Mocker, DeleteCoreHrContractReq, DeleteCoreHrContractResp, Arc<dyn MockFunc>>(
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
        api::gen::core_hr::delete_core_hr_contract::{
            DeleteCoreHrContractReq, DeleteCoreHrContractResp, DeleteCoreHrContractRespInner,
        },
        core::{model::CommonResponse, Lark},
    };

    #[tokio::test]
    async fn test_mock() {
        let lark = Lark::new("".to_owned(), "".into());
        let mocker = lark
            .core_hr()
            .mock()
            .mock_delete_core_hr_contract(|_| {
                Ok((
                    DeleteCoreHrContractResp::default(),
                    CommonResponse::default(),
                ))
            })
            .build();
        let res = lark
            .core_hr()
            .delete_core_hr_contract(DeleteCoreHrContractReq::default())
            .await;
        assert!(res.is_ok());
        mocker.clear();
        let res = lark
            .core_hr()
            .delete_core_hr_contract(DeleteCoreHrContractReq::default())
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
        let res = serde_json::from_str::<DeleteCoreHrContractRespInner>(RESP);
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
