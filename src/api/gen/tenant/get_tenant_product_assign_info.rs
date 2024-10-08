//! doc url: <https://open.larkoffice.com/document/uAjLw4CM/ukTMukTMukTM/tenant-v2/tenant-product_assign_info/query>
// Code generated by gen_api. DO NOT EDIT.

use serde::{Deserialize, Serialize};

use crate::api::BaseResp;
use crate::api::HasBaseResp;
use crate::core::model::{ApiRequest, CommonResponse, MethodOption, ReqParam, StreamReqParam};
use crate::{
    core::{http_client::HttpClient, store::Store},
    error::Error,
};

use crate::api::gen::tenant::TenantService;

impl<'c, IStore: Store, IClient: HttpClient> TenantService<'c, IStore, IClient> {
    /// **api 版本: 2023-07-07T08:16:03+00:00**
    ///
    /// ## 获取企业席位信息
    ///
    /// 获取租户下待分配的席位列表，包含席位名称、席位ID、数量及对应有效期。
    ///

    ///
    /// doc: <https://open.larkoffice.com/document/uAjLw4CM/ukTMukTMukTM/tenant-v2/tenant-product_assign_info/query>
    ///
    /// new doc: <https://open.larkoffice.com/document/server-docs/tenant-v2/tenant-product_assign_info/query>
    ///
    /// api url: <https://open.feishu.cn/document_portal/v1/document/get_detail?fullPath=%2Fserver-docs%2Ftenant-v2%2Ftenant-product_assign_info%2Fquery>
    pub async fn get_tenant_product_assign_info(
        &self,
        req: GetTenantProductAssignInfoReq,
    ) -> Result<(GetTenantProductAssignInfoResp, CommonResponse), Error> {
        self.get_tenant_product_assign_info_with_opt(req, Default::default())
            .await
    }

