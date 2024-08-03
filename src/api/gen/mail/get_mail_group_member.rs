//! doc url: <https://open.larkoffice.com/document/uAjLw4CM/ukTMukTMukTM/reference/mail-v1/mailgroup-member/get>
// Code generated by gen_api. DO NOT EDIT.

use serde::{Deserialize, Serialize};

use crate::api::BaseResp;
use crate::api::HasBaseResp;
use crate::core::model::{ApiRequest, CommonResponse, MethodOption, ReqParam, StreamReqParam};
use crate::{
    core::{http_client::HttpClient, store::Store},
    error::Error,
};

use crate::api::gen::mail::MailService;

impl<'c, IStore: Store, IClient: HttpClient> MailService<'c, IStore, IClient> {
    /// **api 版本: 2024-07-25T02:11:10+00:00**
    ///
    /// ## 查询指定邮件组成员
    ///
    /// 获取邮件组单个成员信息。
    ///

    ///
    /// doc: <https://open.larkoffice.com/document/uAjLw4CM/ukTMukTMukTM/reference/mail-v1/mailgroup-member/get>
    ///
    /// new doc: <https://open.larkoffice.com/document/server-docs/mail-v1/mail-group/mailgroup-member/get>
    ///
    /// api url: <https://open.feishu.cn/document_portal/v1/document/get_detail?fullPath=%2Fserver-docs%2Fmail-v1%2Fmail-group%2Fmailgroup-member%2Fget>
    pub async fn get_mail_group_member(
        &self,
        req: GetMailGroupMemberReq,
    ) -> Result<(GetMailGroupMemberResp, CommonResponse), Error> {
        self.get_mail_group_member_with_opt(req, Default::default())
            .await
    }

