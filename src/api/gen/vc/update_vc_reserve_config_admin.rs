//! doc url: <https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/vc-v1/reserve_config-admin/patch>
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
    /// **api 版本: 2024-07-23T07:33:01+00:00**
    ///
    /// ## 更新会议室预定管理员
    ///
    /// 更新会议室预定管理员。
    ///

    ///
    /// doc: <https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/vc-v1/reserve_config-admin/patch>
    ///
    /// new doc: <https://open.feishu.cn/document/server-docs/vc-v1/scope_config/patch-3>
    ///
    /// api url: <https://open.feishu.cn/document_portal/v1/document/get_detail?fullPath=%2Fserver-docs%2Fvc-v1%2Fscope_config%2Fpatch-3>
    pub async fn update_vc_reserve_config_admin(
        &self,
        req: UpdateVcReserveConfigAdminReq,
    ) -> Result<(UpdateVcReserveConfigAdminResp, CommonResponse), Error> {
        self.update_vc_reserve_config_admin_with_opt(req, Default::default())
            .await
    }

    /// 参见 [update_vc_reserve_config_admin](#method.update_vc_reserve_config_admin) 函数
    pub async fn update_vc_reserve_config_admin_with_opt(
        &self,
        req: UpdateVcReserveConfigAdminReq,
        method_option: MethodOption,
    ) -> Result<(UpdateVcReserveConfigAdminResp, CommonResponse), Error> {
        #[cfg(feature = "test-util")]
        {
            if let Some(f) = self.mock().get_mock_update_vc_reserve_config_admin(&req) {
                tracing::info!("[lark] Vc#UpdateVcReserveConfigAdmin **mocking** api");
                return f(req);
            }
        }

        tracing::info!("[lark] Vc#UpdateVcReserveConfigAdmin call api");

        let req = ApiRequest {
            scope: "Vc",
            api: "UpdateVcReserveConfigAdmin",
            method: http::Method::PATCH,
            url: String::new()
                + self.cli.open_base_url.as_ref()
                + "/open-apis/vc/v1/reserve_configs/:reserve_config_id/admin",
            param_data: req.gen_param(),
            method_option,
            need_tenant_access_token: true,
            ..Default::default()
        };

        let (resp, common_resp): (UpdateVcReserveConfigAdminRespInner, _) =
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
pub struct UpdateVcReserveConfigAdminReq {
    /// 会议室或层级id
    ///
    /// **示例值**: "omm_3c5dd7e09bac0c1758fcf9511bd1a771"
    #[api(kind = "path", name = "reserve_config_id")]
    pub reserve_config_id: String,
    /// 用户 ID 类型
    ///
    /// **示例值**: "open_id"
    ///
    /// **可选值**:
    ///
    /// `open_id`: 标识一个用户在某个应用中的身份。同一个用户在不同应用中的 Open ID 不同。[了解更多：如何获取 Open ID](https://open.feishu.cn/document/uAjLw4CM/ugTN1YjL4UTN24CO1UjN/trouble-shooting/how-to-obtain-openid)
    ///
    /// `union_id`: 标识一个用户在某个应用开发商下的身份。同一用户在同一开发商下的应用中的 Union ID 是相同的，在不同开发商下的应用中的 Union ID 是不同的。通过 Union ID，应用开发商可以把同个用户在多个应用中的身份关联起来。[了解更多：如何获取 Union ID？](https://open.feishu.cn/document/uAjLw4CM/ugTN1YjL4UTN24CO1UjN/trouble-shooting/how-to-obtain-union-id)
    ///
    /// `user_id`: 标识一个用户在某个租户内的身份。同一个用户在租户 A 和租户 B 内的 User ID 是不同的。在同一个租户内，一个用户的 User ID 在所有应用（包括商店应用）中都保持一致。User ID 主要用于在不同的应用间打通用户数据。[了解更多：如何获取 User ID？](https://open.feishu.cn/document/uAjLw4CM/ugTN1YjL4UTN24CO1UjN/trouble-shooting/how-to-obtain-user-id)
    #[api(
        kind = "query",
        name = "user_id_type",
        v_type = "var",
        option = "false"
    )]
    pub user_id_type: String,
    /// 1代表层级，2代表会议室
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "2"
    #[api(kind = "body", name = "scope_type")]
    pub scope_type: i64,
    /// 预定管理员或部门
    ///
    /// **是否必填**: 是
    #[api(kind = "body", name = "reserve_admin_config")]
    pub reserve_admin_config: ReserveAdminConfigSubReq,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(default)]
