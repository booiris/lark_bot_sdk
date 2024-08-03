//! doc url: <https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/corehr-v1/job_data/create>
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
    /// **api 版本: 2024-07-08T09:05:29+00:00**
    ///
    /// ## 创建任职信息
    ///
    /// 在系统中第一次创建员工任职数据，通常在员工入职或者做数据批量导入的时候使用，【任职原因】只支持填写“onboarding”。
    ///

    ///
    /// doc: <https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/corehr-v1/job_data/create>
    ///
    /// new doc: <https://open.feishu.cn/document/server-docs/corehr-v1/employee/job_data/create>
    ///
    /// api url: <https://open.feishu.cn/document_portal/v1/document/get_detail?fullPath=%2Fserver-docs%2Fcorehr-v1%2Femployee%2Fjob_data%2Fcreate>
    pub async fn create_core_hr_job_data(
        &self,
        req: CreateCoreHrJobDataReq,
    ) -> Result<(CreateCoreHrJobDataResp, CommonResponse), Error> {
        self.create_core_hr_job_data_with_opt(req, Default::default())
            .await
    }

    /// 参见 [create_core_hr_job_data](#method.create_core_hr_job_data) 函数
    pub async fn create_core_hr_job_data_with_opt(
        &self,
        req: CreateCoreHrJobDataReq,
        method_option: MethodOption,
    ) -> Result<(CreateCoreHrJobDataResp, CommonResponse), Error> {
        #[cfg(feature = "test-util")]
        {
            if let Some(f) = self.mock().get_mock_create_core_hr_job_data(&req) {
                tracing::info!("[lark] CoreHr#CreateCoreHrJobData **mocking** api");
                return f(req);
            }
        }

        tracing::info!("[lark] CoreHr#CreateCoreHrJobData call api");

        let req = ApiRequest {
            scope: "CoreHr",
            api: "CreateCoreHrJobData",
            method: http::Method::POST,
            url: String::new() + self.cli.open_base_url.as_ref() + "/open-apis/corehr/v1/job_datas",
            param_data: req.gen_param(),
            method_option,
            need_tenant_access_token: true,
            ..Default::default()
        };

        let (resp, common_resp): (CreateCoreHrJobDataRespInner, _) = self.cli.do_req(req).await?;
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
pub struct CreateCoreHrJobDataReq {
    /// 操作的唯一标识，用于幂等的进行更新操作，格式为标准的 UUIDV4。此值为空表示将发起一次新的请求，此值非空表示幂等的进行更新操作。
    ///
    /// **示例值**: ""fe599b60-450f-46ff-b2ef-9f6675625b97""
    #[api(
        kind = "query",
        name = "client_token",
        v_type = "var",
        option = "false"
    )]
    pub client_token: String,
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
    /// 此次调用中使用的部门 ID 类型
    ///
    /// **示例值**: "open_department_id"
    ///
    /// **可选值**:
    ///
    /// `open_department_id`: 【飞书】用来在具体某个应用中标识一个部门，同一个 department_id 在不同应用中的 open_department_id 相同。
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
    /// 职务级别 ID，枚举值及详细信息可通过[【批量查询职级】](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/corehr-v1/job_level/list)接口查询获得
    ///
    /// **示例值**: "6890452208593372679"
    #[api(kind = "body", name = "job_level_id")]
    pub job_level_id: Option<String>,
    /// 职等 ID，枚举值及详细信息可通过[【查询职等】](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/corehr-v2/job_grade/query)接口查询获得
    ///
    /// **示例值**: "6890452208593372679"
    #[api(kind = "body", name = "job_grade_id")]
    pub job_grade_id: Option<String>,
    /// 人员类型 ID，枚举值及详细信息可通过[【批量查询人员类型】](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/corehr-v1/employee_type/list)接口查询获得
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "6890452208593372679"
    #[api(kind = "body", name = "employee_type_id")]
    pub employee_type_id: String,
    /// 工时制度 ID，枚举值及详细信息可通过[【批量查询工时制度】](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/corehr-v1/working_hours_type/list)接口查询获得
    ///
    /// **示例值**: "6890452208593372679"
    #[api(kind = "body", name = "working_hours_type_id")]
    pub working_hours_type_id: Option<String>,
    /// 工作地点 ID，枚举值及详细信息可通过[【批量查询地点】](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/corehr-v1/location/list)接口查询获得
    ///
    /// **示例值**: "6890452208593372679"
    #[api(kind = "body", name = "work_location_id")]
    pub work_location_id: Option<String>,
    /// 部门 ID，枚举值及详细信息可通过[【批量查询部门】](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/corehr-v2/department/batch_get)接口查询获得
    ///
    /// 与 department_id_type 类型一致
    ///
    /// **示例值**: "6890452208593372679"
    #[api(kind = "body", name = "department_id")]
    pub department_id: Option<String>,
    /// 职务 ID，枚举值及详细信息可通过[【批量查询职务】](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/corehr-v2/job/list)接口查询获得
    ///
    /// **示例值**: "6890452208593372679"
    #[api(kind = "body", name = "job_id")]
    pub job_id: Option<String>,
    /// 试用期开始日期
    ///
    /// **示例值**: "2018-03-16"
    #[api(kind = "body", name = "probation_start_date")]
    pub probation_start_date: Option<String>,
    /// 试用期结束日期（实际结束日期）
    ///
    /// **示例值**: "2019-05-24"
    #[api(kind = "body", name = "probation_end_date")]
    pub probation_end_date: Option<String>,
    /// 是否为主任职
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "true"
    #[api(kind = "body", name = "primary_job_data")]
    pub primary_job_data: bool,
    /// 雇佣 ID，详细信息可以通过[【搜索员工信息】](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/corehr-v2/employee/search)接口查询获得
    ///
    /// 与 user_id_type 类型一致
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "6893014062142064135"
    #[api(kind = "body", name = "employment_id")]
    pub employment_id: String,
    /// 生效时间
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "2020-05-01 00:00:00"
    #[api(kind = "body", name = "effective_time")]
    pub effective_time: String,
    /// 失效时间
    ///
    /// **示例值**: "2020-05-02 00:00:00"
    #[api(kind = "body", name = "expiration_time")]
    pub expiration_time: Option<String>,
    /// 职务序列 ID，枚举值及详细信息可通过[【批量查询序列】](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/corehr-v1/job_family/list)接口查询获得
    ///
    /// **示例值**: "1245678"
    #[api(kind = "body", name = "job_family_id")]
    pub job_family_id: Option<String>,
    /// 任职原因，枚举值可通过文档[【飞书人事枚举常量】](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/corehr-v1/feishu-people-enum-constant)任职原因（assignment_start_reason）枚举定义部分获得，这里只支持填写"onboarding"
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "onboarding"
    #[api(kind = "body", name = "assignment_start_reason")]
    pub assignment_start_reason: EnumSubReq,
    /// 预计试用期结束日期
    ///
    /// **示例值**: "2006-01-02"
    #[api(kind = "body", name = "probation_expected_end_date")]
    pub probation_expected_end_date: Option<String>,
    /// 直属上级的任职记录 ID，详细信息可通过[【批量查询员工任职信息】](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/corehr-v2/employees-job_data/batch_get)接口查询获得
    ///
    /// **示例值**: "6890452208593372679"
    #[api(kind = "body", name = "direct_manager_id")]
    pub direct_manager_id: Option<String>,
    /// 虚线上级的任职记录 ID，详细信息可通过[【批量查询员工任职信息】](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/corehr-v2/employees-job_data/batch_get)接口查询获得
    ///
    /// **示例值**: "["6971723901730686501"]"
    #[api(kind = "body", name = "dotted_line_manager_id_list")]
    pub dotted_line_manager_id_list: Vec<Option<String>>,
    /// 第二直属上级的任职记录 ID，详细信息可通过[【批量查询员工任职信息】](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/corehr-v2/employees-job_data/batch_get)接口查询获得
    ///
    /// **示例值**: "6890452208593372679"
    #[api(kind = "body", name = "second_direct_manager_id")]
    pub second_direct_manager_id: Option<String>,
    /// 成本中心分摊信息
    #[api(kind = "body", name = "cost_center_rate")]
    pub cost_center_rate: Vec<Option<SupportCostCenterItemSubReq>>,
    /// 排班类型，枚举值 api_name 可通过[【获取字段详情】](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/corehr-v1/custom_field/get_by_param)接口查询，查询参数如下：
    ///
    /// - object_api_name = "job_data"
    ///
    /// - custom_api_name = "work_shift"
    #[api(kind = "body", name = "work_shift")]
    pub work_shift: Option<EnumSubReq>,
    /// 薪资类型，枚举值 api_name 可通过[【获取字段详情】](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/corehr-v1/custom_field/get_by_param)接口查询，查询参数如下：
    ///
    /// - object_api_name = "job_data"
    ///
    /// - custom_api_name = "compensation_type"
    #[api(kind = "body", name = "compensation_type")]
    pub compensation_type: Option<EnumSubReq>,
    /// 任职公司 ID，详细信息可通过[【批量查询公司】](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/corehr-v1/company/list)接口查询获得
    ///
    /// **示例值**: "6890452208593372680"
    #[api(kind = "body", name = "service_company")]
    pub service_company: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(default)]
