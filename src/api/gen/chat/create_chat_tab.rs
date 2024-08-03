//! doc url: <https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/im-v1/chat-tab/create>
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
    /// **api 版本: 2024-07-15T08:05:45+00:00**
    ///
    /// ## 添加会话标签页
    ///
    /// 添加自定义会话标签页。
    ///
    /// 注意事项：
    ///
    /// - 应用需要开启[机器人能力](https://open.feishu.cn/document/uAjLw4CM/ugTN1YjL4UTN24CO1UjN/trouble-shooting/how-to-enable-bot-ability)
    ///
    /// - 机器人或授权用户必须在群里
    ///
    /// - 只允许添加类型为`doc`和`url`的会话标签页
    ///
    /// - 添加doc类型时，操作者（access token对应的身份）需要拥有对应文档的权限
    ///
    /// - tab_config字段当前只对`url`类型的会话标签页生效
    ///
    /// - 在开启 ==仅群主和管理员可管理标签页== 的设置时，仅群主和群管理员可以添加会话标签页
    ///
    /// - 操作内部群时，操作者须与群组在同一租户下
    ///
    /// doc: <https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/im-v1/chat-tab/create>
    ///
    /// new doc: <https://open.feishu.cn/document/server-docs/group/chat-tab/create>
    ///
    /// api url: <https://open.feishu.cn/document_portal/v1/document/get_detail?fullPath=%2Fserver-docs%2Fgroup%2Fchat-tab%2Fcreate>
    pub async fn create_chat_tab(
        &self,
        req: CreateChatTabReq,
    ) -> Result<(CreateChatTabResp, CommonResponse), Error> {
        self.create_chat_tab_with_opt(req, Default::default()).await
    }

    /// 参见 [create_chat_tab](#method.create_chat_tab) 函数
    pub async fn create_chat_tab_with_opt(
        &self,
        req: CreateChatTabReq,
        method_option: MethodOption,
    ) -> Result<(CreateChatTabResp, CommonResponse), Error> {
        #[cfg(feature = "test-util")]
        {
            if let Some(f) = self.mock().get_mock_create_chat_tab(&req) {
                tracing::info!("[lark] Chat#CreateChatTab **mocking** api");
                return f(req);
            }
        }

        tracing::info!("[lark] Chat#CreateChatTab call api");

        let req = ApiRequest {
            scope: "Chat",
            api: "CreateChatTab",
            method: http::Method::POST,
            url: String::new()
                + self.cli.open_base_url.as_ref()
                + "/open-apis/im/v1/chats/:chat_id/chat_tabs",
            param_data: req.gen_param(),
            method_option,
            need_tenant_access_token: true,
            need_user_access_token: true,
            ..Default::default()
        };

        let (resp, common_resp): (CreateChatTabRespInner, _) = self.cli.do_req(req).await?;
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
pub struct CreateChatTabReq {
    /// 群ID，详情参见[群ID 说明](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/im-v1/chat-id-description)
    ///
    /// **注意**：支持群模式为`p2p`与`group`的群ID
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "oc_a0553eda9014c201e6969b478895c230"
    #[api(kind = "path", name = "chat_id")]
    pub chat_id: String,

    /// 会话标签页
    ///
    /// **注意**：一个群内最多只允许添加20个自定义会话标签页
    ///
    /// **是否必填**: 是
    #[api(kind = "body", name = "chat_tabs")]
    pub chat_tabs: Vec<Option<ChatTabSubReq>>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(default)]
pub struct ChatTabSubReq {
    /// Tab名称
    ///
    /// **注意**：会话标签页的名称不能超过30个字符（最多 10 个汉字）
    ///
    /// **示例值**: "文档"
    #[serde(
        rename = "tab_name",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub tab_name: Option<String>,
    /// Tab类型
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "doc"
    ///
    /// **可选值**:
    ///
    /// `Message`: 消息类型
    ///
    /// `DocList`: 云文档列表
    ///
    /// `Doc`: 文档
    ///
    /// `Pin`: Pin
    ///
    /// `MeetingMinute`: 会议纪要
    ///
    /// `ChatAnnouncement`: 群公告
    ///
    /// `URL`: URL
    ///
    /// `File`: 文件
    ///
    /// `Files_Resources`: 合并类型, 包含文件、Doc文档、URL链接
    ///
    /// `Images_Videos`: 合并类型，包含图片、视频
    #[serde(
        rename = "tab_type",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub tab_type: String,
    /// Tab内容
    #[serde(
        rename = "tab_content",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub tab_content: Option<ChatTabContentSubReq>,
    /// Tab的配置
    #[serde(
        rename = "tab_config",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub tab_config: Option<ChatTabConfigSubReq>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(default)]
pub struct ChatTabContentSubReq {
    /// URL类型
    ///
    /// **示例值**: "https://www.feishu.cn"
    #[serde(
        rename = "url",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub url: Option<String>,
    /// Doc链接
    ///
    /// **示例值**: "https://example.feishu.cn/wiki/wikcnPIcqWjJQwkwDzrB9t40123xz"
    #[serde(
        rename = "doc",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub doc: Option<String>,
    /// 会议纪要
    ///
    /// **示例值**: "https://example.feishu.cn/docs/doccnvIXbV22i6hSD3utar4123dx"
    #[serde(
        rename = "meeting_minute",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub meeting_minute: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(default)]
pub struct ChatTabConfigSubReq {
    /// 群Tab图标
    ///
    /// **示例值**: "img_v2_b99741-7628-4abd-aad0-b881e4db83ig"
    #[serde(
        rename = "icon_key",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub icon_key: Option<String>,
    /// 群tab是否App内嵌打开
    ///
    /// **示例值**: "false"
    #[serde(
        rename = "is_built_in",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub is_built_in: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize, Clone, lark_bot_sdk_macros::ApiBaseResp)]
