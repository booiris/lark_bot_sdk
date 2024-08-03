//! doc url: <https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/im-v1/chat-menu_tree/delete>
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
    /// **api 版本: 2024-06-21T09:04:29+00:00**
    ///
    /// ## 删除群菜单
    ///
    /// 该接口用于删除群内已经添加的群菜单。
    ///
    /// 注意事项：
    ///
    /// - 应用需要开启[机器人能力](https://open.feishu.cn/document/uAjLw4CM/ugTN1YjL4UTN24CO1UjN/trouble-shooting/how-to-enable-bot-ability)。
    ///
    /// - 机器人必须在群里。
    ///
    /// - 操作API后，将返回群内所有菜单。
    ///
    /// doc: <https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/im-v1/chat-menu_tree/delete>
    ///
    /// new doc: <https://open.feishu.cn/document/server-docs/group/chat-menu_tree/delete>
    ///
    /// api url: <https://open.feishu.cn/document_portal/v1/document/get_detail?fullPath=%2Fserver-docs%2Fgroup%2Fchat-menu_tree%2Fdelete>
    pub async fn delete_chat_menu_tree(
        &self,
        req: DeleteChatMenuTreeReq,
    ) -> Result<(DeleteChatMenuTreeResp, CommonResponse), Error> {
        self.delete_chat_menu_tree_with_opt(req, Default::default())
            .await
    }

    /// 参见 [delete_chat_menu_tree](#method.delete_chat_menu_tree) 函数
    pub async fn delete_chat_menu_tree_with_opt(
        &self,
        req: DeleteChatMenuTreeReq,
        method_option: MethodOption,
    ) -> Result<(DeleteChatMenuTreeResp, CommonResponse), Error> {
        #[cfg(feature = "test-util")]
        {
            if let Some(f) = self.mock().get_mock_delete_chat_menu_tree(&req) {
                tracing::info!("[lark] Chat#DeleteChatMenuTree **mocking** api");
                return f(req);
            }
        }

        tracing::info!("[lark] Chat#DeleteChatMenuTree call api");

        let req = ApiRequest {
            scope: "Chat",
            api: "DeleteChatMenuTree",
            method: http::Method::DELETE,
            url: String::new()
                + self.cli.open_base_url.as_ref()
                + "/open-apis/im/v1/chats/:chat_id/menu_tree",
            param_data: req.gen_param(),
            method_option,
            need_tenant_access_token: true,
            ..Default::default()
        };

        let (resp, common_resp): (DeleteChatMenuTreeRespInner, _) = self.cli.do_req(req).await?;
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
pub struct DeleteChatMenuTreeReq {
    /// 群ID，详情参见[群ID 说明](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/im-v1/chat-id-description)
    ///
    /// **注意**：仅支持群模式为`group`的群ID
    ///
    /// **示例值**: "oc_a0553eda9014c201e6969b478895c230"
    #[api(kind = "path", name = "chat_id")]
    pub chat_id: String,

    /// 群内要删除的一级菜单ID。通过 [获取群菜单](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/im-v1/chat-menu_tree/get) 接口获取群内菜单详情。
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "7156553273518882844"
    #[api(kind = "body", name = "chat_menu_top_level_ids")]
    pub chat_menu_top_level_ids: Vec<Option<String>>,
}

#[derive(Debug, Serialize, Deserialize, Clone, lark_bot_sdk_macros::ApiBaseResp)]
struct DeleteChatMenuTreeRespInner {
    #[serde(flatten)]
    data: Option<DeleteChatMenuTreeResp>,
    #[serde(flatten)]
    base: BaseResp,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct DeleteChatMenuTreeResp {
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
    /// 群内现有菜单
    #[serde(
        rename = "menu_tree",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub menu_tree: ChatMenuTreeSubResp,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(default)]
pub struct ChatMenuTreeSubResp {
    /// 一级菜单列表
    #[serde(
        rename = "chat_menu_top_levels",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub chat_menu_top_levels: Vec<ChatMenuTopLevelSubResp>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(default)]
pub struct ChatMenuTopLevelSubResp {
    /// 一级菜单ID
    ///
    /// **示例值**: "7117116451961487361"
    #[serde(
        rename = "chat_menu_top_level_id",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub chat_menu_top_level_id: String,
    /// 一级菜单信息
    #[serde(
        rename = "chat_menu_item",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub chat_menu_item: ChatMenuItemSubResp,
    /// 二级菜单列表
    #[serde(
        rename = "children",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub children: Vec<ChatMenuSecondLevelSubResp>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(default)]
pub struct ChatMenuItemSubResp {
    /// 菜单类型
    ///
    /// **注意**
    ///
    /// - 如果一级菜单有二级菜单时，则此一级菜单的值必须为NONE。
    ///
    /// **示例值**: "NONE"
    ///
    /// **可选值**:
    ///
    /// `NONE`: 无类型
    ///
    /// `REDIRECT_LINK`: 跳转链接类型
    #[serde(
        rename = "action_type",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub action_type: String,
    /// 跳转链接
    #[serde(
        rename = "redirect_link",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub redirect_link: ChatMenuItemRedirectLinkSubResp,
    /// 图片的key值。通过 [上传图片](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/im-v1/image/create) 接口上传message类型图片获取image_key
    ///
    /// **注意**
    ///
    /// - 如果一级菜单有二级菜单，则此一级菜单不能有图标。
    ///
    /// **示例值**: "img_v2_b0fbe905-7988-4282-b882-82edd010336j"
    #[serde(
        rename = "image_key",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub image_key: String,
    /// 菜单名称。
    ///
    /// **注意**
    ///
    /// - 一级、二级菜单名称字符数要在1到120范围内
    ///
    /// **示例值**: "评审报名"
    #[serde(
        rename = "name",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub name: String,
    /// 菜单国际化名称。
    ///
    /// **注意**
    ///
    /// - 一级、二级菜单名称字符数要在1到120范围内
    #[serde(
        rename = "i18n_names",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub i18n_names: I18nNamesSubResp,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(default)]
pub struct ChatMenuItemRedirectLinkSubResp {
    /// 公用跳转链接，必须以http开头。
    ///
    /// **示例值**: "https://open.feishu.cn/"
    #[serde(
        rename = "common_url",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub common_url: String,
    /// IOS端跳转链接，当该字段不设置时，IOS端会使用common_url。必须以http开头。
    ///
    /// **示例值**: "https://open.feishu.cn/"
    #[serde(
        rename = "ios_url",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub ios_url: String,
    /// Android端跳转链接，当该字段不设置时，Android端会使用common_url。必须以http开头。
    ///
    /// **示例值**: "https://open.feishu.cn/"
    #[serde(
        rename = "android_url",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub android_url: String,
    /// PC端跳转链接，当该字段不设置时，PC端会使用common_url。必须以http开头。在PC端点击群菜单后，如果需要url对应的页面在飞书侧边栏展开，可以在url前加上https://applink.feishu.cn/client/web_url/open?mode=sidebar-semi&url=，比如https://applink.feishu.cn/client/web_url/open?mode=sidebar-semi&url=https://open.feishu.cn/
    ///
    /// **示例值**: "https://open.feishu.cn/"
    #[serde(
        rename = "pc_url",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub pc_url: String,
    /// Web端跳转链接，当该字段不设置时，Web端会使用common_url。必须以http开头。
    ///
    /// **示例值**: "https://open.feishu.cn/"
    #[serde(
        rename = "web_url",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub web_url: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(default)]
pub struct I18nNamesSubResp {
    /// 中文名
    ///
    /// **示例值**: "评审报名"
    #[serde(
        rename = "zh_cn",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub zh_cn: String,
    /// 英文名
    ///
    /// **示例值**: "Sign up"
    #[serde(
        rename = "en_us",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub en_us: String,
    /// 日文名
    ///
    /// **示例值**: "サインアップ"
    #[serde(
        rename = "ja_jp",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub ja_jp: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(default)]
pub struct ChatMenuSecondLevelSubResp {
    /// 二级菜单ID
    ///
    /// **示例值**: "7039638308221468675"
    #[serde(
        rename = "chat_menu_second_level_id",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub chat_menu_second_level_id: String,
    /// 二级菜单信息
    #[serde(
        rename = "chat_menu_item",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub chat_menu_item: ChatMenuItemSubResp,
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
        Fn(DeleteChatMenuTreeReq) -> Result<(DeleteChatMenuTreeResp, CommonResponse), Error>
        + Send
        + Sync
        + 'static
    {
    }
    impl<
            T: Fn(DeleteChatMenuTreeReq) -> Result<(DeleteChatMenuTreeResp, CommonResponse), Error>
                + Send
                + Sync
                + 'static,
        > MockFunc for T
    {
    }

    impl<'c, IStore: Store, IClient: HttpClient> ChatServiceMocker<'c, IStore, IClient> {
        pub fn mock_delete_chat_menu_tree<F: MockFunc>(
            &self,
            f: F,
        ) -> MockerBuilder<Mocker, DeleteChatMenuTreeReq, DeleteChatMenuTreeResp, Arc<dyn MockFunc>>
        {
            MockerBuilder::new(self.cli.instance_id, Arc::new(f))
        }

        pub(super) fn get_mock_delete_chat_menu_tree(
            &self,
            req: &DeleteChatMenuTreeReq,
        ) -> Option<Arc<dyn MockFunc>> {
            do_mock::<Mocker, DeleteChatMenuTreeReq, DeleteChatMenuTreeResp, Arc<dyn MockFunc>>(
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
        api::gen::chat::delete_chat_menu_tree::{
            DeleteChatMenuTreeReq, DeleteChatMenuTreeResp, DeleteChatMenuTreeRespInner,
        },
        core::{model::CommonResponse, Lark},
    };

    #[tokio::test]
    async fn test_mock() {
        let lark = Lark::new("".to_owned(), "".into());
        let mocker = lark
            .chat()
            .mock()
            .mock_delete_chat_menu_tree(|_| {
                Ok((DeleteChatMenuTreeResp::default(), CommonResponse::default()))
            })
            .build();
        let res = lark
            .chat()
            .delete_chat_menu_tree(DeleteChatMenuTreeReq::default())
            .await;
        assert!(res.is_ok());
        mocker.clear();
        let res = lark
            .chat()
            .delete_chat_menu_tree(DeleteChatMenuTreeReq::default())
            .await;
        assert!(res.is_err());
    }

    const REQ: &str = r#"{
    "chat_menu_top_level_ids": [
        "6936075528890826780"
    ]
}"#;

    #[test]
    fn test_req() {
        if REQ == "{}" {
            return;
        }
        if let Err(e) = serde_json::from_str::<super::DeleteChatMenuTreeReqBody>(REQ) {
            panic!("{}", e);
        }
    }

    const RESP: &str = r#"{
    "code": 0,
    "msg": "success",
    "data": {
        "menu_tree": {
            "chat_menu_top_levels": [
                {
                    "chat_menu_top_level_id": "7117116451961487361",
                    "chat_menu_item": {
                        "action_type": "NONE",
                        "redirect_link": {
                            "common_url": "https://open.feishu.cn/",
                            "ios_url": "https://open.feishu.cn/",
                            "android_url": "https://open.feishu.cn/",
                            "pc_url": "https://open.feishu.cn/",
                            "web_url": "https://open.feishu.cn/"
                        },
                        "image_key": "img_v2_b0fbe905-7988-4282-b882-82edd010336j",
                        "name": "菜单",
                        "i18n_names": {
                            "zh_cn": "菜单",
                            "en_us": "Menu",
                            "ja_jp": "メニュー"
                        }
                    },
                    "children": [
                        {
                            "chat_menu_second_level_id": "7039638308221468675",
                            "chat_menu_item": {
                                "action_type": "REDIRECT_LINK",
                                "redirect_link": {
                                    "common_url": "https://open.feishu.cn/",
                                    "ios_url": "https://open.feishu.cn/",
                                    "android_url": "https://open.feishu.cn/",
                                    "pc_url": "https://open.feishu.cn/",
                                    "web_url": "https://open.feishu.cn/"
                                },
                                "image_key": "img_v2_b0fbe905-7988-4282-b882-82edd010336j",
                                "name": "报名",
                                "i18n_names": {
                                    "zh_cn": "报名",
                                    "en_us": "Sign up",
                                    "ja_jp": "サインアップ"
                                }
                            }
                        }
                    ]
                }
            ]
        }
    }
}"#;
    #[test]
    fn test_resp() {
        let res = serde_json::from_str::<DeleteChatMenuTreeRespInner>(RESP);
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
