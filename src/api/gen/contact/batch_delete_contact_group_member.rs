//! doc url: <https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/contact-v3/group-member/batch_remove>
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
    /// **api 版本: 2024-07-05T08:10:21+00:00**
    ///
    /// ## 批量移除用户组成员
    ///
    /// 调用该接口从指定普通用户组内移除一个或多个成员。
    ///

    ///
    /// doc: <https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/contact-v3/group-member/batch_remove>
    ///
    /// new doc: <https://open.feishu.cn/document/server-docs/contact-v3/group-member/batch_remove>
    ///
    /// api url: <https://open.feishu.cn/document_portal/v1/document/get_detail?fullPath=%2Fserver-docs%2Fcontact-v3%2Fgroup-member%2Fbatch_remove>
    pub async fn batch_delete_contact_group_member(
        &self,
        req: BatchDeleteContactGroupMemberReq,
    ) -> Result<(BatchDeleteContactGroupMemberResp, CommonResponse), Error> {
        self.batch_delete_contact_group_member_with_opt(req, Default::default())
            .await
    }

    /// 参见 [batch_delete_contact_group_member](#method.batch_delete_contact_group_member) 函数
    pub async fn batch_delete_contact_group_member_with_opt(
        &self,
        req: BatchDeleteContactGroupMemberReq,
        method_option: MethodOption,
    ) -> Result<(BatchDeleteContactGroupMemberResp, CommonResponse), Error> {
        #[cfg(feature = "test-util")]
        {
            if let Some(f) = self.mock().get_mock_batch_delete_contact_group_member(&req) {
                tracing::info!("[lark] Contact#BatchDeleteContactGroupMember **mocking** api");
                return f(req);
            }
        }

        tracing::info!("[lark] Contact#BatchDeleteContactGroupMember call api");

        let req = ApiRequest {
            scope: "Contact",
            api: "BatchDeleteContactGroupMember",
            method: http::Method::POST,
            url: String::new()
                + self.cli.open_base_url.as_ref()
                + "/open-apis/contact/v3/group/:group_id/member/batch_remove",
            param_data: req.gen_param(),
            method_option,
            need_tenant_access_token: true,
            ..Default::default()
        };

        let (resp, common_resp): (BatchDeleteContactGroupMemberRespInner, _) =
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
pub struct BatchDeleteContactGroupMemberReq {
    /// 用户组 ID。
    ///
    /// 用户组 ID 可在创建用户组时从返回值中获取，你也可以调用[查询用户组列表](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/contact-v3/group/simplelist)接口，获取用户组的 ID。
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "test_group"
    #[api(kind = "path", name = "group_id")]
    pub group_id: String,

    /// 待移除成员信息。
    ///
    /// **是否必填**: 是
    ///
    /// **数据校验规则**：
    ///
    /// - **长度范围**: `1` 字符- `100` 字符
    #[api(kind = "body", name = "members")]
    pub members: Vec<Option<MemberlistSubReq>>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(default)]
pub struct MemberlistSubReq {
    /// 移除的用户 ID，ID 类型与 member_id_type 的取值保持一致。
    ///
    /// 你可以调用[查询用户组成员列表](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/contact-v3/group-member/simplelist)接口，获取用户组内的成员 ID，并将需要移除的成员 ID 传入当前参数。注意仅支持移除用户类型的成员，且需要使用相同的用户 ID 类型，否则会报错。
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "u287xj12"
    #[serde(
        rename = "member_id",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub member_id: String,
    /// 用户组成员的类型，目前仅支持选择 user，表示用户类型。
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "user"
    #[serde(
        rename = "member_type",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub member_type: String,
    /// 当 `member_type` 取值为 `user`时，该参数必填，需通过该参数设置用户 ID 类型，包括：
    ///
    /// - open_id：标识一个用户在某个应用中的身份。同一个用户在不同应用中的 Open ID 不同。
    ///
    /// - union_id：标识一个用户在某个应用开发商下的身份。同一用户在同一开发商下的应用中的 Union ID 是相同的，在不同开发商下的应用中的 Union ID 是不同的。通过 Union ID，应用开发商可以把同个用户在多个应用中的身份关联起来。
    ///
    /// - user_id：标识一个用户在某个租户内的身份。同一个用户在租户 A 和租户 B 内的 User ID 是不同的。在同一个租户内，一个用户的 User ID 在所有应用中都保持一致。User ID 主要用于在不同的应用间打通用户数据。
    ///
    /// **示例值**: "user_id"
    #[serde(
        rename = "member_id_type",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub member_id_type: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, lark_bot_sdk_macros::ApiBaseResp)]
