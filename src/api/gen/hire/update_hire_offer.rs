//! doc url: <https://open.feishu.cn/document/ukTMukTMukTM/uMzM1YjLzMTN24yMzUjN/hire-v1/offer/update>
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
    /// **api 版本: 2024-04-26T02:16:46+00:00**
    ///
    /// ## 更新 Offer 信息
    ///
    /// 1. 更新 Offer 时，需传入本文档中标注为必传的参数，其余参数是否必传参考「获取 Offer 申请表模板信息」的参数定义；
    ///
    /// 2. 对系统中已存在的 offer 进行更新的，若更新 offer 中含有「修改需审批」的字段，更新后原 Offer 的审批会自动撤回，需要重新发起审批。
    ///

    ///
    /// doc: <https://open.feishu.cn/document/ukTMukTMukTM/uMzM1YjLzMTN24yMzUjN/hire-v1/offer/update>
    ///
    /// new doc: <https://open.feishu.cn/document/server-docs/hire-v1/candidate-management/delivery-process-management/offer/update>
    ///
    /// api url: <https://open.feishu.cn/document_portal/v1/document/get_detail?fullPath=%2Fserver-docs%2Fhire-v1%2Fcandidate-management%2Fdelivery-process-management%2Foffer%2Fupdate>
    pub async fn update_hire_offer(
        &self,
        req: UpdateHireOfferReq,
    ) -> Result<(UpdateHireOfferResp, CommonResponse), Error> {
        self.update_hire_offer_with_opt(req, Default::default())
            .await
    }

    /// 参见 [update_hire_offer](#method.update_hire_offer) 函数
    pub async fn update_hire_offer_with_opt(
        &self,
        req: UpdateHireOfferReq,
        method_option: MethodOption,
    ) -> Result<(UpdateHireOfferResp, CommonResponse), Error> {
        #[cfg(feature = "test-util")]
        {
            if let Some(f) = self.mock().get_mock_update_hire_offer(&req) {
                tracing::info!("[lark] Hire#UpdateHireOffer **mocking** api");
                return f(req);
            }
        }

        tracing::info!("[lark] Hire#UpdateHireOffer call api");

        let req = ApiRequest {
            scope: "Hire",
            api: "UpdateHireOffer",
            method: http::Method::PUT,
            url: String::new()
                + self.cli.open_base_url.as_ref()
                + "/open-apis/hire/v1/offers/:offer_id",
            param_data: req.gen_param(),
            method_option,
            need_tenant_access_token: true,
            ..Default::default()
        };

        let (resp, common_resp): (UpdateHireOfferRespInner, _) = self.cli.do_req(req).await?;
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
pub struct UpdateHireOfferReq {
    /// Offer ID
    ///
    /// **示例值**: "7016605170635213100"
    #[api(kind = "path", name = "offer_id")]
    pub offer_id: String,
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
    /// 此次调用中使用的部门 ID 的类型
    ///
    /// **示例值**: "department_id"
    ///
    /// **可选值**:
    ///
    /// `open_department_id`: 以 open_department_id 来标识部门
    ///
    /// `department_id`: 以 department_id 来标识部门
    #[api(
        kind = "query",
        name = "department_id_type",
        v_type = "var",
        option = "false"
    )]
    pub department_id_type: String,
    /// 此次调用中使用的「职级 ID」的类型
    ///
    /// **示例值**: "6942778198054125570"
    ///
    /// **可选值**:
    ///
    /// `people_admin_job_level_id`: 「人力系统管理后台」适用的职级 ID。人力系统管理后台逐步下线中，建议不继续使用此 ID。
    ///
    /// `job_level_id`: 「飞书管理后台」适用的职级 ID，通过[「获取租户职级列表」](https://open.feishu.cn/document/server-docs/contact-v3/job_level/list)接口获取
    #[api(
        kind = "query",
        name = "job_level_id_type",
        v_type = "var",
        option = "false"
    )]
    pub job_level_id_type: String,
    /// 此次调用中使用的「序列 ID」的类型
    ///
    /// **示例值**: "6942778198054125571"
    ///
    /// **可选值**:
    ///
    /// `people_admin_job_category_id`: 「人力系统管理后台」适用的序列 ID。人力系统管理后台逐步下线中，建议不继续使用此 ID。
    ///
    /// `job_family_id`: 「飞书管理后台」适用的序列 ID，通过[「获取租户序列列表」](https://open.feishu.cn/document/server-docs/contact-v3/job_family/list)接口获取
    #[api(
        kind = "query",
        name = "job_family_id_type",
        v_type = "var",
        option = "false"
    )]
    pub job_family_id_type: String,
    /// 此次调用中使用的「人员类型 ID」的类型
    ///
    /// **示例值**: "1"
    ///
    /// **可选值**:
    ///
    /// `people_admin_employee_type_id`: 「人力系统管理后台」适用的人员类型 ID。人力系统管理后台逐步下线中，建议不继续使用此 ID。
    ///
    /// `employee_type_enum_id`: 「飞书管理后台」适用的人员类型 ID，通过[「查询人员类型」](https://open.feishu.cn/document/server-docs/contact-v3/employee_type_enum/list)接口获取
    #[api(
        kind = "query",
        name = "employee_type_id_type",
        v_type = "var",
        option = "false"
    )]
    pub employee_type_id_type: String,
    /// Offer 申请表模板 ID，用于描述申请表单结构的元数据定义，即对申请表内容的描述。用户每一次更改 Offer 申请表模板信息，都会生成新的 schema_id，创建 Offer 时应传入最新的 schema_id，可从「获取Offer申请表模板信息」接口中获取
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "7013318077945596204"
    #[api(kind = "body", name = "schema_id")]
    pub schema_id: String,
    /// Offer 基本信息
    ///
    /// **是否必填**: 是
    #[api(kind = "body", name = "basic_info")]
    pub basic_info: OfferBasicInfoSubReq,
    /// Offer 薪资信息
    #[api(kind = "body", name = "salary_info")]
    pub salary_info: Option<OfferSalaryInfoSubReq>,
    /// 自定义信息
    #[api(kind = "body", name = "customized_info_list")]
    pub customized_info_list: Vec<Option<OfferCustomizedInfoSubReq>>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(default)]
