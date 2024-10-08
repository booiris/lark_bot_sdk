//! doc url: <https://open.larkoffice.com/document/uAjLw4CM/ukTMukTMukTM/reference/contact-v3/group/get>
// Code generated by gen_api. DO NOT EDIT.

use serde::{Deserialize, Serialize};

use crate::api::BaseResp;
use crate::api::HasBaseResp;
use crate::core::model::{ApiRequest, CommonResponse, MethodOption, ReqParam, StreamReqParam};
use crate::{
    core::{http_client::HttpClient, store::Store},
    error::Error,
};

use crate::api::gen::contact::ContactService;

impl<'c, IStore: Store, IClient: HttpClient> ContactService<'c, IStore, IClient> {
    /// **api 版本: 2024-07-05T08:08:03+00:00**
    ///
    /// ## 查询指定用户组
    ///
    /// 调用该接口通过用户组 ID 查询指定用户组的基本信息，包括用户组名称、成员数量和类型等。
    ///

    ///
    /// doc: <https://open.larkoffice.com/document/uAjLw4CM/ukTMukTMukTM/reference/contact-v3/group/get>
    ///
    /// new doc: <https://open.larkoffice.com/document/server-docs/contact-v3/group/get>
    ///
    /// api url: <https://open.feishu.cn/document_portal/v1/document/get_detail?fullPath=%2Fserver-docs%2Fcontact-v3%2Fgroup%2Fget>
    pub async fn get_contact_group(
        &self,
        req: GetContactGroupReq,
    ) -> Result<(GetContactGroupResp, CommonResponse), Error> {
        self.get_contact_group_with_opt(req, Default::default())
            .await
    }

    /// 参见 [get_contact_group](#method.get_contact_group) 函数
    pub async fn get_contact_group_with_opt(
        &self,
        req: GetContactGroupReq,
        method_option: MethodOption,
    ) -> Result<(GetContactGroupResp, CommonResponse), Error> {
        #[cfg(feature = "test-util")]
        {
            if let Some(f) = self.mock().get_mock_get_contact_group(&req) {
                tracing::info!("[lark] Contact#GetContactGroup **mocking** api");
                return f(req);
            }
        }

        tracing::info!("[lark] Contact#GetContactGroup call api");

        let req = ApiRequest {
            scope: "Contact",
            api: "GetContactGroup",
            method: http::Method::GET,
            url: String::new()
                + self.cli.open_base_url.as_ref()
                + "/open-apis/contact/v3/group/:group_id",
            param_data: req.gen_param(),
            method_option,
            need_tenant_access_token: true,
            ..Default::default()
        };

        let (resp, common_resp): (GetContactGroupRespInner, _) = self.cli.do_req(req).await?;
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
pub struct GetContactGroupReq {
    /// 用户组 ID。
    ///
    /// 用户组 ID 可在创建用户组时从返回值中获取，你也可以调用[查询用户组列表](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/contact-v3/group/simplelist)接口，获取用户组的 ID。
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "g193821"
    #[api(kind = "path", name = "group_id")]
    pub group_id: String,
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
    /// 此次调用中的部门 ID 类型。关于部门 ID 的详细介绍，可参见[部门 ID 说明](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/contact-v3/department/field-overview#23857fe0)。
    ///
    /// **示例值**: "open_department_id"
    ///
    /// **可选值**:
    ///
    /// `open_department_id`: 由系统自动生成的部门 ID，ID 前缀固定为 `od-`，在租户内全局唯一。
    ///
    /// `department_id`: 支持用户自定义配置的部门 ID。自定义配置时可复用已删除的 department_id，因此在未删除的部门范围内 department_id 具有唯一性。
    #[api(
        kind = "query",
        name = "department_id_type",
        v_type = "var",
        option = "false"
    )]
    pub department_id_type: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, lark_bot_sdk_macros::ApiBaseResp)]
