//! doc url: <https://open.larkoffice.com/document/uAjLw4CM/ukTMukTMukTM/reference/im-v1/chat/link>
// Code generated by gen_api. DO NOT EDIT.

use serde::{Deserialize, Serialize};

use crate::api::BaseResp;
use crate::api::HasBaseResp;
use crate::core::model::{ApiRequest, CommonResponse, MethodOption, ReqParam, StreamReqParam};
use crate::{
    core::{http_client::HttpClient, store::Store},
    error::Error,
};

use crate::api::gen::chat::ChatService;

impl<'c, IStore: Store, IClient: HttpClient> ChatService<'c, IStore, IClient> {
    /// **api 版本: 2024-06-21T09:04:27+00:00**
    ///
    /// ## 获取群分享链接
    ///
    /// 获取指定群的分享链接。
    ///
    /// 注意事项:
    ///
    /// - 应用需要开启[机器人能力](https://open.feishu.cn/document/uAjLw4CM/ugTN1YjL4UTN24CO1UjN/trouble-shooting/how-to-enable-bot-ability)
    ///
    /// - access_token所对应的 **机器人** 或 **授权用户** 必须在`chat_id`参数指定的群组中
    ///
    /// - 单聊、密聊、团队群不支持分享群链接
    ///
    /// - 当Bot被停用或Bot退出群组时，Bot生成的群链接也将停用
    ///
    /// - 当群聊开启了 ==仅群主和群管理员可添加群成员/分享群== 设置时，仅**群主**和**群管理员**可以获取群分享链接
    ///
    /// - 获取内部群分享链接时，操作者须与群组在同一租户下
    ///
    /// doc: <https://open.larkoffice.com/document/uAjLw4CM/ukTMukTMukTM/reference/im-v1/chat/link>
    ///
    /// new doc: <https://open.larkoffice.com/document/server-docs/group/chat/link>
    ///
    /// api url: <https://open.feishu.cn/document_portal/v1/document/get_detail?fullPath=%2Fserver-docs%2Fgroup%2Fchat%2Flink>
    pub async fn gen_chat_share_link(
        &self,
        req: GenChatShareLinkReq,
    ) -> Result<(GenChatShareLinkResp, CommonResponse), Error> {
        self.gen_chat_share_link_with_opt(req, Default::default())
            .await
    }

    /// 参见 [gen_chat_share_link](#method.gen_chat_share_link) 函数
    pub async fn gen_chat_share_link_with_opt(
        &self,
        req: GenChatShareLinkReq,
        method_option: MethodOption,
    ) -> Result<(GenChatShareLinkResp, CommonResponse), Error> {
        #[cfg(feature = "test-util")]
        {
            if let Some(f) = self.mock().get_mock_gen_chat_share_link(&req) {
                tracing::info!("[lark] Chat#GenChatShareLink **mocking** api");
                return f(req);
            }
        }

        tracing::info!("[lark] Chat#GenChatShareLink call api");

        let req = ApiRequest {
            scope: "Chat",
            api: "GenChatShareLink",
            method: http::Method::POST,
            url: String::new()
                + self.cli.open_base_url.as_ref()
                + "/open-apis/im/v1/chats/:chat_id/link",
            param_data: req.gen_param(),
            method_option,
            need_tenant_access_token: true,
            need_user_access_token: true,
            ..Default::default()
        };

        let (resp, common_resp): (GenChatShareLinkRespInner, _) = self.cli.do_req(req).await?;
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
pub struct GenChatShareLinkReq {
    /// 待获取分享链接的群ID，详情参见[群ID 说明](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/im-v1/chat-id-description)
    ///
    /// **注意**：单聊、密聊、团队群不支持分享群链接
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "oc_a0553eda9014c201e6969b478895c230"
    #[api(kind = "path", name = "chat_id")]
    pub chat_id: String,

    /// 群分享链接有效时长，可选值week、year、permanently，分别表示7天、1年以及永久有效
    ///
    /// **示例值**: "week"
    ///
    /// **可选值**:
    ///
    /// `one_week`: 有效期7天
    ///
    /// `one_year`: 有效期1年
    ///
    /// `permanently`: 永久有效
    #[api(kind = "body", name = "validity_period")]
    pub validity_period: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, lark_bot_sdk_macros::ApiBaseResp)]
struct GenChatShareLinkRespInner {
    #[serde(flatten)]
    data: Option<GenChatShareLinkResp>,
    #[serde(flatten)]
    base: BaseResp,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct GenChatShareLinkResp {
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
    /// 群分享链接
    ///
    /// **示例值**: "https://applink.feishu.cn/client/chat/chatter/add_by_link?link_token=3nf8789-4rfx-427d-a6bf-ed1d2df348aabd"
    #[serde(
        rename = "share_link",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub share_link: String,
    /// 分享链接过期时间戳（秒级）
    ///
    /// **示例值**: "1609296809"
    #[serde(
        rename = "expire_time",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub expire_time: String,
    /// 分享链接是否永久有效
    ///
    /// **示例值**: "false"
    #[serde(
        rename = "is_permanent",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub is_permanent: bool,
}

#[cfg(feature = "test-util")]
mod test_utils {
    use std::sync::Arc;

