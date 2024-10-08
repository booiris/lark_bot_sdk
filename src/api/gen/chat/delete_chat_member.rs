//! doc url: <https://open.larkoffice.com/document/uAjLw4CM/ukTMukTMukTM/reference/im-v1/chat-members/delete>
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
    /// **api 版本: 2024-06-05T03:20:05+00:00**
    ///
    /// ## 将用户或机器人移出群聊
    ///
    /// 将用户或机器人移出群聊。
    ///
    /// 注意事项：
    ///
    /// - 应用需要开启[机器人能力](https://open.feishu.cn/document/uAjLw4CM/ugTN1YjL4UTN24CO1UjN/trouble-shooting/how-to-enable-bot-ability)
    ///
    /// - 用户或机器人在任何条件下均可移除自己出群（即主动退群）
    ///
    /// - 仅有群主/管理员 或 创建群组并且具备 ==更新应用所创建群的群信息== 权限的机器人，可以移除其他用户或者机器人
    ///
    /// - 每次请求，最多移除50个用户或者5个机器人
    ///
    /// - 操作内部群时，操作者须与群组在同一租户下
    ///
    /// doc: <https://open.larkoffice.com/document/uAjLw4CM/ukTMukTMukTM/reference/im-v1/chat-members/delete>
    ///
    /// new doc: <https://open.larkoffice.com/document/server-docs/group/chat-member/delete>
    ///
    /// api url: <https://open.feishu.cn/document_portal/v1/document/get_detail?fullPath=%2Fserver-docs%2Fgroup%2Fchat-member%2Fdelete>
    pub async fn delete_chat_member(
        &self,
        req: DeleteChatMemberReq,
    ) -> Result<(DeleteChatMemberResp, CommonResponse), Error> {
        self.delete_chat_member_with_opt(req, Default::default())
            .await
    }

    /// 参见 [delete_chat_member](#method.delete_chat_member) 函数
    pub async fn delete_chat_member_with_opt(
        &self,
        req: DeleteChatMemberReq,
        method_option: MethodOption,
    ) -> Result<(DeleteChatMemberResp, CommonResponse), Error> {
        #[cfg(feature = "test-util")]
        {
            if let Some(f) = self.mock().get_mock_delete_chat_member(&req) {
                tracing::info!("[lark] Chat#DeleteChatMember **mocking** api");
                return f(req);
            }
        }

        tracing::info!("[lark] Chat#DeleteChatMember call api");

        let req = ApiRequest {
            scope: "Chat",
            api: "DeleteChatMember",
            method: http::Method::DELETE,
            url: String::new()
                + self.cli.open_base_url.as_ref()
                + "/open-apis/im/v1/chats/:chat_id/members",
            param_data: req.gen_param(),
            method_option,
            need_tenant_access_token: true,
            need_user_access_token: true,
            ..Default::default()
        };

        let (resp, common_resp): (DeleteChatMemberRespInner, _) = self.cli.do_req(req).await?;
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
pub struct DeleteChatMemberReq {
    /// 群 ID，详情参见[群ID 说明](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/im-v1/chat-id-description)
    ///
    /// **注意**：仅支持群模式为`group`、`topic`的群组ID
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
    /// `open_id`: 标识一个用户在某个应用中的身份。同一个用户在不同应用中的 Open ID 不同。[了解更多：如何获取 Open ID？](https://open.feishu.cn/document/uAjLw4CM/ugTN1YjL4UTN24CO1UjN/trouble-shooting/how-to-obtain-openid)
    ///
    /// `union_id`: 标识一个用户在某个应用开发商下的身份。同一用户在同一开发商下的应用中的 Union ID 是相同的，在不同开发商下的应用中的 Union ID 是不同的。通过 Union ID，应用开发商可以把同个用户在多个应用中的身份关联起来。[了解更多：如何获取 Union ID？](https://open.feishu.cn/document/uAjLw4CM/ugTN1YjL4UTN24CO1UjN/trouble-shooting/how-to-obtain-union-id)
    ///
    /// `user_id`: 标识一个用户在某个租户内的身份。同一个用户在租户 A 和租户 B 内的 User ID 是不同的。在同一个租户内，一个用户的 User ID 在所有应用（包括商店应用）中都保持一致。User ID 主要用于在不同的应用间打通用户数据。[了解更多：如何获取 User ID？](https://open.feishu.cn/document/uAjLw4CM/ugTN1YjL4UTN24CO1UjN/trouble-shooting/how-to-obtain-user-id)
    ///
    /// `app_id`: 飞书开放平台应用的唯一标识。在创建应用时，由系统自动生成，用户不能自行修改。[了解更多：如何获取应用的 App ID？](https://open.feishu.cn/document/uAjLw4CM/ugTN1YjL4UTN24CO1UjN/trouble-shooting/how-to-obtain-app-id)
    #[api(
        kind = "query",
        name = "member_id_type",
        v_type = "var",
        option = "false"
    )]
    pub member_id_type: String,
    /// 成员列表。移除群内的用户时推荐使用 OpenID，获取方式可参考文档[如何获取 Open ID？](https://open.feishu.cn/document/uAjLw4CM/ugTN1YjL4UTN24CO1UjN/trouble-shooting/how-to-obtain-openid)；移除群内的机器人时需填写应用的App ID，请参考[如何获取应用的 App ID？](https://open.feishu.cn/document/uAjLw4CM/ugTN1YjL4UTN24CO1UjN/trouble-shooting/how-to-obtain-app-id)
    ///
    /// **注意**：
    ///
    /// - 成员列表不可为空
    ///
    /// - 列表中填写的成员ID类型应与 ==member_id_type== 参数中选择的类型相对应
    ///
    /// **示例值**: "["ou_9204a37300b3700d61effaa439f34295"]"
    #[api(kind = "body", name = "id_list")]
    pub id_list: Vec<Option<String>>,
}

