//! doc url: <https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/im-v1/chat-menu_item/patch>
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
    /// ## 修改群菜单元信息
    ///
    /// 修改某个一级菜单或者二级菜单的元信息，包括群菜单的图标、名称、国际化名称和跳转链接。
    ///
    /// 注意事项：
    ///
    /// - 应用需要开启[机器人能力](https://open.feishu.cn/document/uAjLw4CM/ugTN1YjL4UTN24CO1UjN/trouble-shooting/how-to-enable-bot-ability)。
    ///
    /// - 机器人必须在群里。
    ///
    /// - 该API暂时不支持在一级菜单上添加或者删除二级菜单。
    ///
    /// doc: <https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/im-v1/chat-menu_item/patch>
    ///
    /// new doc: <https://open.feishu.cn/document/server-docs/group/chat-menu_tree/patch>
    ///
    /// api url: <https://open.feishu.cn/document_portal/v1/document/get_detail?fullPath=%2Fserver-docs%2Fgroup%2Fchat-menu_tree%2Fpatch>
    pub async fn update_chat_menu_tree(
        &self,
        req: UpdateChatMenuTreeReq,
    ) -> Result<(UpdateChatMenuTreeResp, CommonResponse), Error> {
        self.update_chat_menu_tree_with_opt(req, Default::default())
            .await
    }

    /// 参见 [update_chat_menu_tree](#method.update_chat_menu_tree) 函数
    pub async fn update_chat_menu_tree_with_opt(
        &self,
        req: UpdateChatMenuTreeReq,
        method_option: MethodOption,
    ) -> Result<(UpdateChatMenuTreeResp, CommonResponse), Error> {
        #[cfg(feature = "test-util")]
        {
            if let Some(f) = self.mock().get_mock_update_chat_menu_tree(&req) {
                tracing::info!("[lark] Chat#UpdateChatMenuTree **mocking** api");
                return f(req);
            }
        }

        tracing::info!("[lark] Chat#UpdateChatMenuTree call api");

        let req = ApiRequest {
            scope: "Chat",
            api: "UpdateChatMenuTree",
            method: http::Method::PATCH,
            url: String::new()
                + self.cli.open_base_url.as_ref()
                + "/open-apis/im/v1/chats/:chat_id/menu_items/:menu_item_id",
            param_data: req.gen_param(),
            method_option,
            need_tenant_access_token: true,
            ..Default::default()
        };

        let (resp, common_resp): (UpdateChatMenuTreeRespInner, _) = self.cli.do_req(req).await?;
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
pub struct UpdateChatMenuTreeReq {
    /// 群ID，详情参见[群ID 说明](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/im-v1/chat-id-description)
    ///
    /// **注意**：仅支持群模式为`group`的群ID
    ///
    /// **示例值**: "oc_a0553eda9014c201e6969b478895c230"
    #[api(kind = "path", name = "chat_id")]
    pub chat_id: String,
    /// 一级或二级菜单ID，通过 [获取群菜单](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/im-v1/chat-menu_tree/get) 接口通过群ID获取菜单ID。
    ///
    /// **示例值**: "7156553273518882844"
    #[api(kind = "path", name = "menu_item_id")]
    pub menu_item_id: String,

    /// 要修改的字段
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "["ICON"]"
    #[api(kind = "body", name = "update_fields")]
    pub update_fields: Vec<Option<String>>,
    /// 元信息
    ///
    /// **是否必填**: 是
    #[api(kind = "body", name = "chat_menu_item")]
    pub chat_menu_item: ChatMenuItemSubReq,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(default)]
pub struct ChatMenuItemSubReq {
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
    pub action_type: Option<String>,
    /// 跳转链接
    #[serde(
        rename = "redirect_link",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub redirect_link: Option<ChatMenuItemRedirectLinkSubReq>,
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
    pub image_key: Option<String>,
    /// 菜单名称。
    ///
    /// **注意**
    ///
    /// - 一级、二级菜单名称字符数要在1到120范围内
    ///
    /// **示例值**: "群聊"
    #[serde(
        rename = "name",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub name: Option<String>,
    /// 菜单国际化名称。
    ///
    /// **注意**
    ///
    /// - 一级、二级菜单名称字符数要在1到120范围内
    #[serde(
        rename = "i18n_names",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub i18n_names: Option<I18nNamesSubReq>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(default)]
pub struct ChatMenuItemRedirectLinkSubReq {
    /// 公用跳转链接，必须以http开头。
    ///
    /// **示例值**: "https://open.feishu.cn/"
    #[serde(
        rename = "common_url",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub common_url: Option<String>,
    /// IOS端跳转链接，当该字段不设置时，IOS端会使用common_url。必须以http开头。
    ///
    /// **示例值**: "https://open.feishu.cn/"
    #[serde(
        rename = "ios_url",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub ios_url: Option<String>,
    /// Android端跳转链接，当该字段不设置时，Android端会使用common_url。必须以http开头。
    ///
    /// **示例值**: "https://open.feishu.cn/"
    #[serde(
        rename = "android_url",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub android_url: Option<String>,
    /// PC端跳转链接，当该字段不设置时，PC端会使用common_url。必须以http开头。在PC端点击群菜单后，如果需要url对应的页面在飞书侧边栏展开，可以在url前加上https://applink.feishu.cn/client/web_url/open?mode=sidebar-semi&url=，比如https://applink.feishu.cn/client/web_url/open?mode=sidebar-semi&url=https://open.feishu.cn/
    ///
    /// **示例值**: "https://open.feishu.cn/"
    #[serde(
        rename = "pc_url",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub pc_url: Option<String>,
    /// Web端跳转链接，当该字段不设置时，Web端会使用common_url。必须以http开头。
    ///
    /// **示例值**: "https://open.feishu.cn/"
    #[serde(
        rename = "web_url",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub web_url: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(default)]
pub struct I18nNamesSubReq {
    /// 中文名
    ///
    /// **示例值**: "评审报名"
    #[serde(
        rename = "zh_cn",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub zh_cn: Option<String>,
    /// 英文名
    ///
    /// **示例值**: "Sign up"
    #[serde(
        rename = "en_us",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub en_us: Option<String>,
    /// 日文名
    ///
    /// **示例值**: "サインアップ"
    #[serde(
        rename = "ja_jp",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub ja_jp: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, lark_bot_sdk_macros::ApiBaseResp)]