struct CreateChatTabRespInner {
    #[serde(flatten)]
    data: Option<CreateChatTabResp>,
    #[serde(flatten)]
    base: BaseResp,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct CreateChatTabResp {
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
    /// 会话标签页
    #[serde(
        rename = "chat_tabs",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub chat_tabs: Vec<ChatTabSubResp>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(default)]
pub struct ChatTabSubResp {
    /// Tab ID
    ///
    /// **示例值**: "7101214603622940671"
    #[serde(
        rename = "tab_id",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub tab_id: String,
    /// Tab名称
    ///
    /// **注意**：会话标签页的名称不能超过30个字符
    ///
    /// **示例值**: "文档"
    #[serde(
        rename = "tab_name",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub tab_name: String,
    /// Tab类型
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "doc"
    ///
    /// **可选值**:
    ///
    /// `Message`: 消息类型
    ///
    /// `DocList`: 云文档列表
    ///
    /// `Doc`: 文档
    ///
    /// `Pin`: Pin
    ///
    /// `MeetingMinute`: 会议纪要
    ///
    /// `ChatAnnouncement`: 群公告
    ///
    /// `URL`: URL
    ///
    /// `File`: 文件
    ///
    /// `Files_Resources`: 合并类型, 包含文件、Doc文档、URL链接
    ///
    /// `Images_Videos`: 合并类型，包含图片、视频
    #[serde(
        rename = "tab_type",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub tab_type: String,
    /// Tab内容
    #[serde(
        rename = "tab_content",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub tab_content: ChatTabContentSubResp,
    /// Tab的配置
    #[serde(
        rename = "tab_config",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub tab_config: ChatTabConfigSubResp,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(default)]
pub struct ChatTabContentSubResp {
    /// URL类型
    ///
    /// **示例值**: "https://www.feishu.cn"
    #[serde(
        rename = "url",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub url: String,
    /// Doc链接
    ///
    /// **示例值**: "https://example.feishu.cn/wiki/wikcnPIcqWjJQwkwDzrB9t40123xz"
    #[serde(
        rename = "doc",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub doc: String,
    /// 会议纪要
    ///
    /// **示例值**: "https://example.feishu.cn/docs/doccnvIXbV22i6hSD3utar4123dx"
    #[serde(
        rename = "meeting_minute",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub meeting_minute: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(default)]
pub struct ChatTabConfigSubResp {
    /// 群Tab图标
    ///
    /// **示例值**: "img_v2_b99741-7628-4abd-aad0-b881e4db83ig"
    #[serde(
        rename = "icon_key",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub icon_key: String,
    /// 群tab是否App内嵌打开
    ///
    /// **示例值**: "false"
    #[serde(
        rename = "is_built_in",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub is_built_in: bool,
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
        Fn(CreateChatTabReq) -> Result<(CreateChatTabResp, CommonResponse), Error>
        + Send
        + Sync
        + 'static
    {
    }
    impl<
            T: Fn(CreateChatTabReq) -> Result<(CreateChatTabResp, CommonResponse), Error>
                + Send
                + Sync
                + 'static,
        > MockFunc for T
    {
    }

