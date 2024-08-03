//! doc url: <https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/corehr-v1/compensation_standard/match>
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
    /// **api 版本: 2024-07-12T09:26:01+00:00**
    ///
    /// ## 获取员工薪资标准
    ///
    /// - 调用此接口来获取员工匹配的薪资标准信息
    ///
    /// - 请求时，可选传递员工的部门 ID、职级 ID 、序列 ID 等筛选条件，用于匹配薪资标准
    ///
    /// - 此接口将返回员工可匹配到的薪资标准全部信息，包括薪资标准表 ID、薪级薪等、薪资带宽、薪资标准值等
    ///
    /// 该接口会按照应用拥有的「薪资标准资源」的权限范围返回数据，请确定在「开发者后台 - 权限管理 - 数据权限-飞书人事(企业版)数据权限范围」中已申请「薪资标准资源」权限范围
    ///
    /// doc: <https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/corehr-v1/compensation_standard/match>
    ///
    /// new doc: <https://open.feishu.cn/document/server-docs/corehr-v1/compensation_standard/match>
    ///
    /// api url: <https://open.feishu.cn/document_portal/v1/document/get_detail?fullPath=%2Fserver-docs%2Fcorehr-v1%2Fcompensation_standard%2Fmatch>
    pub async fn match_core_hr_compensation_standard(
        &self,
        req: MatchCoreHrCompensationStandardReq,
    ) -> Result<(MatchCoreHrCompensationStandardResp, CommonResponse), Error> {
        self.match_core_hr_compensation_standard_with_opt(req, Default::default())
            .await
    }

    /// 参见 [match_core_hr_compensation_standard](#method.match_core_hr_compensation_standard) 函数
    pub async fn match_core_hr_compensation_standard_with_opt(
        &self,
        req: MatchCoreHrCompensationStandardReq,
        method_option: MethodOption,
    ) -> Result<(MatchCoreHrCompensationStandardResp, CommonResponse), Error> {
        #[cfg(feature = "test-util")]
        {
            if let Some(f) = self
                .mock()
                .get_mock_match_core_hr_compensation_standard(&req)
            {
                tracing::info!("[lark] CoreHr#MatchCoreHrCompensationStandard **mocking** api");
                return f(req);
            }
        }

        tracing::info!("[lark] CoreHr#MatchCoreHrCompensationStandard call api");

        let req = ApiRequest {
            scope: "CoreHr",
            api: "MatchCoreHrCompensationStandard",
            method: http::Method::GET,
            url: String::new()
                + self.cli.open_base_url.as_ref()
                + "/open-apis/corehr/v1/compensation_standards/match",
            param_data: req.gen_param(),
            method_option,
            need_tenant_access_token: true,
            ..Default::default()
        };

        let (resp, common_resp): (MatchCoreHrCompensationStandardRespInner, _) =
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
pub struct MatchCoreHrCompensationStandardReq {
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
    /// `people_corehr_id`: 以飞书人事的ID来识别用户
    #[api(
        kind = "query",
        name = "user_id_type",
        v_type = "var",
        option = "false"
    )]
    pub user_id_type: String,
    /// 此次调用中使用的部门 ID 类型，传入部门ID时传入
    ///
    /// **示例值**: "open_department_id"
    ///
    /// **可选值**:
    ///
    /// `open_department_id`: 【飞书】用来在具体某个应用中标识一个部门，同一个department_id 在不同应用中的 open_department_id 相同。
    ///
    /// `department_id`: 【飞书】用来标识租户内一个唯一的部门。
    ///
    /// `people_corehr_department_id`: 【飞书人事】用来标识「飞书人事」中的部门。
    #[api(
        kind = "query",
        name = "department_id_type",
        v_type = "var",
        option = "false"
    )]
    pub department_id_type: String,
    /// 雇员ID，可通过接口[【批量查询员工信息】](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/corehr-v2/employee/batch_get)获取
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "7124293751317038636"
    #[api(
        kind = "query",
        name = "employment_id",
        v_type = "var",
        option = "false"
    )]
    pub employment_id: String,
    /// 薪资标准的关联对象，项目或者指标
    ///
    /// **示例值**: "cpst_item"
    ///
    /// **可选值**:
    ///
    /// `cpst_item`: 薪资项目
    ///
    /// `cpst_indicator`: 薪资统计指标
    #[api(
        kind = "query",
        name = "reference_object_api",
        v_type = "var",
        option = "false"
    )]
    pub reference_object_api: String,
    /// 薪资标准表关联对象ID，即薪资项目/统计指标ID，可通过接口[【批量查询薪资项】](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/compensation-v1/item/list)、[【批量查询薪资统计指标】](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/compensation-v1/indicator/list)获取
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "7156853394442044972"
    #[api(
        kind = "query",
        name = "reference_object_id",
        v_type = "var",
        option = "false"
    )]
    pub reference_object_id: String,
    /// 部门ID，可通过接口[【批量查询部门】](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/corehr-v1/department/list)获取
    ///
    /// **示例值**: "od-53899868dd0da32292a2d809f0518c8f"
    #[api(
        kind = "query",
        name = "department_id",
        v_type = "var",
        option = "false"
    )]
    pub department_id: String,
    /// 工作地点ID，可通过接口[【批量查询地点】](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/corehr-v1/location/list)获取
    ///
    /// **示例值**: "7094869485965870636"
    #[api(
        kind = "query",
        name = "work_location_id",
        v_type = "var",
        option = "false"
    )]
    pub work_location_id: String,
    /// 公司ID，可通过接口[【批量查询公司】](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/corehr-v1/company/list)获取
    ///
    /// **示例值**: "7091599096804394540"
    #[api(kind = "query", name = "company_id", v_type = "var", option = "false")]
    pub company_id: String,
    /// 职务序列ID，可通过接口[【批量查询职务序列】](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/corehr-v1/job_family/list)获取
    ///
    /// **示例值**: "7039313681989502508"
    #[api(
        kind = "query",
        name = "job_family_id",
        v_type = "var",
        option = "false"
    )]
    pub job_family_id: String,
    /// 职级ID，可通过接口[【批量查询职务级别】](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/corehr-v1/job_level/list)获取
    ///
    /// **示例值**: "7086415175263258156"
    #[api(
        kind = "query",
        name = "job_level_id",
        v_type = "var",
        option = "false"
    )]
    pub job_level_id: String,
    /// 人员类型ID，可通过接口[【批量查询人员类型】](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/corehr-v1/employee_type/list)获取
    ///
    /// **示例值**: "7039310401359775276"
    #[api(
        kind = "query",
        name = "employee_type_id",
        v_type = "var",
        option = "false"
    )]
    pub employee_type_id: String,
    /// 招聘类型
    ///
    /// **示例值**: "experienced_professionals"
    ///
    /// **可选值**:
    ///
    /// `experienced_professionals`: 社招
    ///
    /// `recent_graduates`: 校招
    ///
    /// `routine_intern`: 日常实习
    #[api(
        kind = "query",
        name = "recruitment_type",
        v_type = "var",
        option = "false"
    )]
    pub recruitment_type: String,
    /// 定调薪原因ID，可通过接口[【批量查询定调薪原因】](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/compensation-v1/change_reason/list)获取
    ///
    /// **示例值**: "6967639606963471117"
    #[api(
        kind = "query",
        name = "cpst_change_reason_id",
        v_type = "var",
        option = "false"
    )]
    pub cpst_change_reason_id: String,
    /// 薪资方案ID，可通过接口[【批量查询薪资方案】](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/compensation-v1/plan/list)获取
    ///
    /// **示例值**: "6967639606963471118"
    #[api(
        kind = "query",
        name = "cpst_plan_id",
        v_type = "var",
        option = "false"
    )]
    pub cpst_plan_id: String,
    /// 薪级薪等ID
    ///
    /// **示例值**: "6967639606963471119"
    #[api(
        kind = "query",
        name = "cpst_salary_level_id",
        v_type = "var",
        option = "false"
    )]
    pub cpst_salary_level_id: String,
    /// 生效时间（毫秒级时间戳）
    ///
    /// **示例值**: "1660924800000"
    #[api(
        kind = "query",
        name = "effective_time",
        v_type = "var",
        option = "false"
    )]
    pub effective_time: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, lark_bot_sdk_macros::ApiBaseResp)]
