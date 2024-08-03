//! doc url: <https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/contact-v3/job_family/update>
// Code generated by gen_api. DO NOT EDIT.

use serde::{Deserialize, Serialize};

use crate::api::BaseResp;
use crate::api::HasBaseResp;
use crate::core::model::{ApiRequest, CommonResponse, MethodOption, ReqParam, StreamReqParam};
use crate::{
    core::{http_client::HttpClient, store::Store},
    error::Error,
};

use crate::api::gen::contact::ContactService;

impl<'c, IStore: Store, IClient: HttpClient> ContactService<'c, IStore, IClient> {
    /// **api 版本: 2024-07-05T08:22:04+00:00**
    ///
    /// ## 更新序列
    ///
    /// 调用该接口更新指定序列的信息。
    ///

    ///
    /// doc: <https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/contact-v3/job_family/update>
    ///
    /// new doc: <https://open.feishu.cn/document/server-docs/contact-v3/job_family/update>
    ///
    /// api url: <https://open.feishu.cn/document_portal/v1/document/get_detail?fullPath=%2Fserver-docs%2Fcontact-v3%2Fjob_family%2Fupdate>
    pub async fn update_contact_job_family(
        &self,
        req: UpdateContactJobFamilyReq,
    ) -> Result<(UpdateContactJobFamilyResp, CommonResponse), Error> {
        self.update_contact_job_family_with_opt(req, Default::default())
            .await
    }

    /// 参见 [update_contact_job_family](#method.update_contact_job_family) 函数
    pub async fn update_contact_job_family_with_opt(
        &self,
        req: UpdateContactJobFamilyReq,
        method_option: MethodOption,
    ) -> Result<(UpdateContactJobFamilyResp, CommonResponse), Error> {
        #[cfg(feature = "test-util")]
        {
            if let Some(f) = self.mock().get_mock_update_contact_job_family(&req) {
                tracing::info!("[lark] Contact#UpdateContactJobFamily **mocking** api");
                return f(req);
            }
        }

        tracing::info!("[lark] Contact#UpdateContactJobFamily call api");

        let req = ApiRequest {
            scope: "Contact",
            api: "UpdateContactJobFamily",
            method: http::Method::PUT,
            url: String::new()
                + self.cli.open_base_url.as_ref()
                + "/open-apis/contact/v3/job_families/:job_family_id",
            param_data: req.gen_param(),
            method_option,
            need_tenant_access_token: true,
            ..Default::default()
        };

        let (resp, common_resp): (UpdateContactJobFamilyRespInner, _) =
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
pub struct UpdateContactJobFamilyReq {
    /// 序列 ID。获取方式：
    ///
    /// - [创建序列](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/contact-v3/job_family/create)时可以从返回结果中获取（job_family_id）。
    ///
    /// - 调用[获取租户序列列表](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/contact-v3/job_family/list)接口获取序列 ID。
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "mga5oa8ayjlpkzy"
    #[api(kind = "path", name = "job_family_id")]
    pub job_family_id: String,

    /// 序列名称，租户内唯一。取值支持中、英文及符号。
    ///
    /// **默认值**：空，表示不更新
    ///
    /// **示例值**: "产品"
    ///
    /// **数据校验规则**：
    ///
    /// - **长度范围**: `1` 字符- `100` 字符
    #[api(kind = "body", name = "name")]
    pub name: Option<String>,
    /// 序列描述，描述序列详情信息。字符长度上限为 5,000。
    ///
    /// **默认值**：空，表示不更新
    ///
    /// **示例值**: "负责产品策略制定的相关工作"
    #[api(kind = "body", name = "description")]
    pub description: Option<String>,
    /// 上级序列 ID。你可以调用[获取租户序列列表](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/contact-v3/job_family/list)接口，获取序列 ID。
    ///
    /// **默认值**：空，表示不更新
    ///
    /// **示例值**: "mga5oa8ayjlpzjq"
    #[api(kind = "body", name = "parent_job_family_id")]
    pub parent_job_family_id: Option<String>,
    /// 是否启用序列。
    ///
    /// **可选值有**：
    ///
    /// - true：启用
    ///
    /// - false：禁用
    ///
    /// **默认值**：空，表示不更新
    ///
    /// **示例值**: "true"
    #[api(kind = "body", name = "status")]
    pub status: Option<bool>,
    /// 多语言序列名称。
    ///
    /// **默认值**：空，表示不更新
    #[api(kind = "body", name = "i18n_name")]
    pub i18n_name: Vec<Option<I18nContentSubReq>>,
    /// 多语言序列描述。
    ///
    /// **默认值**：空，表示不更新
    #[api(kind = "body", name = "i18n_description")]
    pub i18n_description: Vec<Option<I18nContentSubReq>>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(default)]
pub struct I18nContentSubReq {
    /// 语言版本。可选值有：
    ///
    /// - zh_cn：中文
    ///
    /// - en_us：英语
    ///
    /// - ja_jp：日语
    ///
    /// **示例值**: "zh_cn"
    #[serde(
        rename = "locale",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub locale: Option<String>,
    /// 语言版本对应的值。
    ///
    /// **示例值**: "多语言内容"
    #[serde(
        rename = "value",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub value: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, lark_bot_sdk_macros::ApiBaseResp)]
struct UpdateContactJobFamilyRespInner {
    #[serde(flatten)]
    data: Option<UpdateContactJobFamilyResp>,
    #[serde(flatten)]
    base: BaseResp,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct UpdateContactJobFamilyResp {
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
    /// 更新后的序列信息。
    #[serde(
        rename = "job_family",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub job_family: JobFamilySubResp,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(default)]
pub struct JobFamilySubResp {
    /// 序列名称。
    ///
    /// **示例值**: "产品"
    ///
    /// **数据校验规则**：
    ///
    /// - **长度范围**: `1` 字符- `100` 字符
    #[serde(
        rename = "name",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub name: String,
    /// 序列描述。
    ///
    /// **示例值**: "负责产品策略制定的相关工作"
    #[serde(
        rename = "description",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub description: String,
    /// 上级序列 ID。
    ///
    /// **示例值**: "mga5oa8ayjlpzjq"
    #[serde(
        rename = "parent_job_family_id",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub parent_job_family_id: String,
    /// 是否启用序列。
    ///
    /// **可能值有**：
    ///
    /// - true：启用
    ///
    /// - false：禁用
    ///
    /// **示例值**: "true"
    #[serde(
        rename = "status",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub status: bool,
    /// 多语言序列名称。
    #[serde(
        rename = "i18n_name",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub i18n_name: Vec<I18nContentSubResp>,
    /// 多语言序列描述。
    #[serde(
        rename = "i18n_description",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub i18n_description: Vec<I18nContentSubResp>,
    /// 序列 ID。后续可通过该 ID 更新、查询、删除序列。
    ///
    /// **示例值**: "mga5oa8ayjlpkzy"
    #[serde(
        rename = "job_family_id",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub job_family_id: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(default)]
pub struct I18nContentSubResp {
    /// 语言版本。
    ///
    /// **示例值**: "zh_cn"
    #[serde(
        rename = "locale",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub locale: String,
    /// 语言版本对应的值。
    ///
    /// **示例值**: "多语言内容"
    #[serde(
        rename = "value",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub value: String,
}

#[cfg(feature = "test-util")]
mod test_utils {
    use std::sync::Arc;

