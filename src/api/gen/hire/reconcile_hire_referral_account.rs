//! doc url: <https://open.feishu.cn/document/ukTMukTMukTM/uMzM1YjLzMTN24yMzUjN/hire-v1/referral_account/reconciliation>
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
    /// **api 版本: 2024-08-02T03:52:59+00:00**
    ///
    /// ## 内推账户提现数据对账
    ///
    /// 对一段时间内的内推账户积分提现数据进行对账，调用方需传入调用方系统的内推账户积分变动信息。
    ///

    ///
    /// doc: <https://open.feishu.cn/document/ukTMukTMukTM/uMzM1YjLzMTN24yMzUjN/hire-v1/referral_account/reconciliation>
    ///
    /// new doc: <https://open.feishu.cn/document/ukTMukTMukTM/uMzM1YjLzMTN24yMzUjN/hire-v1/referral_account/reconciliation>
    ///
    /// api url: <https://open.feishu.cn/document_portal/v1/document/get_detail?fullPath=%2FukTMukTMukTM%2FuMzM1YjLzMTN24yMzUjN%2Fhire-v1%2Freferral_account%2Freconciliation>
    pub async fn reconcile_hire_referral_account(
        &self,
        req: ReconcileHireReferralAccountReq,
    ) -> Result<(ReconcileHireReferralAccountResp, CommonResponse), Error> {
        self.reconcile_hire_referral_account_with_opt(req, Default::default())
            .await
    }

    /// 参见 [reconcile_hire_referral_account](#method.reconcile_hire_referral_account) 函数
    pub async fn reconcile_hire_referral_account_with_opt(
        &self,
        req: ReconcileHireReferralAccountReq,
        method_option: MethodOption,
    ) -> Result<(ReconcileHireReferralAccountResp, CommonResponse), Error> {
        #[cfg(feature = "test-util")]
        {
            if let Some(f) = self.mock().get_mock_reconcile_hire_referral_account(&req) {
                tracing::info!("[lark] Hire#ReconcileHireReferralAccount **mocking** api");
                return f(req);
            }
        }

        tracing::info!("[lark] Hire#ReconcileHireReferralAccount call api");

        let req = ApiRequest {
            scope: "Hire",
            api: "ReconcileHireReferralAccount",
            method: http::Method::POST,
            url: String::new()
                + self.cli.open_base_url.as_ref()
                + "/open-apis/hire/v1/referral_account/reconciliation",
            param_data: req.gen_param(),
            method_option,
            need_tenant_access_token: true,
            ..Default::default()
        };

        let (resp, common_resp): (ReconcileHireReferralAccountRespInner, _) =
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
pub struct ReconcileHireReferralAccountReq {
    /// 对账时段的起始交易时间，毫秒时间戳
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "1685416831621"
    #[api(kind = "body", name = "start_trans_time")]
    pub start_trans_time: String,
    /// 对账时段的截止交易时间，毫秒时间戳
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "1685416831622"
    #[api(kind = "body", name = "end_trans_time")]
    pub end_trans_time: String,
    /// 账户积分变动信息
    #[api(kind = "body", name = "trade_details")]
    pub trade_details: Vec<Option<TradeDetailSubReq>>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(default)]
pub struct TradeDetailSubReq {
    /// 内推账户ID，通过[注册内推账户](https://open.feishu.cn/document/ukTMukTMukTM/uMzM1YjLzMTN24yMzUjN/hire-v1/referral_account/create)生成
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "6930815272790114324"
    #[serde(
        rename = "account_id",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub account_id: String,
    /// 时段内该账户发生在调用方系统的积分之和
    #[serde(
        rename = "total_recharge_reward_info",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub total_recharge_reward_info: Option<BonusAmountSubReq>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(default)]
pub struct BonusAmountSubReq {
    /// 变动的积分数量
    ///
    /// **示例值**: "100"
    #[serde(
        rename = "point_bonus",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub point_bonus: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize, Clone, lark_bot_sdk_macros::ApiBaseResp)]
