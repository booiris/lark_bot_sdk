//! doc url: <https://open.larkoffice.com/document/uAjLw4CM/ukTMukTMukTM/corehr-v2/cost_center/patch>
// Code generated by gen_api. DO NOT EDIT.

use serde::{Deserialize, Serialize};

use crate::api::BaseResp;
use crate::api::HasBaseResp;
use crate::core::model::{ApiRequest, CommonResponse, MethodOption, ReqParam, StreamReqParam};
use crate::{
    core::{http_client::HttpClient, store::Store},
    error::Error,
};

use crate::api::gen::core_hr::CoreHrService;

impl<'c, IStore: Store, IClient: HttpClient> CoreHrService<'c, IStore, IClient> {
    /// **api 版本: 2024-02-06T02:25:41+00:00**
    ///
    /// ## 启用 / 停用成本中心
    ///
    /// 启用或停用成本中心
    ///

    ///
    /// doc: <https://open.larkoffice.com/document/uAjLw4CM/ukTMukTMukTM/corehr-v2/cost_center/patch>
    ///
    /// new doc: <https://open.larkoffice.com/document/server-docs/corehr-v1/organization-management/cost_center/patch>
    ///
    /// api url: <https://open.feishu.cn/document_portal/v1/document/get_detail?fullPath=%2Fserver-docs%2Fcorehr-v1%2Forganization-management%2Fcost_center%2Fpatch>
    pub async fn active_core_hr_cost_center(
        &self,
        req: ActiveCoreHrCostCenterReq,
    ) -> Result<(ActiveCoreHrCostCenterResp, CommonResponse), Error> {
        self.active_core_hr_cost_center_with_opt(req, Default::default())
            .await
    }

    /// 参见 [active_core_hr_cost_center](#method.active_core_hr_cost_center) 函数
    pub async fn active_core_hr_cost_center_with_opt(
        &self,
        req: ActiveCoreHrCostCenterReq,
        method_option: MethodOption,
    ) -> Result<(ActiveCoreHrCostCenterResp, CommonResponse), Error> {
        #[cfg(feature = "test-util")]
        {
            if let Some(f) = self.mock().get_mock_active_core_hr_cost_center(&req) {
                tracing::info!("[lark] CoreHr#ActiveCoreHrCostCenter **mocking** api");
                return f(req);
            }
        }

        tracing::info!("[lark] CoreHr#ActiveCoreHrCostCenter call api");

        let req = ApiRequest {
            scope: "CoreHr",
            api: "ActiveCoreHrCostCenter",
            method: http::Method::PATCH,
            url: String::new()
                + self.cli.open_base_url.as_ref()
                + "/open-apis/corehr/v2/cost_centers/:cost_center_id",
            param_data: req.gen_param(),
            method_option,
            need_tenant_access_token: true,
            ..Default::default()
        };

        let (resp, common_resp): (ActiveCoreHrCostCenterRespInner, _) =
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
pub struct ActiveCoreHrCostCenterReq {
    /// 成本中心ID
    ///
    /// **示例值**: "6862995757234914824"
    #[api(kind = "path", name = "cost_center_id")]
    pub cost_center_id: String,
    /// 用户 ID 类型
    ///
    /// **示例值**: "people_corehr_id"
    ///
    /// **可选值**:
    ///
    /// `open_id`: 标识一个用户在某个应用中的身份。同一个用户在不同应用中的 Open ID 不同。[了解更多：如何获取 Open ID](https://open.feishu.cn/document/uAjLw4CM/ugTN1YjL4UTN24CO1UjN/trouble-shooting/how-to-obtain-openid)
    ///
    /// `union_id`: 标识一个用户在某个应用开发商下的身份。同一用户在同一开发商下的应用中的 Union ID 是相同的，在不同开发商下的应用中的 Union ID 是不同的。通过 Union ID，应用开发商可以把同个用户在多个应用中的身份关联起来。[了解更多：如何获取 Union ID？](https://open.feishu.cn/document/uAjLw4CM/ugTN1YjL4UTN24CO1UjN/trouble-shooting/how-to-obtain-union-id)
    ///
    /// `user_id`: 标识一个用户在某个租户内的身份。同一个用户在租户 A 和租户 B 内的 User ID 是不同的。在同一个租户内，一个用户的 User ID 在所有应用（包括商店应用）中都保持一致。User ID 主要用于在不同的应用间打通用户数据。[了解更多：如何获取 User ID？](https://open.feishu.cn/document/uAjLw4CM/ugTN1YjL4UTN24CO1UjN/trouble-shooting/how-to-obtain-user-id)
    ///
    /// `people_corehr_id`: 以飞书人事的 ID 来识别用户
    #[api(
        kind = "query",
        name = "user_id_type",
        v_type = "var",
        option = "false"
    )]
    pub user_id_type: String,
    /// 生效时间
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "2020-01-01"
    #[api(kind = "body", name = "effective_time")]
    pub effective_time: String,
    /// 启用停用状态
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "true"
    #[api(kind = "body", name = "active")]
    pub active: bool,
    /// 操作原因
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "强行操作"
    #[api(kind = "body", name = "operation_reason")]
    pub operation_reason: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, lark_bot_sdk_macros::ApiBaseResp)]
