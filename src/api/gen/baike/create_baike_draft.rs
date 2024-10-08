//! doc url: <https://open.larkoffice.com/document/uAjLw4CM/ukTMukTMukTM/reference/baike-v1/draft/create>
// Code generated by gen_api. DO NOT EDIT.

use serde::{Deserialize, Serialize};

use crate::api::BaseResp;
use crate::api::HasBaseResp;
use crate::core::model::{ApiRequest, CommonResponse, MethodOption, ReqParam, StreamReqParam};
use crate::{
    core::{http_client::HttpClient, store::Store},
    error::Error,
};

use crate::api::gen::baike::BaikeService;

impl<'c, IStore: Store, IClient: HttpClient> BaikeService<'c, IStore, IClient> {
    /// **api 版本: 2023-10-13T02:23:08+00:00**
    ///
    /// ## 创建草稿
    ///
    /// 草稿并非词条，而是指通过 API 发起创建新词条或更新现有词条的申请。
    ///
    /// 词典管理员审核通过后，草稿将变为新的词条或覆盖已有词条。
    ///
    /// 为了更好地提升接口文档的的易理解性，我们对文档进行了升级，请尽快迁移至[新版本>>](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/lingo-v1/draft/create)
    ///
    /// 以用户身份创建草稿（即 Authorization 使用 user_access_token），对应用户将收到由飞书词典 Bot 发送的审核结果；以应用身份创建草稿（即 Authorization 使用 tenant_access_toke），不会收到任何通知。
    ///
    /// - 创建新的词条时，无需传入 entity_id 字段<br>
    ///
    /// - 更新已有词条时，请传入对应词条的 entity_id 或 outer_info
    ///
    /// doc: <https://open.larkoffice.com/document/uAjLw4CM/ukTMukTMukTM/reference/baike-v1/draft/create>
    ///
    /// new doc: <https://open.larkoffice.com/document/server-docs/baike-v1/draft/create>
    ///
    /// api url: <https://open.feishu.cn/document_portal/v1/document/get_detail?fullPath=%2Fserver-docs%2Fbaike-v1%2Fdraft%2Fcreate>
    pub async fn create_baike_draft(
        &self,
        req: CreateBaikeDraftReq,
    ) -> Result<(CreateBaikeDraftResp, CommonResponse), Error> {
        self.create_baike_draft_with_opt(req, Default::default())
            .await
    }

