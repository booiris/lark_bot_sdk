//! doc url: <https://open.feishu.cn/document/ukTMukTMukTM/uMzM1YjLzMTN24yMzUjN/hire-v1/referral_account/deactivate>
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
    /// **api 版本: 2024-07-26T08:51:11+00:00**
    ///
    /// ## 停用内推账户
    ///
    /// 停用后，对应的内推账号信息将无法通过接口[「内推账户余额变更事件」](https://open.feishu.cn/document/ukTMukTMukTM/uMzM1YjLzMTN24yMzUjN/hire-v1/referral_account/events/assets_update)、[「提取内推账号余额」](https://open.feishu.cn/document/ukTMukTMukTM/uMzM1YjLzMTN24yMzUjN/hire-v1/referral_account/withdraw)获取、修改
    ///

    ///
    /// doc: <https://open.feishu.cn/document/ukTMukTMukTM/uMzM1YjLzMTN24yMzUjN/hire-v1/referral_account/deactivate>
    ///
    /// new doc: <https://open.feishu.cn/document/ukTMukTMukTM/uMzM1YjLzMTN24yMzUjN/hire-v1/referral_account/deactivate>
    ///
    /// api url: <https://open.feishu.cn/document_portal/v1/document/get_detail?fullPath=%2FukTMukTMukTM%2FuMzM1YjLzMTN24yMzUjN%2Fhire-v1%2Freferral_account%2Fdeactivate>
    pub async fn deactivate_hire_referral_account(
        &self,
        req: DeactivateHireReferralAccountReq,
    ) -> Result<(DeactivateHireReferralAccountResp, CommonResponse), Error> {
        self.deactivate_hire_referral_account_with_opt(req, Default::default())
            .await
    }

    /// 参见 [deactivate_hire_referral_account](#method.deactivate_hire_referral_account) 函数
    pub async fn deactivate_hire_referral_account_with_opt(
        &self,
        req: DeactivateHireReferralAccountReq,
        method_option: MethodOption,
    ) -> Result<(DeactivateHireReferralAccountResp, CommonResponse), Error> {
        #[cfg(feature = "test-util")]
        {
            if let Some(f) = self.mock().get_mock_deactivate_hire_referral_account(&req) {
                tracing::info!("[lark] Hire#DeactivateHireReferralAccount **mocking** api");
                return f(req);
            }
        }

        tracing::info!("[lark] Hire#DeactivateHireReferralAccount call api");

        let req = ApiRequest {
            scope: "Hire",
            api: "DeactivateHireReferralAccount",
            method: http::Method::POST,
            url: String::new()
                + self.cli.open_base_url.as_ref()
                + "/open-apis/hire/v1/referral_account/:referral_account_id/deactivate",
            param_data: req.gen_param(),
            method_option,
            need_tenant_access_token: true,
            ..Default::default()
        };

        let (resp, common_resp): (DeactivateHireReferralAccountRespInner, _) =
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
pub struct DeactivateHireReferralAccountReq {
    /// 账户ID，注册账户后获取：[注册内推账户](https://open.feishu.cn/document/ukTMukTMukTM/uMzM1YjLzMTN24yMzUjN/hire-v1/referral_account/create)
    ///
    /// **示例值**: "6942778198054125570"
    #[api(kind = "path", name = "referral_account_id")]
    pub referral_account_id: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, lark_bot_sdk_macros::ApiBaseResp)]
struct DeactivateHireReferralAccountRespInner {
    #[serde(flatten)]
    data: Option<DeactivateHireReferralAccountResp>,
    #[serde(flatten)]
    base: BaseResp,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct DeactivateHireReferralAccountResp {
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
    /// 账号信息
    #[serde(
        rename = "account",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub account: AccountSubResp,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(default)]
pub struct AccountSubResp {
    /// 账户ID
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "6942778198054125570"
    #[serde(
        rename = "account_id",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub account_id: String,
    /// 账户资产
    #[serde(
        rename = "assets",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub assets: AssetsSubResp,
    /// 账号状态
    ///
    /// **示例值**: "1"
    ///
    /// **可选值**:
    ///
    /// `active`: 可用
    ///
    /// `disable`: 停用
    #[serde(
        rename = "status",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub status: i64,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(default)]
pub struct AssetsSubResp {
    /// 已确认的奖励
    #[serde(
        rename = "confirmed_bonus",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub confirmed_bonus: BonusAmountSubResp,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(default)]
pub struct BonusAmountSubResp {
    /// 积分奖励
    ///
    /// **示例值**: "100"
    #[serde(
        rename = "point_bonus",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub point_bonus: i64,
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
            DeactivateHireReferralAccountReq,
        ) -> Result<(DeactivateHireReferralAccountResp, CommonResponse), Error>
        + Send
        + Sync
        + 'static
    {
    }
    impl<
            T: Fn(
                    DeactivateHireReferralAccountReq,
                )
                    -> Result<(DeactivateHireReferralAccountResp, CommonResponse), Error>
                + Send
                + Sync
                + 'static,
        > MockFunc for T
    {
    }

    impl<'c, IStore: Store, IClient: HttpClient> HireServiceMocker<'c, IStore, IClient> {
        pub fn mock_deactivate_hire_referral_account<F: MockFunc>(
            &self,
            f: F,
        ) -> MockerBuilder<
            Mocker,
            DeactivateHireReferralAccountReq,
            DeactivateHireReferralAccountResp,
            Arc<dyn MockFunc>,
        > {
            MockerBuilder::new(self.cli.instance_id, Arc::new(f))
        }

        pub(super) fn get_mock_deactivate_hire_referral_account(
            &self,
            req: &DeactivateHireReferralAccountReq,
        ) -> Option<Arc<dyn MockFunc>> {
            do_mock::<
                Mocker,
                DeactivateHireReferralAccountReq,
                DeactivateHireReferralAccountResp,
                Arc<dyn MockFunc>,
            >(self.cli.instance_id, req)
        }
    }
}

#[cfg(feature = "test-util")]
#[cfg(test)]
mod test {
    use crate::{
        api::gen::hire::deactivate_hire_referral_account::{
            DeactivateHireReferralAccountReq, DeactivateHireReferralAccountResp,
            DeactivateHireReferralAccountRespInner,
        },
        core::{model::CommonResponse, Lark},
    };

    #[tokio::test]
    async fn test_mock() {
        let lark = Lark::new("".to_owned(), "".into());
        let mocker = lark
            .hire()
            .mock()
            .mock_deactivate_hire_referral_account(|_| {
                Ok((
                    DeactivateHireReferralAccountResp::default(),
                    CommonResponse::default(),
                ))
            })
            .build();
        let res = lark
            .hire()
            .deactivate_hire_referral_account(DeactivateHireReferralAccountReq::default())
            .await;
        assert!(res.is_ok());
        mocker.clear();
        let res = lark
            .hire()
            .deactivate_hire_referral_account(DeactivateHireReferralAccountReq::default())
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
        "account": {
            "account_id": "6942778198054125570",
            "assets": {
                "confirmed_bonus": {
                    "point_bonus": 100
                }
            },
            "status": 1
        }
    }
}"#;
    #[test]
    fn test_resp() {
        let res = serde_json::from_str::<DeactivateHireReferralAccountRespInner>(RESP);
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