pub struct EnumSubReq {
    /// 枚举值
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "onboarding"
    #[serde(
        rename = "enum_name",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub enum_name: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(default)]
pub struct SupportCostCenterItemSubReq {
    /// 支持的成本中心 ID，详细信息可通过[【搜索成本中心信息】](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/corehr-v2/cost_center/search)接口查询获得
    ///
    /// **示例值**: "6950635856373745165"
    #[serde(
        rename = "cost_center_id",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub cost_center_id: Option<String>,
    /// 分摊比例
    ///
    /// **示例值**: "100"
    #[serde(
        rename = "rate",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub rate: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize, Clone, lark_bot_sdk_macros::ApiBaseResp)]
struct CreateCoreHrJobDataRespInner {
    #[serde(flatten)]
    data: Option<CreateCoreHrJobDataResp>,
    #[serde(flatten)]
    base: BaseResp,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct CreateCoreHrJobDataResp {
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
    /// 创建成功返回job data信息
    #[serde(
        rename = "job_data",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub job_data: JobDataSubResp,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(default)]
pub struct JobDataSubResp {
    /// 任职信息 ID，用于后续更新和查询
    ///
    /// **示例值**: "6890452208593372679"
    #[serde(
        rename = "id",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub id: String,
    /// 任职记录版本 ID，可用于指定版本更新
    ///
    /// **示例值**: "6890452208593372697"
    #[serde(
        rename = "version_id",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub version_id: String,
    /// 职务级别 ID，枚举值及详细信息可通过[【查询单个职级】](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/corehr-v1/job_level/get)接口查询获得
    ///
    /// **示例值**: "6890452208593372679"
    #[serde(
        rename = "job_level_id",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub job_level_id: String,
    /// 职等 ID，枚举值及详细信息可通过[【查询职等】](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/corehr-v2/job_grade/query)接口查询获得
    ///
    /// **示例值**: "6890452208593372679"
    #[serde(
        rename = "job_grade_id",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub job_grade_id: String,
    /// 人员类型 ID，枚举值及详细信息可通过[【查询单个人员类型】](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/corehr-v1/employee_type/get)接口查询获得
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "6890452208593372679"
    #[serde(
        rename = "employee_type_id",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub employee_type_id: String,
    /// 工时制度 ID，枚举值及详细信息可通过[【查询单个工时制度】](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/corehr-v1/working_hours_type/get)接口查询获得
    ///
    /// **示例值**: "6890452208593372679"
    #[serde(
        rename = "working_hours_type_id",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub working_hours_type_id: String,
    /// 工作地点 ID，枚举值及详细信息可通过[【查询单个地点】](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/corehr-v1/location/get)接口查询获得
    ///
    /// **示例值**: "6890452208593372679"
    #[serde(
        rename = "work_location_id",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub work_location_id: String,
    /// 部门 ID，枚举值及详细信息可通过[【查询单个部门】](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/corehr-v1/department/get)接口查询获得
    ///
    /// 与 department_id_type 类型一致
    ///
    /// **示例值**: "6890452208593372679"
    #[serde(
        rename = "department_id",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub department_id: String,
    /// 职务 ID，枚举值及详细信息可通过[【查询单个职务】](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/corehr-v1/job/get)接口查询获得
    ///
    /// **示例值**: "6890452208593372679"
    #[serde(
        rename = "job_id",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub job_id: String,
    /// 试用期开始日期
    ///
    /// **示例值**: "2018-03-16"
    #[serde(
        rename = "probation_start_date",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub probation_start_date: String,
    /// 试用期结束日期（实际结束日期）
    ///
    /// **示例值**: "2019-05-24"
    #[serde(
        rename = "probation_end_date",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub probation_end_date: String,
    /// 是否为主任职
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "true"
    #[serde(
        rename = "primary_job_data",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub primary_job_data: bool,
    /// 雇佣 ID，详细信息可以通过[【搜索员工信息】](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/corehr-v2/employee/search)接口查询获得
    ///
    /// 与 user_id_type 类型一致
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "6893014062142064135"
    #[serde(
        rename = "employment_id",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub employment_id: String,
    /// 生效时间
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "2020-05-01 00:00:00"
    #[serde(
        rename = "effective_time",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub effective_time: String,
    /// 失效时间
    ///
    /// **示例值**: "2020-05-02 00:00:00"
    #[serde(
        rename = "expiration_time",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub expiration_time: String,
    /// 职务序列 ID，枚举值及详细信息可通过[【查询单个序列】](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/corehr-v1/job_family/get)接口查询获得
    ///
    /// **示例值**: "1245678"
    #[serde(
        rename = "job_family_id",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub job_family_id: String,
    /// 任职原因，枚举值可通过文档[【飞书人事枚举常量】](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/corehr-v1/feishu-people-enum-constant)任职原因（assignment_start_reason）枚举定义部分获得
    ///
    /// **示例值**: "onboarding"
    #[serde(
        rename = "assignment_start_reason",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub assignment_start_reason: EnumSubResp,
    /// 预计试用期结束日期
    ///
    /// **示例值**: "2006-01-02"
    #[serde(
        rename = "probation_expected_end_date",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub probation_expected_end_date: String,
    /// 周工作时长
    ///
    /// **示例值**: "30"
    #[serde(
        rename = "weekly_working_hours",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub weekly_working_hours: i64,
    /// 直属上级的任职记录 ID，详细信息可通过[【批量查询员工任职信息】](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/corehr-v2/employees-job_data/batch_get)接口查询获得
    ///
    /// **示例值**: "6890452208593372679"
    #[serde(
        rename = "direct_manager_id",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub direct_manager_id: String,
    /// 虚线上级的任职记录 ID，详细信息可通过[【批量查询员工任职信息】](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/corehr-v2/employees-job_data/batch_get)接口查询获得
    ///
    /// **示例值**: "["6971723901730686501"]"
    #[serde(
        rename = "dotted_line_manager_id_list",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub dotted_line_manager_id_list: Vec<String>,
    /// 第二直属上级的任职记录 ID，详细信息可通过[【批量查询员工任职信息】](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/corehr-v2/employees-job_data/batch_get)接口查询获得
    ///
    /// **示例值**: "6890452208593372679"
    #[serde(
        rename = "second_direct_manager_id",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub second_direct_manager_id: String,
    /// 成本中心分摊信息
    #[serde(
        rename = "cost_center_rate",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub cost_center_rate: Vec<SupportCostCenterItemSubResp>,
    /// 周工作时长
    ///
    /// **示例值**: "37.5"
    #[serde(
        rename = "weekly_working_hours_v2",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub weekly_working_hours_v2: f64,
    /// 排班类型
    #[serde(
        rename = "work_shift",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub work_shift: EnumSubResp,
    /// 薪资类型
    #[serde(
        rename = "compensation_type",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub compensation_type: EnumSubResp,
    /// 任职公司，枚举值及详细信息可通过[【查询单个序列】](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/corehr-v1/company/get)接口查询获得
    ///
    /// **示例值**: "6890452208593372680"
    #[serde(
        rename = "service_company",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub service_company: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(default)]
pub struct EnumSubResp {
    /// 枚举值
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "onboarding"
    #[serde(
        rename = "enum_name",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub enum_name: String,
    /// 枚举多语展示
    #[serde(
        rename = "display",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub display: Vec<I18nSubResp>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(default)]
pub struct SupportCostCenterItemSubResp {
    /// 支持的成本中心 ID，详细信息可通过[【搜索成本中心信息】](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/corehr-v2/cost_center/search)接口查询获得
    ///
    /// **示例值**: "6950635856373745165"
    #[serde(
        rename = "cost_center_id",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub cost_center_id: String,
    /// 分摊比例
    ///
    /// **示例值**: "100"
    #[serde(
        rename = "rate",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub rate: i64,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(default)]
pub struct I18nSubResp {
    /// 名称信息的语言
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "zh-CN"
    #[serde(
        rename = "lang",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub lang: String,
    /// 名称信息的内容
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
        Fn(CreateCoreHrJobDataReq) -> Result<(CreateCoreHrJobDataResp, CommonResponse), Error>
        + Send
        + Sync
        + 'static
    {
    }
    impl<
            T: Fn(
                    CreateCoreHrJobDataReq,
                ) -> Result<(CreateCoreHrJobDataResp, CommonResponse), Error>
                + Send
                + Sync
                + 'static,
        > MockFunc for T
    {
    }