pub struct OfferBasicInfoSubReq {
    /// 部门 ID
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "od-6b394871807047c7023ebfc1ff37cd3a"
    #[serde(
        rename = "department_id",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub department_id: String,
    /// 直属上级 ID
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "ou_ce613028fe74745421f5dc320bb9c709"
    #[serde(
        rename = "leader_user_id",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub leader_user_id: String,
    /// 职务 ID
    ///
    /// **示例值**: "123"
    #[serde(
        rename = "employment_job_id",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub employment_job_id: Option<String>,
    /// 人员类型 ID
    ///
    /// **示例值**: "2"
    #[serde(
        rename = "employee_type_id",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub employee_type_id: Option<String>,
    /// 职位序列 ID
    ///
    /// **示例值**: "6807407987381831949"
    #[serde(
        rename = "job_family_id",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub job_family_id: Option<String>,
    /// 职位级别 ID
    ///
    /// **示例值**: "6807407987381881101"
    #[serde(
        rename = "job_level_id",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub job_level_id: Option<String>,
    /// 试用期
    ///
    /// **示例值**: "3"
    #[serde(
        rename = "probation_month",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub probation_month: Option<i64>,
    /// 合同期(年)，推荐使用「contract_period」，如果Offer申请表中「合同期(年)」字段已停用，则不可使用该字段
    ///
    /// **示例值**: "3"
    #[serde(
        rename = "contract_year",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub contract_year: Option<i64>,
    /// 合同期（年/月）
    #[serde(
        rename = "contract_period",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub contract_period: Option<ContractPeriodInfoSubReq>,
    /// 预计入职日期
    ///
    /// **示例值**: "{\"date\":\"2022-04-07\"}"
    #[serde(
        rename = "expected_onboard_date",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub expected_onboard_date: Option<String>,
    /// 入职地点 ID
    ///
    /// **示例值**: "6897079709306259719"
    #[serde(
        rename = "onboard_address_id",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub onboard_address_id: Option<String>,
    /// 办公地点 ID
    ///
    /// **示例值**: "6897079709306259719"
    #[serde(
        rename = "work_address_id",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub work_address_id: Option<String>,
    /// Offer负责人 ID
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "ou_ce613028fe74745421f5dc320bb9c709"
    #[serde(
        rename = "owner_user_id",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub owner_user_id: String,
    /// Offer 推荐语
    ///
    /// **示例值**: "十分优秀，推荐入职"
    #[serde(
        rename = "recommended_words",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub recommended_words: Option<String>,
    /// 招聘需求 ID
    ///
    /// **示例值**: "2342352224"
    #[serde(
        rename = "job_requirement_id",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub job_requirement_id: Option<String>,
    /// 招聘流程类型 ID
    ///
    /// **示例值**: "2"
    #[serde(
        rename = "job_process_type_id",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub job_process_type_id: Option<i64>,
    /// 附件ID列表
    ///
    /// **示例值**: "["7081582717280831752"]"
    #[serde(
        rename = "attachment_id_list",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub attachment_id_list: Vec<Option<String>>,
    /// 附件描述
    ///
    /// **示例值**: "张三的简历"
    #[serde(
        rename = "attachment_description",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub attachment_description: Option<String>,
    /// Offer操作人 ID
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "ou_ce613028fe74745421f5dc320bb9c709"
    #[serde(
        rename = "operator_user_id",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub operator_user_id: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(default)]