struct UpdateChatMenuTreeRespInner {
    #[serde(flatten)]
    data: Option<UpdateChatMenuTreeResp>,
    #[serde(flatten)]
    base: BaseResp,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct UpdateChatMenuTreeResp {
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
    /// 修改后的菜单元信息
    #[serde(
        rename = "chat_menu_item",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub chat_menu_item: ChatMenuItemSubResp,
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
    /// **示例值**: "群聊"
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
        Fn(UpdateChatMenuTreeReq) -> Result<(UpdateChatMenuTreeResp, CommonResponse), Error>
        + Send
        + Sync
        + 'static
    {
    }
    impl<
            T: Fn(UpdateChatMenuTreeReq) -> Result<(UpdateChatMenuTreeResp, CommonResponse), Error>
                + Send
                + Sync
                + 'static,
        > MockFunc for T
    {
    }

    impl<'c, IStore: Store, IClient: HttpClient> ChatServiceMocker<'c, IStore, IClient> {
        pub fn mock_update_chat_menu_tree<F: MockFunc>(
            &self,
            f: F,
        ) -> MockerBuilder<Mocker, UpdateChatMenuTreeReq, UpdateChatMenuTreeResp, Arc<dyn MockFunc>>
        {
            MockerBuilder::new(self.cli.instance_id, Arc::new(f))
        }

        pub(super) fn get_mock_update_chat_menu_tree(
            &self,
            req: &UpdateChatMenuTreeReq,
        ) -> Option<Arc<dyn MockFunc>> {
            do_mock::<Mocker, UpdateChatMenuTreeReq, UpdateChatMenuTreeResp, Arc<dyn MockFunc>>(
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
        api::gen::chat::update_chat_menu_tree::{
            UpdateChatMenuTreeReq, UpdateChatMenuTreeResp, UpdateChatMenuTreeRespInner,
        },
        core::{model::CommonResponse, Lark},
    };

    #[tokio::test]
    async fn test_mock() {
        let lark = Lark::new("".to_owned(), "".into());
        let mocker = lark
            .chat()
            .mock()
            .mock_update_chat_menu_tree(|_| {
                Ok((UpdateChatMenuTreeResp::default(), CommonResponse::default()))
            })
            .build();
        let res = lark
            .chat()
            .update_chat_menu_tree(UpdateChatMenuTreeReq::default())
            .await;
        assert!(res.is_ok());
        mocker.clear();
        let res = lark
            .chat()
            .update_chat_menu_tree(UpdateChatMenuTreeReq::default())
            .await;
        assert!(res.is_err());
    }

    const REQ: &str = r#"{
    "update_fields": [
        "ICON"
    ],
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
        "name": "群聊",
        "i18n_names": {
            "zh_cn": "评审报名",
            "en_us": "Sign up",
            "ja_jp": "サインアップ"
        }
    }
}"#;

    #[test]
    fn test_req() {
        if REQ == "{}" {
            return;
        }
        if let Err(e) = serde_json::from_str::<super::UpdateChatMenuTreeReqBody>(REQ) {
            panic!("{}", e);
        }
    }

    const RESP: &str = r#"{
    "code": 0,
    "msg": "success",
    "data": {
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
            "name": "报名",
            "i18n_names": {
                "zh_cn": "报名",
                "en_us": "Sign up",
                "ja_jp": "サインアップ"
            }
        }
    }
}"#;
    #[test]
    fn test_resp() {
        let res = serde_json::from_str::<UpdateChatMenuTreeRespInner>(RESP);
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
