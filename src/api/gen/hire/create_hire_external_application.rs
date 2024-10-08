//! doc url: <https://open.larkoffice.com/document/ukTMukTMukTM/uMzM1YjLzMTN24yMzUjN/hire-v1/external_application/create>
// Code generated by gen_api. DO NOT EDIT.

use serde::{Deserialize, Serialize};

use crate::api::BaseResp;
use crate::api::HasBaseResp;
use crate::core::model::{ApiRequest, CommonResponse, MethodOption, ReqParam, StreamReqParam};
use crate::{
    core::{http_client::HttpClient, store::Store},
    error::Error,
};

use crate::api::gen::hire::HireService;

impl<'c, IStore: Store, IClient: HttpClient> HireService<'c, IStore, IClient> {
    /// **api 版本: 2024-07-03T06:47:52+00:00**
    ///
    /// ## 创建外部投递
    ///
    /// 导入来自其他系统的投递信息，创建为外部投递。
    ///

    ///
    /// doc: <https://open.larkoffice.com/document/ukTMukTMukTM/uMzM1YjLzMTN24yMzUjN/hire-v1/external_application/create>
    ///
    /// new doc: <https://open.larkoffice.com/document/server-docs/hire-v1/get-candidates/import-external-system-information/create>
    ///
    /// api url: <https://open.feishu.cn/document_portal/v1/document/get_detail?fullPath=%2Fserver-docs%2Fhire-v1%2Fget-candidates%2Fimport-external-system-information%2Fcreate>
    pub async fn create_hire_external_application(
        &self,
        req: CreateHireExternalApplicationReq,
    ) -> Result<(CreateHireExternalApplicationResp, CommonResponse), Error> {
        self.create_hire_external_application_with_opt(req, Default::default())
            .await
    }

    /// 参见 [create_hire_external_application](#method.create_hire_external_application) 函数
    pub async fn create_hire_external_application_with_opt(
        &self,
        req: CreateHireExternalApplicationReq,
        method_option: MethodOption,
    ) -> Result<(CreateHireExternalApplicationResp, CommonResponse), Error> {
        #[cfg(feature = "test-util")]
        {
            if let Some(f) = self.mock().get_mock_create_hire_external_application(&req) {
                tracing::info!("[lark] Hire#CreateHireExternalApplication **mocking** api");
                return f(req);
            }
        }

        tracing::info!("[lark] Hire#CreateHireExternalApplication call api");

        let req = ApiRequest {
            scope: "Hire",
            api: "CreateHireExternalApplication",
            method: http::Method::POST,
            url: String::new()
                + self.cli.open_base_url.as_ref()
                + "/open-apis/hire/v1/external_applications",
            param_data: req.gen_param(),
            method_option,
            need_tenant_access_token: true,
            ..Default::default()
        };

        let (resp, common_resp): (CreateHireExternalApplicationRespInner, _) =
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
pub struct CreateHireExternalApplicationReq {
    /// 外部系统投递主键 （仅用于幂等）
    ///
    /// **示例值**: "123"
    #[api(kind = "body", name = "external_id")]
    pub external_id: Option<String>,
    /// 职位招聘类型
    ///
    /// **示例值**: "1"
    ///
    /// **可选值**:
    ///
    /// `social_recruitment`: 社招
    ///
    /// `campus_recruitment`: 校招
    #[api(kind = "body", name = "job_recruitment_type")]
    pub job_recruitment_type: Option<i64>,
    /// 职位名称
    ///
    /// **示例值**: "高级Java"
    #[api(kind = "body", name = "job_title")]
    pub job_title: Option<String>,
    /// 简历来源
    ///
    /// **示例值**: "lagou"
    #[api(kind = "body", name = "resume_source")]
    pub resume_source: Option<String>,
    /// 阶段
    ///
    /// **示例值**: "1"
    #[api(kind = "body", name = "stage")]
    pub stage: Option<String>,
    /// 人才 ID
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "6960663240925956459"
    #[api(kind = "body", name = "talent_id")]
    pub talent_id: String,
    /// 终止原因
    ///
    /// **示例值**: "不合适"
    #[api(kind = "body", name = "termination_reason")]
    pub termination_reason: Option<String>,
    /// 投递类型
    ///
    /// **示例值**: "1"
    ///
    /// **可选值**:
    ///
    /// `HR_visit`: HR 寻访
    ///
    /// `candidate_delivery`: 候选人主动投递
    ///
    /// `talent_recommend`: 人才推荐
    ///
    /// `others`: 其他
    #[api(kind = "body", name = "delivery_type")]
    pub delivery_type: Option<i64>,
    /// 投递在外部系统终止时间
    ///
    /// **示例值**: "1618500278645"
    #[api(kind = "body", name = "modify_time")]
    pub modify_time: Option<i64>,
    /// 投递在外部系统创建时间
    ///
    /// **示例值**: "1618500278644"
    #[api(kind = "body", name = "create_time")]
    pub create_time: Option<i64>,
    /// 终止类型
    ///
    /// **示例值**: "health"
    #[api(kind = "body", name = "termination_type")]
    pub termination_type: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, lark_bot_sdk_macros::ApiBaseResp)]
struct CreateHireExternalApplicationRespInner {
    #[serde(flatten)]
    data: Option<CreateHireExternalApplicationResp>,
    #[serde(flatten)]
    base: BaseResp,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct CreateHireExternalApplicationResp {
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
    /// 外部投递信息
    #[serde(
        rename = "external_application",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub external_application: ExternalApplicationSubResp,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(default)]
pub struct ExternalApplicationSubResp {
    /// 外部投递 ID
    ///
    /// **示例值**: "6989202908470446380"
    #[serde(
        rename = "id",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub id: String,
    /// 职位招聘类型
    ///
    /// **示例值**: "1"
    ///
    /// **可选值**:
    ///
    /// `social_recruitment`: 社招
    ///
    /// `campus_recruitment`: 校招
    #[serde(
        rename = "job_recruitment_type",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub job_recruitment_type: i64,
    /// 职位名称
    ///
    /// **示例值**: "高级Java"
    #[serde(
        rename = "job_title",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub job_title: String,
    /// 简历来源
    ///
    /// **示例值**: "lagou"
    #[serde(
        rename = "resume_source",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub resume_source: String,
    /// 阶段
    ///
    /// **示例值**: "1"
    #[serde(
        rename = "stage",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub stage: String,
    /// 人才 ID
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "6960663240925956459"
    #[serde(
        rename = "talent_id",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub talent_id: String,
    /// 终止原因
    ///
    /// **示例值**: "不合适"
    #[serde(
        rename = "termination_reason",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub termination_reason: String,
    /// 投递类型
    ///
    /// **示例值**: "1"
    ///
    /// **可选值**:
    ///
    /// `HR_visit`: HR 寻访
    ///
    /// `candidate_delivery`: 候选人主动投递
    ///
    /// `talent_recommend`: 人才推荐
    ///
    /// `others`: 其他
    #[serde(
        rename = "delivery_type",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub delivery_type: i64,
    /// 投递在外部系统终止时间
    ///
    /// **示例值**: "1618500278645"
    #[serde(
        rename = "modify_time",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub modify_time: i64,
    /// 投递在外部系统创建时间
    ///
    /// **示例值**: "1618500278644"
    #[serde(
        rename = "create_time",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub create_time: i64,
    /// 终止类型
    ///
    /// **示例值**: "health"
    #[serde(
        rename = "termination_type",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub termination_type: String,
}

#[cfg(feature = "test-util")]
mod test_utils {
    use std::sync::Arc;