    use self::gen::contact::ContactServiceMocker;
    use crate::core::mocker::*;

    use super::*;
    use crate::api::*;
    use crate::core::model::*;

    pub struct Mocker;

    pub trait MockFunc:
        Fn(UpdateContactJobFamilyReq) -> Result<(UpdateContactJobFamilyResp, CommonResponse), Error>
        + Send
        + Sync
        + 'static
    {
    }
    impl<
            T: Fn(
                    UpdateContactJobFamilyReq,
                ) -> Result<(UpdateContactJobFamilyResp, CommonResponse), Error>
                + Send
                + Sync
                + 'static,
        > MockFunc for T
    {
    }

    impl<'c, IStore: Store, IClient: HttpClient> ContactServiceMocker<'c, IStore, IClient> {
        pub fn mock_update_contact_job_family<F: MockFunc>(
            &self,
            f: F,
        ) -> MockerBuilder<
            Mocker,
            UpdateContactJobFamilyReq,
            UpdateContactJobFamilyResp,
            Arc<dyn MockFunc>,
        > {
            MockerBuilder::new(self.cli.instance_id, Arc::new(f))
        }

        pub(super) fn get_mock_update_contact_job_family(
            &self,
            req: &UpdateContactJobFamilyReq,
        ) -> Option<Arc<dyn MockFunc>> {
            do_mock::<
                Mocker,
                UpdateContactJobFamilyReq,
                UpdateContactJobFamilyResp,
                Arc<dyn MockFunc>,
            >(self.cli.instance_id, req)
        }
    }
}

#[cfg(feature = "test-util")]
#[cfg(test)]
mod test {
    use crate::{
        api::gen::contact::update_contact_job_family::{
            UpdateContactJobFamilyReq, UpdateContactJobFamilyResp, UpdateContactJobFamilyRespInner,
        },
        core::{model::CommonResponse, Lark},
    };

    #[tokio::test]
    async fn test_mock() {
        let lark = Lark::new("".to_owned(), "".into());
        let mocker = lark
            .contact()
            .mock()
            .mock_update_contact_job_family(|_| {
                Ok((
                    UpdateContactJobFamilyResp::default(),
                    CommonResponse::default(),
                ))
            })
            .build();
        let res = lark
            .contact()
            .update_contact_job_family(UpdateContactJobFamilyReq::default())
            .await;
        assert!(res.is_ok());
        mocker.clear();
        let res = lark
            .contact()
            .update_contact_job_family(UpdateContactJobFamilyReq::default())
            .await;
        assert!(res.is_err());
    }

    const REQ: &str = r#"{
    "name": "产品",
    "description": "负责产品策略制定的相关工作",
    "parent_job_family_id": "mga5oa8ayjlpzjq",
    "status": true,
    "i18n_name": [
        {
            "locale": "zh_cn",
            "value": "多语言内容"
        }
    ],
    "i18n_description": [
        {
            "locale": "zh_cn",
            "value": "多语言内容"
        }
    ]
}"#;

    #[test]
    fn test_req() {
        if REQ == "{}" {
            return;
        }
        if let Err(e) = serde_json::from_str::<super::UpdateContactJobFamilyReqBody>(REQ) {
            panic!("{}", e);
        }
    }

    const RESP: &str = r#"{
    "code": 0,
    "msg": "success",
    "data": {
        "job_family": {
            "name": "产品",
            "description": "负责产品策略制定的相关工作",
            "parent_job_family_id": "mga5oa8ayjlpzjq",
            "status": true,
            "i18n_name": [
                {
                    "locale": "zh_cn",
                    "value": "多语言内容"
                }
            ],
            "i18n_description": [
                {
                    "locale": "zh_cn",
                    "value": "多语言内容"
                }
            ],
            "job_family_id": "mga5oa8ayjlpkzy"
        }
    }
}"#;
    #[test]
    fn test_resp() {
        let res = serde_json::from_str::<UpdateContactJobFamilyRespInner>(RESP);
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