    use self::gen::chat::ChatServiceMocker;
    use crate::core::mocker::*;

    use super::*;
    use crate::api::*;
    use crate::core::model::*;

    pub struct Mocker;

    pub trait MockFunc:
        Fn(GenChatShareLinkReq) -> Result<(GenChatShareLinkResp, CommonResponse), Error>
        + Send
        + Sync
        + 'static
    {
    }
    impl<
            T: Fn(GenChatShareLinkReq) -> Result<(GenChatShareLinkResp, CommonResponse), Error>
                + Send
                + Sync
                + 'static,
        > MockFunc for T
    {
    }

    impl<'c, IStore: Store, IClient: HttpClient> ChatServiceMocker<'c, IStore, IClient> {
        pub fn mock_gen_chat_share_link<F: MockFunc>(
            &self,
            f: F,
        ) -> MockerBuilder<Mocker, GenChatShareLinkReq, GenChatShareLinkResp, Arc<dyn MockFunc>>
        {
            MockerBuilder::new(self.cli.instance_id, Arc::new(f))
        }

        pub(super) fn get_mock_gen_chat_share_link(
            &self,
            req: &GenChatShareLinkReq,
        ) -> Option<Arc<dyn MockFunc>> {
            do_mock::<Mocker, GenChatShareLinkReq, GenChatShareLinkResp, Arc<dyn MockFunc>>(
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
        api::gen::chat::gen_chat_share_link::{
            GenChatShareLinkReq, GenChatShareLinkResp, GenChatShareLinkRespInner,
        },
        core::{model::CommonResponse, Lark},
    };

    #[tokio::test]
    async fn test_mock() {
        let lark = Lark::new("".to_owned(), "".into());
        let mocker = lark
            .chat()
            .mock()
            .mock_gen_chat_share_link(|_| {
                Ok((GenChatShareLinkResp::default(), CommonResponse::default()))
            })
            .build();
        let res = lark
            .chat()
            .gen_chat_share_link(GenChatShareLinkReq::default())
            .await;
        assert!(res.is_ok());
        mocker.clear();
        let res = lark
            .chat()
            .gen_chat_share_link(GenChatShareLinkReq::default())
            .await;
        assert!(res.is_err());
    }

    const REQ: &str = r#"{
    "validity_period": "week"
}"#;

    #[test]
    fn test_req() {
        if REQ == "{}" {
            return;
        }
        if let Err(e) = serde_json::from_str::<super::GenChatShareLinkReqBody>(REQ) {
            panic!("{}", e);
        }
    }

    const RESP: &str = r#"{
    "code": 0,
    "msg": "success",
    "data": {
        "share_link": "https://applink.feishu.cn/client/chat/chatter/add_by_link?link_token=3nf8789-4rfx-427d-a6bf-ed1d2df348aabd",
        "expire_time": "1609296809",
        "is_permanent": false
    }
}"#;
    #[test]
    fn test_resp() {
        let res = serde_json::from_str::<GenChatShareLinkRespInner>(RESP);
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