struct ActiveCoreHrCostCenterRespInner {
    #[serde(flatten)]
    data: Option<ActiveCoreHrCostCenterResp>,
    #[serde(flatten)]
    base: BaseResp,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct ActiveCoreHrCostCenterResp {
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
    /// 成本中心结果
    #[serde(
        rename = "cost_center",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub cost_center: CostCenterSubResp,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(default)]
pub struct CostCenterSubResp {
    /// 成本中心ID
    ///
    /// **示例值**: "6969828847121885087"
    #[serde(
        rename = "cost_center_id",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub cost_center_id: String,
    /// 成本中心名称
    ///
    /// **是否必填**: 是
    #[serde(
        rename = "name",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub name: Vec<I18nSubResp>,
    /// 编码
    ///
    /// **示例值**: "MDPD00000023"
    #[serde(
        rename = "code",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub code: String,
    /// 上级成本中心ID
    ///
    /// **示例值**: "6862995757234914824"
    #[serde(
        rename = "parent_cost_center_id",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub parent_cost_center_id: String,
    /// 成本中心负责人ID 列表
    #[serde(
        rename = "managers",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub managers: Vec<String>,
    /// 成本中心描述
    #[serde(
        rename = "description",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub description: Vec<I18nSubResp>,
    /// 生效时间
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "2020-01-01"
    #[serde(
        rename = "effective_time",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub effective_time: String,
    /// 过期时间
    ///
    /// **示例值**: "2020-01-01"
    #[serde(
        rename = "expiration_time",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub expiration_time: String,
    /// 当前实体是否启用
    ///
    /// **示例值**: "true"
    #[serde(
        rename = "active",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub active: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(default)]
pub struct I18nSubResp {
    /// 语言
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "zh-CN"
    #[serde(
        rename = "lang",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub lang: String,
    /// 内容
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "张三"
    #[serde(
        rename = "value",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub value: String,
}

#[cfg(feature = "test-util")]
mod test_utils {
    use std::sync::Arc;

    use self::gen::core_hr::CoreHrServiceMocker;
    use crate::core::mocker::*;

    use super::*;
    use crate::api::*;
    use crate::core::model::*;

    pub struct Mocker;

    pub trait MockFunc:
        Fn(ActiveCoreHrCostCenterReq) -> Result<(ActiveCoreHrCostCenterResp, CommonResponse), Error>
        + Send
        + Sync
        + 'static
    {
    }
    impl<
            T: Fn(
                    ActiveCoreHrCostCenterReq,
                ) -> Result<(ActiveCoreHrCostCenterResp, CommonResponse), Error>
                + Send
                + Sync
                + 'static,
        > MockFunc for T
    {
    }

    impl<'c, IStore: Store, IClient: HttpClient> CoreHrServiceMocker<'c, IStore, IClient> {
        pub fn mock_active_core_hr_cost_center<F: MockFunc>(
            &self,
            f: F,
        ) -> MockerBuilder<
            Mocker,
            ActiveCoreHrCostCenterReq,
            ActiveCoreHrCostCenterResp,
            Arc<dyn MockFunc>,
        > {
            MockerBuilder::new(self.cli.instance_id, Arc::new(f))
        }

        pub(super) fn get_mock_active_core_hr_cost_center(
            &self,
            req: &ActiveCoreHrCostCenterReq,
        ) -> Option<Arc<dyn MockFunc>> {
            do_mock::<
                Mocker,
                ActiveCoreHrCostCenterReq,
                ActiveCoreHrCostCenterResp,
                Arc<dyn MockFunc>,
            >(self.cli.instance_id, req)
        }
    }
}

#[cfg(feature = "test-util")]
#[cfg(test)]
mod test {
    use crate::{
        api::gen::core_hr::active_core_hr_cost_center::{
            ActiveCoreHrCostCenterReq, ActiveCoreHrCostCenterResp, ActiveCoreHrCostCenterRespInner,
        },
        core::{model::CommonResponse, Lark},
    };

    #[tokio::test]
    async fn test_mock() {
        let lark = Lark::new("".to_owned(), "".into());
        let mocker = lark
            .core_hr()
            .mock()
            .mock_active_core_hr_cost_center(|_| {
                Ok((
                    ActiveCoreHrCostCenterResp::default(),
                    CommonResponse::default(),
                ))
            })
            .build();
        let res = lark
            .core_hr()
            .active_core_hr_cost_center(ActiveCoreHrCostCenterReq::default())
            .await;
        assert!(res.is_ok());
        mocker.clear();
        let res = lark
            .core_hr()
            .active_core_hr_cost_center(ActiveCoreHrCostCenterReq::default())
            .await;
        assert!(res.is_err());
    }

    const REQ: &str = r#"{
    "effective_time": "2020-01-01",
    "active": true,
    "operation_reason": "强行操作"
}"#;

    #[test]
    fn test_req() {
        if REQ == "{}" {
            return;
        }
        if let Err(e) = serde_json::from_str::<super::ActiveCoreHrCostCenterReqBody>(REQ) {
            panic!("{}", e);
        }
    }

    const RESP: &str = r#"{
    "code": 0,
    "msg": "success",
    "data": {
        "cost_center": {
            "cost_center_id": "6969828847121885087",
            "name": [
                {
                    "lang": "zh-CN",
                    "value": "张三"
                }
            ],
            "code": "MDPD00000023",
            "parent_cost_center_id": "6862995757234914824",
            "managers": [
                "6862995757234914824"
            ],
            "description": [
                {
                    "lang": "zh-CN",
                    "value": "张三"
                }
            ],
            "effective_time": "2020-01-01",
            "expiration_time": "2020-01-01",
            "active": true
        }
    }
}"#;
    #[test]
    fn test_resp() {
        let res = serde_json::from_str::<ActiveCoreHrCostCenterRespInner>(RESP);
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
