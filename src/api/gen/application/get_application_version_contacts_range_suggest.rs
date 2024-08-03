//! doc url: <https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/application-v6/application-app_version/contacts_range_suggest>
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
    /// **api 版本: 2023-07-21T07:21:41+00:00**
    ///
    /// ## 获取应用版本中开发者申请的通讯录权限范围
    ///
    /// 该接口用于根据应用的 App ID 和版本 ID 获取企业自建应用某个版本的通讯录权限范围。
    ///
    /// 由于通讯录权限范围需要提交发布新的应用版本，并且企业管理员审核通过后才会生效，因此该权限范围可能与实际生效的权限范围有差别，如需获取线上实际生效的通讯录权限范围，可通过[获取应用通讯录权限范围配置](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/application-v6/application/contacts_range_configuration) 获取。
    ///
    /// doc: <https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/application-v6/application-app_version/contacts_range_suggest>
    ///
    /// new doc: <https://open.feishu.cn/document/server-docs/application-v6/application/contacts_range_suggest>
    ///
    /// api url: <https://open.feishu.cn/document_portal/v1/document/get_detail?fullPath=%2Fserver-docs%2Fapplication-v6%2Fapplication%2Fcontacts_range_suggest>
    pub async fn get_application_version_contacts_range_suggest(
        &self,
        req: GetApplicationVersionContactsRangeSuggestReq,
    ) -> Result<
        (
            GetApplicationVersionContactsRangeSuggestResp,
            CommonResponse,
        ),
        Error,
    > {
        self.get_application_version_contacts_range_suggest_with_opt(req, Default::default())
            .await
    }

    /// 参见 [get_application_version_contacts_range_suggest](#method.get_application_version_contacts_range_suggest) 函数
    pub async fn get_application_version_contacts_range_suggest_with_opt(
        &self,
        req: GetApplicationVersionContactsRangeSuggestReq,
        method_option: MethodOption,
    ) -> Result<
        (
            GetApplicationVersionContactsRangeSuggestResp,
            CommonResponse,
        ),
        Error,
    > {
        #[cfg(feature = "test-util")]
        {
            if let Some(f) = self
                .mock()
                .get_mock_get_application_version_contacts_range_suggest(&req)
            {
                tracing::info!(
                    "[lark] Application#GetApplicationVersionContactsRangeSuggest **mocking** api"
                );
                return f(req);
            }
        }

        tracing::info!("[lark] Application#GetApplicationVersionContactsRangeSuggest call api");

        let req = ApiRequest {
            scope: "Application",
            api: "GetApplicationVersionContactsRangeSuggest",
            method: http::Method::GET,
            url: String::new() + self.cli.open_base_url.as_ref() + "/open-apis/application/v6/applications/:app_id/app_versions/:version_id/contacts_range_suggest",
            param_data: req.gen_param(),
            method_option,
            need_tenant_access_token: true,
            ..Default::default()
        };

        let (resp, common_resp): (GetApplicationVersionContactsRangeSuggestRespInner, _) =
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
pub struct GetApplicationVersionContactsRangeSuggestReq {
    /// 应用的 AppID，可以在[开发者后台](https://open.feishu.cn/app) > **凭证与基础信息**页查看。<br>
    ///
    /// * 仅查询本应用信息时，可填应用自身App ID 或 `me`。
    ///
    /// <br>
    ///
    /// * 当值为其他应用的App ID时，必须申请以下权限：<md-perm name="admin:app.info:readonly" desc="获取应用信息" support_app_types="custom" tags="">获取应用信息</md-perm>
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "cli_9f3ca975326b501b"
    #[api(kind = "path", name = "app_id")]
    pub app_id: String,
    /// 唯一标识应用版本的 ID，可以调用[获取应用版本列表](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/application-v6/application-app_version/list)接口获取。
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "oav_d317f090b7258ad0372aa53963cda70d"
    #[api(kind = "path", name = "version_id")]
    pub version_id: String,
    /// 返回值的部门ID的类型
    ///
    /// **示例值**: "department_id"
    ///
    /// **可选值**:
    ///
    /// `department_id`: 以自定义department_id来标识部门
    ///
    /// `open_department_id`: 以open_department_id来标识部门
    #[api(
        kind = "query",
        name = "department_id_type",
        v_type = "var",
        option = "false"
    )]
    pub department_id_type: String,
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
}