#[derive(Debug, Serialize, Deserialize, Clone, lark_bot_sdk_macros::ApiBaseResp)]
struct DeleteChatMemberRespInner {
    #[serde(flatten)]
    data: Option<DeleteChatMemberResp>,
    #[serde(flatten)]
    base: BaseResp,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct DeleteChatMemberResp {
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
    /// 无效成员列表
    #[serde(
        rename = "invalid_id_list",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub invalid_id_list: Vec<String>,
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
        Fn(DeleteChatMemberReq) -> Result<(DeleteChatMemberResp, CommonResponse), Error>
        + Send
        + Sync
        + 'static
    {
    }
    impl<
            T: Fn(DeleteChatMemberReq) -> Result<(DeleteChatMemberResp, CommonResponse), Error>
                + Send
                + Sync
                + 'static,
        > MockFunc for T
    {
    }

    impl<'c, IStore: Store, IClient: HttpClient> ChatServiceMocker<'c, IStore, IClient> {
        pub fn mock_delete_chat_member<F: MockFunc>(
            &self,
            f: F,
        ) -> MockerBuilder<Mocker, DeleteChatMemberReq, DeleteChatMemberResp, Arc<dyn MockFunc>>
        {
            MockerBuilder::new(self.cli.instance_id, Arc::new(f))
        }

        pub(super) fn get_mock_delete_chat_member(
            &self,
            req: &DeleteChatMemberReq,
        ) -> Option<Arc<dyn MockFunc>> {
            do_mock::<Mocker, DeleteChatMemberReq, DeleteChatMemberResp, Arc<dyn MockFunc>>(
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
        api::gen::chat::delete_chat_member::{
            DeleteChatMemberReq, DeleteChatMemberResp, DeleteChatMemberRespInner,
        },
        core::{model::CommonResponse, Lark},
    };

    #[tokio::test]
    async fn test_mock() {
        let lark = Lark::new("".to_owned(), "".into());
        let mocker = lark
            .chat()
            .mock()
            .mock_delete_chat_member(|_| {
                Ok((DeleteChatMemberResp::default(), CommonResponse::default()))
            })
            .build();
        let res = lark
            .chat()
            .delete_chat_member(DeleteChatMemberReq::default())
            .await;
        assert!(res.is_ok());
        mocker.clear();
        let res = lark
            .chat()
            .delete_chat_member(DeleteChatMemberReq::default())
            .await;
        assert!(res.is_err());
    }

    const REQ: &str = r#"{
    "id_list": [
        "4d7a3c6g"
    ]
}"#;

    #[test]
    fn test_req() {
        if REQ == "{}" {
            return;
        }
        if let Err(e) = serde_json::from_str::<super::DeleteChatMemberReqBody>(REQ) {
            panic!("{}", e);
        }
    }

    const RESP: &str = r#"{
    "code": 0,
    "msg": "success",
    "data": {
        "invalid_id_list": [
            "4d7a3c6g"
        ]
    }
}"#;
    #[test]
    fn test_resp() {
        let res = serde_json::from_str::<DeleteChatMemberRespInner>(RESP);
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
