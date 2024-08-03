//! doc url: <https://open.larkoffice.com/document/uAjLw4CM/ukTMukTMukTM/helpdesk-v1/faq/create>
// Code generated by gen_api. DO NOT EDIT.

use serde::{Deserialize, Serialize};

use crate::api::BaseResp;
use crate::api::HasBaseResp;
use crate::core::model::{ApiRequest, CommonResponse, MethodOption, ReqParam, StreamReqParam};
use crate::{
    core::{http_client::HttpClient, store::Store},
    error::Error,
};

use crate::api::gen::helpdesk::HelpdeskService;

impl<'c, IStore: Store, IClient: HttpClient> HelpdeskService<'c, IStore, IClient> {
    /// **api 版本: 2024-03-06T11:33:59+00:00**
    ///
    /// ## 创建知识库
    ///
    /// 该接口用于创建知识库。
    ///

    ///
    /// doc: <https://open.larkoffice.com/document/uAjLw4CM/ukTMukTMukTM/helpdesk-v1/faq/create>
    ///
    /// new doc: <https://open.larkoffice.com/document/server-docs/helpdesk-v1/faq-management/faq/create>
    ///
    /// api url: <https://open.feishu.cn/document_portal/v1/document/get_detail?fullPath=%2Fserver-docs%2Fhelpdesk-v1%2Ffaq-management%2Ffaq%2Fcreate>
    pub async fn create_helpdesk_faq(
        &self,
        req: CreateHelpdeskFaqReq,
    ) -> Result<(CreateHelpdeskFaqResp, CommonResponse), Error> {
        self.create_helpdesk_faq_with_opt(req, Default::default())
            .await
    }

