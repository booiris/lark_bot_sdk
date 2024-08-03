//! doc url: <https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/im-v1/chat-announcement/patch>
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
    /// **api 版本: 2024-06-21T09:04:28+00:00**
    ///
    /// ## 更新群公告信息
    ///
    /// 更新会话中的群公告信息，更新公告信息的格式和更新[旧版云文档](https://open.feishu.cn/document/ukTMukTMukTM/uAzM5YjLwMTO24CMzkjN)格式相同，不支持新版文档格式。
    ///
    /// 注意事项：
    ///
    /// - 应用需要开启[机器人能力](https://open.feishu.cn/document/uAjLw4CM/ugTN1YjL4UTN24CO1UjN/trouble-shooting/how-to-enable-bot-ability)
    ///
    /// - 机器人或授权用户必须在群里
    ///
    /// - 操作者需要拥有群公告文档的阅读权限
    ///
    /// - 获取内部群信息时，操作者须与群组在同一租户下
    ///
    /// - 若群开启了 ==仅群主和群管理员可编辑群信息== 配置，群主/群管理员 或 创建群组且具备 ==更新应用所创建群的群信息== 权限的机器人，可更新群公告
    ///
    /// - 若群未开启 ==仅群主和群管理员可编辑群信息== 配置，所有成员可以更新群公告
    ///
    /// doc: <https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/im-v1/chat-announcement/patch>
    ///
    /// new doc: <https://open.feishu.cn/document/server-docs/group/chat-announcement/patch>
    ///
    /// api url: <https://open.feishu.cn/document_portal/v1/document/get_detail?fullPath=%2Fserver-docs%2Fgroup%2Fchat-announcement%2Fpatch>
    pub async fn update_chat_announcement(
        &self,
        req: UpdateChatAnnouncementReq,
    ) -> Result<(UpdateChatAnnouncementResp, CommonResponse), Error> {
        self.update_chat_announcement_with_opt(req, Default::default())
            .await
    }

    /// 参见 [update_chat_announcement](#method.update_chat_announcement) 函数
    pub async fn update_chat_announcement_with_opt(
        &self,
        req: UpdateChatAnnouncementReq,
        method_option: MethodOption,
    ) -> Result<(UpdateChatAnnouncementResp, CommonResponse), Error> {
        #[cfg(feature = "test-util")]
        {
            if let Some(f) = self.mock().get_mock_update_chat_announcement(&req) {
                tracing::info!("[lark] Chat#UpdateChatAnnouncement **mocking** api");
                return f(req);
            }
        }

        tracing::info!("[lark] Chat#UpdateChatAnnouncement call api");

        let req = ApiRequest {
            scope: "Chat",
            api: "UpdateChatAnnouncement",
            method: http::Method::PATCH,
            url: String::new()
                + self.cli.open_base_url.as_ref()
                + "/open-apis/im/v1/chats/:chat_id/announcement",
            param_data: req.gen_param(),
            method_option,
            need_tenant_access_token: true,
            need_user_access_token: true,
            ..Default::default()
        };

        let (resp, common_resp): (UpdateChatAnnouncementRespInner, _) =
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
pub struct UpdateChatAnnouncementReq {
    /// 待修改公告的群 ID，详情参见[群ID 说明](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/im-v1/chat-id-description)
    ///
    /// **注意**：不支持P2P单聊
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "oc_5ad11d72b830411d72b836c20"
    #[api(kind = "path", name = "chat_id")]
    pub chat_id: String,

    /// 文档当前版本号 int64 类型，[获取群公告信息](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/im-v1/chat-announcement/get)接口会返回
    ///
    /// **注意**：传入的版本号和最新版本号的差距不能超过100
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "12"
    #[api(kind = "body", name = "revision")]
    pub revision: String,
    /// 修改文档请求的序列化字段
    ///
    /// 更新公告信息的格式和更新[云文档](https://open.feishu.cn/document/ukTMukTMukTM/uYDM2YjL2AjN24iNwYjN)格式相同
    ///
    /// **示例值**: "{\"requestType\":\"InsertBlocksRequestType\",\"insertBlocksRequest\":{\"payload\":\"{\\\"blocks\\\":[{\\\"type\\\":\\\"paragraph\\\",\\\"paragraph\\\":{\\\"elements\\\":[{\\\"type\\\":\\\"textRun\\\",\\\"textRun\\\":{\\\"text\\\":\\\"ylyyyyyDocs API Sample Content\\\",\\\"style\\\":{}}}],\\\"style\\\":{}}}]}\",\"location\":{\"zoneId\":\"0\",\"index\":0, \"endOfZone\": true}}}"
    #[api(kind = "body", name = "requests")]
    pub requests: Vec<Option<String>>,
}

#[derive(Debug, Serialize, Deserialize, Clone, lark_bot_sdk_macros::ApiBaseResp)]
struct UpdateChatAnnouncementRespInner {
    #[serde(flatten)]
    data: Option<UpdateChatAnnouncementResp>,
    #[serde(flatten)]
    base: BaseResp,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct UpdateChatAnnouncementResp {
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

    use self::gen::chat::ChatServiceMocker;
    use crate::core::mocker::*;

    use super::*;
    use crate::api::*;
    use crate::core::model::*;

    pub struct Mocker;

    pub trait MockFunc:
        Fn(UpdateChatAnnouncementReq) -> Result<(UpdateChatAnnouncementResp, CommonResponse), Error>
        + Send
        + Sync
        + 'static
    {
    }
    impl<
            T: Fn(
                    UpdateChatAnnouncementReq,
                ) -> Result<(UpdateChatAnnouncementResp, CommonResponse), Error>
                + Send
                + Sync
                + 'static,
        > MockFunc for T
    {
    }

    impl<'c, IStore: Store, IClient: HttpClient> ChatServiceMocker<'c, IStore, IClient> {
        pub fn mock_update_chat_announcement<F: MockFunc>(
            &self,
            f: F,
        ) -> MockerBuilder<
            Mocker,
            UpdateChatAnnouncementReq,
            UpdateChatAnnouncementResp,
            Arc<dyn MockFunc>,
        > {
            MockerBuilder::new(self.cli.instance_id, Arc::new(f))
        }

        pub(super) fn get_mock_update_chat_announcement(
            &self,
            req: &UpdateChatAnnouncementReq,
        ) -> Option<Arc<dyn MockFunc>> {
            do_mock::<
                Mocker,
                UpdateChatAnnouncementReq,
                UpdateChatAnnouncementResp,
                Arc<dyn MockFunc>,
            >(self.cli.instance_id, req)
        }
    }
}

#[cfg(feature = "test-util")]
#[cfg(test)]
mod test {
    use crate::{
        api::gen::chat::update_chat_announcement::{
            UpdateChatAnnouncementReq, UpdateChatAnnouncementResp, UpdateChatAnnouncementRespInner,
        },
        core::{model::CommonResponse, Lark},
    };

    #[tokio::test]
    async fn test_mock() {
        let lark = Lark::new("".to_owned(), "".into());
        let mocker = lark
            .chat()
            .mock()
            .mock_update_chat_announcement(|_| {
                Ok((
                    UpdateChatAnnouncementResp::default(),
                    CommonResponse::default(),
                ))
            })
            .build();
        let res = lark
            .chat()
            .update_chat_announcement(UpdateChatAnnouncementReq::default())
            .await;
        assert!(res.is_ok());
        mocker.clear();
        let res = lark
            .chat()
            .update_chat_announcement(UpdateChatAnnouncementReq::default())
            .await;
        assert!(res.is_err());
    }

    const REQ: &str = r#"{
    "revision": "12",
    "requests": [
        "{\"requestType\":\"InsertBlocksRequestType\",\"insertBlocksRequest\":{\"payload\":\"{\\\"blocks\\\":[{\\\"type\\\":\\\"paragraph\\\",\\\"paragraph\\\":{\\\"elements\\\":[{\\\"type\\\":\\\"textRun\\\",\\\"textRun\\\":{\\\"text\\\":\\\"Docs API Sample Content\\\",\\\"style\\\":{}}}],\\\"style\\\":{}}}]}\",\"location\":{\"zoneId\":\"0\",\"index\":0, \"endOfZone\": true}}}"
    ]
}"#;

    #[test]
    fn test_req() {
        if REQ == "{}" {
            return;
        }
        if let Err(e) = serde_json::from_str::<super::UpdateChatAnnouncementReqBody>(REQ) {
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
        let res = serde_json::from_str::<UpdateChatAnnouncementRespInner>(RESP);
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
