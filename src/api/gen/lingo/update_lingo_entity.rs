//! doc url: <https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/lingo-v1/entity/update>
// Code generated by gen_api. DO NOT EDIT.

use serde::{Deserialize, Serialize};

use crate::api::BaseResp;
use crate::api::HasBaseResp;
use crate::core::model::{ApiRequest, CommonResponse, MethodOption, ReqParam, StreamReqParam};
use crate::{
    core::{http_client::HttpClient, store::Store},
    error::Error,
};

use crate::api::gen::lingo::LingoService;

impl<'c, IStore: Store, IClient: HttpClient> LingoService<'c, IStore, IClient> {
    /// **api 版本: 2024-04-19T03:10:55+00:00**
    ///
    /// ## 更新免审词条
    ///
    /// 通过此接口更新已有的词条，无需经过词典管理员审核，直接写入词库。因此，调用该接口时应当慎重操作。
    ///

    ///
    /// doc: <https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/lingo-v1/entity/update>
    ///
    /// new doc: <https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/lingo-v1/entity/update>
    ///
    /// api url: <https://open.feishu.cn/document_portal/v1/document/get_detail?fullPath=%2FuAjLw4CM%2FukTMukTMukTM%2Flingo-v1%2Fentity%2Fupdate>
    pub async fn update_lingo_entity(
        &self,
        req: UpdateLingoEntityReq,
    ) -> Result<(UpdateLingoEntityResp, CommonResponse), Error> {
        self.update_lingo_entity_with_opt(req, Default::default())
            .await
    }

