//! doc url: <https://open.larkoffice.com/document/uAjLw4CM/ukTMukTMukTM/reference/contact-v3/job_title/get>
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
    /// **api 版本: 2024-07-05T08:23:37+00:00**
    ///
    /// ## 获取单个职务信息
    ///
    /// 调用该接口获取指定职务的信息，包括职务的 ID、名称、多语言名称以及启用状态。
    ///

    ///
    /// doc: <https://open.larkoffice.com/document/uAjLw4CM/ukTMukTMukTM/reference/contact-v3/job_title/get>
    ///
    /// new doc: <https://open.larkoffice.com/document/uAjLw4CM/ukTMukTMukTM/reference/contact-v3/job_title/get>
    ///
    /// api url: <https://open.feishu.cn/document_portal/v1/document/get_detail?fullPath=%2FuAjLw4CM%2FukTMukTMukTM%2Freference%2Fcontact-v3%2Fjob_title%2Fget>
    pub async fn get_contact_job_title(
        &self,
        req: GetContactJobTitleReq,
    ) -> Result<(GetContactJobTitleResp, CommonResponse), Error> {
        self.get_contact_job_title_with_opt(req, Default::default())
            .await
    }

    /// 参见 [get_contact_job_title](#method.get_contact_job_title) 函数
    pub async fn get_contact_job_title_with_opt(
        &self,
        req: GetContactJobTitleReq,
        method_option: MethodOption,
    ) -> Result<(GetContactJobTitleResp, CommonResponse), Error> {
        #[cfg(feature = "test-util")]
        {
            if let Some(f) = self.mock().get_mock_get_contact_job_title(&req) {
                tracing::info!("[lark] Contact#GetContactJobTitle **mocking** api");
                return f(req);
            }
        }

        tracing::info!("[lark] Contact#GetContactJobTitle call api");

        let req = ApiRequest {
            scope: "Contact",
            api: "GetContactJobTitle",
            method: http::Method::GET,
            url: String::new()
                + self.cli.open_base_url.as_ref()
                + "/open-apis/contact/v3/job_titles/:job_title_id",
            param_data: req.gen_param(),
            method_option,
            need_tenant_access_token: true,
            need_user_access_token: true,
            ..Default::default()
        };

        let (resp, common_resp): (GetContactJobTitleRespInner, _) = self.cli.do_req(req).await?;
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
pub struct GetContactJobTitleReq {
    /// 职务 ID。你可以调用[获取租户职务列表](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/contact-v3/job_title/list)接口获取职务 ID。
    ///
    /// **示例值**: "dd39369b19b9"
    #[api(kind = "path", name = "job_title_id")]
    pub job_title_id: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, lark_bot_sdk_macros::ApiBaseResp)]
struct GetContactJobTitleRespInner {
    #[serde(flatten)]
    data: Option<GetContactJobTitleResp>,
    #[serde(flatten)]
    base: BaseResp,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct GetContactJobTitleResp {
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
    /// 职务信息。
    #[serde(
        rename = "job_title",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub job_title: JobTitleSubResp,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(default)]
pub struct JobTitleSubResp {
    /// 职务 ID。
    ///
    /// **示例值**: "b5565c46b749"
    #[serde(
        rename = "job_title_id",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub job_title_id: String,
    /// 职务名称。
    ///
    /// **示例值**: "高级工程师"
    ///
    /// **数据校验规则**：
    ///
    /// - **长度范围**: `1` 字符- `100` 字符
    #[serde(
        rename = "name",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub name: String,
    /// 多语言职务名称。
    #[serde(
        rename = "i18n_name",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub i18n_name: Vec<I18nContentSubResp>,
    /// 是否启用职务。
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
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(default)]
pub struct I18nContentSubResp {
    /// 语言版本。例如：
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
    pub locale: String,
    /// 多语言版本对应的值。
    ///
    /// **示例值**: "专家"
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
        Fn(GetContactJobTitleReq) -> Result<(GetContactJobTitleResp, CommonResponse), Error>
        + Send
        + Sync
        + 'static
    {
    }
    impl<
            T: Fn(GetContactJobTitleReq) -> Result<(GetContactJobTitleResp, CommonResponse), Error>
                + Send
                + Sync
                + 'static,
        > MockFunc for T
    {
    }

    impl<'c, IStore: Store, IClient: HttpClient> ContactServiceMocker<'c, IStore, IClient> {
        pub fn mock_get_contact_job_title<F: MockFunc>(
            &self,
            f: F,
        ) -> MockerBuilder<Mocker, GetContactJobTitleReq, GetContactJobTitleResp, Arc<dyn MockFunc>>
        {
            MockerBuilder::new(self.cli.instance_id, Arc::new(f))
        }

        pub(super) fn get_mock_get_contact_job_title(
            &self,
            req: &GetContactJobTitleReq,
        ) -> Option<Arc<dyn MockFunc>> {
            do_mock::<Mocker, GetContactJobTitleReq, GetContactJobTitleResp, Arc<dyn MockFunc>>(
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
        api::gen::contact::get_contact_job_title::{
            GetContactJobTitleReq, GetContactJobTitleResp, GetContactJobTitleRespInner,
        },
        core::{model::CommonResponse, Lark},
    };

    #[tokio::test]
    async fn test_mock() {
        let lark = Lark::new("".to_owned(), "".into());
        let mocker = lark
            .contact()
            .mock()
            .mock_get_contact_job_title(|_| {
                Ok((GetContactJobTitleResp::default(), CommonResponse::default()))
            })
            .build();
        let res = lark
            .contact()
            .get_contact_job_title(GetContactJobTitleReq::default())
            .await;
        assert!(res.is_ok());
        mocker.clear();
        let res = lark
            .contact()
            .get_contact_job_title(GetContactJobTitleReq::default())
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

    const RESP: &str = r#"{"code":0,
"msg":"success",
"data":{"job_title":{"job_title_id":"b5565c46b749",
"name":"高级工程师",
"i18n_name":[{
    "locale": "zh_cn",
    "value": "专家"
}],
"status":true}}}"#;
    #[test]
    fn test_resp() {
        let res = serde_json::from_str::<GetContactJobTitleRespInner>(RESP);
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