    impl<'c, IStore: Store, IClient: HttpClient> ChatServiceMocker<'c, IStore, IClient> {
        pub fn mock_create_chat_tab<F: MockFunc>(
            &self,
            f: F,
        ) -> MockerBuilder<Mocker, CreateChatTabReq, CreateChatTabResp, Arc<dyn MockFunc>> {
            MockerBuilder::new(self.cli.instance_id, Arc::new(f))
        }

        pub(super) fn get_mock_create_chat_tab(
            &self,
            req: &CreateChatTabReq,
        ) -> Option<Arc<dyn MockFunc>> {
            do_mock::<Mocker, CreateChatTabReq, CreateChatTabResp, Arc<dyn MockFunc>>(
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
        api::gen::chat::create_chat_tab::{
            CreateChatTabReq, CreateChatTabResp, CreateChatTabRespInner,
        },
        core::{model::CommonResponse, Lark},
    };

    #[tokio::test]
    async fn test_mock() {
        let lark = Lark::new("".to_owned(), "".into());
        let mocker = lark
            .chat()
            .mock()
            .mock_create_chat_tab(|_| Ok((CreateChatTabResp::default(), CommonResponse::default())))
            .build();
        let res = lark
            .chat()
            .create_chat_tab(CreateChatTabReq::default())
            .await;
        assert!(res.is_ok());
        mocker.clear();
        let res = lark
            .chat()
            .create_chat_tab(CreateChatTabReq::default())
            .await;
        assert!(res.is_err());
    }

    const REQ: &str = r#"{
    "chat_tabs": [
        {
            "tab_name": "文档",
            "tab_type": "doc",
            "tab_content": {
                "url": "https://www.feishu.cn",
                "doc": "https://example.feishu.cn/wiki/wikcnPIcqWjJQwkwDzrB9t40123xz",
                "meeting_minute": "https://example.feishu.cn/docs/doccnvIXbV22i6hSD3utar4123dx"
            },
            "tab_config": {
                "icon_key": "img_v2_b99741-7628-4abd-aad0-b881e4db83ig",
                "is_built_in": false
            }
        }
    ]
}"#;

    #[test]
    fn test_req() {
        if REQ == "{}" {
            return;
        }
        if let Err(e) = serde_json::from_str::<super::CreateChatTabReqBody>(REQ) {
            panic!("{}", e);
        }
    }

    const RESP: &str = r#"{
    "code": 0,
    "msg": "ok",
    "data": {
        "chat_tabs": [
            {
               "tab_id": "7101214603622940633",
               "tab_type": "message"
            },
            {
                "tab_id": "7101214603622940671",
                "tab_name": "文档",
                "tab_type": "doc",
                "tab_content": {
                    "doc": "https://example.feishu.cn/wiki/wikcnPIcqWjJQwkwDzrB9t40123xz"
                }
            },
            {
                "tab_id": "7158333373373759422",
                "tab_name": "测试",
                "tab_type": "url",
                "tab_content": {
                    "url": "https://www.test.cn"
                },
                "tab_config": {
                    "icon_key": "img_v2_b99741-7628-4abd-aad0-b881e4db83ig",
                    "is_built_in": true
                }
            }
        ]
    }
}"#;
    #[test]
    fn test_resp() {
        let res = serde_json::from_str::<CreateChatTabRespInner>(RESP);
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