    /// 参见 [update_lingo_entity](#method.update_lingo_entity) 函数
    pub async fn update_lingo_entity_with_opt(
        &self,
        req: UpdateLingoEntityReq,
        method_option: MethodOption,
    ) -> Result<(UpdateLingoEntityResp, CommonResponse), Error> {
        #[cfg(feature = "test-util")]
        {
            if let Some(f) = self.mock().get_mock_update_lingo_entity(&req) {
                tracing::info!("[lark] Lingo#UpdateLingoEntity **mocking** api");
                return f(req);
            }
        }

        tracing::info!("[lark] Lingo#UpdateLingoEntity call api");

        let req = ApiRequest {
            scope: "Lingo",
            api: "UpdateLingoEntity",
            method: http::Method::PUT,
            url: String::new()
                + self.cli.open_base_url.as_ref()
                + "/open-apis/lingo/v1/entities/:entity_id",
            param_data: req.gen_param(),
            method_option,
            need_tenant_access_token: true,
            ..Default::default()
        };

        let (resp, common_resp): (UpdateLingoEntityRespInner, _) = self.cli.do_req(req).await?;
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
pub struct UpdateLingoEntityReq {
    /// 词条 ID
    ///
    /// **示例值**: "enterprise_40217521"
    #[api(kind = "path", name = "entity_id")]
    pub entity_id: String,
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
    /// 词条名
    ///
    /// **是否必填**: 是
    ///
    /// **数据校验规则**：
    ///
    /// - **最大长度**: `1` 字符
    #[api(kind = "body", name = "main_keys")]
    pub main_keys: Vec<Option<TermSubReq>>,
    /// 别名
    ///
    /// **数据校验规则**：
    ///
    /// - **最大长度**: `10` 字符
    #[api(kind = "body", name = "aliases")]
    pub aliases: Vec<Option<TermSubReq>>,
    /// 详情描述
    ///
    /// **示例值**: "词典是飞书提供的一款知识管理工具，通过飞书词典可以帮助企业将分散的知识信息进行聚合，并通过UGC的方式，促进企业知识的保鲜和流通"
    ///
    /// **数据校验规则**：
    ///
    /// - **最大长度**: `5000` 字符
    #[api(kind = "body", name = "description")]
    pub description: Option<String>,
    /// 相关数据
    #[api(kind = "body", name = "related_meta")]
    pub related_meta: Option<RelatedMetaSubReq>,
    /// 外部 id 关联数据
    #[api(kind = "body", name = "outer_info")]
    pub outer_info: Option<OuterInfoSubReq>,
    /// 富文本格式（当填写富文本内容时，description字段将会失效可不填写），支持的格式参考[飞书词典指南](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/baike-v1/overview)中的释义部分
    ///
    /// **示例值**: "&lt;b&gt;加粗&lt;/b&gt;&lt;i&gt;斜体&lt;/i&gt;&lt;p&gt;&lt;a href=\"https://feishu.cn\"&gt;链接&lt;/a&gt;&lt;/p&gt;&lt;p&gt;&lt;span&gt;词典是飞书提供的一款知识管理工具，通过飞书词典可以帮助企业将分散的知识信息进行聚合，并通过UGC的方式，促进企业知识的保鲜和流通&lt;/span&gt;&lt;/p&gt;"
    ///
    /// **数据校验规则**：
    ///
    /// - **最大长度**: `5000` 字符
    #[api(kind = "body", name = "rich_text")]
    pub rich_text: Option<String>,
    /// 国际化的词条释义
    ///
    /// **数据校验规则**：
    ///
    /// - **最大长度**: `3` 字符
    #[api(kind = "body", name = "i18n_descs")]
    pub i18n_descs: Vec<I18nEntryDescSubReq>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(default)]
pub struct RelatedMetaSubReq {
    /// 关联用户信息
    #[serde(
        rename = "users",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub users: Vec<Option<RefererSubReq>>,
    /// 相关服务中的相关公开群
    #[serde(
        rename = "chats",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub chats: Vec<Option<RefererSubReq>>,
    /// 关联文档信息
    #[serde(
        rename = "docs",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub docs: Vec<Option<RefererSubReq>>,
    /// 相关服务中的相关值班号
    #[serde(
        rename = "oncalls",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub oncalls: Vec<Option<RefererSubReq>>,
    /// 相关链接
    #[serde(
        rename = "links",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub links: Vec<Option<RefererSubReq>>,
    /// 相关词条信息
    #[serde(
        rename = "abbreviations",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub abbreviations: Vec<Option<AbbreviationSubReq>>,
    /// 当前词条所属分类<br>
    ///
    /// 词条只能属于二级分类，且每个一级分类下只能选择一个二级分类。
    #[serde(
        rename = "classifications",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub classifications: Vec<Option<ClassificationSubReq>>,
    /// 上传的相关图片
    ///
    /// **数据校验规则**：
    ///
    /// - **最大长度**: `10` 字符
    #[serde(
        rename = "images",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub images: Vec<Option<BaikeImageSubReq>>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(default)]
pub struct OuterInfoSubReq {
    /// 数据提供方（不能包含中横线 "-"）
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "星云"
    ///
    /// **数据校验规则**：
    ///
    /// - **长度范围**: `2` 字符- `32` 字符
    #[serde(
        rename = "provider",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub provider: String,
    /// 唯一标识，可用来和其他平台的内容进行绑定。需保证和百科词条唯一对应（不能包含中横线 "-"）
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "12345abc"
    ///
    /// **数据校验规则**：
    ///
    /// - **长度范围**: `1` 字符- `64` 字符
    #[serde(
        rename = "outer_id",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub outer_id: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(default)]
pub struct TermSubReq {
    /// 名称
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "企业百科"
    #[serde(
        rename = "key",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub key: String,
    /// 展示状态
    ///
    /// **是否必填**: 是
    #[serde(
        rename = "display_status",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub display_status: DisplayStatusSubReq,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(default)]
pub struct I18nEntryDescSubReq {
    /// 语言类型
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "1"
    ///
    /// **可选值**:
    ///
    /// `ZH_CN`: 中文
    ///
    /// `EN_US`: 英文
    ///
    /// `JA_JP`: 日文
    #[serde(
        rename = "language",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub language: i64,
    /// 纯文本释义
    ///
    /// **示例值**: "词典是飞书提供的一款知识管理工具，通过飞书词典可以帮助企业将分散的知识信息进行聚合，并通过UGC的方式，促进企业知识的保鲜和流通"
    ///
    /// **数据校验规则**：
    ///
    /// - **长度范围**: `1` 字符- `5000` 字符
    #[serde(
        rename = "description",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub description: Option<String>,
    /// 富文本描述
    ///
    /// **示例值**: "<p><span>词典是飞书提供的一款知识管理工具，通过飞书词典可以帮助企业将分散的知识信息进行聚合，并通过UGC的方式，促进企业知识的保鲜和流通</span></p>"
    ///
    /// **数据校验规则**：
    ///
    /// - **长度范围**: `1` 字符- `5000` 字符
    #[serde(
        rename = "rich_text",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub rich_text: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(default)]
pub struct DisplayStatusSubReq {
    /// 是否允许在 IM 和 Doc 等场景进行高亮提示
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "true"
    #[serde(
        rename = "allow_highlight",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub allow_highlight: bool,
    /// 是否允许在飞书中被搜索到
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "true"
    #[serde(
        rename = "allow_search",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub allow_search: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(default)]
pub struct RefererSubReq {
    /// 对应相关信息 ID
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "格式请看请求体示例"
    #[serde(
        rename = "id",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub id: String,
    /// 标题
    ///
    /// **示例值**: "飞书官网"
    #[serde(
        rename = "title",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub title: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(default)]
pub struct AbbreviationSubReq {
    /// 相关其他词条 id
    ///
    /// **示例值**: "enterprise_51587960"
    #[serde(
        rename = "id",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub id: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(default)]
pub struct ClassificationSubReq {
    /// 二级分类 ID
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "7049606926****37761"
    #[serde(
        rename = "id",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub id: String,
    /// 对应一级分类 ID
    ///
    /// **示例值**: "7049606926****37777"
    #[serde(
        rename = "father_id",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub father_id: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(default)]
pub struct BaikeImageSubReq {
    /// 通过文件接口上传后的图片 token
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "boxbcEcmKiDia3evgqWTpvdc7jc"
    #[serde(
        rename = "token",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub token: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, lark_bot_sdk_macros::ApiBaseResp)]
struct UpdateLingoEntityRespInner {
    #[serde(flatten)]
    data: Option<UpdateLingoEntityResp>,
    #[serde(flatten)]
    base: BaseResp,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct UpdateLingoEntityResp {
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
    /// 词条信息
    #[serde(
        rename = "entity",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub entity: EntitySubResp,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(default)]
pub struct EntitySubResp {
    /// 词条 ID （需要更新某个词条时填写，若是创建新词条可不填写）
    ///
    /// **示例值**: "enterprise_40217521"
    #[serde(
        rename = "id",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub id: String,
    /// 词条名
    ///
    /// **是否必填**: 是
    ///
    /// **数据校验规则**：
    ///
    /// - **最大长度**: `1` 字符
    #[serde(
        rename = "main_keys",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub main_keys: Vec<TermSubResp>,
    /// 别名
    ///
    /// **数据校验规则**：
    ///
    /// - **最大长度**: `10` 字符
    #[serde(
        rename = "aliases",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub aliases: Vec<TermSubResp>,
    /// 纯文本格式词条释义。注：description 和 rich_text 至少有一个，否则会报错：1540001
    ///
    /// **示例值**: "词典是飞书提供的一款知识管理工具，通过飞书词典可以帮助企业将分散的知识信息进行聚合，并通过UGC的方式，促进企业知识的保鲜和流通"
    ///
    /// **数据校验规则**：
    ///
    /// - **最大长度**: `5000` 字符
    #[serde(
        rename = "description",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub description: String,
    /// 创建者
    ///
    /// **示例值**: "ou_30b0***89914dac63d36"
    #[serde(
        rename = "creator",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub creator: String,
    /// 词条创建时间
    ///
    /// **示例值**: "1649318125"
    #[serde(
        rename = "create_time",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub create_time: String,
    /// 最近一次更新者
    ///
    /// **示例值**: "ou_30b0***89914dac63d36"
    #[serde(
        rename = "updater",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub updater: String,
    /// 词条最近更新时间
    ///
    /// **示例值**: "1649318125"
    #[serde(
        rename = "update_time",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub update_time: String,
    /// 更多相关信息
    #[serde(
        rename = "related_meta",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub related_meta: RelatedMetaSubResp,
    /// 当前词条收到的反馈数据
    #[serde(
        rename = "statistics",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub statistics: StatisticsSubResp,
    /// 外部 id 关联数据
    #[serde(
        rename = "outer_info",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub outer_info: OuterInfoSubResp,
    /// 富文本格式（当填写富文本内容时，description字段将会失效可不填写），支持的格式参考[飞书词典指南](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/baike-v1/overview)中的释义部分
    ///
    /// **示例值**: "&lt;b&gt;加粗&lt;/b&gt;&lt;i&gt;斜体&lt;/i&gt;&lt;p&gt;&lt;a href=\"https://feishu.cn\"&gt;链接&lt;/a&gt;&lt;/p&gt;&lt;p&gt;&lt;span&gt;词典是飞书提供的一款知识管理工具，通过飞书词典可以帮助企业将分散的知识信息进行聚合，并通过UGC的方式，促进企业知识的保鲜和流通&lt;/span&gt;&lt;/p&gt;"
    ///
    /// **数据校验规则**：
    ///
    /// - **最大长度**: `5000` 字符
    #[serde(
        rename = "rich_text",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub rich_text: String,
    /// 词条的创建来源，1：用户主动创建，2：批量导入，3：官方词，4：OpenAPI 创建
    ///
    /// **示例值**: "1"
    #[serde(
        rename = "source",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub source: i64,
    /// 国际化的词条释义
    ///
    /// **数据校验规则**：
    ///
    /// - **最大长度**: `3` 字符
    #[serde(
        rename = "i18n_descs",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub i18n_descs: Vec<I18nEntryDescSubResp>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(default)]
pub struct RelatedMetaSubResp {
    /// 相关联系人
    #[serde(
        rename = "users",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub users: Vec<RefererSubResp>,
    /// 相关服务中的相关公开群
    #[serde(
        rename = "chats",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub chats: Vec<RefererSubResp>,
    /// 相关云文档
    #[serde(
        rename = "docs",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub docs: Vec<RefererSubResp>,
    /// 相关服务中的相关值班号
    #[serde(
        rename = "oncalls",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub oncalls: Vec<RefererSubResp>,
    /// 关联链接信息
    #[serde(
        rename = "links",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub links: Vec<RefererSubResp>,
    /// 相关词条信息
    #[serde(
        rename = "abbreviations",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub abbreviations: Vec<AbbreviationSubResp>,
    /// 当前词条所属分类<br>
    ///
    /// 词条只能属于二级分类，且每个一级分类下只能选择一个二级分类。
    #[serde(
        rename = "classifications",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub classifications: Vec<ClassificationSubResp>,
    /// 上传的相关图片
    ///
    /// **数据校验规则**：
    ///
    /// - **最大长度**: `10` 字符
    #[serde(
        rename = "images",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub images: Vec<BaikeImageSubResp>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(default)]
pub struct StatisticsSubResp {
    /// 点赞数量
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "55"
    #[serde(
        rename = "like_count",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub like_count: i64,
    /// 当前词条版本收到的负反馈数量
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "3"
    #[serde(
        rename = "dislike_count",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub dislike_count: i64,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(default)]
pub struct OuterInfoSubResp {
    /// 外部系统（不能包含中横线 "-"）
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "星云"
    ///
    /// **数据校验规则**：
    ///
    /// - **长度范围**: `2` 字符- `32` 字符
    #[serde(
        rename = "provider",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub provider: String,
    /// 唯一标识，可用来和其他平台的内容进行绑定。需保证和词典词条唯一对应（不能包含中横线 "-"）
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "12345abc"
    ///
    /// **数据校验规则**：
    ///
    /// - **长度范围**: `1` 字符- `64` 字符
    #[serde(
        rename = "outer_id",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub outer_id: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(default)]
pub struct TermSubResp {
    /// 名称
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "飞书词典"
    #[serde(
        rename = "key",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub key: String,
    /// 展示状态
    ///
    /// **是否必填**: 是
    #[serde(
        rename = "display_status",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub display_status: DisplayStatusSubResp,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(default)]
pub struct I18nEntryDescSubResp {
    /// 语言类型
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "1"
    ///
    /// **可选值**:
    ///
    /// `ZH_CN`: 中文
    ///
    /// `EN_US`: 英文
    ///
    /// `JA_JP`: 日文
    #[serde(
        rename = "language",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub language: i64,
    /// 纯文本释义
    ///
    /// **示例值**: "词典是飞书提供的一款知识管理工具，通过飞书词典可以帮助企业将分散的知识信息进行聚合，并通过UGC的方式，促进企业知识的保鲜和流通"
    ///
    /// **数据校验规则**：
    ///
    /// - **长度范围**: `1` 字符- `5000` 字符
    #[serde(
        rename = "description",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub description: String,
    /// 富文本描述
    ///
    /// **示例值**: "<p><span>词典是飞书提供的一款知识管理工具，通过飞书词典可以帮助企业将分散的知识信息进行聚合，并通过UGC的方式，促进企业知识的保鲜和流通</span></p>"
    ///
    /// **数据校验规则**：
    ///
    /// - **长度范围**: `1` 字符- `5000` 字符
    #[serde(
        rename = "rich_text",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub rich_text: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(default)]
pub struct DisplayStatusSubResp {
    /// 是否允许在 IM 和 Doc 等场景进行高亮提示
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "true"
    #[serde(
        rename = "allow_highlight",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub allow_highlight: bool,
    /// 是否允许在飞书中被搜索到
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "true"
    #[serde(
        rename = "allow_search",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub allow_search: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(default)]
pub struct RefererSubResp {
    /// 对应相关信息 ID
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "格式请看请求体示例"
    #[serde(
        rename = "id",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub id: String,
    /// 对应相关信息的描述，如相关联系人的描述、相关链接的标题
    ///
    /// **示例值**: "飞书官网"
    #[serde(
        rename = "title",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub title: String,
    /// 链接地址
    ///
    /// **示例值**: "https://www.feishu.cn/hc/zh-CN"
    #[serde(
        rename = "url",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub url: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(default)]
pub struct AbbreviationSubResp {
    /// 相关词条 ID
    ///
    /// **示例值**: "enterprise_51587960"
    #[serde(
        rename = "id",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub id: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(default)]
pub struct ClassificationSubResp {
    /// 二级分类 ID
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "7049606926702***761"
    #[serde(
        rename = "id",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub id: String,
    /// 对应一级分类 ID
    ///
    /// **示例值**: "704960692670***777"
    #[serde(
        rename = "father_id",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub father_id: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(default)]
pub struct BaikeImageSubResp {
    /// 通过文件接口上传后的图片 token
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "boxbcEcmKiDia3evgqWTpvdc7jc"
    #[serde(
        rename = "token",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub token: String,
}

#[cfg(feature = "test-util")]
mod test_utils {
    use std::sync::Arc;