pub struct OfferSalaryInfoSubReq {
    /// 币种
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "CNY"
    #[serde(
        rename = "currency",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub currency: String,
    /// 基本工资，当启用 Offer 申请表中的「薪资信息」模块时，「基本工资」字段为必传项
    ///
    /// **示例值**: "1000000"
    #[serde(
        rename = "basic_salary",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub basic_salary: Option<String>,
    /// 试用期百分比
    ///
    /// **示例值**: "0.8"
    #[serde(
        rename = "probation_salary_percentage",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub probation_salary_percentage: Option<String>,
    /// 年终奖月数
    ///
    /// **示例值**: "3"
    #[serde(
        rename = "award_salary_multiple",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub award_salary_multiple: Option<String>,
    /// 期权股数
    ///
    /// **示例值**: "30"
    #[serde(
        rename = "option_shares",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub option_shares: Option<String>,
    /// 季度奖金额
    ///
    /// **示例值**: "3000"
    #[serde(
        rename = "quarterly_bonus",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub quarterly_bonus: Option<String>,
    /// 半年奖金额
    ///
    /// **示例值**: "10000"
    #[serde(
        rename = "half_year_bonus",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub half_year_bonus: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(default)]
pub struct ContractPeriodInfoSubReq {
    /// 合同周期类型
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "1"
    ///
    /// **可选值**:
    ///
    /// `month`: 月
    ///
    /// `year`: 年
    #[serde(
        rename = "period_type",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub period_type: i64,
    /// 合同时长
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "3"
    #[serde(
        rename = "period",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub period: i64,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(default)]
pub struct OfferCustomizedInfoSubReq {
    /// 自定义字段 ID
    ///
    /// **示例值**: "6972464088568269100"
    #[serde(
        rename = "id",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub id: Option<String>,
    /// 自定义字段信息，以字符串形式传入，如：
    ///
    /// 1. 单选： "1"
    ///
    /// 2. 多选："[\"1\", \"2\"]"
    ///
    /// 3. 日期："{"date":"2022-01-01"}"
    ///
    /// 4. 年份选择："{"date":"2022"}"
    ///
    /// 5. 月份选择："{"date":"2022-01"}"
    ///
    /// 6. 单行文本："xxx "
    ///
    /// 7. 多行文本："xxx "
    ///
    /// 8. 数字："123"
    ///
    /// 9. 金额："123.1"
    ///
    /// **示例值**: "1"
    #[serde(
        rename = "value",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub value: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, lark_bot_sdk_macros::ApiBaseResp)]
