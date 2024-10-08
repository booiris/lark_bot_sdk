//! doc url: <https://open.larkoffice.com/document/uAjLw4CM/ukTMukTMukTM/helpdesk-v1/agent/agent_email>
// Code generated by gen_api. DO NOT EDIT.

use serde::{Deserialize, Serialize};

use crate::api::BaseResp;
use crate::api::HasBaseResp;
use crate::core::model::{ApiRequest, CommonResponse, MethodOption, ReqParam, StreamReqParam};
use crate::{
    core::{http_client::HttpClient, store::Store},
    error::Error,
};

use crate::api::gen::helpdesk::HelpdeskService;

impl<'c, IStore: Store, IClient: HttpClient> HelpdeskService<'c, IStore, IClient> {
    /// **api 版本: 2023-08-15T07:34:18+00:00**
    ///
    /// ## 获取客服邮箱
    ///
    /// 该接口用于获取客服邮箱地址。
    ///

    ///
    /// doc: <https://open.larkoffice.com/document/uAjLw4CM/ukTMukTMukTM/helpdesk-v1/agent/agent_email>
    ///
    /// new doc: <https://open.larkoffice.com/document/server-docs/helpdesk-v1/agent-function/agent/agent_email>
    ///
    /// api url: <https://open.feishu.cn/document_portal/v1/document/get_detail?fullPath=%2Fserver-docs%2Fhelpdesk-v1%2Fagent-function%2Fagent%2Fagent_email>
    pub async fn get_helpdesk_agent_email(
        &self,
        req: GetHelpdeskAgentEmailReq,
    ) -> Result<(GetHelpdeskAgentEmailResp, CommonResponse), Error> {
        self.get_helpdesk_agent_email_with_opt(req, Default::default())
            .await
    }

    /// 参见 [get_helpdesk_agent_email](#method.get_helpdesk_agent_email) 函数
    pub async fn get_helpdesk_agent_email_with_opt(
        &self,
        req: GetHelpdeskAgentEmailReq,
        method_option: MethodOption,
    ) -> Result<(GetHelpdeskAgentEmailResp, CommonResponse), Error> {
        #[cfg(feature = "test-util")]
        {
            if let Some(f) = self.mock().get_mock_get_helpdesk_agent_email(&req) {
                tracing::info!("[lark] Helpdesk#GetHelpdeskAgentEmail **mocking** api");
                return f(req);
            }
        }

        tracing::info!("[lark] Helpdesk#GetHelpdeskAgentEmail call api");

        let req = ApiRequest {
            scope: "Helpdesk",
            api: "GetHelpdeskAgentEmail",
            method: http::Method::GET,
            url: String::new()
                + self.cli.open_base_url.as_ref()
                + "/open-apis/helpdesk/v1/agent_emails",
            param_data: req.gen_param(),
            method_option,
            need_tenant_access_token: true,
            ..Default::default()
        };

        let (resp, common_resp): (GetHelpdeskAgentEmailRespInner, _) = self.cli.do_req(req).await?;
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
pub struct GetHelpdeskAgentEmailReq {}

#[derive(Debug, Serialize, Deserialize, Clone, lark_bot_sdk_macros::ApiBaseResp)]
struct GetHelpdeskAgentEmailRespInner {
    #[serde(flatten)]
    data: Option<GetHelpdeskAgentEmailResp>,
    #[serde(flatten)]
    base: BaseResp,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct GetHelpdeskAgentEmailResp {
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
    /// agent emails
    ///
    /// **示例值**: "{\"ou_xxx\":\"xxx\",\"ou_yyy\":\"yyy\"}"
    #[serde(
        rename = "agents",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub agents: String,
}

#[cfg(feature = "test-util")]
mod test_utils {
    use std::sync::Arc;

    use self::gen::helpdesk::HelpdeskServiceMocker;
    use crate::core::mocker::*;

    use super::*;
    use crate::api::*;
    use crate::core::model::*;

    pub struct Mocker;

    pub trait MockFunc:
        Fn(GetHelpdeskAgentEmailReq) -> Result<(GetHelpdeskAgentEmailResp, CommonResponse), Error>
        + Send
        + Sync
        + 'static
    {
    }
    impl<
            T: Fn(
                    GetHelpdeskAgentEmailReq,
                ) -> Result<(GetHelpdeskAgentEmailResp, CommonResponse), Error>
                + Send
                + Sync
                + 'static,
        > MockFunc for T
    {
    }

    impl<'c, IStore: Store, IClient: HttpClient> HelpdeskServiceMocker<'c, IStore, IClient> {
        pub fn mock_get_helpdesk_agent_email<F: MockFunc>(
            &self,
            f: F,
        ) -> MockerBuilder<
            Mocker,
            GetHelpdeskAgentEmailReq,
            GetHelpdeskAgentEmailResp,
            Arc<dyn MockFunc>,
        > {
            MockerBuilder::new(self.cli.instance_id, Arc::new(f))
        }

        pub(super) fn get_mock_get_helpdesk_agent_email(
            &self,
            req: &GetHelpdeskAgentEmailReq,
        ) -> Option<Arc<dyn MockFunc>> {
            do_mock::<Mocker, GetHelpdeskAgentEmailReq, GetHelpdeskAgentEmailResp, Arc<dyn MockFunc>>(
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
        api::gen::helpdesk::get_helpdesk_agent_email::{
            GetHelpdeskAgentEmailReq, GetHelpdeskAgentEmailResp, GetHelpdeskAgentEmailRespInner,
        },
        core::{model::CommonResponse, Lark},
    };

    #[tokio::test]
    async fn test_mock() {
        let lark = Lark::new("".to_owned(), "".into());
        let mocker = lark
            .helpdesk()
            .mock()
            .mock_get_helpdesk_agent_email(|_| {
                Ok((
                    GetHelpdeskAgentEmailResp::default(),
                    CommonResponse::default(),
                ))
            })
            .build();
        let res = lark
            .helpdesk()
            .get_helpdesk_agent_email(GetHelpdeskAgentEmailReq::default())
            .await;
        assert!(res.is_ok());
        mocker.clear();
        let res = lark
            .helpdesk()
            .get_helpdesk_agent_email(GetHelpdeskAgentEmailReq::default())
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
        "agents": "{\"ou_xxx\": \"xxx\",\"ou_yyy\": \"yyy\"}"
    }
}"#;
    #[test]
    fn test_resp() {
        let res = serde_json::from_str::<GetHelpdeskAgentEmailRespInner>(RESP);
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