    use self::gen::lingo::LingoServiceMocker;
    use crate::core::mocker::*;

    use super::*;
    use crate::api::*;
    use crate::core::model::*;

    pub struct Mocker;

    pub trait MockFunc:
        Fn(UpdateLingoEntityReq) -> Result<(UpdateLingoEntityResp, CommonResponse), Error>
        + Send
        + Sync
        + 'static
    {
    }
    impl<
            T: Fn(UpdateLingoEntityReq) -> Result<(UpdateLingoEntityResp, CommonResponse), Error>
                + Send
                + Sync
                + 'static,
        > MockFunc for T
    {
    }

    impl<'c, IStore: Store, IClient: HttpClient> LingoServiceMocker<'c, IStore, IClient> {
        pub fn mock_update_lingo_entity<F: MockFunc>(
            &self,
            f: F,
        ) -> MockerBuilder<Mocker, UpdateLingoEntityReq, UpdateLingoEntityResp, Arc<dyn MockFunc>>
        {
            MockerBuilder::new(self.cli.instance_id, Arc::new(f))
        }

        pub(super) fn get_mock_update_lingo_entity(
            &self,
            req: &UpdateLingoEntityReq,
        ) -> Option<Arc<dyn MockFunc>> {
            do_mock::<Mocker, UpdateLingoEntityReq, UpdateLingoEntityResp, Arc<dyn MockFunc>>(
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
        api::gen::lingo::update_lingo_entity::{
            UpdateLingoEntityReq, UpdateLingoEntityResp, UpdateLingoEntityRespInner,
        },
        core::{model::CommonResponse, Lark},
    };

    #[tokio::test]
    async fn test_mock() {
        let lark = Lark::new("".to_owned(), "".into());
        let mocker = lark
            .lingo()
            .mock()
            .mock_update_lingo_entity(|_| {
                Ok((UpdateLingoEntityResp::default(), CommonResponse::default()))
            })
            .build();
        let res = lark
            .lingo()
            .update_lingo_entity(UpdateLingoEntityReq::default())
            .await;
        assert!(res.is_ok());
        mocker.clear();
        let res = lark
            .lingo()
            .update_lingo_entity(UpdateLingoEntityReq::default())
            .await;
        assert!(res.is_err());
    }

    const REQ: &str = r#"{
    "main_keys": [
        {
            "key": "飞书词典",
            "display_status": {
                "allow_highlight": true,
                "allow_search": true
            }
        }
    ],
    "aliases": [
        {
            "key": "词典",
            "display_status": {
                "allow_highlight": true,
                "allow_search": true
            }
        }
    ],
    "description": "词典是飞书提供的一款知识管理工具，通过飞书词典可以帮助企业将分散的知识信息进行聚合，并通过UGC的方式，促进企业知识的保鲜和流通",
    "rich_text": "<p>词典是飞书提供的一款知识管理工具，通过飞书词典可以帮助企业将分散的知识信息进行聚合，并通过UGC的方式，促进企业知识的保鲜和流通</p>",
    "i18n_descs": [
        {
            "language": 1,
            "description": "国际化中文释义",
            "rich_text": "<p>国际化中文释义</p>"
        }
    ],
    "related_meta": {
        "users": [
            {
                "id": "ou_30b07b6****ea46518789914dac63d36",
                "title": "负责人"
            },
            {
                "id": "ou_b292c0****c14754639fa4501e80c36a",
                "title": ""
            }
        ],
        "chats": [
            {
                "id": "oc_c13831833e****92c52befa759ea4806"
            },
            {
                "id": "oc_c8161c910****a24127e73b10233b295"
            }
        ],
        "docs": [
            {
                "title": "猜你想问 / FAQs",
                "url": "https://example.feishu.cn/wiki/wikcnZ8Lq4f9DMCDOtdcIzCUjAh"
            },
            {
                "title": "快速了解飞书文档 | Introducing Feishu Docs",
                "url": "https://example.feishu.cn/docs/doccnxlVCCFjMsJE15I7PLAjIWc"
            }
        ],
        "links": [
            {
                "title": "飞书官网",
                "url": "https://feishu.cn"
            }
        ],
        "oncalls": [
            {
                "id": "702368904****548034"
            },
            {
                "id": "70240637****0910850"
            }
        ],
        "abbreviations": [
            {
                "id": "enterprise_44***90"
            },
            {
                "id": "enterprise_70348****374354564"
            },
            {
                "id": "enterprise_70365****3106796547"
            }
        ],
        "classifications": [
            {
                "id": "7049606926****37761",
                "father_id": "7049606926****37777"
            }
        ],
        "images": [
            {
                "token": "boxbcEcmKiD3SGHvgqWTpvdc7jc"
            }
        ]
    },
    "outer_info": {
        "provider": "星云",
        "outer_id": "client_653****98d"
    }
}"#;

