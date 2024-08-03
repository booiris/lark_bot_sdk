//! doc url: <https://open.feishu.cn/document/ukTMukTMukTM/uMzM1YjLzMTN24yMzUjN/hire-v1/referral/get_by_application>
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
    /// **api 版本: 2024-07-10T14:30:24+00:00**
    ///
    /// ## 获取内推信息
    ///
    /// 根据投递 ID 获取内推信息。
    ///

    ///
    /// doc: <https://open.feishu.cn/document/ukTMukTMukTM/uMzM1YjLzMTN24yMzUjN/hire-v1/referral/get_by_application>
    ///
    /// new doc: <https://open.feishu.cn/document/server-docs/hire-v1/get-candidates/referral/get_by_application>
    ///
    /// api url: <https://open.feishu.cn/document_portal/v1/document/get_detail?fullPath=%2Fserver-docs%2Fhire-v1%2Fget-candidates%2Freferral%2Fget_by_application>
    pub async fn get_hire_referral_by_application(
        &self,
        req: GetHireReferralByApplicationReq,
    ) -> Result<(GetHireReferralByApplicationResp, CommonResponse), Error> {
        self.get_hire_referral_by_application_with_opt(req, Default::default())
            .await
    }

    /// 参见 [get_hire_referral_by_application](#method.get_hire_referral_by_application) 函数
    pub async fn get_hire_referral_by_application_with_opt(
        &self,
        req: GetHireReferralByApplicationReq,
        method_option: MethodOption,
    ) -> Result<(GetHireReferralByApplicationResp, CommonResponse), Error> {
        #[cfg(feature = "test-util")]
        {
            if let Some(f) = self.mock().get_mock_get_hire_referral_by_application(&req) {
                tracing::info!("[lark] Hire#GetHireReferralByApplication **mocking** api");
                return f(req);
            }
        }

        tracing::info!("[lark] Hire#GetHireReferralByApplication call api");

        let req = ApiRequest {
            scope: "Hire",
            api: "GetHireReferralByApplication",
            method: http::Method::GET,
            url: String::new()
                + self.cli.open_base_url.as_ref()
                + "/open-apis/hire/v1/referrals/get_by_application",
            param_data: req.gen_param(),
            method_option,
            need_tenant_access_token: true,
            ..Default::default()
        };

        let (resp, common_resp): (GetHireReferralByApplicationRespInner, _) =
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
pub struct GetHireReferralByApplicationReq {
    /// 投递的 ID
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "6134134355464633"
    #[api(
        kind = "query",
        name = "application_id",
        v_type = "var",
        option = "false"
    )]
    pub application_id: String,
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
    ///
    /// `people_admin_id`: 以people_admin_id来识别用户
    #[api(
        kind = "query",
        name = "user_id_type",
        v_type = "var",
        option = "false"
    )]
    pub user_id_type: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, lark_bot_sdk_macros::ApiBaseResp)]
struct GetHireReferralByApplicationRespInner {
    #[serde(flatten)]
    data: Option<GetHireReferralByApplicationResp>,
    #[serde(flatten)]
    base: BaseResp,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct GetHireReferralByApplicationResp {
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
    /// 内推信息
    #[serde(
        rename = "referral",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub referral: ReferralSubResp,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(default)]
pub struct ReferralSubResp {
    /// 内推的 ID
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "6643786345878"
    #[serde(
        rename = "id",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub id: String,
    /// 投递 ID
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "643452344576878"
    #[serde(
        rename = "application_id",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub application_id: String,
    /// 创建时间（ms）
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "1618899376474"
    #[serde(
        rename = "create_time",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub create_time: i64,
    /// 内推人的 ID
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "ou_xxxx"
    #[serde(
        rename = "referral_user_id",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub referral_user_id: String,
    /// 内推人信息
    #[serde(
        rename = "referral_user",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub referral_user: IdNameObjectSubResp,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(default)]
pub struct IdNameObjectSubResp {
    /// ID
    ///
    /// **示例值**: "1213213123123"
    #[serde(
        rename = "id",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub id: String,
    /// 名称
    #[serde(
        rename = "name",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub name: I18nSubResp,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(default)]
pub struct I18nSubResp {
    /// 中文
    ///
    /// **示例值**: "测试"
    #[serde(
        rename = "zh_cn",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub zh_cn: String,
    /// 英文
    ///
    /// **示例值**: "test"
    #[serde(
        rename = "en_us",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub en_us: String,
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
            GetHireReferralByApplicationReq,
        ) -> Result<(GetHireReferralByApplicationResp, CommonResponse), Error>
        + Send
        + Sync
        + 'static
    {
    }
    impl<
            T: Fn(
                    GetHireReferralByApplicationReq,
                )
                    -> Result<(GetHireReferralByApplicationResp, CommonResponse), Error>
                + Send
                + Sync
                + 'static,
        > MockFunc for T
    {
    }

    impl<'c, IStore: Store, IClient: HttpClient> HireServiceMocker<'c, IStore, IClient> {
        pub fn mock_get_hire_referral_by_application<F: MockFunc>(
            &self,
            f: F,
        ) -> MockerBuilder<
            Mocker,
            GetHireReferralByApplicationReq,
            GetHireReferralByApplicationResp,
            Arc<dyn MockFunc>,
        > {
            MockerBuilder::new(self.cli.instance_id, Arc::new(f))
        }

        pub(super) fn get_mock_get_hire_referral_by_application(
            &self,
            req: &GetHireReferralByApplicationReq,
        ) -> Option<Arc<dyn MockFunc>> {
            do_mock::<
                Mocker,
                GetHireReferralByApplicationReq,
                GetHireReferralByApplicationResp,
                Arc<dyn MockFunc>,
            >(self.cli.instance_id, req)
        }
    }
}

#[cfg(feature = "test-util")]
#[cfg(test)]
mod test {
    use crate::{
        api::gen::hire::get_hire_referral_by_application::{
            GetHireReferralByApplicationReq, GetHireReferralByApplicationResp,
            GetHireReferralByApplicationRespInner,
        },
        core::{model::CommonResponse, Lark},
    };

    #[tokio::test]
    async fn test_mock() {
        let lark = Lark::new("".to_owned(), "".into());
        let mocker = lark
            .hire()
            .mock()
            .mock_get_hire_referral_by_application(|_| {
                Ok((
                    GetHireReferralByApplicationResp::default(),
                    CommonResponse::default(),
                ))
            })
            .build();
        let res = lark
            .hire()
            .get_hire_referral_by_application(GetHireReferralByApplicationReq::default())
            .await;
        assert!(res.is_ok());
        mocker.clear();
        let res = lark
            .hire()
            .get_hire_referral_by_application(GetHireReferralByApplicationReq::default())
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
        "referral": {
            "id": "6643786345878",
            "application_id": "643452344576878",
            "create_time": 1618899376474,
            "referral_user_id": "ou_xxxx",
            "referral_user": {
                "id": "1213213123123",
                "name": {
                    "zh_cn": "测试",
                    "en_us": "test"
                }
            }
        }
    }
}"#;
    #[test]
    fn test_resp() {
        let res = serde_json::from_str::<GetHireReferralByApplicationRespInner>(RESP);
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
