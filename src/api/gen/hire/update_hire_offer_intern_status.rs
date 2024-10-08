//! doc url: <https://open.larkoffice.com/document/ukTMukTMukTM/uMzM1YjLzMTN24yMzUjN/hire-v1/offer/intern_offer_status>
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
    /// **api 版本: 2024-07-11T03:51:01+00:00**
    ///
    /// ## 更新实习 Offer 入/离职状态
    ///
    /// 对「实习待入职」状态的实习 Offer 确认入职、放弃入职，或对「实习已入职」状态的实习 Offer 操作离职。
    ///

    ///
    /// doc: <https://open.larkoffice.com/document/ukTMukTMukTM/uMzM1YjLzMTN24yMzUjN/hire-v1/offer/intern_offer_status>
    ///
    /// new doc: <https://open.larkoffice.com/document/server-docs/hire-v1/candidate-management/delivery-process-management/offer/intern_offer_status>
    ///
    /// api url: <https://open.feishu.cn/document_portal/v1/document/get_detail?fullPath=%2Fserver-docs%2Fhire-v1%2Fcandidate-management%2Fdelivery-process-management%2Foffer%2Fintern_offer_status>
    pub async fn update_hire_offer_intern_status(
        &self,
        req: UpdateHireOfferInternStatusReq,
    ) -> Result<(UpdateHireOfferInternStatusResp, CommonResponse), Error> {
        self.update_hire_offer_intern_status_with_opt(req, Default::default())
            .await
    }

    /// 参见 [update_hire_offer_intern_status](#method.update_hire_offer_intern_status) 函数
    pub async fn update_hire_offer_intern_status_with_opt(
        &self,
        req: UpdateHireOfferInternStatusReq,
        method_option: MethodOption,
    ) -> Result<(UpdateHireOfferInternStatusResp, CommonResponse), Error> {
        #[cfg(feature = "test-util")]
        {
            if let Some(f) = self.mock().get_mock_update_hire_offer_intern_status(&req) {
                tracing::info!("[lark] Hire#UpdateHireOfferInternStatus **mocking** api");
                return f(req);
            }
        }

        tracing::info!("[lark] Hire#UpdateHireOfferInternStatus call api");

        let req = ApiRequest {
            scope: "Hire",
            api: "UpdateHireOfferInternStatus",
            method: http::Method::POST,
            url: String::new()
                + self.cli.open_base_url.as_ref()
                + "/open-apis/hire/v1/offers/:offer_id/intern_offer_status",
            param_data: req.gen_param(),
            method_option,
            need_tenant_access_token: true,
            ..Default::default()
        };

        let (resp, common_resp): (UpdateHireOfferInternStatusRespInner, _) =
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
pub struct UpdateHireOfferInternStatusReq {
    /// Offer ID，如何获取请参考[获取 Offer 列表](https://open.feishu.cn/document/ukTMukTMukTM/uMzM1YjLzMTN24yMzUjN/hire-v1/offer/list)
    ///
    /// **示例值**: "7016605170635213100"
    #[api(kind = "path", name = "offer_id")]
    pub offer_id: String,

    /// 更新入/离职状态的操作
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "confirm_onboarding"
    ///
    /// **可选值**:
    ///
    /// `confirm_onboarding`: 确认入职
    ///
    /// `cancel_onboarding`: 放弃入职
    ///
    /// `offboard`: 操作离职
    #[api(kind = "body", name = "operation")]
    pub operation: String,
    /// 入职表单信息
    ///
    /// <br>
    ///
    /// **注意**：当 operation 为 `confirm_onboarding` 时，该字段必填
    #[api(kind = "body", name = "onboarding_info")]
    pub onboarding_info: Option<InternOfferOnboardingInfoSubReq>,
    /// 离职表单信息
    ///
    /// <br>
    ///
    /// **注意**：当 operation 为 `offboard` 时，该字段必填
    #[api(kind = "body", name = "offboarding_info")]
    pub offboarding_info: Option<InternOfferOffboardingInfoSubReq>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(default)]
pub struct InternOfferOnboardingInfoSubReq {
    /// 实际入职日期
    ///
    /// <br>
    ///
    /// **值格式**："YYYY-MM-DD"
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "2022-01-01"
    #[serde(
        rename = "actual_onboarding_date",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub actual_onboarding_date: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(default)]
pub struct InternOfferOffboardingInfoSubReq {
    /// 实际离职日期
    ///
    /// <br>
    ///
    /// **注意**：实际离职日期需晚于实际入职日期
    ///
    /// <br>
    ///
    /// **值格式**："YYYY-MM-DD"
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "2022-03-02"
    #[serde(
        rename = "actual_offboarding_date",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub actual_offboarding_date: String,
    /// 备注
    ///
    /// **示例值**: "主动离职"
    #[serde(
        rename = "notes",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub notes: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, lark_bot_sdk_macros::ApiBaseResp)]