pub struct ReserveAdminConfigSubReq {
    /// 预定管理部门
    #[serde(
        rename = "depts",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub depts: Vec<Option<SubscribeDepartmentSubReq>>,
    /// 预定管理员
    #[serde(
        rename = "users",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub users: Vec<Option<SubscribeUserSubReq>>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(default)]
pub struct SubscribeDepartmentSubReq {
    /// 预定管理部门ID，使用open_department_id
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "od-47d8b570b0a011e9679a755efcc5f61a"
    #[serde(
        rename = "department_id",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub department_id: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(default)]
pub struct SubscribeUserSubReq {
    /// 预定管理员ID
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "ou_a27b07a9071d90577c0177bcec98f856"
    #[serde(
        rename = "user_id",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub user_id: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, lark_bot_sdk_macros::ApiBaseResp)]
struct UpdateVcReserveConfigAdminRespInner {
    #[serde(flatten)]
    data: Option<UpdateVcReserveConfigAdminResp>,
    #[serde(flatten)]
    base: BaseResp,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct UpdateVcReserveConfigAdminResp {
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

    use self::gen::vc::VcServiceMocker;
    use crate::core::mocker::*;

    use super::*;
    use crate::api::*;
    use crate::core::model::*;

    pub struct Mocker;

    pub trait MockFunc:
        Fn(
            UpdateVcReserveConfigAdminReq,
        ) -> Result<(UpdateVcReserveConfigAdminResp, CommonResponse), Error>
        + Send
        + Sync
        + 'static
    {
    }
    impl<
            T: Fn(
                    UpdateVcReserveConfigAdminReq,
                )
                    -> Result<(UpdateVcReserveConfigAdminResp, CommonResponse), Error>
                + Send
                + Sync
                + 'static,
        > MockFunc for T
    {
    }

    impl<'c, IStore: Store, IClient: HttpClient> VcServiceMocker<'c, IStore, IClient> {
        pub fn mock_update_vc_reserve_config_admin<F: MockFunc>(
            &self,
            f: F,
        ) -> MockerBuilder<
            Mocker,
            UpdateVcReserveConfigAdminReq,
            UpdateVcReserveConfigAdminResp,
            Arc<dyn MockFunc>,
        > {
            MockerBuilder::new(self.cli.instance_id, Arc::new(f))
        }

        pub(super) fn get_mock_update_vc_reserve_config_admin(
            &self,
            req: &UpdateVcReserveConfigAdminReq,
        ) -> Option<Arc<dyn MockFunc>> {
            do_mock::<
                Mocker,
                UpdateVcReserveConfigAdminReq,
                UpdateVcReserveConfigAdminResp,
                Arc<dyn MockFunc>,
            >(self.cli.instance_id, req)
        }
    }
}

#[cfg(feature = "test-util")]
#[cfg(test)]
mod test {
    use crate::{
        api::gen::vc::update_vc_reserve_config_admin::{
            UpdateVcReserveConfigAdminReq, UpdateVcReserveConfigAdminResp,
            UpdateVcReserveConfigAdminRespInner,
        },
        core::{model::CommonResponse, Lark},
    };

    #[tokio::test]
    async fn test_mock() {
        let lark = Lark::new("".to_owned(), "".into());
        let mocker = lark
            .vc()
            .mock()
            .mock_update_vc_reserve_config_admin(|_| {
                Ok((
                    UpdateVcReserveConfigAdminResp::default(),
                    CommonResponse::default(),
                ))
            })
            .build();
        let res = lark
            .vc()
            .update_vc_reserve_config_admin(UpdateVcReserveConfigAdminReq::default())
            .await;
        assert!(res.is_ok());
        mocker.clear();
        let res = lark
            .vc()
            .update_vc_reserve_config_admin(UpdateVcReserveConfigAdminReq::default())
            .await;
        assert!(res.is_err());
    }

    const REQ: &str = r#"{
    "scope_type": 2,
    "reserve_admin_config": {
        "depts": [
            {
                "department_id": "od-47d8b570b0a011e9679a755efcc5f61a"
            }
        ],
        "users": [
            {
                "user_id": "ou_a27b07a9071d90577c0177bcec98f856"
            }
        ]
    }
}"#;

    #[test]
    fn test_req() {
        if REQ == "{}" {
            return;
        }
        if let Err(e) = serde_json::from_str::<super::UpdateVcReserveConfigAdminReqBody>(REQ) {
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
        let res = serde_json::from_str::<UpdateVcReserveConfigAdminRespInner>(RESP);
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