    impl<'c, IStore: Store, IClient: HttpClient> CoreHrServiceMocker<'c, IStore, IClient> {
        pub fn mock_create_core_hr_job_data<F: MockFunc>(
            &self,
            f: F,
        ) -> MockerBuilder<Mocker, CreateCoreHrJobDataReq, CreateCoreHrJobDataResp, Arc<dyn MockFunc>>
        {
            MockerBuilder::new(self.cli.instance_id, Arc::new(f))
        }

        pub(super) fn get_mock_create_core_hr_job_data(
            &self,
            req: &CreateCoreHrJobDataReq,
        ) -> Option<Arc<dyn MockFunc>> {
            do_mock::<Mocker, CreateCoreHrJobDataReq, CreateCoreHrJobDataResp, Arc<dyn MockFunc>>(
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
        api::gen::core_hr::create_core_hr_job_data::{
            CreateCoreHrJobDataReq, CreateCoreHrJobDataResp, CreateCoreHrJobDataRespInner,
        },
        core::{model::CommonResponse, Lark},
    };

    #[tokio::test]
    async fn test_mock() {
        let lark = Lark::new("".to_owned(), "".into());
        let mocker = lark
            .core_hr()
            .mock()
            .mock_create_core_hr_job_data(|_| {
                Ok((
                    CreateCoreHrJobDataResp::default(),
                    CommonResponse::default(),
                ))
            })
            .build();
        let res = lark
            .core_hr()
            .create_core_hr_job_data(CreateCoreHrJobDataReq::default())
            .await;
        assert!(res.is_ok());
        mocker.clear();
        let res = lark
            .core_hr()
            .create_core_hr_job_data(CreateCoreHrJobDataReq::default())
            .await;
        assert!(res.is_err());
    }

    const REQ: &str = r#"{
    "job_level_id": "6890452208593372679",
    "job_grade_id": "6890452208593372679",
    "employee_type_id": "6890452208593372679",
    "working_hours_type_id": "6890452208593372679",
    "work_location_id": "6890452208593372679",
    "department_id": "6890452208593372679",
    "job_id": "6890452208593372679",
    "probation_start_date": "2018-03-16",
    "probation_end_date": "2019-05-24",
    "primary_job_data": true,
    "employment_id": "6893014062142064135",
    "effective_time": "2020-05-01 00:00:00",
    "expiration_time": "2020-05-02 00:00:00",
    "job_family_id": "1245678",
    "assignment_start_reason": {
        "enum_name": "onboarding"
    },
    "probation_expected_end_date": "2006-01-02",
    "direct_manager_id": "6890452208593372679",
    "dotted_line_manager_id_list": [
        "6890452208593372681"
    ],
    "second_direct_manager_id": "6890452208593372679",
    "cost_center_rate": [
        {
            "cost_center_id": "6950635856373745165",
            "rate": 100
        }
    ],
    "work_shift": {
        "enum_name": "phone_type"
    },
    "compensation_type": {
        "enum_name": "phone_type"
    },
    "service_company": "6890452208593372680"
}"#;