    #[test]
    fn test_req() {
        if REQ == "{}" {
            return;
        }
        if let Err(e) = serde_json::from_str::<super::UpdateLingoEntityReqBody>(REQ) {
            panic!("{}", e);
        }
    }

    const RESP: &str = r#"{
    "code": 0,
    "msg": "success",
    "data": {
        "entity": {
            "id": "enterprise_402***21",
            "main_keys": [
                {
                    "key": "飞书词典",
                    "display_status": {
                        "allow_highlight": true,
                        "allow_search": true
                    }
                }
            ],
            "aliases": [
                {
                    "key": "词典",
                    "display_status": {
                        "allow_highlight": true,
                        "allow_search": true
                    }
                }
            ],
            "description": "词典是飞书提供的一款知识管理工具，通过飞书词典可以帮助企业将分散的知识信息进行聚合，并通过UGC的方式，促进企业知识的保鲜和流通",
            "rich_text": "<p>词典是飞书提供的一款知识管理工具，通过飞书词典可以帮助企业将分散的知识信息进行聚合，并通过UGC的方式，促进企业知识的保鲜和流通</p>",
            "i18n_descs": [
                {
                    "language": 1,
                    "description": "国际化中文释义",
                    "rich_text": "<p>国际化中文释义</p>"
                }
            ],
            "related_meta": {
                "users": [
                    {
                        "id": "ou_30b07b63089e***18789914dac63d36",
                        "title": "负责人"
                    },
                    {
                        "id": "ou_b292c0d285c1***639fa4501e80c36a",
                        "title": ""
                    }
                ],
                "chats": [
                    {
                        "id": "oc_c13831833eaa8c92***cfa759ea4806"
                    },
                    {
                        "id": "oc_c8161c9109966a24***e73b10233b295"
                    }
                ],
                "docs": [
                    {
                        "title": "猜你想问 / FAQs",
                        "url": "https://example.feishu.cn/wiki/wikcnZ8Lq4***CDOtdcIzCUjAh"
                    },
                    {
                        "title": "快速了解飞书文档 | Introducing Feishu Docs",
                        "url": "https://example.feishu.cn/docs/doccnxlVCs***sJE15I7PLAjIWc"
                    }
                ],
                "links": [
                    {
                        "title": "飞书官网",
                        "url": "https://feishu.cn"
                    }
                ],
                "oncalls": [
                    {
                        "id": "70236890***45548034"
                    },
                    {
                        "id": "70240637***60910850"
                    }
                ],
                "abbreviations": [
                    {
                        "id": "enterprise_4450***890"
                    },
                    {
                        "id": "enterprise_703481***74354564"
                    },
                    {
                        "id": "enterprise_703659***06796547"
                    }
                ],
                "classifications": [
                    {
                        "id": "70496069***2837761",
                        "father_id": "70496069***02837777"
                    }
                ],
                "images": [
                    {
                        "token": "boxbcEcmKiD***vgqWTpvdc7jc"
                    }
                ]
            },
            "statistics": {
                "like_count": 100,
                "dislike_count": 20
            },
            "outer_info": {
                "provider": "星云",
                "outer_id": "client_6539***498d"
            },
            "create_time": "1627540853",
            "update_time": "1627541853"
        }
    }
}"#;
    #[test]
    fn test_resp() {
        let res = serde_json::from_str::<UpdateLingoEntityRespInner>(RESP);
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
