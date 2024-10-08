//! doc url: <https://open.larkoffice.com/document/uAjLw4CM/ukTMukTMukTM/reference/mail-v1/user_mailbox-alias/delete>
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
    /// ## 删除用户邮箱别名
    ///
    /// 删除用户邮箱别名。
    ///

    ///
    /// doc: <https://open.larkoffice.com/document/uAjLw4CM/ukTMukTMukTM/reference/mail-v1/user_mailbox-alias/delete>
    ///
    /// new doc: <https://open.larkoffice.com/document/server-docs/mail-v1/user_mailbox-alias/delete-2>
    ///
    /// api url: <https://open.feishu.cn/document_portal/v1/document/get_detail?fullPath=%2Fserver-docs%2Fmail-v1%2Fuser_mailbox-alias%2Fdelete-2>
    pub async fn delete_mail_user_mailbox_alias(
        &self,
        req: DeleteMailUserMailboxAliasReq,
    ) -> Result<(DeleteMailUserMailboxAliasResp, CommonResponse), Error> {
        self.delete_mail_user_mailbox_alias_with_opt(req, Default::default())
            .await
    }

    /// 参见 [delete_mail_user_mailbox_alias](#method.delete_mail_user_mailbox_alias) 函数
    pub async fn delete_mail_user_mailbox_alias_with_opt(
        &self,
        req: DeleteMailUserMailboxAliasReq,
        method_option: MethodOption,
    ) -> Result<(DeleteMailUserMailboxAliasResp, CommonResponse), Error> {
        #[cfg(feature = "test-util")]
        {
            if let Some(f) = self.mock().get_mock_delete_mail_user_mailbox_alias(&req) {
                tracing::info!("[lark] Mail#DeleteMailUserMailboxAlias **mocking** api");
                return f(req);
            }
        }

        tracing::info!("[lark] Mail#DeleteMailUserMailboxAlias call api");

        let req = ApiRequest {
            scope: "Mail",
            api: "DeleteMailUserMailboxAlias",
            method: http::Method::DELETE,
            url: String::new()
                + self.cli.open_base_url.as_ref()
                + "/open-apis/mail/v1/user_mailboxes/:user_mailbox_id/aliases/:alias_id",
            param_data: req.gen_param(),
            method_option,
            need_tenant_access_token: true,
            ..Default::default()
        };

        let (resp, common_resp): (DeleteMailUserMailboxAliasRespInner, _) =
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
pub struct DeleteMailUserMailboxAliasReq {
    /// 用户邮箱地址
    ///
    /// **示例值**: "user@xxx.xx"
    #[api(kind = "path", name = "user_mailbox_id")]
    pub user_mailbox_id: String,
    /// 别名邮箱地址
    ///
    /// **示例值**: "user_alias@xxx.xx"
    #[api(kind = "path", name = "alias_id")]
    pub alias_id: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, lark_bot_sdk_macros::ApiBaseResp)]
struct DeleteMailUserMailboxAliasRespInner {
    #[serde(flatten)]
    data: Option<DeleteMailUserMailboxAliasResp>,
    #[serde(flatten)]
    base: BaseResp,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct DeleteMailUserMailboxAliasResp {
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
        Fn(
            DeleteMailUserMailboxAliasReq,
        ) -> Result<(DeleteMailUserMailboxAliasResp, CommonResponse), Error>
        + Send
        + Sync
        + 'static
    {
    }
    impl<
            T: Fn(
                    DeleteMailUserMailboxAliasReq,
                )
                    -> Result<(DeleteMailUserMailboxAliasResp, CommonResponse), Error>
                + Send
                + Sync
                + 'static,
        > MockFunc for T
    {
    }

    impl<'c, IStore: Store, IClient: HttpClient> MailServiceMocker<'c, IStore, IClient> {
        pub fn mock_delete_mail_user_mailbox_alias<F: MockFunc>(
            &self,
            f: F,
        ) -> MockerBuilder<
            Mocker,
            DeleteMailUserMailboxAliasReq,
            DeleteMailUserMailboxAliasResp,
            Arc<dyn MockFunc>,
        > {
            MockerBuilder::new(self.cli.instance_id, Arc::new(f))
        }

        pub(super) fn get_mock_delete_mail_user_mailbox_alias(
            &self,
            req: &DeleteMailUserMailboxAliasReq,
        ) -> Option<Arc<dyn MockFunc>> {
            do_mock::<
                Mocker,
                DeleteMailUserMailboxAliasReq,
                DeleteMailUserMailboxAliasResp,
                Arc<dyn MockFunc>,
            >(self.cli.instance_id, req)
        }
    }
}

#[cfg(feature = "test-util")]
#[cfg(test)]
mod test {
    use crate::{
        api::gen::mail::delete_mail_user_mailbox_alias::{
            DeleteMailUserMailboxAliasReq, DeleteMailUserMailboxAliasResp,
            DeleteMailUserMailboxAliasRespInner,
        },
        core::{model::CommonResponse, Lark},
    };

    #[tokio::test]
    async fn test_mock() {
        let lark = Lark::new("".to_owned(), "".into());
        let mocker = lark
            .mail()
            .mock()
            .mock_delete_mail_user_mailbox_alias(|_| {
                Ok((
                    DeleteMailUserMailboxAliasResp::default(),
                    CommonResponse::default(),
                ))
            })
            .build();
        let res = lark
            .mail()
            .delete_mail_user_mailbox_alias(DeleteMailUserMailboxAliasReq::default())
            .await;
        assert!(res.is_ok());
        mocker.clear();
        let res = lark
            .mail()
            .delete_mail_user_mailbox_alias(DeleteMailUserMailboxAliasReq::default())
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
        let res = serde_json::from_str::<DeleteMailUserMailboxAliasRespInner>(RESP);
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
