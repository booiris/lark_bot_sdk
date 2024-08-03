//! doc url: <https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/mail-v1/public_mailbox/create>
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
    /// **api 版本: 2024-07-25T02:11:12+00:00**
    ///
    /// ## 创建公共邮箱
    ///
    /// 创建一个公共邮箱。
    ///

    ///
    /// doc: <https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/mail-v1/public_mailbox/create>
    ///
    /// new doc: <https://open.feishu.cn/document/server-docs/mail-v1/public-mailbox/public_mailbox/create>
    ///
    /// api url: <https://open.feishu.cn/document_portal/v1/document/get_detail?fullPath=%2Fserver-docs%2Fmail-v1%2Fpublic-mailbox%2Fpublic_mailbox%2Fcreate>
    pub async fn create_public_mailbox(
        &self,
        req: CreatePublicMailboxReq,
    ) -> Result<(CreatePublicMailboxResp, CommonResponse), Error> {
        self.create_public_mailbox_with_opt(req, Default::default())
            .await
    }

    /// 参见 [create_public_mailbox](#method.create_public_mailbox) 函数
    pub async fn create_public_mailbox_with_opt(
        &self,
        req: CreatePublicMailboxReq,
        method_option: MethodOption,
    ) -> Result<(CreatePublicMailboxResp, CommonResponse), Error> {
        #[cfg(feature = "test-util")]
        {
            if let Some(f) = self.mock().get_mock_create_public_mailbox(&req) {
                tracing::info!("[lark] Mail#CreatePublicMailbox **mocking** api");
                return f(req);
            }
        }

        tracing::info!("[lark] Mail#CreatePublicMailbox call api");

        let req = ApiRequest {
            scope: "Mail",
            api: "CreatePublicMailbox",
            method: http::Method::POST,
            url: String::new()
                + self.cli.open_base_url.as_ref()
                + "/open-apis/mail/v1/public_mailboxes",
            param_data: req.gen_param(),
            method_option,
            need_tenant_access_token: true,
            ..Default::default()
        };

        let (resp, common_resp): (CreatePublicMailboxRespInner, _) = self.cli.do_req(req).await?;
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
pub struct CreatePublicMailboxReq {
    /// 公共邮箱地址
    ///
    /// **示例值**: "test_public_mailbox@xxx.xx"
    #[api(kind = "body", name = "email")]
    pub email: Option<String>,
    /// 公共邮箱名称
    ///
    /// **示例值**: "test public mailbox"
    #[api(kind = "body", name = "name")]
    pub name: Option<String>,
    /// 数据驻留地
    ///
    /// **示例值**: "cn"
    #[api(kind = "body", name = "geo")]
    pub geo: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, lark_bot_sdk_macros::ApiBaseResp)]
struct CreatePublicMailboxRespInner {
    #[serde(flatten)]
    data: Option<CreatePublicMailboxResp>,
    #[serde(flatten)]
    base: BaseResp,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct CreatePublicMailboxResp {
    /// \-
    #[serde(
        rename = "data",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub data: PublicMailboxSubResp,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(default)]
pub struct PublicMailboxSubResp {
    /// 公共邮箱唯一标识
    ///
    /// **示例值**: "xxxxxxxxxxxxxxx"
    #[serde(
        rename = "public_mailbox_id",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub public_mailbox_id: String,
    /// 公共邮箱地址
    ///
    /// **示例值**: "test_public_mailbox@xxx.xx"
    #[serde(
        rename = "email",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub email: String,
    /// 公共邮箱名称
    ///
    /// **示例值**: "test public mailbox"
    #[serde(
        rename = "name",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub name: String,
    /// 数据驻留地
    ///
    /// **示例值**: "cn"
    #[serde(
        rename = "geo",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub geo: String,
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
        Fn(CreatePublicMailboxReq) -> Result<(CreatePublicMailboxResp, CommonResponse), Error>
        + Send
        + Sync
        + 'static
    {
    }
    impl<
            T: Fn(
                    CreatePublicMailboxReq,
                ) -> Result<(CreatePublicMailboxResp, CommonResponse), Error>
                + Send
                + Sync
                + 'static,
        > MockFunc for T
    {
    }

    impl<'c, IStore: Store, IClient: HttpClient> MailServiceMocker<'c, IStore, IClient> {
        pub fn mock_create_public_mailbox<F: MockFunc>(
            &self,
            f: F,
        ) -> MockerBuilder<Mocker, CreatePublicMailboxReq, CreatePublicMailboxResp, Arc<dyn MockFunc>>
        {
            MockerBuilder::new(self.cli.instance_id, Arc::new(f))
        }

        pub(super) fn get_mock_create_public_mailbox(
            &self,
            req: &CreatePublicMailboxReq,
        ) -> Option<Arc<dyn MockFunc>> {
            do_mock::<Mocker, CreatePublicMailboxReq, CreatePublicMailboxResp, Arc<dyn MockFunc>>(
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
        api::gen::mail::create_public_mailbox::{
            CreatePublicMailboxReq, CreatePublicMailboxResp, CreatePublicMailboxRespInner,
        },
        core::{model::CommonResponse, Lark},
    };

    #[tokio::test]
    async fn test_mock() {
        let lark = Lark::new("".to_owned(), "".into());
        let mocker = lark
            .mail()
            .mock()
            .mock_create_public_mailbox(|_| {
                Ok((
                    CreatePublicMailboxResp::default(),
                    CommonResponse::default(),
                ))
            })
            .build();
        let res = lark
            .mail()
            .create_public_mailbox(CreatePublicMailboxReq::default())
            .await;
        assert!(res.is_ok());
        mocker.clear();
        let res = lark
            .mail()
            .create_public_mailbox(CreatePublicMailboxReq::default())
            .await;
        assert!(res.is_err());
    }

    const REQ: &str = r#"{
    "email": "test_public_mailbox@xxx.xx",
    "name": "test public mailbox",
    "geo": "cn"
}"#;

    #[test]
    fn test_req() {
        if REQ == "{}" {
            return;
        }
        if let Err(e) = serde_json::from_str::<super::CreatePublicMailboxReqBody>(REQ) {
            panic!("{}", e);
        }
    }

    const RESP: &str = r#"{
    "code": 0,
    "msg": "success",
    "data": {
        "public_mailbox_id": "xxxxxxxxxxxxxxx",
        "email": "test_public_mailbox@xxx.xx",
        "name": "test public mailbox",
        "geo": "cn"
    }
}"#;
    #[test]
    fn test_resp() {
        let res = serde_json::from_str::<CreatePublicMailboxRespInner>(RESP);
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