struct MatchCoreHrCompensationStandardRespInner {
    #[serde(flatten)]
    data: Option<MatchCoreHrCompensationStandardResp>,
    #[serde(flatten)]
    base: BaseResp,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct MatchCoreHrCompensationStandardResp {
    /// \-
    #[serde(
        rename = "data",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub data: CpstMatchItemSubResp,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(default)]
pub struct CpstMatchItemSubResp {
    /// 薪资标准表ID
    ///
    /// **示例值**: "7174758593538295340"
    #[serde(
        rename = "standard_id",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub standard_id: String,
    /// 薪资等级
    #[serde(
        rename = "grade",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub grade: CpstGradeSubResp,
    /// 生效时间（毫秒级时间戳）
    ///
    /// **示例值**: "1660924800000"
    #[serde(
        rename = "effective_time",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub effective_time: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(default)]
pub struct CpstGradeSubResp {
    /// 薪资等级ID
    ///
    /// **示例值**: "7174758360888215084"
    #[serde(
        rename = "grade_id",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub grade_id: String,
    /// 薪资等级时间轴ID
    ///
    /// **示例值**: "7174758360888247852"
    #[serde(
        rename = "grade_tid",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub grade_tid: String,
    /// 带宽上下限和标准值
    #[serde(
        rename = "grade_standard_value",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub grade_standard_value: CpstGradeStandardValueSubResp,
    /// 币种
    #[serde(
        rename = "currency",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub currency: CpstCurrencySubResp,
    /// 薪资标准描述
    #[serde(
        rename = "description",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub description: CpstI18nSubResp,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(default)]
pub struct CpstGradeStandardValueSubResp {
    /// 薪资标准的关联对象，项目或者指标
    #[serde(
        rename = "reference_object",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub reference_object: ReferenceObjectSubResp,
    /// 薪资标准类型
    #[serde(
        rename = "standard_type",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub standard_type: CpstStandardTypeSubResp,
    /// 上下限
    #[serde(
        rename = "band_width",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub band_width: CpstBandWidthSubResp,
    /// 标准值
    ///
    /// **示例值**: "12000"
    #[serde(
        rename = "standard_value",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub standard_value: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(default)]
pub struct CpstCurrencySubResp {
    /// 币种ID，可通过接口[【查询货币信息】](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/corehr-v2/basic_info-currency/search)获取
    ///
    /// **示例值**: "6863329932261459464"
    #[serde(
        rename = "currency_id",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub currency_id: String,
    /// 币种code
    ///
    /// **示例值**: "CNY"
    #[serde(
        rename = "code",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub code: String,
    /// 币种名称
    #[serde(
        rename = "name",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub name: CpstI18nSubResp,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(default)]
pub struct CpstI18nSubResp {
    /// 中文
    ///
    /// **示例值**: "中文名称"
    #[serde(
        rename = "zh_cn",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub zh_cn: String,
    /// 英文
    ///
    /// **示例值**: "english name"
    #[serde(
        rename = "en_us",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub en_us: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(default)]
pub struct ReferenceObjectSubResp {
    /// cpst_item(项目)、 cpst_indicator(指标)
    ///
    /// **示例值**: "cpst_item"
    ///
    /// **可选值**:
    ///
    /// `cpst_item`: 项目
    ///
    /// `cpst_indicator`: 指标
    #[serde(
        rename = "api_name",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub api_name: String,
    /// 值列表 例如薪资项和薪资统计指标ID
    ///
    /// **示例值**: "7156853394442044972"
    #[serde(
        rename = "id",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub id: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(default)]
pub struct CpstStandardTypeSubResp {
    /// 薪资标准类型
    ///
    /// **示例值**: "standard_value"
    ///
    /// **可选值**:
    ///
    /// `standard_value`: standard_value
    ///
    /// `bandwidth_and_standard_value`: bandwidth_and_standard_value
    ///
    /// `bandwidth_upper_and_lower_limit`: bandwidth_upper_and_lower_limit
    #[serde(
        rename = "api_name",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub api_name: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(default)]
pub struct CpstBandWidthSubResp {
    /// 上限
    ///
    /// **示例值**: "15000"
    #[serde(
        rename = "upper_limit",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub upper_limit: String,
    /// 下限
    ///
    /// **示例值**: "10000"
    #[serde(
        rename = "lower_limit",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub lower_limit: String,
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
            MatchCoreHrCompensationStandardReq,
        ) -> Result<(MatchCoreHrCompensationStandardResp, CommonResponse), Error>
        + Send
        + Sync
        + 'static
    {
    }
    impl<
            T: Fn(
                    MatchCoreHrCompensationStandardReq,
                )
                    -> Result<(MatchCoreHrCompensationStandardResp, CommonResponse), Error>
                + Send
                + Sync
                + 'static,
        > MockFunc for T
    {
    }