    /// 参见 [get_tenant_product_assign_info](#method.get_tenant_product_assign_info) 函数
    pub async fn get_tenant_product_assign_info_with_opt(
        &self,
        req: GetTenantProductAssignInfoReq,
        method_option: MethodOption,
    ) -> Result<(GetTenantProductAssignInfoResp, CommonResponse), Error> {
        #[cfg(feature = "test-util")]
        {
            if let Some(f) = self.mock().get_mock_get_tenant_product_assign_info(&req) {
                tracing::info!("[lark] Tenant#GetTenantProductAssignInfo **mocking** api");
                return f(req);
            }
        }

        tracing::info!("[lark] Tenant#GetTenantProductAssignInfo call api");

        let req = ApiRequest {
            scope: "Tenant",
            api: "GetTenantProductAssignInfo",
            method: http::Method::GET,
            url: String::new()
                + self.cli.open_base_url.as_ref()
                + "/open-apis/tenant/v2/tenant/assign_info_list/query",
            param_data: req.gen_param(),
            method_option,
            need_tenant_access_token: true,
            ..Default::default()
        };

        let (resp, common_resp): (GetTenantProductAssignInfoRespInner, _) =
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
pub struct GetTenantProductAssignInfoReq {}

#[derive(Debug, Serialize, Deserialize, Clone, lark_bot_sdk_macros::ApiBaseResp)]
struct GetTenantProductAssignInfoRespInner {
    #[serde(flatten)]
    data: Option<GetTenantProductAssignInfoResp>,
    #[serde(flatten)]
    base: BaseResp,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct GetTenantProductAssignInfoResp {
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
    /// 租户待分配席位列表
    #[serde(
        rename = "assign_info_list",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub assign_info_list: Vec<TenantAssignInfoSubResp>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(default)]
pub struct TenantAssignInfoSubResp {
    /// 席位id
    ///
    /// **示例值**: "7079609167680782300"
    #[serde(
        rename = "subscription_id",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub subscription_id: String,
    /// license_plan_key
    ///
    /// **示例值**: "suite_enterprise_e5"
    #[serde(
        rename = "license_plan_key",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub license_plan_key: String,
    /// 商业化产品名称
    ///
    /// **示例值**: "旗舰版 E5"
    #[serde(
        rename = "product_name",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub product_name: String,
    /// 国际化名称
    #[serde(
        rename = "i18n_name",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub i18n_name: ProductI18nNameSubResp,
    /// 席位总数
    ///
    /// **示例值**: "500"
    #[serde(
        rename = "total_seats",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub total_seats: String,
    /// 已分配席位数
    ///
    /// **示例值**: "20"
    #[serde(
        rename = "assigned_seats",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub assigned_seats: String,
    /// 席位起始时间
    ///
    /// **示例值**: "1674981000"
    #[serde(
        rename = "start_time",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub start_time: String,
    /// 席位结束时间
    ///
    /// **示例值**: "1674991000"
    #[serde(
        rename = "end_time",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub end_time: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(default)]
pub struct ProductI18nNameSubResp {
    /// 商业化产品的中文名
    ///
    /// **示例值**: "zh_cn_name"
    #[serde(
        rename = "zh_cn",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub zh_cn: String,
    /// 商业化产品的日文名
    ///
    /// **示例值**: "ja_jp_name"
    #[serde(
        rename = "ja_jp",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub ja_jp: String,
    /// 商业化产品的英文名
    ///
    /// **示例值**: "en_name"
    #[serde(
        rename = "en_us",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub en_us: String,
}

#[cfg(feature = "test-util")]
mod test_utils {
    use std::sync::Arc;

    use self::gen::tenant::TenantServiceMocker;
    use crate::core::mocker::*;

    use super::*;
    use crate::api::*;
    use crate::core::model::*;

    pub struct Mocker;

    pub trait MockFunc:
        Fn(
            GetTenantProductAssignInfoReq,
        ) -> Result<(GetTenantProductAssignInfoResp, CommonResponse), Error>
        + Send
        + Sync
        + 'static
    {
    }
    impl<
            T: Fn(
                    GetTenantProductAssignInfoReq,
                )
                    -> Result<(GetTenantProductAssignInfoResp, CommonResponse), Error>
                + Send
                + Sync
                + 'static,
        > MockFunc for T
    {
    }

    impl<'c, IStore: Store, IClient: HttpClient> TenantServiceMocker<'c, IStore, IClient> {
        pub fn mock_get_tenant_product_assign_info<F: MockFunc>(
            &self,
            f: F,
        ) -> MockerBuilder<
            Mocker,
            GetTenantProductAssignInfoReq,
            GetTenantProductAssignInfoResp,
            Arc<dyn MockFunc>,
        > {
            MockerBuilder::new(self.cli.instance_id, Arc::new(f))
        }

        pub(super) fn get_mock_get_tenant_product_assign_info(
            &self,
            req: &GetTenantProductAssignInfoReq,
        ) -> Option<Arc<dyn MockFunc>> {
            do_mock::<
                Mocker,
                GetTenantProductAssignInfoReq,
                GetTenantProductAssignInfoResp,
                Arc<dyn MockFunc>,
            >(self.cli.instance_id, req)
        }
    }
}

#[cfg(feature = "test-util")]
#[cfg(test)]
mod test {
    use crate::{
        api::gen::tenant::get_tenant_product_assign_info::{
            GetTenantProductAssignInfoReq, GetTenantProductAssignInfoResp,
            GetTenantProductAssignInfoRespInner,
        },
        core::{model::CommonResponse, Lark},
    };

    #[tokio::test]
    async fn test_mock() {
        let lark = Lark::new("".to_owned(), "".into());
        let mocker = lark
            .tenant()
            .mock()
            .mock_get_tenant_product_assign_info(|_| {
                Ok((
                    GetTenantProductAssignInfoResp::default(),
                    CommonResponse::default(),
                ))
            })
            .build();
        let res = lark
            .tenant()
            .get_tenant_product_assign_info(GetTenantProductAssignInfoReq::default())
            .await;
        assert!(res.is_ok());
        mocker.clear();
        let res = lark
            .tenant()
            .get_tenant_product_assign_info(GetTenantProductAssignInfoReq::default())
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
        "assign_info_list": [
            {
                "subscription_id": "7079609167680782300",
                "license_plan_key": "suite_enterprise_e5",
                "product_name": "旗舰版 E5",
                "i18n_name": {
                    "zh_cn": "zh_cn_name",
                    "ja_jp": "ja_jp_name",
                    "en_us": "en_name"
                },
                "total_seats": "500",
                "assigned_seats": "20",
                "start_time": "1674981000",
                "end_time": "1674991000"
            }
        ]
    }
}"#;
    #[test]
    fn test_resp() {
        let res = serde_json::from_str::<GetTenantProductAssignInfoRespInner>(RESP);
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
