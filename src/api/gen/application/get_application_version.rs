//! doc url: <https://open.larkoffice.com/document/uAjLw4CM/ukTMukTMukTM/application-v6/application-app_version/get>
// Code generated by gen_api. DO NOT EDIT.

use serde::{Deserialize, Serialize};

use crate::api::BaseResp;
use crate::api::HasBaseResp;
use crate::core::model::{ApiRequest, CommonResponse, MethodOption, ReqParam, StreamReqParam};
use crate::{
    core::{http_client::HttpClient, store::Store},
    error::Error,
};

use crate::api::gen::application::ApplicationService;

impl<'c, IStore: Store, IClient: HttpClient> ApplicationService<'c, IStore, IClient> {
    /// **api 版本: 2024-05-23T12:56:18+00:00**
    ///
    /// ## 获取应用版本信息
    ///
    /// 根据应用 ID 和应用版本 ID 来获取同租户下的应用版本的信息
    ///

    ///
    /// doc: <https://open.larkoffice.com/document/uAjLw4CM/ukTMukTMukTM/application-v6/application-app_version/get>
    ///
    /// new doc: <https://open.larkoffice.com/document/server-docs/application-v6/application/get-2>
    ///
    /// api url: <https://open.feishu.cn/document_portal/v1/document/get_detail?fullPath=%2Fserver-docs%2Fapplication-v6%2Fapplication%2Fget-2>
    pub async fn get_application_version(
        &self,
        req: GetApplicationVersionReq,
    ) -> Result<(GetApplicationVersionResp, CommonResponse), Error> {
        self.get_application_version_with_opt(req, Default::default())
            .await
    }