    impl<'c, IStore: Store, IClient: HttpClient> CoreHrServiceMocker<'c, IStore, IClient> {
        pub fn mock_match_core_hr_compensation_standard<F: MockFunc>(
            &self,
            f: F,
        ) -> MockerBuilder<
            Mocker,
            MatchCoreHrCompensationStandardReq,
            MatchCoreHrCompensationStandardResp,
            Arc<dyn MockFunc>,
        > {
            MockerBuilder::new(self.cli.instance_id, Arc::new(f))
        }

        pub(super) fn get_mock_match_core_hr_compensation_standard(
            &self,
            req: &MatchCoreHrCompensationStandardReq,
        ) -> Option<Arc<dyn MockFunc>> {
            do_mock::<
                Mocker,
                MatchCoreHrCompensationStandardReq,
                MatchCoreHrCompensationStandardResp,
                Arc<dyn MockFunc>,
            >(self.cli.instance_id, req)
        }
    }
}

#[cfg(feature = "test-util")]
#[cfg(test)]
mod test {
    use crate::{
        api::gen::core_hr::match_core_hr_compensation_standard::{
            MatchCoreHrCompensationStandardReq, MatchCoreHrCompensationStandardResp,
            MatchCoreHrCompensationStandardRespInner,
        },
        core::{model::CommonResponse, Lark},
    };

