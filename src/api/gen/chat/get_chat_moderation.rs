//! doc url: <https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/im-v1/chat-moderation/get>
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
    /// ## 获取群成员发言权限
    ///
    /// 获取群发言模式、可发言用户名单等。
    ///
    /// 注意事项：
    ///
    /// - 应用需要开启[机器人能力](https://open.feishu.cn/document/uAjLw4CM/ugTN1YjL4UTN24CO1UjN/trouble-shooting/how-to-enable-bot-ability)
    ///
    /// - 机器人 或 授权用户 必须在群里
    ///
    /// doc: <https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/im-v1/chat-moderation/get>
    ///
    /// new doc: <https://open.feishu.cn/document/server-docs/group/chat/get>
    ///
    /// api url: <https://open.feishu.cn/document_portal/v1/document/get_detail?fullPath=%2Fserver-docs%2Fgroup%2Fchat%2Fget>
    pub async fn get_chat_moderation(
        &self,
        req: GetChatModerationReq,
    ) -> Result<(GetChatModerationResp, CommonResponse), Error> {
        self.get_chat_moderation_with_opt(req, Default::default())
            .await
    }

    /// 参见 [get_chat_moderation](#method.get_chat_moderation) 函数
    pub async fn get_chat_moderation_with_opt(
        &self,
        req: GetChatModerationReq,
        method_option: MethodOption,
    ) -> Result<(GetChatModerationResp, CommonResponse), Error> {
        #[cfg(feature = "test-util")]
        {
            if let Some(f) = self.mock().get_mock_get_chat_moderation(&req) {
                tracing::info!("[lark] Chat#GetChatModeration **mocking** api");
                return f(req);
            }
        }

        tracing::info!("[lark] Chat#GetChatModeration call api");

        let req = ApiRequest {
            scope: "Chat",
            api: "GetChatModeration",
            method: http::Method::GET,
            url: String::new()
                + self.cli.open_base_url.as_ref()
                + "/open-apis/im/v1/chats/:chat_id/moderation",
            param_data: req.gen_param(),
            method_option,
            need_tenant_access_token: true,
            need_user_access_token: true,
            ..Default::default()
        };

        let (resp, common_resp): (GetChatModerationRespInner, _) = self.cli.do_req(req).await?;
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
pub struct GetChatModerationReq {
    /// 群 ID，详情参见[群ID 说明](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/im-v1/chat-id-description)
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "oc_a0553eda9014c201e6969b478895c230"
    #[api(kind = "path", name = "chat_id")]
    pub chat_id: String,
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
    /// 分页大小
    ///
    /// **示例值**: "dmJCRHhpd3JRbGV1VEVNRFFyTitRWDY5ZFkybmYrMEUwMUFYT0VMMWdENEtuYUhsNUxGMDIwemtvdE5ORjBNQQ=="
    #[api(kind = "query", name = "page_size", v_type = "var", option = "false")]
    pub page_size: i64,
    /// 分页标记，第一次请求不填，表示从头开始遍历；分页查询结果还有更多项时会同时返回新的 page_token，下次遍历可采用该 page_token 获取查询结果
    ///
    /// **示例值**: "10"
    #[api(kind = "query", name = "page_token", v_type = "var", option = "false")]
    pub page_token: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, lark_bot_sdk_macros::ApiBaseResp)]
struct GetChatModerationRespInner {
    #[serde(flatten)]
    data: Option<GetChatModerationResp>,
    #[serde(flatten)]
    base: BaseResp,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct GetChatModerationResp {
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
    /// 群发言模式（all_members/only_owner/moderator_list，其中 moderator_list 表示部分用户可发言的模式）
    ///
    /// **示例值**: "moderator_list"
    #[serde(
        rename = "moderation_setting",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub moderation_setting: String,
    /// 分页标记，当 has_more 为 true 时，会同时返回新的 page_token，否则不返回 page_token
    ///
    /// **示例值**: "0"
    #[serde(
        rename = "page_token",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub page_token: String,
    /// 是否还有更多项
    ///
    /// **示例值**: "false"
    #[serde(
        rename = "has_more",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub has_more: bool,
    /// 可发言用户列表
    #[serde(
        rename = "items",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub items: Vec<ListModeratorSubResp>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(default)]
pub struct ListModeratorSubResp {
    /// 可发言用户 ID 类型
    ///
    /// **示例值**: "user_id"
    #[serde(
        rename = "user_id_type",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub user_id_type: String,
    /// 可发言用户 ID
    ///
    /// **示例值**: "4d7a3c6g"
    #[serde(
        rename = "user_id",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub user_id: String,
    /// 租户Key，为租户在飞书上的唯一标识，用来换取对应的tenant_access_token，也可以用作租户在应用中的唯一标识
    ///
    /// **示例值**: "2ca1d211f64f6438"
    #[serde(
        rename = "tenant_key",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub tenant_key: String,
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
        Fn(GetChatModerationReq) -> Result<(GetChatModerationResp, CommonResponse), Error>
        + Send
        + Sync
        + 'static
    {
    }
    impl<
            T: Fn(GetChatModerationReq) -> Result<(GetChatModerationResp, CommonResponse), Error>
                + Send
                + Sync
                + 'static,
        > MockFunc for T
    {
    }

    impl<'c, IStore: Store, IClient: HttpClient> ChatServiceMocker<'c, IStore, IClient> {
        pub fn mock_get_chat_moderation<F: MockFunc>(
            &self,
            f: F,
        ) -> MockerBuilder<Mocker, GetChatModerationReq, GetChatModerationResp, Arc<dyn MockFunc>>
        {
            MockerBuilder::new(self.cli.instance_id, Arc::new(f))
        }

        pub(super) fn get_mock_get_chat_moderation(
            &self,
            req: &GetChatModerationReq,
        ) -> Option<Arc<dyn MockFunc>> {
            do_mock::<Mocker, GetChatModerationReq, GetChatModerationResp, Arc<dyn MockFunc>>(
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
        api::gen::chat::get_chat_moderation::{
            GetChatModerationReq, GetChatModerationResp, GetChatModerationRespInner,
        },
        core::{model::CommonResponse, Lark},
    };

    #[tokio::test]
    async fn test_mock() {
        let lark = Lark::new("".to_owned(), "".into());
        let mocker = lark
            .chat()
            .mock()
            .mock_get_chat_moderation(|_| {
                Ok((GetChatModerationResp::default(), CommonResponse::default()))
            })
            .build();
        let res = lark
            .chat()
            .get_chat_moderation(GetChatModerationReq::default())
            .await;
        assert!(res.is_ok());
        mocker.clear();
        let res = lark
            .chat()
            .get_chat_moderation(GetChatModerationReq::default())
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
        "moderation_setting": "moderator_list",
        "page_token": "dmJCRHhpd3JRbGV1VEVNRFFyTitRWDY5ZFkybmYrMEUwMUFYT0VMMWdENEtuYUhsNUxGMDIwemtvdE5ORjBNQQ==",
        "has_more": true,
        "items": [
            {
                "user_id_type": "user_id",
                "user_id": "4d7a3c6g",
                "tenant_key": "2ca1d211f64f6438"
            }
        ]
    }
}"#;
    #[test]
    fn test_resp() {
        let res = serde_json::from_str::<GetChatModerationRespInner>(RESP);
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