#[derive(Debug, Serialize, Deserialize, Clone, lark_bot_sdk_macros::ApiBaseResp)]
struct GetApplicationVersionContactsRangeSuggestRespInner {
    #[serde(flatten)]
    data: Option<GetApplicationVersionContactsRangeSuggestResp>,
    #[serde(flatten)]
    base: BaseResp,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct GetApplicationVersionContactsRangeSuggestResp {
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
    /// 应用版本通讯录权限范围建议信息。开发者在提交该版本时如果修改了通讯录权限范围则返回申请的通讯录权限范围。不代表最终应用生效的通讯录权限范围。如果没有修改,则为空。【如果通讯录权限范围与应用可用范围保持一致，上次的配置也是如此，则认为没变化。】
    #[serde(
        rename = "contacts_range",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub contacts_range: ApplicationAppContactsRangeSubResp,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(default)]
pub struct ApplicationAppContactsRangeSubResp {
    /// 通讯录可见性类型
    ///
    /// **示例值**: "some"
    ///
    /// **可选值**:
    ///
    /// `equal_to_availability`: 与应用可用范围一致，可通过[获取应用在企业内的可用范围](https://open.feishu.cn/document/ukTMukTMukTM/uIjM3UjLyIzN14iMycTN)接口查询具体人员
    ///
    /// `some`: 部分成员，具体人员参见visible_list
    ///
    /// `all`: 全部成员
    #[serde(
        rename = "contacts_scope_type",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub contacts_scope_type: String,
    /// 通讯录权限范围的可用名单
    #[serde(
        rename = "visible_list",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub visible_list: AppVisibleListSubResp,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(default)]
pub struct AppVisibleListSubResp {
    /// 通讯录权限范围的可用成员id列表，按照user_id_type返回
    #[serde(
        rename = "open_ids",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub open_ids: Vec<String>,
    /// 通讯录权限范围的可用部门的 id 列表
    #[serde(
        rename = "department_ids",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub department_ids: Vec<String>,
    /// 通讯录权限范围的可用用户组 id 列表
    #[serde(
        rename = "group_ids",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub group_ids: Vec<String>,
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
            GetApplicationVersionContactsRangeSuggestReq,
        ) -> Result<
            (
                GetApplicationVersionContactsRangeSuggestResp,
                CommonResponse,
            ),
            Error,
        > + Send
        + Sync
        + 'static
    {
    }
    impl<
            T: Fn(
                    GetApplicationVersionContactsRangeSuggestReq,
                ) -> Result<
                    (
                        GetApplicationVersionContactsRangeSuggestResp,
                        CommonResponse,
                    ),
                    Error,
                > + Send
                + Sync
                + 'static,
        > MockFunc for T
    {
    }

    impl<'c, IStore: Store, IClient: HttpClient> ApplicationServiceMocker<'c, IStore, IClient> {
        pub fn mock_get_application_version_contacts_range_suggest<F: MockFunc>(
            &self,
            f: F,
        ) -> MockerBuilder<
            Mocker,
            GetApplicationVersionContactsRangeSuggestReq,
            GetApplicationVersionContactsRangeSuggestResp,
            Arc<dyn MockFunc>,
        > {
            MockerBuilder::new(self.cli.instance_id, Arc::new(f))
        }

        pub(super) fn get_mock_get_application_version_contacts_range_suggest(
            &self,
            req: &GetApplicationVersionContactsRangeSuggestReq,
        ) -> Option<Arc<dyn MockFunc>> {
            do_mock::<
                Mocker,
                GetApplicationVersionContactsRangeSuggestReq,
                GetApplicationVersionContactsRangeSuggestResp,
                Arc<dyn MockFunc>,
            >(self.cli.instance_id, req)
        }
    }
}

#[cfg(feature = "test-util")]
#[cfg(test)]
mod test {
    use crate::{
        api::gen::application::get_application_version_contacts_range_suggest::{
            GetApplicationVersionContactsRangeSuggestReq,
            GetApplicationVersionContactsRangeSuggestResp,
            GetApplicationVersionContactsRangeSuggestRespInner,
        },
        core::{model::CommonResponse, Lark},
    };

    #[tokio::test]
    async fn test_mock() {
        let lark = Lark::new("".to_owned(), "".into());
        let mocker = lark
            .application()
            .mock()
            .mock_get_application_version_contacts_range_suggest(|_| {
                Ok((
                    GetApplicationVersionContactsRangeSuggestResp::default(),
                    CommonResponse::default(),
                ))
            })
            .build();
        let res = lark
            .application()
            .get_application_version_contacts_range_suggest(
                GetApplicationVersionContactsRangeSuggestReq::default(),
            )
            .await;
        assert!(res.is_ok());
        mocker.clear();
        let res = lark
            .application()
            .get_application_version_contacts_range_suggest(
                GetApplicationVersionContactsRangeSuggestReq::default(),
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
    "data": {
        "contacts_range": {
            "contacts_scope_type": "some",
            "visible_list": {
                "open_ids": [
                    "ou_4065981088f8ef67a504ba8bd6b24d85"
                ],
                "department_ids": [
                    "od-4b4a6907ad726ea07b27b0d2882b7c65"
                ],
                "group_ids": [
                    "b6d1g5dd6fd26186"
                ]
            }
        }
    }
}"#;
    #[test]
    fn test_resp() {
        let res = serde_json::from_str::<GetApplicationVersionContactsRangeSuggestRespInner>(RESP);
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