    use self::gen::hire::HireServiceMocker;
    use crate::core::mocker::*;

    use super::*;
    use crate::api::*;
    use crate::core::model::*;

    pub struct Mocker;

    pub trait MockFunc:
        Fn(
            CreateHireExternalApplicationReq,
        ) -> Result<(CreateHireExternalApplicationResp, CommonResponse), Error>
        + Send
        + Sync
        + 'static
    {
    }
    impl<
            T: Fn(
                    CreateHireExternalApplicationReq,
                )
                    -> Result<(CreateHireExternalApplicationResp, CommonResponse), Error>
                + Send
                + Sync
                + 'static,
        > MockFunc for T
    {
    }

    impl<'c, IStore: Store, IClient: HttpClient> HireServiceMocker<'c, IStore, IClient> {
        pub fn mock_create_hire_external_application<F: MockFunc>(
            &self,
            f: F,
        ) -> MockerBuilder<
            Mocker,
            CreateHireExternalApplicationReq,
            CreateHireExternalApplicationResp,
            Arc<dyn MockFunc>,
        > {
            MockerBuilder::new(self.cli.instance_id, Arc::new(f))
        }

        pub(super) fn get_mock_create_hire_external_application(
            &self,
            req: &CreateHireExternalApplicationReq,
        ) -> Option<Arc<dyn MockFunc>> {
            do_mock::<
                Mocker,
                CreateHireExternalApplicationReq,
                CreateHireExternalApplicationResp,
                Arc<dyn MockFunc>,
            >(self.cli.instance_id, req)
        }
    }
}

#[cfg(feature = "test-util")]
#[cfg(test)]
mod test {
    use crate::{
        api::gen::hire::create_hire_external_application::{
            CreateHireExternalApplicationReq, CreateHireExternalApplicationResp,
            CreateHireExternalApplicationRespInner,
        },
        core::{model::CommonResponse, Lark},
    };

    #[tokio::test]
    async fn test_mock() {
        let lark = Lark::new("".to_owned(), "".into());
        let mocker = lark
            .hire()
            .mock()
            .mock_create_hire_external_application(|_| {
                Ok((
                    CreateHireExternalApplicationResp::default(),
                    CommonResponse::default(),
                ))
            })
            .build();
        let res = lark
            .hire()
            .create_hire_external_application(CreateHireExternalApplicationReq::default())
            .await;
        assert!(res.is_ok());
        mocker.clear();
        let res = lark
            .hire()
            .create_hire_external_application(CreateHireExternalApplicationReq::default())
            .await;
        assert!(res.is_err());
    }

    const REQ: &str = r#"{
    "external_id": "123",
    "job_recruitment_type": 1,
    "job_title": "高级Java",
    "resume_source": "lagou",
    "stage": "1",
    "talent_id": "6960663240925956459",
    "termination_reason": "不合适",
    "delivery_type": 1,
    "modify_time": 1618500278645,
    "create_time": 1618500278644,
    "termination_type": "health"
}"#;

    #[test]
    fn test_req() {
        if REQ == "{}" {
            return;
        }
        if let Err(e) = serde_json::from_str::<super::CreateHireExternalApplicationReqBody>(REQ) {
            panic!("{}", e);
        }
    }

    const RESP: &str = r#"{
    "code": 0,
    "msg": "success",
    "data": {
        "external_application": {
            "id": "6989202908470446380",
            "job_recruitment_type": 1,
            "job_title": "高级Java",
            "resume_source": "lagou",
            "stage": "1",
            "talent_id": "6960663240925956459",
            "termination_reason": "不合适",
            "delivery_type": 1,
            "modify_time": 1618500278645,
            "create_time": 1618500278644,
            "termination_type": "health"
        }
    }
}"#;
    #[test]
    fn test_resp() {
        let res = serde_json::from_str::<CreateHireExternalApplicationRespInner>(RESP);
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