struct UpdateHireOfferRespInner {
    #[serde(flatten)]
    data: Option<UpdateHireOfferResp>,
    #[serde(flatten)]
    base: BaseResp,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct UpdateHireOfferResp {
    /// \-
    #[serde(
        rename = "data",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub data: OfferInfoSubResp,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(default)]
pub struct OfferInfoSubResp {
    /// Offer ID
    ///
    /// **示例值**: "7016605170635213100"
    #[serde(
        rename = "offer_id",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub offer_id: String,
    /// 模板 ID
    ///
    /// **示例值**: "7013318077945596204"
    #[serde(
        rename = "schema_id",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub schema_id: String,
    /// Offer 基本信息
    #[serde(
        rename = "basic_info",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub basic_info: OfferBasicInfoSubResp,
    /// Offer 薪资信息
    #[serde(
        rename = "salary_info",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub salary_info: OfferSalaryInfoSubResp,
    /// 自定义信息
    #[serde(
        rename = "customized_info_list",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub customized_info_list: Vec<OfferCustomizedInfoSubResp>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(default)]
pub struct OfferBasicInfoSubResp {
    /// 部门 ID
    ///
    /// **示例值**: "od-6b394871807047c7023ebfc1ff37cd3a"
    #[serde(
        rename = "department_id",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub department_id: String,
    /// 直属上级 ID
    ///
    /// **示例值**: "ou_ce613028fe74745421f5dc320bb9c709"
    #[serde(
        rename = "leader_user_id",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub leader_user_id: String,
    /// 职务 ID
    ///
    /// **示例值**: "123"
    #[serde(
        rename = "employment_job_id",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub employment_job_id: String,
    /// 人员类型 ID
    ///
    /// **示例值**: "2"
    #[serde(
        rename = "employee_type_id",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub employee_type_id: String,
    /// 职位序列 ID
    ///
    /// **示例值**: "6807407987381831949"
    #[serde(
        rename = "job_family_id",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub job_family_id: String,
    /// 职位级别 ID
    ///
    /// **示例值**: "6807407987381881101"
    #[serde(
        rename = "job_level_id",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub job_level_id: String,
    /// 试用期
    ///
    /// **示例值**: "3"
    #[serde(
        rename = "probation_month",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub probation_month: i64,
    /// 合同期(年)，推荐使用「contract_period」，如果Offer申请表中「合同期(年)」字段已停用，则不可使用该字段
    ///
    /// **示例值**: "3"
    #[serde(
        rename = "contract_year",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub contract_year: i64,
    /// 合同期（年/月）
    #[serde(
        rename = "contract_period",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub contract_period: ContractPeriodInfoSubResp,
    /// 预计入职日期
    ///
    /// **示例值**: "{"date":"2022-04-07"}"
    #[serde(
        rename = "expected_onboard_date",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub expected_onboard_date: String,
    /// 入职地点 ID
    ///
    /// **示例值**: "6897079709306259719"
    #[serde(
        rename = "onboard_address_id",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub onboard_address_id: String,
    /// 办公地点 ID
    ///
    /// **示例值**: "6897079709306259719"
    #[serde(
        rename = "work_address_id",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub work_address_id: String,
    /// Offer负责人 ID
    ///
    /// **示例值**: "ou_ce613028fe74745421f5dc320bb9c709"
    #[serde(
        rename = "owner_user_id",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub owner_user_id: String,
    /// Offer 推荐语
    ///
    /// **示例值**: "十分优秀，推荐入职"
    #[serde(
        rename = "recommended_words",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub recommended_words: String,
    /// 招聘需求 ID
    ///
    /// **示例值**: "2342352224"
    #[serde(
        rename = "job_requirement_id",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub job_requirement_id: String,
    /// 招聘流程类型 ID
    ///
    /// **示例值**: "2"
    #[serde(
        rename = "job_process_type_id",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub job_process_type_id: i64,
    /// 附件ID列表
    #[serde(
        rename = "attachment_id_list",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub attachment_id_list: Vec<String>,
    /// 附件描述
    ///
    /// **示例值**: "张三的简历"
    #[serde(
        rename = "attachment_description",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub attachment_description: String,
    /// Offer操作人 ID
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "ou_ce613028fe74745421f5dc320bb9c709"
    #[serde(
        rename = "operator_user_id",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub operator_user_id: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(default)]
pub struct OfferSalaryInfoSubResp {
    /// 币种
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "CNY"
    #[serde(
        rename = "currency",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub currency: String,
    /// 基本薪资
    ///
    /// **示例值**: "1000000"
    #[serde(
        rename = "basic_salary",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub basic_salary: String,
    /// 试用期百分比
    ///
    /// **示例值**: "0.8"
    #[serde(
        rename = "probation_salary_percentage",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub probation_salary_percentage: String,
    /// 年终奖月数
    ///
    /// **示例值**: "3"
    #[serde(
        rename = "award_salary_multiple",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub award_salary_multiple: String,
    /// 期权股数
    ///
    /// **示例值**: "11"
    #[serde(
        rename = "option_shares",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub option_shares: String,
    /// 季度奖金额
    ///
    /// **示例值**: "12"
    #[serde(
        rename = "quarterly_bonus",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub quarterly_bonus: String,
    /// 半年奖金额
    ///
    /// **示例值**: "16"
    #[serde(
        rename = "half_year_bonus",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub half_year_bonus: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(default)]
pub struct ContractPeriodInfoSubResp {
    /// 合同周期类型
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "1"
    ///
    /// **可选值**:
    ///
    /// `month`: 月
    ///
    /// `year`: 年
    #[serde(
        rename = "period_type",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub period_type: i64,
    /// 合同时长
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "3"
    #[serde(
        rename = "period",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub period: i64,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(default)]
pub struct OfferCustomizedInfoSubResp {
    /// 自定义字段 ID
    ///
    /// **示例值**: "6972464088568269100"
    #[serde(
        rename = "id",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub id: String,
    /// 自定义字段信息
    ///
    /// **示例值**: "1"
    #[serde(
        rename = "value",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub value: String,
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
        Fn(UpdateHireOfferReq) -> Result<(UpdateHireOfferResp, CommonResponse), Error>
        + Send
        + Sync
        + 'static
    {
    }
    impl<
            T: Fn(UpdateHireOfferReq) -> Result<(UpdateHireOfferResp, CommonResponse), Error>
                + Send
                + Sync
                + 'static,
        > MockFunc for T
    {
    }

