//! doc url: <https://open.larkoffice.com/document/uAjLw4CM/ukTMukTMukTM/reference/mail-v1/mailgroup/delete>
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
    /// **api 版本: 2024-07-25T02:11:08+00:00**
    ///
    /// ## 删除邮件组
    ///
    /// 删除一个邮件组
    ///

    ///
    /// doc: <https://open.larkoffice.com/document/uAjLw4CM/ukTMukTMukTM/reference/mail-v1/mailgroup/delete>
    ///
    /// new doc: <https://open.larkoffice.com/document/server-docs/mail-v1/mail-group/mailgroup/delete>
    ///
    /// api url: <https://open.feishu.cn/document_portal/v1/document/get_detail?fullPath=%2Fserver-docs%2Fmail-v1%2Fmail-group%2Fmailgroup%2Fdelete>
    pub async fn delete_mail_group(
        &self,
        req: DeleteMailGroupReq,
    ) -> Result<(DeleteMailGroupResp, CommonResponse), Error> {
        self.delete_mail_group_with_opt(req, Default::default())
            .await
    }

    /// 参见 [delete_mail_group](#method.delete_mail_group) 函数
    pub async fn delete_mail_group_with_opt(
        &self,
        req: DeleteMailGroupReq,
        method_option: MethodOption,
    ) -> Result<(DeleteMailGroupResp, CommonResponse), Error> {
        #[cfg(feature = "test-util")]
        {
            if let Some(f) = self.mock().get_mock_delete_mail_group(&req) {
                tracing::info!("[lark] Mail#DeleteMailGroup **mocking** api");
                return f(req);
            }
        }

        tracing::info!("[lark] Mail#DeleteMailGroup call api");

        let req = ApiRequest {
            scope: "Mail",
            api: "DeleteMailGroup",
            method: http::Method::DELETE,
            url: String::new()
                + self.cli.open_base_url.as_ref()
                + "/open-apis/mail/v1/mailgroups/:mailgroup_id",
            param_data: req.gen_param(),
            method_option,
            need_tenant_access_token: true,
            ..Default::default()
        };

        let (resp, common_resp): (DeleteMailGroupRespInner, _) = self.cli.do_req(req).await?;
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
pub struct DeleteMailGroupReq {
    /// 邮件组ID或者邮件组地址
    ///
    /// **示例值**: "xxxxxxxxxxxxxxx 或 test_mail_group@xxx.xx"
    #[api(kind = "path", name = "mailgroup_id")]
    pub mailgroup_id: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, lark_bot_sdk_macros::ApiBaseResp)]
struct DeleteMailGroupRespInner {
    #[serde(flatten)]
    data: Option<DeleteMailGroupResp>,
    #[serde(flatten)]
    base: BaseResp,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct DeleteMailGroupResp {
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
        Fn(DeleteMailGroupReq) -> Result<(DeleteMailGroupResp, CommonResponse), Error>
        + Send
        + Sync
        + 'static
    {
    }
    impl<
            T: Fn(DeleteMailGroupReq) -> Result<(DeleteMailGroupResp, CommonResponse), Error>
                + Send
                + Sync
                + 'static,
        > MockFunc for T
    {
    }

    impl<'c, IStore: Store, IClient: HttpClient> MailServiceMocker<'c, IStore, IClient> {
        pub fn mock_delete_mail_group<F: MockFunc>(
            &self,
            f: F,
        ) -> MockerBuilder<Mocker, DeleteMailGroupReq, DeleteMailGroupResp, Arc<dyn MockFunc>>
        {
            MockerBuilder::new(self.cli.instance_id, Arc::new(f))
        }

        pub(super) fn get_mock_delete_mail_group(
            &self,
            req: &DeleteMailGroupReq,
        ) -> Option<Arc<dyn MockFunc>> {
            do_mock::<Mocker, DeleteMailGroupReq, DeleteMailGroupResp, Arc<dyn MockFunc>>(
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
        api::gen::mail::delete_mail_group::{
            DeleteMailGroupReq, DeleteMailGroupResp, DeleteMailGroupRespInner,
        },
        core::{model::CommonResponse, Lark},
    };

    #[tokio::test]
    async fn test_mock() {
        let lark = Lark::new("".to_owned(), "".into());
        let mocker = lark
            .mail()
            .mock()
            .mock_delete_mail_group(|_| {
                Ok((DeleteMailGroupResp::default(), CommonResponse::default()))
            })
            .build();
        let res = lark
            .mail()
            .delete_mail_group(DeleteMailGroupReq::default())
            .await;
        assert!(res.is_ok());
        mocker.clear();
        let res = lark
            .mail()
            .delete_mail_group(DeleteMailGroupReq::default())
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
        let res = serde_json::from_str::<DeleteMailGroupRespInner>(RESP);
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
