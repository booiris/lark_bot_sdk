//! doc url: <https://open.larkoffice.com/document/uAjLw4CM/ukTMukTMukTM/application-v6/app_badge/set>
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
    /// **api 版本: 2024-07-23T07:32:42+00:00**
    ///
    /// ## 更新应用红点
    ///
    /// 更新应用红点信息，用于工作台场景
    ///

    ///
    /// doc: <https://open.larkoffice.com/document/uAjLw4CM/ukTMukTMukTM/application-v6/app_badge/set>
    ///
    /// new doc: <https://open.larkoffice.com/document/server-docs/application-v6/app_badge/set>
    ///
    /// api url: <https://open.feishu.cn/document_portal/v1/document/get_detail?fullPath=%2Fserver-docs%2Fapplication-v6%2Fapp_badge%2Fset>
    pub async fn set_application_app_badge(
        &self,
        req: SetApplicationAppBadgeReq,
    ) -> Result<(SetApplicationAppBadgeResp, CommonResponse), Error> {
        self.set_application_app_badge_with_opt(req, Default::default())
            .await
    }

    /// 参见 [set_application_app_badge](#method.set_application_app_badge) 函数
    pub async fn set_application_app_badge_with_opt(
        &self,
        req: SetApplicationAppBadgeReq,
        method_option: MethodOption,
    ) -> Result<(SetApplicationAppBadgeResp, CommonResponse), Error> {
        #[cfg(feature = "test-util")]
        {
            if let Some(f) = self.mock().get_mock_set_application_app_badge(&req) {
                tracing::info!("[lark] Application#SetApplicationAppBadge **mocking** api");
                return f(req);
            }
        }

        tracing::info!("[lark] Application#SetApplicationAppBadge call api");

        let req = ApiRequest {
            scope: "Application",
            api: "SetApplicationAppBadge",
            method: http::Method::POST,
            url: String::new()
                + self.cli.open_base_url.as_ref()
                + "/open-apis/application/v6/app_badge/set",
            param_data: req.gen_param(),
            method_option,
            need_tenant_access_token: true,
            ..Default::default()
        };

        let (resp, common_resp): (SetApplicationAppBadgeRespInner, _) =
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
pub struct SetApplicationAppBadgeReq {
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
    /// 用户ID
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "ou_d317f090b7258ad0372aa53963cda70d"
    #[api(kind = "body", name = "user_id")]
    pub user_id: String,
    /// badge数据版本号
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "1664360599355"
    #[api(kind = "body", name = "version")]
    pub version: String,
    /// badge extra 信息
    ///
    /// **示例值**: "{}"
    #[api(kind = "body", name = "extra")]
    pub extra: Option<String>,
    /// pc端badge数量
    #[api(kind = "body", name = "pc")]
    pub pc: Option<ClientBadgeNumSubReq>,
    /// 移动端badge数量
    #[api(kind = "body", name = "mobile")]
    pub mobile: Option<ClientBadgeNumSubReq>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(default)]
pub struct ClientBadgeNumSubReq {
    /// h5能力的badge数量
    ///
    /// **示例值**: "1"
    #[serde(
        rename = "web_app",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub web_app: Option<i64>,
    /// 小程序能力的badge数量
    ///
    /// **示例值**: "2"
    #[serde(
        rename = "gadget",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub gadget: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize, Clone, lark_bot_sdk_macros::ApiBaseResp)]
struct SetApplicationAppBadgeRespInner {
    #[serde(flatten)]
    data: Option<SetApplicationAppBadgeResp>,
    #[serde(flatten)]
    base: BaseResp,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct SetApplicationAppBadgeResp {
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
        Fn(SetApplicationAppBadgeReq) -> Result<(SetApplicationAppBadgeResp, CommonResponse), Error>
        + Send
        + Sync
        + 'static
    {
    }
    impl<
            T: Fn(
                    SetApplicationAppBadgeReq,
                ) -> Result<(SetApplicationAppBadgeResp, CommonResponse), Error>
                + Send
                + Sync
                + 'static,
        > MockFunc for T
    {
    }

    impl<'c, IStore: Store, IClient: HttpClient> ApplicationServiceMocker<'c, IStore, IClient> {
        pub fn mock_set_application_app_badge<F: MockFunc>(
            &self,
            f: F,
        ) -> MockerBuilder<
            Mocker,
            SetApplicationAppBadgeReq,
            SetApplicationAppBadgeResp,
            Arc<dyn MockFunc>,
        > {
            MockerBuilder::new(self.cli.instance_id, Arc::new(f))
        }

        pub(super) fn get_mock_set_application_app_badge(
            &self,
            req: &SetApplicationAppBadgeReq,
        ) -> Option<Arc<dyn MockFunc>> {
            do_mock::<
                Mocker,
                SetApplicationAppBadgeReq,
                SetApplicationAppBadgeResp,
                Arc<dyn MockFunc>,
            >(self.cli.instance_id, req)
        }
    }
}

#[cfg(feature = "test-util")]
#[cfg(test)]
mod test {
    use crate::{
        api::gen::application::set_application_app_badge::{
            SetApplicationAppBadgeReq, SetApplicationAppBadgeResp, SetApplicationAppBadgeRespInner,
        },
        core::{model::CommonResponse, Lark},
    };

    #[tokio::test]
    async fn test_mock() {
        let lark = Lark::new("".to_owned(), "".into());
        let mocker = lark
            .application()
            .mock()
            .mock_set_application_app_badge(|_| {
                Ok((
                    SetApplicationAppBadgeResp::default(),
                    CommonResponse::default(),
                ))
            })
            .build();
        let res = lark
            .application()
            .set_application_app_badge(SetApplicationAppBadgeReq::default())
            .await;
        assert!(res.is_ok());
        mocker.clear();
        let res = lark
            .application()
            .set_application_app_badge(SetApplicationAppBadgeReq::default())
            .await;
        assert!(res.is_err());
    }

    const REQ: &str = r#"{
    "user_id": "ou_d317f090b7258ad0372aa53963cda70d",
    "version": "1664360599355",
    "extra": "{}",
    "pc": {
        "web_app": 1,
        "gadget": 2
    },
    "mobile": {
        "web_app": 1,
        "gadget": 2
    }
}"#;

    #[test]
    fn test_req() {
        if REQ == "{}" {
            return;
        }
        if let Err(e) = serde_json::from_str::<super::SetApplicationAppBadgeReqBody>(REQ) {
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
        let res = serde_json::from_str::<SetApplicationAppBadgeRespInner>(RESP);
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