    impl<'c, IStore: Store, IClient: HttpClient> HireServiceMocker<'c, IStore, IClient> {
        pub fn mock_update_hire_offer<F: MockFunc>(
            &self,
            f: F,
        ) -> MockerBuilder<Mocker, UpdateHireOfferReq, UpdateHireOfferResp, Arc<dyn MockFunc>>
        {
            MockerBuilder::new(self.cli.instance_id, Arc::new(f))
        }

        pub(super) fn get_mock_update_hire_offer(
            &self,
            req: &UpdateHireOfferReq,
        ) -> Option<Arc<dyn MockFunc>> {
            do_mock::<Mocker, UpdateHireOfferReq, UpdateHireOfferResp, Arc<dyn MockFunc>>(
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
        api::gen::hire::update_hire_offer::{
            UpdateHireOfferReq, UpdateHireOfferResp, UpdateHireOfferRespInner,
        },
        core::{model::CommonResponse, Lark},
    };

    #[tokio::test]
    async fn test_mock() {
        let lark = Lark::new("".to_owned(), "".into());
        let mocker = lark
            .hire()
            .mock()
            .mock_update_hire_offer(|_| {
                Ok((UpdateHireOfferResp::default(), CommonResponse::default()))
            })
            .build();
        let res = lark
            .hire()
            .update_hire_offer(UpdateHireOfferReq::default())
            .await;
        assert!(res.is_ok());
        mocker.clear();
        let res = lark
            .hire()
            .update_hire_offer(UpdateHireOfferReq::default())
            .await;
        assert!(res.is_err());
    }

    const REQ: &str = r#"{
    "schema_id": "7013318077945596204",
    "basic_info": {
        "department_id": "od-6b394871807047c7023ebfc1ff37cd3a",
        "leader_user_id": "ou_ce613028fe74745421f5dc320bb9c709",
        "employment_job_id": "123",
        "employee_type_id": "2",
        "job_family_id": "6807407987381831949",
        "job_level_id": "6807407987381881101",
        "probation_month": 3,
        "contract_year": 3,
        "contract_period": {
            "period_type": 1,
            "period": 3
        },
        "expected_onboard_date": "{\"date\":\"2022-04-07\"}",
        "onboard_address_id": "6897079709306259719",
        "work_address_id": "6897079709306259719",
        "owner_user_id": "ou_ce613028fe74745421f5dc320bb9c709",
        "recommended_words": "十分优秀，推荐入职",
        "job_requirement_id": "2342352224",
        "job_process_type_id": 2,
        "attachment_id_list": [
            "7159169181052061965"
        ],
        "attachment_description": "张三的简历",
        "operator_user_id": "ou_ce613028fe74745421f5dc320bb9c709"
    },
    "salary_info": {
        "currency": "CNY",
        "basic_salary": "1000000",
        "probation_salary_percentage": "0.8",
        "award_salary_multiple": "3",
        "option_shares": "30",
        "quarterly_bonus": "3000",
        "half_year_bonus": "10000"
    },
    "customized_info_list": [
        {
            "id": "6972464088568269100",
            "value": "1"
        }
    ]
}"#;