    #[tokio::test]
    async fn test_mock() {
        let lark = Lark::new("".to_owned(), "".into());
        let mocker = lark
            .core_hr()
            .mock()
            .mock_match_core_hr_compensation_standard(|_| {
                Ok((
                    MatchCoreHrCompensationStandardResp::default(),
                    CommonResponse::default(),
                ))
            })
            .build();
        let res = lark
            .core_hr()
            .match_core_hr_compensation_standard(MatchCoreHrCompensationStandardReq::default())
            .await;
        assert!(res.is_ok());
        mocker.clear();
        let res = lark
            .core_hr()
            .match_core_hr_compensation_standard(MatchCoreHrCompensationStandardReq::default())
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
    "data": {
        "effective_time": "1660924800000",
        "grade": {
            "currency": {
                "code": "CNY",
                "currency_id": "6863329932261459464",
                "name": {
                    "en_us": "Renminbi (Chinese) yuan",
                    "zh_cn": "人民币元"
                }
            },
            "grade_id": "7174758360888215084",
            "grade_standard_value": {
                "band_width": {
                    "lower_limit": "5000",
                    "upper_limit": "10000"
                },
                "reference_object": {
                    "api_name": "cpst_item",
                    "id": "7156853394442044972"
                },
                "standard_type": {
                    "api_name": "bandwidth_and_standard_value"
                },
                "standard_value": "8888"
            },
            "grade_tid": "7174758360888247852"
        },
        "standard_id": "7174758593538295340"
    },
    "msg": "success"
}"#;
    #[test]
    fn test_resp() {
        let res = serde_json::from_str::<MatchCoreHrCompensationStandardRespInner>(RESP);
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