struct UpdateHireOfferInternStatusRespInner {
    #[serde(flatten)]
    data: Option<UpdateHireOfferInternStatusResp>,
    #[serde(flatten)]
    base: BaseResp,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct UpdateHireOfferInternStatusResp {
    /// \-
    #[serde(
        rename = "data",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub data: InternOfferStatusSubResp,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(default)]
pub struct InternOfferStatusSubResp {
    /// Offer ID，详细信息请参考[获取 Offer 信息](https://open.feishu.cn/document/ukTMukTMukTM/uMzM1YjLzMTN24yMzUjN/hire-v1/application/offer)
    ///
    /// **示例值**: "6949805467799537964"
    #[serde(
        rename = "offer_id",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub offer_id: String,
    /// 更新入/离职状态的操作
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "confirm_onboarding"
    ///
    /// **可选值**:
    ///
    /// `confirm_onboarding`: 确认入职
    ///
    /// `cancel_onboarding`: 放弃入职
    ///
    /// `offboard`: 操作离职
    #[serde(
        rename = "operation",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub operation: String,
    /// 入职表单信息
    #[serde(
        rename = "onboarding_info",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub onboarding_info: InternOfferOnboardingInfoSubResp,
    /// 离职表单信息
    #[serde(
        rename = "offboarding_info",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub offboarding_info: InternOfferOffboardingInfoSubResp,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(default)]
pub struct InternOfferOnboardingInfoSubResp {
    /// 实际入职日期
    ///
    /// <br>
    ///
    /// **值格式**："YYYY-MM-DD"
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "2022-01-01"
    #[serde(
        rename = "actual_onboarding_date",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub actual_onboarding_date: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(default)]
pub struct InternOfferOffboardingInfoSubResp {
    /// 实际离职日期
    ///
    /// <br>
    ///
    /// **值格式**："YYYY-MM-DD"
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "2022-03-02"
    #[serde(
        rename = "actual_offboarding_date",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub actual_offboarding_date: String,
    /// 备注
    ///
    /// **示例值**: "主动离职"
    #[serde(
        rename = "notes",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub notes: String,
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
            UpdateHireOfferInternStatusReq,
        ) -> Result<(UpdateHireOfferInternStatusResp, CommonResponse), Error>
        + Send
        + Sync
        + 'static
    {
    }
    impl<
            T: Fn(
                    UpdateHireOfferInternStatusReq,
                )
                    -> Result<(UpdateHireOfferInternStatusResp, CommonResponse), Error>
                + Send
                + Sync
                + 'static,
        > MockFunc for T
    {
    }

    impl<'c, IStore: Store, IClient: HttpClient> HireServiceMocker<'c, IStore, IClient> {
        pub fn mock_update_hire_offer_intern_status<F: MockFunc>(
            &self,
            f: F,
        ) -> MockerBuilder<
            Mocker,
            UpdateHireOfferInternStatusReq,
            UpdateHireOfferInternStatusResp,
            Arc<dyn MockFunc>,
        > {
            MockerBuilder::new(self.cli.instance_id, Arc::new(f))
        }

        pub(super) fn get_mock_update_hire_offer_intern_status(
            &self,
            req: &UpdateHireOfferInternStatusReq,
        ) -> Option<Arc<dyn MockFunc>> {
            do_mock::<
                Mocker,
                UpdateHireOfferInternStatusReq,
                UpdateHireOfferInternStatusResp,
                Arc<dyn MockFunc>,
            >(self.cli.instance_id, req)
        }
    }
}

#[cfg(feature = "test-util")]
#[cfg(test)]
mod test {
    use crate::{
        api::gen::hire::update_hire_offer_intern_status::{
            UpdateHireOfferInternStatusReq, UpdateHireOfferInternStatusResp,
            UpdateHireOfferInternStatusRespInner,
        },
        core::{model::CommonResponse, Lark},
    };

    #[tokio::test]
    async fn test_mock() {
        let lark = Lark::new("".to_owned(), "".into());
        let mocker = lark
            .hire()
            .mock()
            .mock_update_hire_offer_intern_status(|_| {
                Ok((
                    UpdateHireOfferInternStatusResp::default(),
                    CommonResponse::default(),
                ))
            })
            .build();
        let res = lark
            .hire()
            .update_hire_offer_intern_status(UpdateHireOfferInternStatusReq::default())
            .await;
        assert!(res.is_ok());
        mocker.clear();
        let res = lark
            .hire()
            .update_hire_offer_intern_status(UpdateHireOfferInternStatusReq::default())
            .await;
        assert!(res.is_err());
    }

    const REQ: &str = r#"{
    "operation": "confirm_onboarding",
    "onboarding_info": {
        "actual_onboarding_date": "2022-01-01"
    },
    "offboarding_info": {
        "actual_offboarding_date": "2022-03-02",
        "notes": "主动离职"
    }
}"#;

    #[test]
    fn test_req() {
        if REQ == "{}" {
            return;
        }
        if let Err(e) = serde_json::from_str::<super::UpdateHireOfferInternStatusReqBody>(REQ) {
            panic!("{}", e);
        }
    }

    const RESP: &str = r#"{
    "code": 0,
    "msg": "success",
    "data": {
        "offer_id": "6949805467799537964",
        "operation": "confirm_onboarding",
        "onboarding_info": {
            "actual_onboarding_date": "2022-01-01"
        },
        "offboarding_info": {
            "actual_offboarding_date": "2022-03-02",
            "notes": "主动离职"
        }
    }
}"#;
    #[test]
    fn test_resp() {
        let res = serde_json::from_str::<UpdateHireOfferInternStatusRespInner>(RESP);
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