    /// 参见 [get_application_version](#method.get_application_version) 函数
    pub async fn get_application_version_with_opt(
        &self,
        req: GetApplicationVersionReq,
        method_option: MethodOption,
    ) -> Result<(GetApplicationVersionResp, CommonResponse), Error> {
        #[cfg(feature = "test-util")]
        {
            if let Some(f) = self.mock().get_mock_get_application_version(&req) {
                tracing::info!("[lark] Application#GetApplicationVersion **mocking** api");
                return f(req);
            }
        }

        tracing::info!("[lark] Application#GetApplicationVersion call api");

        let req = ApiRequest {
            scope: "Application",
            api: "GetApplicationVersion",
            method: http::Method::GET,
            url: String::new()
                + self.cli.open_base_url.as_ref()
                + "/open-apis/application/v6/applications/:app_id/app_versions/:version_id",
            param_data: req.gen_param(),
            method_option,
            need_tenant_access_token: true,
            ..Default::default()
        };

        let (resp, common_resp): (GetApplicationVersionRespInner, _) = self.cli.do_req(req).await?;
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
pub struct GetApplicationVersionReq {
    /// 应用的 app_id，需要查询其他应用版本信息时，必须申请[获取应用版本信息](https://open.feishu.cn/document/ukTMukTMukTM/uQjN3QjL0YzN04CN2cDN)权限，仅查询本应用版本信息时，可填入 "me" 或者应用自身 app_id
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "cli_9f3ca975326b501b"
    #[api(kind = "path", name = "app_id")]
    pub app_id: String,
    /// 唯一标识应用版本的 ID
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "oav_d317f090b7258ad0372aa53963cda70d"
    #[api(kind = "path", name = "version_id")]
    pub version_id: String,
    /// 应用信息的语言版本
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "zh_cn"
    ///
    /// **可选值**:
    ///
    /// `zh_cn`: 中文
    ///
    /// `en_us`: 英文
    ///
    /// `ja_jp`: 日文
    #[api(kind = "query", name = "lang", v_type = "var", option = "false")]
    pub lang: String,
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
}

#[derive(Debug, Serialize, Deserialize, Clone, lark_bot_sdk_macros::ApiBaseResp)]
struct GetApplicationVersionRespInner {
    #[serde(flatten)]
    data: Option<GetApplicationVersionResp>,
    #[serde(flatten)]
    base: BaseResp,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct GetApplicationVersionResp {
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
    /// 应用版本信息
    #[serde(
        rename = "app_version",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub app_version: ApplicationAppVersionSubResp,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(default)]
pub struct ApplicationAppVersionSubResp {
    /// 应用 id
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "cli_9f3ca975326b501b"
    #[serde(
        rename = "app_id",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub app_id: String,
    /// 在开发者后台填入的应用版本号
    ///
    /// **示例值**: "1.0.0"
    ///
    /// **数据校验规则**：
    ///
    /// - **最小长度**: `1` 字符
    #[serde(
        rename = "version",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub version: String,
    /// 唯一标识应用版本的 ID
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "oav_d317f090b7258ad0372aa53963cda70d"
    #[serde(
        rename = "version_id",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub version_id: String,
    /// 应用默认名称
    ///
    /// **示例值**: "应用名称"
    ///
    /// **数据校验规则**：
    ///
    /// - **最小长度**: `1` 字符
    #[serde(
        rename = "app_name",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub app_name: String,
    /// 应用头像 url
    ///
    /// **示例值**: "https://www.example.com"
    #[serde(
        rename = "avatar_url",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub avatar_url: String,
    /// 应用默认描述
    ///
    /// **示例值**: "应用描述"
    ///
    /// **数据校验规则**：
    ///
    /// - **最小长度**: `1` 字符
    #[serde(
        rename = "description",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub description: String,
    /// 应用权限列表
    #[serde(
        rename = "scopes",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub scopes: Vec<AppScopeSubResp>,
    /// 后台主页地址
    ///
    /// **示例值**: "https://www.example.com"
    #[serde(
        rename = "back_home_url",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub back_home_url: String,
    /// 应用的国际化信息列表
    ///
    /// **数据校验规则**：
    ///
    /// - **最小长度**: `1` 字符
    #[serde(
        rename = "i18n",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub i18n: Vec<AppI18nInfoSubResp>,
    /// 应用分类的国际化描述
    ///
    /// **数据校验规则**：
    ///
    /// - **最大长度**: `3` 字符
    #[serde(
        rename = "common_categories",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub common_categories: Vec<String>,
    /// 应用已订阅开放平台事件列表
    #[serde(
        rename = "events",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub events: Vec<String>,
    /// 版本状态
    ///
    /// **示例值**: "1"
    ///
    /// **可选值**:
    ///
    /// `unknown`: 未知状态
    ///
    /// `audited`: 审核通过
    ///
    /// `reject`: 审核拒绝
    ///
    /// `under_audit`: 审核中
    ///
    /// `unaudit`: 未提交审核
    #[serde(
        rename = "status",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub status: i64,
    /// 版本创建时间（单位：s）
    ///
    /// **示例值**: "1610462759"
    #[serde(
        rename = "create_time",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub create_time: String,
    /// 版本发布时间（单位：s）
    ///
    /// **示例值**: "1610462759"
    #[serde(
        rename = "publish_time",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub publish_time: String,
    /// 当前版本下应用开启的能力
    #[serde(
        rename = "ability",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub ability: AppAbilitySubResp,
    /// 跟随应用版本的信息
    #[serde(
        rename = "remark",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub remark: AppVersionRemarkSubResp,
    /// 应用已订阅事件详情列表
    #[serde(
        rename = "event_infos",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub event_infos: Vec<EventSubResp>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(default)]
pub struct AppAbilitySubResp {
    /// 小程序能力
    #[serde(
        rename = "gadget",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub gadget: GadgetSubResp,
    /// 网页能力
    #[serde(
        rename = "web_app",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub web_app: WebAppSubResp,
    /// 机器人能力
    #[serde(
        rename = "bot",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub bot: BotSubResp,
    /// 小组件能力
    #[serde(
        rename = "workplace_widgets",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub workplace_widgets: Vec<WorkplaceWidgetSubResp>,
    /// 主导航小程序
    #[serde(
        rename = "navigate",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub navigate: NavigateSubResp,
    /// 云文档应用
    #[serde(
        rename = "cloud_doc",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub cloud_doc: CloudDocSubResp,
    /// 云文档小组件
    #[serde(
        rename = "docs_blocks",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub docs_blocks: Vec<DocsBlockSubResp>,
    /// 消息快捷操作
    #[serde(
        rename = "message_action",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub message_action: MessageActionSubResp,
    /// 加号菜单
    #[serde(
        rename = "plus_menu",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub plus_menu: PlusMenuSubResp,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(default)]
pub struct AppVersionRemarkSubResp {
    /// 备注说明
    ///
    /// **示例值**: "备注说明"
    #[serde(
        rename = "remark",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub remark: String,
    /// 更新说明
    ///
    /// **示例值**: "更新说明"
    #[serde(
        rename = "update_remark",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub update_remark: String,
    /// 应用当前版本开发者编辑的可见性建议，若开发者未编辑可见性建议，则该字段无内容
    #[serde(
        rename = "visibility",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub visibility: AppVisibilitySubResp,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(default)]
pub struct AppScopeSubResp {
    /// 应用权限
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "contact:user.base"
    #[serde(
        rename = "scope",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub scope: String,
    /// 应用权限的国际化描述
    ///
    /// **示例值**: "获取应用信息"
    #[serde(
        rename = "description",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub description: String,
    /// 权限等级描述
    ///
    /// **示例值**: "1"
    ///
    /// **可选值**:
    ///
    /// `low_level`: 普通权限
    ///
    /// `high_level`: 高级权限
    ///
    /// `super_level`: 超敏感权限
    ///
    /// `unknown_level`: 未知等级
    #[serde(
        rename = "level",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub level: i64,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(default)]
pub struct AppI18nInfoSubResp {
    /// 国际化语言的 key
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "zh_cn"
    ///
    /// **可选值**:
    ///
    /// `zh_cn`: 中文
    ///
    /// `en_us`: 英文
    ///
    /// `ja_jp`: 日文
    ///
    /// `zh_hk`: 繁体中文(中国香港)
    ///
    /// `zh_tw`: 繁体中文(中国台湾)
    ///
    /// `id_id`: 印度尼西亚语
    ///
    /// `ms_my`: 马来语
    ///
    /// `de_de`: 德语
    ///
    /// `es_es`: 西班牙语
    ///
    /// `fr_fr`: 法语
    ///
    /// `it_it`: 意大利语
    ///
    /// `pt_br`: 葡萄牙语(巴西)
    ///
    /// `vi_vn`: 越南语
    ///
    /// `ru_ru`: 俄语
    ///
    /// `th_th`: 泰语
    ///
    /// `ko_kr`: 韩语
    #[serde(
        rename = "i18n_key",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub i18n_key: String,
    /// 应用国际化名称
    ///
    /// **示例值**: "应用名称"
    ///
    /// **数据校验规则**：
    ///
    /// - **最小长度**: `1` 字符
    #[serde(
        rename = "name",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub name: String,
    /// 应用国际化描述（副标题）
    ///
    /// **示例值**: "应用描述"
    ///
    /// **数据校验规则**：
    ///
    /// - **最小长度**: `1` 字符
    #[serde(
        rename = "description",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub description: String,
    /// 国际化帮助文档链接
    ///
    /// **示例值**: "https://www.example.com"
    #[serde(
        rename = "help_use",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub help_use: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(default)]
pub struct GadgetSubResp {
    /// pc 支持的小程序模式，bit 位表示
    ///
    /// **示例值**: "1"
    ///
    /// **可选值**:
    ///
    /// `sidebar`: sidebar 模式
    ///
    /// `pc`: pc 模式
    ///
    /// `navigate`: 主导航模式
    #[serde(
        rename = "enable_pc_mode",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub enable_pc_mode: i64,
    /// schema url 列表
    #[serde(
        rename = "schema_urls",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub schema_urls: Vec<String>,
    /// pc 端是否使用小程序版本
    ///
    /// **示例值**: "false"
    #[serde(
        rename = "pc_use_mobile_pkg",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub pc_use_mobile_pkg: bool,
    /// pc 的小程序版本号
    ///
    /// **示例值**: "1.0.0"
    ///
    /// **数据校验规则**：
    ///
    /// - **最小长度**: `1` 字符
    #[serde(
        rename = "pc_version",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub pc_version: String,
    /// 移动端小程序版本号
    ///
    /// **示例值**: "1.0.0"
    ///
    /// **数据校验规则**：
    ///
    /// - **最小长度**: `1` 字符
    #[serde(
        rename = "mobile_version",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub mobile_version: String,
    /// 移动端兼容的最低飞书版本
    ///
    /// **示例值**: "2.0"
    ///
    /// **数据校验规则**：
    ///
    /// - **最小长度**: `1` 字符
    #[serde(
        rename = "mobile_min_lark_version",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub mobile_min_lark_version: String,
    /// pc 端兼容的最低飞书版本
    ///
    /// **示例值**: "2.0"
    ///
    /// **数据校验规则**：
    ///
    /// - **最小长度**: `1` 字符
    #[serde(
        rename = "pc_min_lark_version",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub pc_min_lark_version: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(default)]
pub struct WebAppSubResp {
    /// pc 端 url
    ///
    /// **示例值**: "https://www.example.com"
    #[serde(
        rename = "pc_url",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub pc_url: String,
    /// 移动端 url
    ///
    /// **示例值**: "https://www.example.com"
    #[serde(
        rename = "mobile_url",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub mobile_url: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(default)]
pub struct BotSubResp {
    /// 消息卡片回调地址
    ///
    /// **示例值**: "https://www.example.com"
    #[serde(
        rename = "card_request_url",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub card_request_url: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(default)]
pub struct NavigateSubResp {
    /// pc 端主导航信息
    #[serde(
        rename = "pc",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub pc: NavigateMetaSubResp,
    /// 移动端主导航信息
    #[serde(
        rename = "mobile",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub mobile: NavigateMetaSubResp,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(default)]
pub struct CloudDocSubResp {
    /// 云空间重定向 url
    ///
    /// **示例值**: "https://www.example.com"
    #[serde(
        rename = "space_url",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub space_url: String,
    /// 国际化信息
    ///
    /// **数据校验规则**：
    ///
    /// - **最小长度**: `1` 字符
    #[serde(
        rename = "i18n",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub i18n: Vec<CloudDocI18nInfoSubResp>,
    /// 图标链接
    ///
    /// **示例值**: "https://www.example.com"
    #[serde(
        rename = "icon_url",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub icon_url: String,
    /// 云文档支持模式
    ///
    /// **示例值**: "1"
    ///
    /// **可选值**:
    ///
    /// `unknown`: 未知
    ///
    /// `mobile`: 移动端
    #[serde(
        rename = "mode",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub mode: i64,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(default)]
pub struct MessageActionSubResp {
    /// pc 端链接
    ///
    /// **示例值**: "https://www.example.com"
    #[serde(
        rename = "pc_app_link",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub pc_app_link: String,
    /// 移动端链接
    ///
    /// **示例值**: "https://www.example.com"
    #[serde(
        rename = "mobile_app_link",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub mobile_app_link: String,
    /// 国际化信息
    ///
    /// **数据校验规则**：
    ///
    /// - **最小长度**: `1` 字符
    #[serde(
        rename = "i18n",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub i18n: Vec<MessageActionI18nInfoSubResp>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(default)]
pub struct PlusMenuSubResp {
    /// pc 端链接
    ///
    /// **示例值**: "https://www.example.com"
    #[serde(
        rename = "pc_app_link",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub pc_app_link: String,
    /// 移动端链接
    ///
    /// **示例值**: "https://www.example.com"
    #[serde(
        rename = "mobile_app_link",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub mobile_app_link: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(default)]
pub struct AppVisibilitySubResp {
    /// 是否全员可见
    ///
    /// **示例值**: "false"
    #[serde(
        rename = "is_all",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub is_all: bool,
    /// 可见名单
    #[serde(
        rename = "visible_list",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub visible_list: AppVisibleListSubResp,
    /// 不可见名单
    #[serde(
        rename = "invisible_list",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub invisible_list: AppVisibleListSubResp,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(default)]
pub struct EventSubResp {
    /// 事件类型，事件唯一标识
    ///
    /// **示例值**: "im.chat.updated_v1"
    #[serde(
        rename = "event_type",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub event_type: String,
    /// 事件名称
    ///
    /// **示例值**: "群配置修改事件"
    #[serde(
        rename = "event_name",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub event_name: String,
    /// 事件描述
    ///
    /// **示例值**: "群聊名称、头像、描述以及群编辑权限、群分享权限等被修改时推送事件"
    #[serde(
        rename = "event_description",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub event_description: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(default)]
pub struct WorkplaceWidgetSubResp {
    /// 最低兼容飞书版本号
    ///
    /// **示例值**: "1.0.0"
    #[serde(
        rename = "min_lark_version",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub min_lark_version: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(default)]
pub struct NavigateMetaSubResp {
    /// 主导航小程序版本号
    ///
    /// **示例值**: "1.0.0"
    ///
    /// **数据校验规则**：
    ///
    /// - **最小长度**: `1` 字符
    #[serde(
        rename = "version",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub version: String,
    /// 默认图片 url
    ///
    /// **示例值**: "https://www.example.com"
    #[serde(
        rename = "image_url",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub image_url: String,
    /// 选中态图片 url
    ///
    /// **示例值**: "https://www.example.com"
    #[serde(
        rename = "hover_image_url",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub hover_image_url: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(default)]
pub struct DocsBlockSubResp {
    /// BlockTypeID
    ///
    /// **示例值**: "blk_4fb61568435880110854c1d0"
    #[serde(
        rename = "block_type_id",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub block_type_id: String,
    /// block 的国际化信息
    ///
    /// **数据校验规则**：
    ///
    /// - **最小长度**: `1` 字符
    #[serde(
        rename = "i18n",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub i18n: Vec<BlockI18nInfoSubResp>,
    /// 移动端 icon 链接
    ///
    /// **示例值**: "https://www.example.com"
    #[serde(
        rename = "mobile_icon_url",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub mobile_icon_url: String,
    /// pc 端口 icon 链接
    ///
    /// **示例值**: "https://www.example.com"
    #[serde(
        rename = "pc_icon_url",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub pc_icon_url: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(default)]
pub struct AppVisibleListSubResp {
    /// 可见性成员 open_id 列表
    #[serde(
        rename = "open_ids",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub open_ids: Vec<String>,
    /// 可见性部门的 id 列表
    #[serde(
        rename = "department_ids",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub department_ids: Vec<String>,
    /// 可见性成员 group_id 列表
    #[serde(
        rename = "group_ids",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub group_ids: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(default)]
pub struct CloudDocI18nInfoSubResp {
    /// 国际化语言的 key
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "zh_cn"
    ///
    /// **可选值**:
    ///
    /// `zh_cn`: 中文
    ///
    /// `en_us`: 英文
    ///
    /// `ja_jp`: 日文
    #[serde(
        rename = "i18n_key",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub i18n_key: String,
    /// 云文档国际化名称
    ///
    /// **示例值**: "名称"
    #[serde(
        rename = "name",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub name: String,
    /// 云文档国际化读权限说明
    ///
    /// **示例值**: "读权限说明"
    #[serde(
        rename = "read_description",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub read_description: String,
    /// 云文档国际化写权限说明
    ///
    /// **示例值**: "写权限说明"
    #[serde(
        rename = "write_description",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub write_description: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(default)]
pub struct MessageActionI18nInfoSubResp {
    /// 国际化语言的 key
    ///
    /// **示例值**: "zh_cn"
    ///
    /// **可选值**:
    ///
    /// `zh_cn`: 中文
    ///
    /// `en_us`: 英文
    ///
    /// `ja_jp`: 日文
    #[serde(
        rename = "i18n_key",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub i18n_key: String,
    /// 国际化名称
    ///
    /// **示例值**: "名称"
    ///
    /// **数据校验规则**：
    ///
    /// - **最小长度**: `1` 字符
    #[serde(
        rename = "name",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(default)]
pub struct BlockI18nInfoSubResp {
    /// 国际化语言的 key
    ///
    /// **示例值**: "zh_cn"
    ///
    /// **可选值**:
    ///
    /// `zh_cn`: 中文
    ///
    /// `en_us`: 英文
    ///
    /// `ja_jp`: 日文
    #[serde(
        rename = "i18n_key",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub i18n_key: String,
    /// 名称
    ///
    /// **示例值**: "名称"
    ///
    /// **数据校验规则**：
    ///
    /// - **最小长度**: `1` 字符
    #[serde(
        rename = "name",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub name: String,
}

#[cfg(feature = "test-util")]
mod test_utils {
    use std::sync::Arc;

    use self::gen::application::ApplicationServiceMocker;
    use crate::core::mocker::*;

    use super::*;
    use crate::api::*;
    use crate::core::model::*;

    pub struct Mocker;

    pub trait MockFunc:
        Fn(GetApplicationVersionReq) -> Result<(GetApplicationVersionResp, CommonResponse), Error>
        + Send
        + Sync
        + 'static
    {
    }
    impl<
            T: Fn(
                    GetApplicationVersionReq,
                ) -> Result<(GetApplicationVersionResp, CommonResponse), Error>
                + Send
                + Sync
                + 'static,
        > MockFunc for T
    {
    }

    impl<'c, IStore: Store, IClient: HttpClient> ApplicationServiceMocker<'c, IStore, IClient> {
        pub fn mock_get_application_version<F: MockFunc>(
            &self,
            f: F,
        ) -> MockerBuilder<
            Mocker,
            GetApplicationVersionReq,
            GetApplicationVersionResp,
            Arc<dyn MockFunc>,
        > {
            MockerBuilder::new(self.cli.instance_id, Arc::new(f))
        }

        pub(super) fn get_mock_get_application_version(
            &self,
            req: &GetApplicationVersionReq,
        ) -> Option<Arc<dyn MockFunc>> {
            do_mock::<Mocker, GetApplicationVersionReq, GetApplicationVersionResp, Arc<dyn MockFunc>>(
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
        api::gen::application::get_application_version::{
            GetApplicationVersionReq, GetApplicationVersionResp, GetApplicationVersionRespInner,
        },
        core::{model::CommonResponse, Lark},
    };

    #[tokio::test]
    async fn test_mock() {
        let lark = Lark::new("".to_owned(), "".into());
        let mocker = lark
            .application()
            .mock()
            .mock_get_application_version(|_| {
                Ok((
                    GetApplicationVersionResp::default(),
                    CommonResponse::default(),
                ))
            })
            .build();
        let res = lark
            .application()
            .get_application_version(GetApplicationVersionReq::default())
            .await;
        assert!(res.is_ok());
        mocker.clear();
        let res = lark
            .application()
            .get_application_version(GetApplicationVersionReq::default())
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
        "app_version": {
            "app_id": "cli_9f3ca975326b501b",
            "version": "1.0.0",
            "version_id": "oav_d317f090b7258ad0372aa53963cda70d",
            "app_name": "应用名称",
            "avatar_url": "https://www.example.com",
            "description": "应用描述",
            "scopes": [
                {
                    "scope": "contact:user.base",
                    "description": "获取应用信息",
                    "level": 1
                }
            ],
            "back_home_url": "https://www.example.com",
            "i18n": [
                {
                    "i18n_key": "zh_cn",
                    "name": "应用名称",
                    "description": "应用描述",
                    "help_use": "https://www.example.com"
                }
            ],
            "common_categories": [
                "分析工具"
            ],
            "events": [
                "contacts.open_platform.depart"
            ],
            "status": 1,
            "create_time": "1610462759",
            "publish_time": "1610462759",
            "ability": {
                "gadget": {
                    "enable_pc_mode": 1,
                    "schema_urls": [
                        "https://www.example.com"
                    ],
                    "pc_use_mobile_pkg": false,
                    "pc_version": "1.0.0",
                    "mobile_version": "1.0.0",
                    "mobile_min_lark_version": "2.0",
                    "pc_min_lark_version": "2.0"
                },
                "web_app": {
                    "pc_url": "https://www.example.com",
                    "mobile_url": "https://www.example.com"
                },
                "bot": {
                    "card_request_url": "https://www.example.com"
                },
                "workplace_widgets": [
                    {
                        "min_lark_version": "1.0.0"
                    }
                ],
                "navigate": {
                    "pc": {
                        "version": "1.0.0",
                        "image_url": "https://www.example.com",
                        "hover_image_url": "https://www.example.com"
                    },
                    "mobile": {
                        "version": "1.0.0",
                        "image_url": "https://www.example.com",
                        "hover_image_url": "https://www.example.com"
                    }
                },
                "cloud_doc": {
                    "space_url": "https://www.example.com",
                    "i18n": [
                        {
                            "i18n_key": "zh_cn",
                            "name": "名称",
                            "read_description": "读权限说明",
                            "write_description": "写权限说明"
                        }
                    ],
                    "icon_url": "https://www.example.com",
                    "mode": 1
                },
                "docs_blocks": [
                    {
                        "block_type_id": "blk_4fb61568435880110854c1d0",
                        "i18n": [
                            {
                                "i18n_key": "zh_cn",
                                "name": "名称"
                            }
                        ],
                        "mobile_icon_url": "https://www.example.com",
                        "pc_icon_url": "https://www.example.com"
                    }
                ],
                "message_action": {
                    "pc_app_link": "https://www.example.com",
                    "mobile_app_link": "https://www.example.com",
                    "i18n": [
                        {
                            "i18n_key": "zh_cn",
                            "name": "名称"
                        }
                    ]
                },
                "plus_menu": {
                    "pc_app_link": "https://www.example.com",
                    "mobile_app_link": "https://www.example.com"
                }
            },
            "remark": {
                "remark": "备注说明",
                "update_remark": "更新说明",
                "visibility": {
                    "is_all": false,
                    "visible_list": {
                        "open_ids": [
                            "ou_4065981088f8ef67a504ba8bd6b24d85"
                        ],
                        "department_ids": [
                            "od-4b4a6907ad726ea07b27b0d2882b7c65"
                        ],
                        "group_ids": [
                            "b6d1g5dd6fd26186"
                        ]
                    },
                    "invisible_list": {
                        "open_ids": [
                            "ou_4065981088f8ef67a504ba8bd6b24d85"
                        ],
                        "department_ids": [
                            "od-4b4a6907ad726ea07b27b0d2882b7c65"
                        ],
                        "group_ids": [
                            "b6d1g5dd6fd26186"
                        ]
                    }
                }
            },
            "event_infos": [
                {
                    "event_type": "im.chat.updated_v1",
                    "event_name": "群配置修改事件",
                    "event_description": "群聊名称、头像、描述以及群编辑权限、群分享权限等被修改时推送事件"
                }
            ]
        }
    }
}"#;
    #[test]
    fn test_resp() {
        let res = serde_json::from_str::<GetApplicationVersionRespInner>(RESP);
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
