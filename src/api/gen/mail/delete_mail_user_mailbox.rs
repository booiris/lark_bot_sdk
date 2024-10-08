//! doc url: <https://open.larkoffice.com/document/uAjLw4CM/ukTMukTMukTM/reference/mail-v1/user_mailbox/delete>
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
    /// **api 版本: 2024-07-25T02:11:26+00:00**
    ///
    /// ## 从回收站删除用户邮箱地址
    ///
    /// 该接口会永久删除用户邮箱地址。可用于删除位于邮箱回收站中的用户邮箱地址，一旦删除，将无法恢复。该接口支持邮件的转移，可以将被释放邮箱的邮件转移到另外一个可以使用的邮箱中。
    ///

    ///
    /// doc: <https://open.larkoffice.com/document/uAjLw4CM/ukTMukTMukTM/reference/mail-v1/user_mailbox/delete>
    ///
    /// new doc: <https://open.larkoffice.com/document/server-docs/mail-v1/user_mailbox-alias/delete>
    ///
    /// api url: <https://open.feishu.cn/document_portal/v1/document/get_detail?fullPath=%2Fserver-docs%2Fmail-v1%2Fuser_mailbox-alias%2Fdelete>
    pub async fn delete_mail_user_mailbox(
        &self,
        req: DeleteMailUserMailboxReq,
    ) -> Result<(DeleteMailUserMailboxResp, CommonResponse), Error> {
        self.delete_mail_user_mailbox_with_opt(req, Default::default())
            .await
    }

    /// 参见 [delete_mail_user_mailbox](#method.delete_mail_user_mailbox) 函数
    pub async fn delete_mail_user_mailbox_with_opt(
        &self,
        req: DeleteMailUserMailboxReq,
        method_option: MethodOption,
    ) -> Result<(DeleteMailUserMailboxResp, CommonResponse), Error> {
        #[cfg(feature = "test-util")]
        {
            if let Some(f) = self.mock().get_mock_delete_mail_user_mailbox(&req) {
                tracing::info!("[lark] Mail#DeleteMailUserMailbox **mocking** api");
                return f(req);
            }
        }

        tracing::info!("[lark] Mail#DeleteMailUserMailbox call api");

        let req = ApiRequest {
            scope: "Mail",
            api: "DeleteMailUserMailbox",
            method: http::Method::DELETE,
            url: String::new()
                + self.cli.open_base_url.as_ref()
                + "/open-apis/mail/v1/user_mailboxes/:user_mailbox_id",
            param_data: req.gen_param(),
            method_option,
            need_tenant_access_token: true,
            ..Default::default()
        };

        let (resp, common_resp): (DeleteMailUserMailboxRespInner, _) = self.cli.do_req(req).await?;
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
pub struct DeleteMailUserMailboxReq {
    /// 要释放的邮箱地址
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "111111@abc.com"
    #[api(kind = "path", name = "user_mailbox_id")]
    pub user_mailbox_id: String,
    /// 用于接受转移的邮箱地址
    ///
    /// **示例值**: "888888@abc.com"
    #[api(
        kind = "query",
        name = "transfer_mailbox",
        v_type = "var",
        option = "false"
    )]
    pub transfer_mailbox: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, lark_bot_sdk_macros::ApiBaseResp)]
struct DeleteMailUserMailboxRespInner {
    #[serde(flatten)]
    data: Option<DeleteMailUserMailboxResp>,
    #[serde(flatten)]
    base: BaseResp,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct DeleteMailUserMailboxResp {
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

    use self::gen::mail::MailServiceMocker;
    use crate::core::mocker::*;

    use super::*;
    use crate::api::*;
    use crate::core::model::*;

    pub struct Mocker;

    pub trait MockFunc:
        Fn(DeleteMailUserMailboxReq) -> Result<(DeleteMailUserMailboxResp, CommonResponse), Error>
        + Send
        + Sync
        + 'static
    {
    }
    impl<
            T: Fn(
                    DeleteMailUserMailboxReq,
                ) -> Result<(DeleteMailUserMailboxResp, CommonResponse), Error>
                + Send
                + Sync
                + 'static,
        > MockFunc for T
    {
    }

    impl<'c, IStore: Store, IClient: HttpClient> MailServiceMocker<'c, IStore, IClient> {
        pub fn mock_delete_mail_user_mailbox<F: MockFunc>(
            &self,
            f: F,
        ) -> MockerBuilder<
            Mocker,
            DeleteMailUserMailboxReq,
            DeleteMailUserMailboxResp,
            Arc<dyn MockFunc>,
        > {
            MockerBuilder::new(self.cli.instance_id, Arc::new(f))
        }

        pub(super) fn get_mock_delete_mail_user_mailbox(
            &self,
            req: &DeleteMailUserMailboxReq,
        ) -> Option<Arc<dyn MockFunc>> {
            do_mock::<Mocker, DeleteMailUserMailboxReq, DeleteMailUserMailboxResp, Arc<dyn MockFunc>>(
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
        api::gen::mail::delete_mail_user_mailbox::{
            DeleteMailUserMailboxReq, DeleteMailUserMailboxResp, DeleteMailUserMailboxRespInner,
        },
        core::{model::CommonResponse, Lark},
    };

    #[tokio::test]
    async fn test_mock() {
        let lark = Lark::new("".to_owned(), "".into());
        let mocker = lark
            .mail()
            .mock()
            .mock_delete_mail_user_mailbox(|_| {
                Ok((
                    DeleteMailUserMailboxResp::default(),
                    CommonResponse::default(),
                ))
            })
            .build();
        let res = lark
            .mail()
            .delete_mail_user_mailbox(DeleteMailUserMailboxReq::default())
            .await;
        assert!(res.is_ok());
        mocker.clear();
        let res = lark
            .mail()
            .delete_mail_user_mailbox(DeleteMailUserMailboxReq::default())
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
    "data": {},
    "msg": "release mail address success"
}"#;
    #[test]
    fn test_resp() {
        let res = serde_json::from_str::<DeleteMailUserMailboxRespInner>(RESP);
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
