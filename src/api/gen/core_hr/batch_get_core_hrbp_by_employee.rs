//! doc url: <https://open.larkoffice.com/document/uAjLw4CM/ukTMukTMukTM/corehr-v2/employees-bp/batch_get>
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
    /// **api 版本: 2024-07-12T08:36:23+00:00**
    ///
    /// ## 查询员工 HRBP / 属地 BP
    ///
    /// 查询员工的 HRBP 和属地 BP，包括来自上级部门的 HRBP 和属地 BP。
    ///
    /// 该接口会按照应用拥有的「员工资源」的权限范围返回数据，请确定在「开发者后台 - 权限管理 - 数据权限 - 飞书人事（企业版）数据权限范围」中已申请「员工资源」权限范围
    ///
    /// doc: <https://open.larkoffice.com/document/uAjLw4CM/ukTMukTMukTM/corehr-v2/employees-bp/batch_get>
    ///
    /// new doc: <https://open.larkoffice.com/document/uAjLw4CM/ukTMukTMukTM/corehr-v2/employees-bp/batch_get>
    ///
    /// api url: <https://open.feishu.cn/document_portal/v1/document/get_detail?fullPath=%2FuAjLw4CM%2FukTMukTMukTM%2Fcorehr-v2%2Femployees-bp%2Fbatch_get>
    pub async fn batch_get_core_hrbp_by_employee(
        &self,
        req: BatchGetCoreHrbpByEmployeeReq,
    ) -> Result<(BatchGetCoreHrbpByEmployeeResp, CommonResponse), Error> {
        self.batch_get_core_hrbp_by_employee_with_opt(req, Default::default())
            .await
    }

    /// 参见 [batch_get_core_hrbp_by_employee](#method.batch_get_core_hrbp_by_employee) 函数
    pub async fn batch_get_core_hrbp_by_employee_with_opt(
        &self,
        req: BatchGetCoreHrbpByEmployeeReq,
        method_option: MethodOption,
    ) -> Result<(BatchGetCoreHrbpByEmployeeResp, CommonResponse), Error> {
        #[cfg(feature = "test-util")]
        {
            if let Some(f) = self.mock().get_mock_batch_get_core_hrbp_by_employee(&req) {
                tracing::info!("[lark] CoreHr#BatchGetCoreHrbpByEmployee **mocking** api");
                return f(req);
            }
        }

        tracing::info!("[lark] CoreHr#BatchGetCoreHrbpByEmployee call api");

        let req = ApiRequest {
            scope: "CoreHr",
            api: "BatchGetCoreHrbpByEmployee",
            method: http::Method::POST,
            url: String::new()
                + self.cli.open_base_url.as_ref()
                + "/open-apis/corehr/v2/employees/bps/batch_get",
            param_data: req.gen_param(),
            method_option,
            need_tenant_access_token: true,
            ..Default::default()
        };

        let (resp, common_resp): (BatchGetCoreHrbpByEmployeeRespInner, _) =
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
pub struct BatchGetCoreHrbpByEmployeeReq {
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
    /// `people_corehr_id`: 以飞书人事的 ID 来识别用户
    #[api(
        kind = "query",
        name = "user_id_type",
        v_type = "var",
        option = "false"
    )]
    pub user_id_type: String,
    /// 员工ID，ID类型与user_id_type的取值意义一致。
    ///
    /// >
    ///
    /// 如果你需要不同类型的ID进行转换，可以使用 [ID转换服务](https://open.larkoffice.com/document/uAjLw4CM/ukTMukTMukTM/reference/corehr-v1/common_data-id/convert) 换取 ==employment_id==
    ///
    /// **是否必填**: 是
    ///
    /// **数据校验规则**：
    ///
    /// - **长度范围**: `1` 字符- `100` 字符
    #[api(kind = "body", name = "employment_ids")]
    pub employment_ids: Vec<Option<String>>,
    /// 是否获取全部 BP，true 为获取员工所在部门及来自上级部门的全部 HRBP 和属地 BP，false 为仅获取员工的直属 HRBP 和属地 BP（当员工所在部门、属地无 BP 时，会上钻找到最近的 BP），默认为 false
    ///
    /// **示例值**: "true"
    #[api(kind = "body", name = "get_all")]
    pub get_all: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize, Clone, lark_bot_sdk_macros::ApiBaseResp)]