    /// 参见 [get_mail_group_member](#method.get_mail_group_member) 函数
    pub async fn get_mail_group_member_with_opt(
        &self,
        req: GetMailGroupMemberReq,
        method_option: MethodOption,
    ) -> Result<(GetMailGroupMemberResp, CommonResponse), Error> {
        #[cfg(feature = "test-util")]
        {
            if let Some(f) = self.mock().get_mock_get_mail_group_member(&req) {
                tracing::info!("[lark] Mail#GetMailGroupMember **mocking** api");
                return f(req);
            }
        }

        tracing::info!("[lark] Mail#GetMailGroupMember call api");

        let req = ApiRequest {
            scope: "Mail",
            api: "GetMailGroupMember",
            method: http::Method::GET,
            url: String::new()
                + self.cli.open_base_url.as_ref()
                + "/open-apis/mail/v1/mailgroups/:mailgroup_id/members/:member_id",
            param_data: req.gen_param(),
            method_option,
            need_tenant_access_token: true,
            ..Default::default()
        };

        let (resp, common_resp): (GetMailGroupMemberRespInner, _) = self.cli.do_req(req).await?;
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
pub struct GetMailGroupMemberReq {
    /// 邮件组ID或者邮件组地址
    ///
    /// **示例值**: "xxxxxxxxxxxxxxx 或 test_mail_group@xxx.xx"
    #[api(kind = "path", name = "mailgroup_id")]
    pub mailgroup_id: String,
    /// 邮件组内成员唯一标识
    ///
    /// **示例值**: "xxxxxxxxxxxxxxx"
    #[api(kind = "path", name = "member_id")]
    pub member_id: String,
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
    /// 此次调用中使用的部门ID的类型
    ///
    /// **示例值**: "open_department_id"
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
}

#[derive(Debug, Serialize, Deserialize, Clone, lark_bot_sdk_macros::ApiBaseResp)]
struct GetMailGroupMemberRespInner {
    #[serde(flatten)]
    data: Option<GetMailGroupMemberResp>,
    #[serde(flatten)]
    base: BaseResp,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct GetMailGroupMemberResp {
    /// \-
    #[serde(
        rename = "data",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub data: MailgroupMemberSubResp,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(default)]
pub struct MailgroupMemberSubResp {
    /// 邮件组内成员唯一标识（在请求体中不用填）
    ///
    /// **示例值**: "xxxxxxxxxxxxxxx"
    #[serde(
        rename = "member_id",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub member_id: String,
    /// 成员邮箱地址（当成员类型是EXTERNAL_USER/MAIL_GROUP/OTHER_MEMBER时有值）
    ///
    /// **示例值**: "test_memeber@xxx.xx"
    #[serde(
        rename = "email",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub email: String,
    /// 租户内用户的唯一标识（当成员类型是USER时有值）
    ///
    /// **示例值**: "xxxxxxxxxx"
    #[serde(
        rename = "user_id",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub user_id: String,
    /// 租户内部门的唯一标识（当成员类型是DEPARTMENT时有值）
    ///
    /// **示例值**: "xxxxxxxxxx"
    #[serde(
        rename = "department_id",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub department_id: String,
    /// 成员类型
    ///
    /// **示例值**: "USER"
    ///
    /// **可选值**:
    ///
    /// `USER`: 内部用户
    ///
    /// `DEPARTMENT`: 部门
    ///
    /// `COMPANY`: 全员
    ///
    /// `EXTERNAL_USER`: 外部用户
    ///
    /// `MAIL_GROUP`: 邮件组
    ///
    /// `PUBLIC_MAILBOX`: member is a public mailbox
    ///
    /// `OTHER_MEMBER`: 内部成员
    #[serde(
        rename = "type",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub body_type: String,
}

#[cfg(feature = "test-util")]
mod test_utils {
    use std::sync::Arc;

    use self::gen::mail::MailServiceMocker;
    use crate::core::mocker::*;

    use super::*;
    use crate::api::*;
    use crate::core::model::*;

    pub struct Mocker;

    pub trait MockFunc:
        Fn(GetMailGroupMemberReq) -> Result<(GetMailGroupMemberResp, CommonResponse), Error>
        + Send
        + Sync
        + 'static
    {
    }
    impl<
            T: Fn(GetMailGroupMemberReq) -> Result<(GetMailGroupMemberResp, CommonResponse), Error>
                + Send
                + Sync
                + 'static,
        > MockFunc for T
    {
    }

    impl<'c, IStore: Store, IClient: HttpClient> MailServiceMocker<'c, IStore, IClient> {
        pub fn mock_get_mail_group_member<F: MockFunc>(
            &self,
            f: F,
        ) -> MockerBuilder<Mocker, GetMailGroupMemberReq, GetMailGroupMemberResp, Arc<dyn MockFunc>>
        {
            MockerBuilder::new(self.cli.instance_id, Arc::new(f))
        }

        pub(super) fn get_mock_get_mail_group_member(
            &self,
            req: &GetMailGroupMemberReq,
        ) -> Option<Arc<dyn MockFunc>> {
            do_mock::<Mocker, GetMailGroupMemberReq, GetMailGroupMemberResp, Arc<dyn MockFunc>>(
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
        api::gen::mail::get_mail_group_member::{
            GetMailGroupMemberReq, GetMailGroupMemberResp, GetMailGroupMemberRespInner,
        },
        core::{model::CommonResponse, Lark},
    };

    #[tokio::test]
    async fn test_mock() {
        let lark = Lark::new("".to_owned(), "".into());
        let mocker = lark
            .mail()
            .mock()
            .mock_get_mail_group_member(|_| {
                Ok((GetMailGroupMemberResp::default(), CommonResponse::default()))
            })
            .build();
        let res = lark
            .mail()
            .get_mail_group_member(GetMailGroupMemberReq::default())
            .await;
        assert!(res.is_ok());
        mocker.clear();
        let res = lark
            .mail()
            .get_mail_group_member(GetMailGroupMemberReq::default())
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
        "member_id": "xxxxxxxxxxxxxxx",
        "email": "test_memeber@xxx.xx",
        "user_id": "xxxxxxxxxx",
        "department_id": "xxxxxxxxxx",
        "type": "USER"
    }
}"#;
    #[test]
    fn test_resp() {
        let res = serde_json::from_str::<GetMailGroupMemberRespInner>(RESP);
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