    #[test]
    fn test_req() {
        if REQ == "{}" {
            return;
        }
        if let Err(e) = serde_json::from_str::<super::CreateCoreHrJobDataReqBody>(REQ) {
            panic!("{}", e);
        }
    }

    const RESP: &str = r#"{
    "code": 0,
    "msg": "success",
    "data": {
        "job_data": {
            "id": "6890452208593372679",
            "version_id": "6890452208593372697",
            "job_level_id": "6890452208593372679",
            "job_grade_id": "6890452208593372679",
            "employee_type_id": "6890452208593372679",
            "working_hours_type_id": "6890452208593372679",
            "work_location_id": "6890452208593372679",
            "department_id": "6890452208593372679",
            "job_id": "6890452208593372679",
            "probation_start_date": "2018-03-16",
            "probation_end_date": "2019-05-24",
            "primary_job_data": true,
            "employment_id": "6893014062142064135",
            "effective_time": "2020-05-01 00:00:00",
            "expiration_time": "2020-05-02 00:00:00",
            "job_family_id": "1245678",
            "assignment_start_reason": {
                "enum_name": "onboarding",
                "display": [
                    {
                        "lang": "zh-CN",
                        "value": "张三"
                    }
                ]
            },
            "probation_expected_end_date": "2006-01-02",
            "weekly_working_hours": 30,
            "direct_manager_id": "6890452208593372679",
            "dotted_line_manager_id_list": [
                "6890452208593372681"
            ],
            "second_direct_manager_id": "6890452208593372679",
            "cost_center_rate": [
                {
                    "cost_center_id": "6950635856373745165",
                    "rate": 100
                }
            ],
            "weekly_working_hours_v2": 37.5,
            "work_shift": {
                "enum_name": "phone_type",
                "display": [
                    {
                        "lang": "zh-CN",
                        "value": "刘梓新"
                    }
                ]
            },
            "compensation_type": {
                "enum_name": "phone_type",
                "display": [
                    {
                        "lang": "zh-CN",
                        "value": "刘梓新"
                    }
                ]
            },
            "service_company": "6890452208593372680"
        }
    }
}"#;
    #[test]
    fn test_resp() {
        let res = serde_json::from_str::<CreateCoreHrJobDataRespInner>(RESP);
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