    /// 参见 [create_helpdesk_faq](#method.create_helpdesk_faq) 函数
    pub async fn create_helpdesk_faq_with_opt(
        &self,
        req: CreateHelpdeskFaqReq,
        method_option: MethodOption,
    ) -> Result<(CreateHelpdeskFaqResp, CommonResponse), Error> {
        #[cfg(feature = "test-util")]
        {
            if let Some(f) = self.mock().get_mock_create_helpdesk_faq(&req) {
                tracing::info!("[lark] Helpdesk#CreateHelpdeskFaq **mocking** api");
                return f(req);
            }
        }

        tracing::info!("[lark] Helpdesk#CreateHelpdeskFaq call api");

        let req = ApiRequest {
            scope: "Helpdesk",
            api: "CreateHelpdeskFaq",
            method: http::Method::POST,
            url: String::new() + self.cli.open_base_url.as_ref() + "/open-apis/helpdesk/v1/faqs",
            param_data: req.gen_param(),
            method_option,
            need_user_access_token: true,
            ..Default::default()
        };

        let (resp, common_resp): (CreateHelpdeskFaqRespInner, _) = self.cli.do_req(req).await?;
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
pub struct CreateHelpdeskFaqReq {
    /// 知识库详情
    #[api(kind = "body", name = "faq")]
    pub faq: Option<FaqUpdateInfoSubReq>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(default)]
pub struct FaqUpdateInfoSubReq {
    /// 知识库分类ID
    ///
    /// **示例值**: "6836004780707807251"
    #[serde(
        rename = "category_id",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub category_id: Option<String>,
    /// 问题
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "问题"
    #[serde(
        rename = "question",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub question: String,
    /// 答案
    ///
    /// **示例值**: "答案"
    #[serde(
        rename = "answer",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub answer: Option<String>,
    /// 富文本答案和答案必须有一个必填。Json Array格式，富文本结构请见[了解更多: 富文本](https://open.feishu.cn/document/ukTMukTMukTM/uITM0YjLyEDN24iMxQjN)
    ///
    /// **示例值**: "{\"content\":\"这只是一个测试，医保问题\",\"type\":\"text\"}"
    #[serde(
        rename = "answer_richtext",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub answer_richtext: Option<String>,
    /// 相似问题
    ///
    /// **示例值**: "["tag1","tag2","tag3"]"
    #[serde(
        rename = "tags",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub tags: Vec<Option<String>>,
}

#[derive(Debug, Serialize, Deserialize, Clone, lark_bot_sdk_macros::ApiBaseResp)]
struct CreateHelpdeskFaqRespInner {
    #[serde(flatten)]
    data: Option<CreateHelpdeskFaqResp>,
    #[serde(flatten)]
    base: BaseResp,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct CreateHelpdeskFaqResp {
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
    /// 知识库详情
    #[serde(
        rename = "faq",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub faq: FaqSubResp,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(default)]
pub struct FaqSubResp {
    /// 知识库ID
    ///
    /// **示例值**: "6936004780707807231"
    #[serde(
        rename = "faq_id",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub faq_id: String,
    /// 知识库旧版ID，请使用faq_id
    ///
    /// **示例值**: "6936004780707807231"
    #[serde(
        rename = "id",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub id: String,
    /// 服务台ID
    ///
    /// **示例值**: "6936004780707807251"
    #[serde(
        rename = "helpdesk_id",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub helpdesk_id: String,
    /// 问题
    ///
    /// **示例值**: "问题"
    #[serde(
        rename = "question",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub question: String,
    /// 答案
    ///
    /// **示例值**: "答案"
    #[serde(
        rename = "answer",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub answer: String,
    /// 富文本答案
    #[serde(
        rename = "answer_richtext",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub answer_richtext: Vec<RichtextSubResp>,
    /// 创建时间
    ///
    /// **示例值**: "1596379008"
    #[serde(
        rename = "create_time",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub create_time: i64,
    /// 修改时间
    ///
    /// **示例值**: "1596379008"
    #[serde(
        rename = "update_time",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub update_time: i64,
    /// 分类
    #[serde(
        rename = "categories",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub categories: Vec<CategorySubResp>,
    /// 相似问题列表
    #[serde(
        rename = "tags",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub tags: Vec<String>,
    /// 失效时间
    ///
    /// **示例值**: "1596379008"
    #[serde(
        rename = "expire_time",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub expire_time: i64,
    /// 更新用户
    #[serde(
        rename = "update_user",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub update_user: TicketUserSubResp,
    /// 创建用户
    #[serde(
        rename = "create_user",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub create_user: TicketUserSubResp,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(default)]
pub struct TicketUserSubResp {
    /// 用户ID
    ///
    /// **示例值**: "ou_37019b7c830210acd88fdce886e25c71"
    #[serde(
        rename = "id",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub id: String,
    /// 用户头像url
    ///
    /// **示例值**: "https://xxxx"
    #[serde(
        rename = "avatar_url",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub avatar_url: String,
    /// 用户名
    ///
    /// **示例值**: "abc"
    #[serde(
        rename = "name",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub name: String,
    /// 所在部门名称
    ///
    /// **示例值**: "用户部门名称(有权限才展示)"
    #[serde(
        rename = "department",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub department: String,
    /// 城市
    ///
    /// **示例值**: "城市"
    #[serde(
        rename = "city",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub city: String,
    /// 国家代号(CountryCode)，参考：http://www.mamicode.com/info-detail-2186501.html
    ///
    /// **示例值**: "国家"
    #[serde(
        rename = "country",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub country: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(default)]
pub struct RichtextSubResp {
    /// 内容
    ///
    /// **示例值**: "我的答案"
    #[serde(
        rename = "content",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub content: String,
    /// 类型
    ///
    /// **示例值**: "text"
    #[serde(
        rename = "type",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub body_type: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(default)]
pub struct CategorySubResp {
    /// 知识库分类ID
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "6948728206392295444"
    #[serde(
        rename = "category_id",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub category_id: String,
    /// 知识库分类ID，（旧版，请使用category_id）
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "6948728206392295444"
    #[serde(
        rename = "id",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub id: String,
    /// 名称
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "创建团队和邀请成员"
    #[serde(
        rename = "name",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub name: String,
    /// 父知识库分类ID
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "0"
    #[serde(
        rename = "parent_id",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub parent_id: String,
    /// 服务台ID
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "6939771743531696147"
    #[serde(
        rename = "helpdesk_id",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub helpdesk_id: String,
    /// 语言
    ///
    /// **示例值**: "zh_cn"
    #[serde(
        rename = "language",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub language: String,
}

#[cfg(feature = "test-util")]
mod test_utils {
    use std::sync::Arc;

    use self::gen::helpdesk::HelpdeskServiceMocker;
    use crate::core::mocker::*;

    use super::*;
    use crate::api::*;
    use crate::core::model::*;

    pub struct Mocker;

    pub trait MockFunc:
        Fn(CreateHelpdeskFaqReq) -> Result<(CreateHelpdeskFaqResp, CommonResponse), Error>
        + Send
        + Sync
        + 'static
    {
    }
    impl<
            T: Fn(CreateHelpdeskFaqReq) -> Result<(CreateHelpdeskFaqResp, CommonResponse), Error>
                + Send
                + Sync
                + 'static,
        > MockFunc for T
    {
    }

    impl<'c, IStore: Store, IClient: HttpClient> HelpdeskServiceMocker<'c, IStore, IClient> {
        pub fn mock_create_helpdesk_faq<F: MockFunc>(
            &self,
            f: F,
        ) -> MockerBuilder<Mocker, CreateHelpdeskFaqReq, CreateHelpdeskFaqResp, Arc<dyn MockFunc>>
        {
            MockerBuilder::new(self.cli.instance_id, Arc::new(f))
        }

        pub(super) fn get_mock_create_helpdesk_faq(
            &self,
            req: &CreateHelpdeskFaqReq,
        ) -> Option<Arc<dyn MockFunc>> {
            do_mock::<Mocker, CreateHelpdeskFaqReq, CreateHelpdeskFaqResp, Arc<dyn MockFunc>>(
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
        api::gen::helpdesk::create_helpdesk_faq::{
            CreateHelpdeskFaqReq, CreateHelpdeskFaqResp, CreateHelpdeskFaqRespInner,
        },
        core::{model::CommonResponse, Lark},
    };

    #[tokio::test]
    async fn test_mock() {
        let lark = Lark::new("".to_owned(), "".into());
        let mocker = lark
            .helpdesk()
            .mock()
            .mock_create_helpdesk_faq(|_| {
                Ok((CreateHelpdeskFaqResp::default(), CommonResponse::default()))
            })
            .build();
        let res = lark
            .helpdesk()
            .create_helpdesk_faq(CreateHelpdeskFaqReq::default())
            .await;
        assert!(res.is_ok());
        mocker.clear();
        let res = lark
            .helpdesk()
            .create_helpdesk_faq(CreateHelpdeskFaqReq::default())
            .await;
        assert!(res.is_err());
    }

    const REQ: &str = r#"{
    "faq": {
        "category_id": "6836004780707807251",
        "question": "问题",
        "answer": "答案",
        "answer_richtext": "{\"content\":\"这只是一个测试，医保问题\",\"type\":\"text\"}",
        "tags": [
            "问",
            "题"
        ]
    }
}"#;

    #[test]
    fn test_req() {
        if REQ == "{}" {
            return;
        }
        if let Err(e) = serde_json::from_str::<super::CreateHelpdeskFaqReqBody>(REQ) {
            panic!("{}", e);
        }
    }

    const RESP: &str = r#"{
    "code": 0,
    "msg": "success",
    "data": {
        "faq": {
            "faq_id": "6936004780707807231",
            "id": "6936004780707807231",
            "helpdesk_id": "6936004780707807251",
            "question": "问题",
            "answer": "答案",
            "answer_richtext": [
                {
                    "content": "我的答案",
                    "type": "text"
                }
            ],
            "create_time": 1596379008,
            "update_time": 1596379008,
            "categories": [
                {
                    "category_id": "6948728206392295444",
                    "id": "6948728206392295444",
                    "name": "创建团队和邀请成员",
                    "parent_id": "0",
                    "helpdesk_id": "6939771743531696147",
                    "language": "zh_cn"
                }
            ],
            "tags": [
                "问",
                "题"
            ],
            "expire_time": 1596379008,
            "update_user": {
                "id": "ou_37019b7c830210acd88fdce886e25c71",
                "avatar_url": "https://xxxx",
                "name": "abc",
                "department": "用户部门名称(有权限才展示)",
                "city": "城市",
                "country": "国家"
            },
            "create_user": {
                "id": "ou_37019b7c830210acd88fdce886e25c71",
                "avatar_url": "https://xxxx",
                "name": "abc",
                "department": "用户部门名称(有权限才展示)",
                "city": "城市",
                "country": "国家"
            }
        }
    }
}"#;
    #[test]
    fn test_resp() {
        let res = serde_json::from_str::<CreateHelpdeskFaqRespInner>(RESP);
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