struct BatchDeleteContactGroupMemberRespInner {
    #[serde(flatten)]
    data: Option<BatchDeleteContactGroupMemberResp>,
    #[serde(flatten)]
    base: BaseResp,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct BatchDeleteContactGroupMemberResp {
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

    use self::gen::contact::ContactServiceMocker;
    use crate::core::mocker::*;

    use super::*;
    use crate::api::*;
    use crate::core::model::*;

    pub struct Mocker;

    pub trait MockFunc:
        Fn(
            BatchDeleteContactGroupMemberReq,
        ) -> Result<(BatchDeleteContactGroupMemberResp, CommonResponse), Error>
        + Send
        + Sync
        + 'static
    {
    }
    impl<
            T: Fn(
                    BatchDeleteContactGroupMemberReq,
                )
                    -> Result<(BatchDeleteContactGroupMemberResp, CommonResponse), Error>
                + Send
                + Sync
                + 'static,
        > MockFunc for T
    {
    }

    impl<'c, IStore: Store, IClient: HttpClient> ContactServiceMocker<'c, IStore, IClient> {
        pub fn mock_batch_delete_contact_group_member<F: MockFunc>(
            &self,
            f: F,
        ) -> MockerBuilder<
            Mocker,
            BatchDeleteContactGroupMemberReq,
            BatchDeleteContactGroupMemberResp,
            Arc<dyn MockFunc>,
        > {
            MockerBuilder::new(self.cli.instance_id, Arc::new(f))
        }

        pub(super) fn get_mock_batch_delete_contact_group_member(
            &self,
            req: &BatchDeleteContactGroupMemberReq,
        ) -> Option<Arc<dyn MockFunc>> {
            do_mock::<
                Mocker,
                BatchDeleteContactGroupMemberReq,
                BatchDeleteContactGroupMemberResp,
                Arc<dyn MockFunc>,
            >(self.cli.instance_id, req)
        }
    }
}

#[cfg(feature = "test-util")]
#[cfg(test)]
mod test {
    use crate::{
        api::gen::contact::batch_delete_contact_group_member::{
            BatchDeleteContactGroupMemberReq, BatchDeleteContactGroupMemberResp,
            BatchDeleteContactGroupMemberRespInner,
        },
        core::{model::CommonResponse, Lark},
    };

    #[tokio::test]
    async fn test_mock() {
        let lark = Lark::new("".to_owned(), "".into());
        let mocker = lark
            .contact()
            .mock()
            .mock_batch_delete_contact_group_member(|_| {
                Ok((
                    BatchDeleteContactGroupMemberResp::default(),
                    CommonResponse::default(),
                ))
            })
            .build();
        let res = lark
            .contact()
            .batch_delete_contact_group_member(BatchDeleteContactGroupMemberReq::default())
            .await;
        assert!(res.is_ok());
        mocker.clear();
        let res = lark
            .contact()
            .batch_delete_contact_group_member(BatchDeleteContactGroupMemberReq::default())
            .await;
        assert!(res.is_err());
    }

    const REQ: &str = r#"{
    "members": [
        {
            "member_id": "u287xj12",
            "member_type": "user",
            "member_id_type": "user_id"
        }
    ]
}"#;

    #[test]
    fn test_req() {
        if REQ == "{}" {
            return;
        }
        if let Err(e) = serde_json::from_str::<super::BatchDeleteContactGroupMemberReqBody>(REQ) {
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
        let res = serde_json::from_str::<BatchDeleteContactGroupMemberRespInner>(RESP);
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