struct GetContactGroupRespInner {
    #[serde(flatten)]
    data: Option<GetContactGroupResp>,
    #[serde(flatten)]
    base: BaseResp,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct GetContactGroupResp {
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
    /// 用户组详情。
    ///
    /// **是否必填**: 是
    #[serde(
        rename = "group",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub group: GroupSubResp,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(default)]
pub struct GroupSubResp {
    /// 用户组 ID。
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "g193821"
    #[serde(
        rename = "id",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub id: String,
    /// 用户组名字。
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "IT 外包组"
    #[serde(
        rename = "name",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub name: String,
    /// 用户组描述。
    ///
    /// **示例值**: "IT 外包组，需要对该组人群进行细颗粒度权限管控。"
    #[serde(
        rename = "description",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub description: String,
    /// 用户组成员中的用户数量。
    ///
    /// **示例值**: "2"
    #[serde(
        rename = "member_user_count",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub member_user_count: i64,
    /// 普通用户组内成员中的部门数量。
    ///
    /// **说明**：动态用户组成员中没有部门。
    ///
    /// **示例值**: "0"
    #[serde(
        rename = "member_department_count",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub member_department_count: i64,
    /// 用户组的类型。
    ///
    /// **示例值**: "1"
    ///
    /// **可选值**:
    ///
    /// `Assign`: 普通用户组
    ///
    /// `Dynamic`: 动态用户组
    #[serde(
        rename = "type",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub body_type: i64,
}

#[cfg(feature = "test-util")]
mod test_utils {
    use std::sync::Arc;

    use self::gen::contact::ContactServiceMocker;
    use crate::core::mocker::*;

    use super::*;
    use crate::api::*;
    use crate::core::model::*;

    pub struct Mocker;

    pub trait MockFunc:
        Fn(GetContactGroupReq) -> Result<(GetContactGroupResp, CommonResponse), Error>
        + Send
        + Sync
        + 'static
    {
    }
    impl<
            T: Fn(GetContactGroupReq) -> Result<(GetContactGroupResp, CommonResponse), Error>
                + Send
                + Sync
                + 'static,
        > MockFunc for T
    {
    }

    impl<'c, IStore: Store, IClient: HttpClient> ContactServiceMocker<'c, IStore, IClient> {
        pub fn mock_get_contact_group<F: MockFunc>(
            &self,
            f: F,
        ) -> MockerBuilder<Mocker, GetContactGroupReq, GetContactGroupResp, Arc<dyn MockFunc>>
        {
            MockerBuilder::new(self.cli.instance_id, Arc::new(f))
        }

        pub(super) fn get_mock_get_contact_group(
            &self,
            req: &GetContactGroupReq,
        ) -> Option<Arc<dyn MockFunc>> {
            do_mock::<Mocker, GetContactGroupReq, GetContactGroupResp, Arc<dyn MockFunc>>(
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
        api::gen::contact::get_contact_group::{
            GetContactGroupReq, GetContactGroupResp, GetContactGroupRespInner,
        },
        core::{model::CommonResponse, Lark},
    };

    #[tokio::test]
    async fn test_mock() {
        let lark = Lark::new("".to_owned(), "".into());
        let mocker = lark
            .contact()
            .mock()
            .mock_get_contact_group(|_| {
                Ok((GetContactGroupResp::default(), CommonResponse::default()))
            })
            .build();
        let res = lark
            .contact()
            .get_contact_group(GetContactGroupReq::default())
            .await;
        assert!(res.is_ok());
        mocker.clear();
        let res = lark
            .contact()
            .get_contact_group(GetContactGroupReq::default())
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
        "group": {
            "id": "g193821",
            "name": "IT 外包组",
            "description": "IT 外包组，需要对该组人群进行细颗粒度权限管控。",
            "member_user_count": 2,
            "member_department_count": 0,
            "type": 1
        }
    }
}"#;
    #[test]
    fn test_resp() {
        let res = serde_json::from_str::<GetContactGroupRespInner>(RESP);
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