struct BatchGetCoreHrbpByEmployeeRespInner {
    #[serde(flatten)]
    data: Option<BatchGetCoreHrbpByEmployeeResp>,
    #[serde(flatten)]
    base: BaseResp,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct BatchGetCoreHrbpByEmployeeResp {
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
    /// 员工直属 BP 信息，当员工所在部门、属地无 BP 时，会上钻找到最近的 BP
    #[serde(
        rename = "employment_direct_bps",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub employment_direct_bps: Vec<EmploymentBpSubResp>,
    /// 员工全部 BP 信息
    #[serde(
        rename = "employment_all_bps",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub employment_all_bps: Vec<EmploymentBpSubResp>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(default)]
pub struct EmploymentBpSubResp {
    /// 员工雇佣 ID
    ///
    /// >
    ///
    /// 可以使用[搜索员工信息](https://open.larkoffice.com/document/server-docs/corehr-v1/employee/search)接口获取员工其他信息。
    ///
    /// **示例值**: "6863326262618752123"
    #[serde(
        rename = "employment_id",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub employment_id: String,
    /// 员工直属 HRBP 雇佣 ID
    #[serde(
        rename = "hrbp_ids",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub hrbp_ids: Vec<String>,
    /// 员工直属属地 BP 雇佣 ID
    #[serde(
        rename = "location_bp_ids",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub location_bp_ids: Vec<String>,
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
        Fn(
            BatchGetCoreHrbpByEmployeeReq,
        ) -> Result<(BatchGetCoreHrbpByEmployeeResp, CommonResponse), Error>
        + Send
        + Sync
        + 'static
    {
    }
    impl<
            T: Fn(
                    BatchGetCoreHrbpByEmployeeReq,
                )
                    -> Result<(BatchGetCoreHrbpByEmployeeResp, CommonResponse), Error>
                + Send
                + Sync
                + 'static,
        > MockFunc for T
    {
    }

    impl<'c, IStore: Store, IClient: HttpClient> CoreHrServiceMocker<'c, IStore, IClient> {
        pub fn mock_batch_get_core_hrbp_by_employee<F: MockFunc>(
            &self,
            f: F,
        ) -> MockerBuilder<
            Mocker,
            BatchGetCoreHrbpByEmployeeReq,
            BatchGetCoreHrbpByEmployeeResp,
            Arc<dyn MockFunc>,
        > {
            MockerBuilder::new(self.cli.instance_id, Arc::new(f))
        }

        pub(super) fn get_mock_batch_get_core_hrbp_by_employee(
            &self,
            req: &BatchGetCoreHrbpByEmployeeReq,
        ) -> Option<Arc<dyn MockFunc>> {
            do_mock::<
                Mocker,
                BatchGetCoreHrbpByEmployeeReq,
                BatchGetCoreHrbpByEmployeeResp,
                Arc<dyn MockFunc>,
            >(self.cli.instance_id, req)
        }
    }
}

#[cfg(feature = "test-util")]
#[cfg(test)]
mod test {
    use crate::{
        api::gen::core_hr::batch_get_core_hrbp_by_employee::{
            BatchGetCoreHrbpByEmployeeReq, BatchGetCoreHrbpByEmployeeResp,
            BatchGetCoreHrbpByEmployeeRespInner,
        },
        core::{model::CommonResponse, Lark},
    };

    #[tokio::test]
    async fn test_mock() {
        let lark = Lark::new("".to_owned(), "".into());
        let mocker = lark
            .core_hr()
            .mock()
            .mock_batch_get_core_hrbp_by_employee(|_| {
                Ok((
                    BatchGetCoreHrbpByEmployeeResp::default(),
                    CommonResponse::default(),
                ))
            })
            .build();
        let res = lark
            .core_hr()
            .batch_get_core_hrbp_by_employee(BatchGetCoreHrbpByEmployeeReq::default())
            .await;
        assert!(res.is_ok());
        mocker.clear();
        let res = lark
            .core_hr()
            .batch_get_core_hrbp_by_employee(BatchGetCoreHrbpByEmployeeReq::default())
            .await;
        assert!(res.is_err());
    }

    const REQ: &str = r#"{
    "employment_ids": [
        "7140964208476371111"
    ],
    "get_all": true
}"#;

    #[test]
    fn test_req() {
        if REQ == "{}" {
            return;
        }
        if let Err(e) = serde_json::from_str::<super::BatchGetCoreHrbpByEmployeeReqBody>(REQ) {
            panic!("{}", e);
        }
    }

    const RESP: &str = r#"{
    "code": 0,
    "msg": "success",
    "data": {
        "employment_direct_bps": [
            {
                "employment_id": "6863326262618752123",
                "hrbp_ids": [
                    "6863326262618752123"
                ],
                "location_bp_ids": [
                    "6863326262618752123"
                ]
            }
        ],
        "employment_all_bps": [
            {
                "employment_id": "6863326262618752123",
                "hrbp_ids": [
                    "6863326262618752123"
                ],
                "location_bp_ids": [
                    "6863326262618752123"
                ]
            }
        ]
    }
}"#;
    #[test]
    fn test_resp() {
        let res = serde_json::from_str::<BatchGetCoreHrbpByEmployeeRespInner>(RESP);
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
