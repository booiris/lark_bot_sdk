//! doc url: <https://open.larkoffice.com/document/uAjLw4CM/ukTMukTMukTM/reference/im-v1/message/patch>
// Code generated by gen_api. DO NOT EDIT.

use serde::{Deserialize, Serialize};

use crate::api::BaseResp;
use crate::api::HasBaseResp;
use crate::core::model::{ApiRequest, CommonResponse, MethodOption, ReqParam, StreamReqParam};
use crate::{
    core::{http_client::HttpClient, store::Store},
    error::Error,
};

use crate::api::gen::message::MessageService;

impl<'c, IStore: Store, IClient: HttpClient> MessageService<'c, IStore, IClient> {
    /// **api 版本: 2024-07-31T09:15:40+00:00**
    ///
    /// ## 更新应用发送的消息卡片
    ///
    /// 更新应用已发送的消息卡片内容。
    ///

    ///
    /// doc: <https://open.larkoffice.com/document/uAjLw4CM/ukTMukTMukTM/reference/im-v1/message/patch>
    ///
    /// new doc: <https://open.larkoffice.com/document/server-docs/im-v1/message-card/patch>
    ///
    /// api url: <https://open.feishu.cn/document_portal/v1/document/get_detail?fullPath=%2Fserver-docs%2Fim-v1%2Fmessage-card%2Fpatch>
    pub async fn update_message(
        &self,
        req: UpdateMessageReq,
    ) -> Result<(UpdateMessageResp, CommonResponse), Error> {
        self.update_message_with_opt(req, Default::default()).await
    }

    /// 参见 [update_message](#method.update_message) 函数
    pub async fn update_message_with_opt(
        &self,
        req: UpdateMessageReq,
        method_option: MethodOption,
    ) -> Result<(UpdateMessageResp, CommonResponse), Error> {
        #[cfg(feature = "test-util")]
        {
            if let Some(f) = self.mock().get_mock_update_message(&req) {
                tracing::info!("[lark] Message#UpdateMessage **mocking** api");
                return f(req);
            }
        }

        tracing::info!("[lark] Message#UpdateMessage call api");

        let req = ApiRequest {
            scope: "Message",
            api: "UpdateMessage",
            method: http::Method::PATCH,
            url: String::new()
                + self.cli.open_base_url.as_ref()
                + "/open-apis/im/v1/messages/:message_id",
            param_data: req.gen_param(),
            method_option,
            need_tenant_access_token: true,
            need_user_access_token: true,
            ..Default::default()
        };

        let (resp, common_resp): (UpdateMessageRespInner, _) = self.cli.do_req(req).await?;
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
pub struct UpdateMessageReq {
    /// 待更新的消息的ID，仅支持更新消息卡片(`interactive`类型)，详情参见[消息ID说明](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/im-v1/message/intro#ac79c1c2)
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "om_dc13264520392913993dd051dba21dcf"
    #[api(kind = "path", name = "message_id")]
    pub message_id: String,

    /// 消息内容 json 格式，[发送消息 content 说明](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/im-v1/message/create_json)，参考文档中的卡片格式
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "参考链接"
    #[api(kind = "body", name = "content")]
    pub content: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, lark_bot_sdk_macros::ApiBaseResp)]
struct UpdateMessageRespInner {
    #[serde(flatten)]
    data: Option<UpdateMessageResp>,
    #[serde(flatten)]
    base: BaseResp,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct UpdateMessageResp {
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

    use self::gen::message::MessageServiceMocker;
    use crate::core::mocker::*;

    use super::*;
    use crate::api::*;
    use crate::core::model::*;

    pub struct Mocker;

    pub trait MockFunc:
        Fn(UpdateMessageReq) -> Result<(UpdateMessageResp, CommonResponse), Error>
        + Send
        + Sync
        + 'static
    {
    }
    impl<
            T: Fn(UpdateMessageReq) -> Result<(UpdateMessageResp, CommonResponse), Error>
                + Send
                + Sync
                + 'static,
        > MockFunc for T
    {
    }

    impl<'c, IStore: Store, IClient: HttpClient> MessageServiceMocker<'c, IStore, IClient> {
        pub fn mock_update_message<F: MockFunc>(
            &self,
            f: F,
        ) -> MockerBuilder<Mocker, UpdateMessageReq, UpdateMessageResp, Arc<dyn MockFunc>> {
            MockerBuilder::new(self.cli.instance_id, Arc::new(f))
        }

        pub(super) fn get_mock_update_message(
            &self,
            req: &UpdateMessageReq,
        ) -> Option<Arc<dyn MockFunc>> {
            do_mock::<Mocker, UpdateMessageReq, UpdateMessageResp, Arc<dyn MockFunc>>(
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
        api::gen::message::update_message::{
            UpdateMessageReq, UpdateMessageResp, UpdateMessageRespInner,
        },
        core::{model::CommonResponse, Lark},
    };

    #[tokio::test]
    async fn test_mock() {
        let lark = Lark::new("".to_owned(), "".into());
        let mocker = lark
            .message()
            .mock()
            .mock_update_message(|_| Ok((UpdateMessageResp::default(), CommonResponse::default())))
            .build();
        let res = lark
            .message()
            .update_message(UpdateMessageReq::default())
            .await;
        assert!(res.is_ok());
        mocker.clear();
        let res = lark
            .message()
            .update_message(UpdateMessageReq::default())
            .await;
        assert!(res.is_err());
    }

    const REQ: &str = r#"{
    "content": "参考链接"
}"#;

    #[test]
    fn test_req() {
        if REQ == "{}" {
            return;
        }
        if let Err(e) = serde_json::from_str::<super::UpdateMessageReqBody>(REQ) {
            panic!("{}", e);
        }
    }

    const RESP: &str = r#"{
    "code": 0,
    "data": {},
    "msg": "ok"
}"#;
    #[test]
    fn test_resp() {
        let res = serde_json::from_str::<UpdateMessageRespInner>(RESP);
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
