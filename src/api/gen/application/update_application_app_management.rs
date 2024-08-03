//! doc url: <https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/application-v6/application-management/update>
// Code generated by gen_api. DO NOT EDIT.

use serde::{Deserialize, Serialize};

use crate::api::BaseResp;
use crate::api::HasBaseResp;
use crate::core::model::{ApiRequest, CommonResponse, MethodOption, ReqParam, StreamReqParam};
use crate::{
    core::{http_client::HttpClient, store::Store},
    error::Error,
};

use crate::api::gen::application::ApplicationService;

impl<'c, IStore: Store, IClient: HttpClient> ApplicationService<'c, IStore, IClient> {
    /// **api 版本: 2024-01-26T11:11:44+00:00**
    ///
    /// ## 启停用应用
    ///
    /// 可停用或启用企业内已安装的自建应用与商店应用。
    ///

    ///
    /// doc: <https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/application-v6/application-management/update>
    ///
    /// new doc: <https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/application-v6/application-management/update>
    ///
    /// api url: <https://open.feishu.cn/document_portal/v1/document/get_detail?fullPath=%2FuAjLw4CM%2FukTMukTMukTM%2Fapplication-v6%2Fapplication-management%2Fupdate>
    pub async fn update_application_app_management(
        &self,
        req: UpdateApplicationAppManagementReq,
    ) -> Result<(UpdateApplicationAppManagementResp, CommonResponse), Error> {
        self.update_application_app_management_with_opt(req, Default::default())
            .await
    }

    /// 参见 [update_application_app_management](#method.update_application_app_management) 函数
    pub async fn update_application_app_management_with_opt(
        &self,
        req: UpdateApplicationAppManagementReq,
        method_option: MethodOption,
    ) -> Result<(UpdateApplicationAppManagementResp, CommonResponse), Error> {
        #[cfg(feature = "test-util")]
        {
            if let Some(f) = self.mock().get_mock_update_application_app_management(&req) {
                tracing::info!("[lark] Application#UpdateApplicationAppManagement **mocking** api");
                return f(req);
            }
        }

        tracing::info!("[lark] Application#UpdateApplicationAppManagement call api");

        let req = ApiRequest {
            scope: "Application",
            api: "UpdateApplicationAppManagement",
            method: http::Method::PUT,
            url: String::new()
                + self.cli.open_base_url.as_ref()
                + "/open-apis/application/v6/applications/:app_id/management",
            param_data: req.gen_param(),
            method_option,
            need_tenant_access_token: true,
            ..Default::default()
        };

        let (resp, common_resp): (UpdateApplicationAppManagementRespInner, _) =
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
pub struct UpdateApplicationAppManagementReq {
    /// 应用ID
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "cli_a4517c8461f8100a"
    #[api(kind = "path", name = "app_id")]
    pub app_id: String,

    /// 启用/停用应用
    ///
    /// **示例值**: "true"
    #[api(kind = "body", name = "enable")]
    pub enable: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize, Clone, lark_bot_sdk_macros::ApiBaseResp)]
struct UpdateApplicationAppManagementRespInner {
    #[serde(flatten)]
    data: Option<UpdateApplicationAppManagementResp>,
    #[serde(flatten)]
    base: BaseResp,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct UpdateApplicationAppManagementResp {
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

    use self::gen::application::ApplicationServiceMocker;
    use crate::core::mocker::*;

    use super::*;
    use crate::api::*;
    use crate::core::model::*;

    pub struct Mocker;

    pub trait MockFunc:
        Fn(
            UpdateApplicationAppManagementReq,
        ) -> Result<(UpdateApplicationAppManagementResp, CommonResponse), Error>
        + Send
        + Sync
        + 'static
    {
    }
    impl<
            T: Fn(
                    UpdateApplicationAppManagementReq,
                )
                    -> Result<(UpdateApplicationAppManagementResp, CommonResponse), Error>
                + Send
                + Sync
                + 'static,
        > MockFunc for T
    {
    }

    impl<'c, IStore: Store, IClient: HttpClient> ApplicationServiceMocker<'c, IStore, IClient> {
        pub fn mock_update_application_app_management<F: MockFunc>(
            &self,
            f: F,
        ) -> MockerBuilder<
            Mocker,
            UpdateApplicationAppManagementReq,
            UpdateApplicationAppManagementResp,
            Arc<dyn MockFunc>,
        > {
            MockerBuilder::new(self.cli.instance_id, Arc::new(f))
        }

        pub(super) fn get_mock_update_application_app_management(
            &self,
            req: &UpdateApplicationAppManagementReq,
        ) -> Option<Arc<dyn MockFunc>> {
            do_mock::<
                Mocker,
                UpdateApplicationAppManagementReq,
                UpdateApplicationAppManagementResp,
                Arc<dyn MockFunc>,
            >(self.cli.instance_id, req)
        }
    }
}

#[cfg(feature = "test-util")]
#[cfg(test)]
mod test {
    use crate::{
        api::gen::application::update_application_app_management::{
            UpdateApplicationAppManagementReq, UpdateApplicationAppManagementResp,
            UpdateApplicationAppManagementRespInner,
        },
        core::{model::CommonResponse, Lark},
    };

    #[tokio::test]
    async fn test_mock() {
        let lark = Lark::new("".to_owned(), "".into());
        let mocker = lark
            .application()
            .mock()
            .mock_update_application_app_management(|_| {
                Ok((
                    UpdateApplicationAppManagementResp::default(),
                    CommonResponse::default(),
                ))
            })
            .build();
        let res = lark
            .application()
            .update_application_app_management(UpdateApplicationAppManagementReq::default())
            .await;
        assert!(res.is_ok());
        mocker.clear();
        let res = lark
            .application()
            .update_application_app_management(UpdateApplicationAppManagementReq::default())
            .await;
        assert!(res.is_err());
    }

    const REQ: &str = r#"{
    "enable": true
}"#;

    #[test]
    fn test_req() {
        if REQ == "{}" {
            return;
        }
        if let Err(e) = serde_json::from_str::<super::UpdateApplicationAppManagementReqBody>(REQ) {
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
        let res = serde_json::from_str::<UpdateApplicationAppManagementRespInner>(RESP);
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
