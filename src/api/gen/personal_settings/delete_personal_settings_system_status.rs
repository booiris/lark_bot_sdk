//! doc url: <https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/personal_settings-v1/system_status/delete>
// Code generated by gen_api. DO NOT EDIT.

use serde::{Deserialize, Serialize};

use crate::api::BaseResp;
use crate::api::HasBaseResp;
use crate::core::model::{ApiRequest, CommonResponse, MethodOption, ReqParam, StreamReqParam};
use crate::{
    core::{http_client::HttpClient, store::Store},
    error::Error,
};

use crate::api::gen::personal_settings::PersonalSettingsService;

impl<'c, IStore: Store, IClient: HttpClient> PersonalSettingsService<'c, IStore, IClient> {
    /// **api 版本: 2023-07-12T03:57:29+00:00**
    ///
    /// ## 删除系统状态
    ///
    /// 删除租户维度的系统状态。
    ///
    /// 注意事项：
    ///
    /// - 操作的数据为租户维度数据，请小心操作。
    ///
    /// - 删除系统状态后，并不影响正在使用该状态用户下系统状态的客户端展示。
    ///
    /// doc: <https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/personal_settings-v1/system_status/delete>
    ///
    /// new doc: <https://open.feishu.cn/document/server-docs/personal_settings-v1/system_status/delete>
    ///
    /// api url: <https://open.feishu.cn/document_portal/v1/document/get_detail?fullPath=%2Fserver-docs%2Fpersonal_settings-v1%2Fsystem_status%2Fdelete>
    pub async fn delete_personal_settings_system_status(
        &self,
        req: DeletePersonalSettingsSystemStatusReq,
    ) -> Result<(DeletePersonalSettingsSystemStatusResp, CommonResponse), Error> {
        self.delete_personal_settings_system_status_with_opt(req, Default::default())
            .await
    }

    /// 参见 [delete_personal_settings_system_status](#method.delete_personal_settings_system_status) 函数
    pub async fn delete_personal_settings_system_status_with_opt(
        &self,
        req: DeletePersonalSettingsSystemStatusReq,
        method_option: MethodOption,
    ) -> Result<(DeletePersonalSettingsSystemStatusResp, CommonResponse), Error> {
        #[cfg(feature = "test-util")]
        {
            if let Some(f) = self
                .mock()
                .get_mock_delete_personal_settings_system_status(&req)
            {
                tracing::info!(
                    "[lark] PersonalSettings#DeletePersonalSettingsSystemStatus **mocking** api"
                );
                return f(req);
            }
        }

        tracing::info!("[lark] PersonalSettings#DeletePersonalSettingsSystemStatus call api");

        let req = ApiRequest {
            scope: "PersonalSettings",
            api: "DeletePersonalSettingsSystemStatus",
            method: http::Method::DELETE,
            url: String::new()
                + self.cli.open_base_url.as_ref()
                + "/open-apis/personal_settings/v1/system_statuses/:system_status_id",
            param_data: req.gen_param(),
            method_option,
            need_tenant_access_token: true,
            ..Default::default()
        };

        let (resp, common_resp): (DeletePersonalSettingsSystemStatusRespInner, _) =
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
pub struct DeletePersonalSettingsSystemStatusReq {
    /// 系统状态ID
    ///
    /// [获取系统状态ID](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/personal_settings-v1/system_status/list)
    ///
    /// **示例值**: "7101214603622940633"
    #[api(kind = "path", name = "system_status_id")]
    pub system_status_id: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, lark_bot_sdk_macros::ApiBaseResp)]
struct DeletePersonalSettingsSystemStatusRespInner {
    #[serde(flatten)]
    data: Option<DeletePersonalSettingsSystemStatusResp>,
    #[serde(flatten)]
    base: BaseResp,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct DeletePersonalSettingsSystemStatusResp {
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

    use self::gen::personal_settings::PersonalSettingsServiceMocker;
    use crate::core::mocker::*;

    use super::*;
    use crate::api::*;
    use crate::core::model::*;

    pub struct Mocker;

    pub trait MockFunc:
        Fn(
            DeletePersonalSettingsSystemStatusReq,
        ) -> Result<(DeletePersonalSettingsSystemStatusResp, CommonResponse), Error>
        + Send
        + Sync
        + 'static
    {
    }
    impl<
            T: Fn(
                    DeletePersonalSettingsSystemStatusReq,
                )
                    -> Result<(DeletePersonalSettingsSystemStatusResp, CommonResponse), Error>
                + Send
                + Sync
                + 'static,
        > MockFunc for T
    {
    }

    impl<'c, IStore: Store, IClient: HttpClient> PersonalSettingsServiceMocker<'c, IStore, IClient> {
        pub fn mock_delete_personal_settings_system_status<F: MockFunc>(
            &self,
            f: F,
        ) -> MockerBuilder<
            Mocker,
            DeletePersonalSettingsSystemStatusReq,
            DeletePersonalSettingsSystemStatusResp,
            Arc<dyn MockFunc>,
        > {
            MockerBuilder::new(self.cli.instance_id, Arc::new(f))
        }

        pub(super) fn get_mock_delete_personal_settings_system_status(
            &self,
            req: &DeletePersonalSettingsSystemStatusReq,
        ) -> Option<Arc<dyn MockFunc>> {
            do_mock::<
                Mocker,
                DeletePersonalSettingsSystemStatusReq,
                DeletePersonalSettingsSystemStatusResp,
                Arc<dyn MockFunc>,
            >(self.cli.instance_id, req)
        }
    }
}

#[cfg(feature = "test-util")]
#[cfg(test)]
mod test {
    use crate::{
        api::gen::personal_settings::delete_personal_settings_system_status::{
            DeletePersonalSettingsSystemStatusReq, DeletePersonalSettingsSystemStatusResp,
            DeletePersonalSettingsSystemStatusRespInner,
        },
        core::{model::CommonResponse, Lark},
    };

    #[tokio::test]
    async fn test_mock() {
        let lark = Lark::new("".to_owned(), "".into());
        let mocker = lark
            .personal_settings()
            .mock()
            .mock_delete_personal_settings_system_status(|_| {
                Ok((
                    DeletePersonalSettingsSystemStatusResp::default(),
                    CommonResponse::default(),
                ))
            })
            .build();
        let res =
            lark.personal_settings()
                .delete_personal_settings_system_status(
                    DeletePersonalSettingsSystemStatusReq::default(),
                )
                .await;
        assert!(res.is_ok());
        mocker.clear();
        let res =
            lark.personal_settings()
                .delete_personal_settings_system_status(
                    DeletePersonalSettingsSystemStatusReq::default(),
                )
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
        let res = serde_json::from_str::<DeletePersonalSettingsSystemStatusRespInner>(RESP);
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