struct ReconcileHireReferralAccountRespInner {
    #[serde(flatten)]
    data: Option<ReconcileHireReferralAccountResp>,
    #[serde(flatten)]
    base: BaseResp,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct ReconcileHireReferralAccountResp {
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
    /// 核对失败的信息
    #[serde(
        rename = "check_failed_list",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub check_failed_list: Vec<CheckFailedAccountInfoSubResp>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(default)]
pub struct CheckFailedAccountInfoSubResp {
    /// 内推账户ID
    ///
    /// **示例值**: "6930815272790114324"
    #[serde(
        rename = "account_id",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub account_id: String,
    /// 飞书招聘系统内的账户积分提取数量，若该时段内未发生任何提取记录，则该字段不返回
    #[serde(
        rename = "total_withdraw_reward_info",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub total_withdraw_reward_info: BonusAmountSubResp,
    /// 调用方系统的积分变动数量
    #[serde(
        rename = "total_recharge_reward_info",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub total_recharge_reward_info: BonusAmountSubResp,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(default)]
pub struct BonusAmountSubResp {
    /// 积分数量
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
            ReconcileHireReferralAccountReq,
        ) -> Result<(ReconcileHireReferralAccountResp, CommonResponse), Error>
        + Send
        + Sync
        + 'static
    {
    }
    impl<
            T: Fn(
                    ReconcileHireReferralAccountReq,
                )
                    -> Result<(ReconcileHireReferralAccountResp, CommonResponse), Error>
                + Send
                + Sync
                + 'static,
        > MockFunc for T
    {
    }

    impl<'c, IStore: Store, IClient: HttpClient> HireServiceMocker<'c, IStore, IClient> {
        pub fn mock_reconcile_hire_referral_account<F: MockFunc>(
            &self,
            f: F,
        ) -> MockerBuilder<
            Mocker,
            ReconcileHireReferralAccountReq,
            ReconcileHireReferralAccountResp,
            Arc<dyn MockFunc>,
        > {
            MockerBuilder::new(self.cli.instance_id, Arc::new(f))
        }

        pub(super) fn get_mock_reconcile_hire_referral_account(
            &self,
            req: &ReconcileHireReferralAccountReq,
        ) -> Option<Arc<dyn MockFunc>> {
            do_mock::<
                Mocker,
                ReconcileHireReferralAccountReq,
                ReconcileHireReferralAccountResp,
                Arc<dyn MockFunc>,
            >(self.cli.instance_id, req)
        }
    }
}

#[cfg(feature = "test-util")]
#[cfg(test)]
mod test {
    use crate::{
        api::gen::hire::reconcile_hire_referral_account::{
            ReconcileHireReferralAccountReq, ReconcileHireReferralAccountResp,
            ReconcileHireReferralAccountRespInner,
        },
        core::{model::CommonResponse, Lark},
    };

    #[tokio::test]
    async fn test_mock() {
        let lark = Lark::new("".to_owned(), "".into());
        let mocker = lark
            .hire()
            .mock()
            .mock_reconcile_hire_referral_account(|_| {
                Ok((
                    ReconcileHireReferralAccountResp::default(),
                    CommonResponse::default(),
                ))
            })
            .build();
        let res = lark
            .hire()
            .reconcile_hire_referral_account(ReconcileHireReferralAccountReq::default())
            .await;
        assert!(res.is_ok());
        mocker.clear();
        let res = lark
            .hire()
            .reconcile_hire_referral_account(ReconcileHireReferralAccountReq::default())
            .await;
        assert!(res.is_err());
    }

    const REQ: &str = r#"{
    "start_trans_time": "1685416831621",
    "end_trans_time": "1685416831622",
    "trade_details": [
        {
            "account_id": "6930815272790114324",
            "total_recharge_reward_info": {
                "point_bonus": 100
            }
        }
    ]
}"#;

    #[test]
    fn test_req() {
        if REQ == "{}" {
            return;
        }
        if let Err(e) = serde_json::from_str::<super::ReconcileHireReferralAccountReqBody>(REQ) {
            panic!("{}", e);
        }
    }

    const RESP: &str = r#"{
    "code": 0,
    "msg": "SUCCESS",
    "data": {
        "check_failed_list": [
            {
                "account_id": "6930815272790114324",
                "total_withdraw_reward_info": {
                    "point_bonus": 100
                },
                "total_recharge_reward_info": {
                    "point_bonus": 100
                }
            }
        ]
    }
}"#;
    #[test]
    fn test_resp() {
        let res = serde_json::from_str::<ReconcileHireReferralAccountRespInner>(RESP);
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