    /// 参见 [create_baike_draft](#method.create_baike_draft) 函数
    pub async fn create_baike_draft_with_opt(
        &self,
        req: CreateBaikeDraftReq,
        method_option: MethodOption,
    ) -> Result<(CreateBaikeDraftResp, CommonResponse), Error> {
        #[cfg(feature = "test-util")]
        {
            if let Some(f) = self.mock().get_mock_create_baike_draft(&req) {
                tracing::info!("[lark] Baike#CreateBaikeDraft **mocking** api");
                return f(req);
            }
        }

        tracing::info!("[lark] Baike#CreateBaikeDraft call api");

        let req = ApiRequest {
            scope: "Baike",
            api: "CreateBaikeDraft",
            method: http::Method::POST,
            url: String::new() + self.cli.open_base_url.as_ref() + "/open-apis/baike/v1/drafts",
            param_data: req.gen_param(),
            method_option,
            need_tenant_access_token: true,
            need_user_access_token: true,
            ..Default::default()
        };

        let (resp, common_resp): (CreateBaikeDraftRespInner, _) = self.cli.do_req(req).await?;
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
pub struct CreateBaikeDraftReq {
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
    /// 词条 ID （需要更新某个词条时填写，若是创建新词条可不填写）
    ///
    /// **示例值**: "enterprise_40217521"
    #[api(kind = "body", name = "id")]
    pub id: Option<String>,
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
    /// 纯文本格式词条释义。注：description 和 rich_text 至少有一个，否则会报错：1540001
    ///
    /// **示例值**: "词典是飞书提供的一款知识管理工具，通过飞书词典可以帮助企业将分散的知识信息进行聚合，并通过UGC的方式，促进企业知识的保鲜和流通"
    ///
    /// **数据校验规则**：
    ///
    /// - **最大长度**: `5000` 字符
    #[api(kind = "body", name = "description")]
    pub description: Option<String>,
    /// 更多相关信息
    #[api(kind = "body", name = "related_meta")]
    pub related_meta: Option<RelatedMetaSubReq>,
    /// 外部系统关联数据
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
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(default)]
pub struct RelatedMetaSubReq {
    /// 相关联系人
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
    /// 相关云文档
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
    /// 相关词条
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
    /// 上传的图片
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
    /// 词条在外部系统中对应的唯一 ID（不能包含中横线 "-"）
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "client_653267498d"
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
    /// 名称的值
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "飞书词典"
    #[serde(
        rename = "key",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub key: String,
    /// 名称展示范围
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
pub struct DisplayStatusSubReq {
    /// 对应名称是否在消息/云文档高亮
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "true"
    #[serde(
        rename = "allow_highlight",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub allow_highlight: bool,
    /// 对应名称是否在搜索结果中展示
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
    /// 对应相关信息的描述，如相关联系人的描述、相关链接的标题
    ///
    /// **示例值**: "飞书词典帮助中心"
    #[serde(
        rename = "title",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub title: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(default)]
pub struct AbbreviationSubReq {
    /// 相关词条 ID
    ///
    /// **示例值**: "enterprise_51527260"
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
    /// 通过文件接口上传图片后，获得的图片 token
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
struct CreateBaikeDraftRespInner {
    #[serde(flatten)]
    data: Option<CreateBaikeDraftResp>,
    #[serde(flatten)]
    base: BaseResp,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct CreateBaikeDraftResp {
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
    /// 草稿信息
    #[serde(
        rename = "draft",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub draft: DraftSubResp,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(default)]
pub struct DraftSubResp {
    /// 草稿 ID
    ///
    /// **示例值**: "42322"
    #[serde(
        rename = "draft_id",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub draft_id: String,
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
    /// 词条创建者的 user id
    ///
    /// **示例值**: "ou_30b07b63089ea46518789914dac63d36"
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
    /// 词条最近更新者的 user id
    ///
    /// **示例值**: "ou_30b07b63089ea46518789914dac63d36"
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
    /// 外部系统关联数据
    #[serde(
        rename = "related_meta",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub related_meta: RelatedMetaSubResp,
    /// 富文本格式（当填写富文本内容时，description字段将会失效可不填写），支持的格式参考[飞书词典指南](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/baike-v1/overview)中的释义部分
    ///
    /// **示例值**: "&lt;b&gt;加粗&lt;/b&gt;&lt;i&gt;斜体&lt;/i&gt;&lt;p&gt;&lt;a href=\"https://feishu.cn\"&gt;链接&lt;/a&gt;&lt;/p&gt;&lt;p&gt;&lt;span&gt;词典是飞书提供的一款知识管理工具，通过飞书词典可以帮助企业将分散的知识信息进行聚合，并通过UGC的方式，促进企业知识的保鲜和流通&lt;/span&gt;&lt;/p&gt;"
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
    /// 富文本格式（当填写富文本内容时，description字段将会失效可不填写），支持的格式参考[企业百科指南](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/baike-v1/overview)中的释义部分
    ///
    /// **示例值**: "&lt;b&gt;加粗&lt;/b&gt;&lt;i&gt;斜体&lt;/i&gt;&lt;p&gt;&lt;a href="https://feishu.cn"&gt;l链接&lt;/a&gt;&lt;/p&gt;&lt;p&gt;&lt;span&gt;企业百科是飞书提供的一款知识管理工具，通过企业百科可以帮助企业将分散的知识信息进行聚合，并通过UGC的方式，促进企业知识的保鲜和流通&lt;/span&gt;&lt;/p&gt;"
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
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(default)]
pub struct RelatedMetaSubResp {
    /// 外部系统（不能包含中横线 "-"）
    ///
    /// **示例值**: "星云"
    #[serde(
        rename = "users",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub users: Vec<RefererSubResp>,
    /// 词条在外部系统中对应的唯一 ID（不能包含中横线 "-"）
    ///
    /// **示例值**: "client_653267498d"
    #[serde(
        rename = "chats",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub chats: Vec<RefererSubResp>,
    /// 关联文档信息
    #[serde(
        rename = "docs",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub docs: Vec<RefererSubResp>,
    /// 关联值班者信息
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
    /// 所属分类信息（不支持传入一级分类。词条不可同时属于同一个一级分类下的多个二级分类，一级分类下的二级分类互斥）
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
    /// 点踩数量
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
pub struct TermSubResp {
    /// 名称的值
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "飞书词典"
    #[serde(
        rename = "key",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub key: String,
    /// 名称展示范围
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
pub struct DisplayStatusSubResp {
    /// 对应名称是否在消息/云文档高亮
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "true"
    #[serde(
        rename = "allow_highlight",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub allow_highlight: bool,
    /// 对应名称是否在搜索结果中展示
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
    /// 数据 id
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "7dab8a3d3cdcc9da365777c7ad535d62"
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
    /// 相关其他词条 id
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
    /// 唯一分类 ID
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "7049606926702837761"
    #[serde(
        rename = "id",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub id: String,
    /// 分类名称
    ///
    /// **示例值**: "行业术语"
    #[serde(
        rename = "name",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub name: String,
    /// 父级分类的 ID
    ///
    /// **示例值**: "7049606926702837777"
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
    /// **示例值**: "xhatdi18973nekn"
    #[serde(
        rename = "token",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub token: String,
}

#[cfg(feature = "test-util")]
mod test_utils {
    use std::sync::Arc;

    use self::gen::baike::BaikeServiceMocker;
    use crate::core::mocker::*;

    use super::*;
    use crate::api::*;
    use crate::core::model::*;

    pub struct Mocker;

    pub trait MockFunc:
        Fn(CreateBaikeDraftReq) -> Result<(CreateBaikeDraftResp, CommonResponse), Error>
        + Send
        + Sync
        + 'static
    {
    }
    impl<
            T: Fn(CreateBaikeDraftReq) -> Result<(CreateBaikeDraftResp, CommonResponse), Error>
                + Send
                + Sync
                + 'static,
        > MockFunc for T
    {
    }

    impl<'c, IStore: Store, IClient: HttpClient> BaikeServiceMocker<'c, IStore, IClient> {
        pub fn mock_create_baike_draft<F: MockFunc>(
            &self,
            f: F,
        ) -> MockerBuilder<Mocker, CreateBaikeDraftReq, CreateBaikeDraftResp, Arc<dyn MockFunc>>
        {
            MockerBuilder::new(self.cli.instance_id, Arc::new(f))
        }

        pub(super) fn get_mock_create_baike_draft(
            &self,
            req: &CreateBaikeDraftReq,
        ) -> Option<Arc<dyn MockFunc>> {
            do_mock::<Mocker, CreateBaikeDraftReq, CreateBaikeDraftResp, Arc<dyn MockFunc>>(
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
        api::gen::baike::create_baike_draft::{
            CreateBaikeDraftReq, CreateBaikeDraftResp, CreateBaikeDraftRespInner,
        },
        core::{model::CommonResponse, Lark},
    };

    #[tokio::test]
    async fn test_mock() {
        let lark = Lark::new("".to_owned(), "".into());
        let mocker = lark
            .baike()
            .mock()
            .mock_create_baike_draft(|_| {
                Ok((CreateBaikeDraftResp::default(), CommonResponse::default()))
            })
            .build();
        let res = lark
            .baike()
            .create_baike_draft(CreateBaikeDraftReq::default())
            .await;
        assert!(res.is_ok());
        mocker.clear();
        let res = lark
            .baike()
            .create_baike_draft(CreateBaikeDraftReq::default())
            .await;
        assert!(res.is_err());
    }

    const REQ: &str = r#"{
    "id": "enterprise_40217521",
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
            "key": "飞书词典",
            "display_status": {
                "allow_highlight": true,
                "allow_search": true
            }
        }
    ],
    "description": "词典是飞书提供的一款知识管理工具，通过飞书词典可以帮助企业将分散的知识信息进行聚合，并通过UGC的方式，促进企业知识的保鲜和流通",
    "related_meta": {
        "users": [
            {
                "id": "格式请看请求体示例",
                "title": "飞书词典帮助中心"
            }
        ],
        "chats": [
            {
                "id": "格式请看请求体示例"
            }
        ],
        "docs": [
            {
                "title": "飞书词典帮助中心",
                "url": "https://www.feishu.cn/hc/zh-CN"
            }
        ],
        "oncalls": [
            {
                "id": "格式请看请求体示例"
            }
        ],
        "links": [
            {
                "title": "飞书词典帮助中心",
                "url": "https://www.feishu.cn/hc/zh-CN"
            }
        ],
        "abbreviations": [
            {
                "id": "enterprise_51527260"
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
                "token": "boxbcEcmKiDia3evgqWTpvdc7jc"
            }
        ]
    },
    "outer_info": {
        "provider": "星云",
        "outer_id": "client_653267498d"
    },
    "rich_text": "&lt;b&gt;加粗&lt;/b&gt;&lt;i&gt;斜体&lt;/i&gt;&lt;p&gt;&lt;a href=\"https://feishu.cn\"&gt;链接&lt;/a&gt;&lt;/p&gt;&lt;p&gt;&lt;span&gt;词典是飞书提供的一款知识管理工具，通过飞书词典可以帮助企业将分散的知识信息进行聚合，并通过UGC的方式，促进企业知识的保鲜和流通&lt;/span&gt;&lt;/p&gt;"
}"#;

    #[test]
    fn test_req() {
        if REQ == "{}" {
            return;
        }
        if let Err(e) = serde_json::from_str::<super::CreateBaikeDraftReqBody>(REQ) {
            panic!("{}", e);
        }
    }

    const RESP: &str = r#"{
    "code": 0,
    "msg": "success",
    "data": {
        "draft": {
            "draft_id": "54*7",
            "entity": {
                "id": "enterprise_40****21",
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
                        "key": "飞书词典",
                        "display_status": {
                            "allow_highlight": true,
                            "allow_search": true
                        }
                    }
                ],
                "description": "词典是飞书提供的一款知识管理工具，通过飞书词典可以帮助企业将分散的知识信息进行聚合，并通过UGC的方式，促进企业知识的保鲜和流通",
                "create_time": "1627540853",
                "update_time": "1627541853",
                "related_meta": {
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
                    "chats": [
                        {
                            "id": "oc_c13831833ea****2c52befa759ea4806"
                        },
                        {
                            "id": "oc_c8161c910996****127e73b10233b295"
                        }
                    ],
                    "docs": [
                        {
                            "title": "猜你想问 / FAQs",
                            "url": "https://bytedance.feishu.cn/wiki/w*****8Lq4f9DMCDOtdcIzCUjAh"
                        },
                        {
                            "title": "快速了解飞书文档 | Introducing Feishu Docs",
                            "url": "https://bytedance.feishu.cn/docs/doccnxlVCCFjMsJE15I****jIWc"
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
                            "id": "702368****445548034"
                        },
                        {
                            "id": "702406378****910850"
                        }
                    ],
                    "users": [
                        {
                            "id": "ou_30b07b63089****518789914dac63d36",
                            "title": "负责人"
                        },
                        {
                            "id": "ou_b292c0d285c****4639fa4501e80c36a",
                            "title": ""
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
                            "token": "boxbcEcmKi****HvgqWTpvdc7jc"
                        }
                    ]
                },
                "statistics": {
                    "like_count": 100,
                    "dislike_count": 20
                },
                "outer_info": {
                    "provider": "星云",
                    "outer_id": "client_6539i3498d"
                },
                "rich_text": "词典是飞书提供的一款知识管理工具，通过飞书词典可以帮助企业将分散的知识信息进行聚合，并通过UGC的方式，促进企业知识的保鲜和流通"
            }
        }
    }
}"#;
    #[test]
    fn test_resp() {
        let res = serde_json::from_str::<CreateBaikeDraftRespInner>(RESP);
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