    #[test]
    fn test_req() {
        if REQ == "{}" {
            return;
        }
        if let Err(e) = serde_json::from_str::<super::UpdateHireOfferReqBody>(REQ) {
            panic!("{}", e);
        }
    }

    const RESP: &str = r#"{
        "code": 0,
        "msg": "success",
        "data": {
            "offer_id": "7016605170635213100",
            "schema_id": "7013318077945596204",
            "basic_info": {
                "department_id": "od-6b394871807047c7023ebfc1ff37cd3a",
                "leader_user_id": "ou_ce613028fe74745421f5dc320bb9c709",
                "employment_job_id": "123",
                "employee_type_id": "2",
                "job_family_id": "6807407987381831949",
                "job_level_id": "6807407987381881101",
                "probation_month": 3,
                "contract_year": 3,
                "contract_period": {
                    "period_type": 1,
                    "period": 3
                },
                "expected_onboard_date": "{\"date\":\"2022-04-07\"}",
                "onboard_address_id": "6897079709306259719",
                "work_address_id": "6897079709306259719",
                "owner_user_id": "ou_ce613028fe74745421f5dc320bb9c709",
                "recommended_words": "十分优秀，推荐入职",
                "job_requirement_id": "2342352224",
                "job_process_type_id": 2,
                "attachment_id_list": [
                    "7159169181052061965"
                ],
                "attachment_description": "张三的简历",
                "operator_user_id": "ou_ce613028fe74745421f5dc320bb9c709"
            },
            "salary_info": {
                "currency": "CNY",
                "basic_salary": "1000000",
                "probation_salary_percentage": "0.8",
                "award_salary_multiple": "3",
                "option_shares": "11",
                "quarterly_bonus": "12",
                "half_year_bonus": "16"
            },
            "customized_info_list": [
                {
                    "id": "6972464088568269100",
                    "value": "1"
                }
            ]
        }
    }"#;
    #[test]
    fn test_resp() {
        let res = serde_json::from_str::<UpdateHireOfferRespInner>(RESP);
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
