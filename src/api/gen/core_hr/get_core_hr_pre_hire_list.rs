//! doc url: <https://open.larkoffice.com/document/uAjLw4CM/ukTMukTMukTM/reference/corehr-v1/pre_hire/list>
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
    /// **api 版本: 2024-07-15T11:20:02+00:00**
    ///
    /// ## 批量查询待入职信息
    ///
    /// 可通过本接口批量查询待入职人员信息，本接口不再推荐使用（个人信息相关数据不完整），请使用[查询待入职](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/corehr-v2/pre_hire/query)接口获取更完整信息。
    ///

    ///
    /// doc: <https://open.larkoffice.com/document/uAjLw4CM/ukTMukTMukTM/reference/corehr-v1/pre_hire/list>
    ///
    /// new doc: <https://open.larkoffice.com/document/server-docs/corehr-v1/pre_hire/list>
    ///
    /// api url: <https://open.feishu.cn/document_portal/v1/document/get_detail?fullPath=%2Fserver-docs%2Fcorehr-v1%2Fpre_hire%2Flist>
    pub async fn get_core_hr_pre_hire_list(
        &self,
        req: GetCoreHrPreHireListReq,
    ) -> Result<(GetCoreHrPreHireListResp, CommonResponse), Error> {
        self.get_core_hr_pre_hire_list_with_opt(req, Default::default())
            .await
    }

    /// 参见 [get_core_hr_pre_hire_list](#method.get_core_hr_pre_hire_list) 函数
    pub async fn get_core_hr_pre_hire_list_with_opt(
        &self,
        req: GetCoreHrPreHireListReq,
        method_option: MethodOption,
    ) -> Result<(GetCoreHrPreHireListResp, CommonResponse), Error> {
        #[cfg(feature = "test-util")]
        {
            if let Some(f) = self.mock().get_mock_get_core_hr_pre_hire_list(&req) {
                tracing::info!("[lark] CoreHr#GetCoreHrPreHireList **mocking** api");
                return f(req);
            }
        }

        tracing::info!("[lark] CoreHr#GetCoreHrPreHireList call api");

        let req = ApiRequest {
            scope: "CoreHr",
            api: "GetCoreHrPreHireList",
            method: http::Method::GET,
            url: String::new() + self.cli.open_base_url.as_ref() + "/open-apis/corehr/v1/pre_hires",
            param_data: req.gen_param(),
            method_option,
            need_tenant_access_token: true,
            ..Default::default()
        };

        let (resp, common_resp): (GetCoreHrPreHireListRespInner, _) = self.cli.do_req(req).await?;
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
pub struct GetCoreHrPreHireListReq {
    /// 分页标记，第一次请求不填，表示从头开始遍历；分页查询结果还有更多项时会同时返回新的 page_token，下次遍历可采用该 page_token 获取查询结果
    ///
    /// **示例值**: "1231231987"
    #[api(kind = "query", name = "page_token", v_type = "var", option = "false")]
    pub page_token: String,
    /// 分页大小，最大值100，最小值 1
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "100"
    #[api(kind = "query", name = "page_size", v_type = "var", option = "false")]
    pub page_size: String,
    /// 待入职ID列表，可通过[搜索待入职](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/corehr-v2/pre_hire/search)接口获取
    ///
    /// **示例值**: "7110120266637772332"
    ///
    /// **数据校验规则**：
    ///
    /// - **最大长度**: `10` 字符
    #[api(
        kind = "query",
        name = "pre_hire_ids",
        v_type = "list",
        option = "false"
    )]
    pub pre_hire_ids: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, lark_bot_sdk_macros::ApiBaseResp)]
struct GetCoreHrPreHireListRespInner {
    #[serde(flatten)]
    data: Option<GetCoreHrPreHireListResp>,
    #[serde(flatten)]
    base: BaseResp,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct GetCoreHrPreHireListResp {
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
    /// 查询的待入职信息
    #[serde(
        rename = "items",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub items: Vec<PreHireQuerySubResp>,
    /// 是否还有更多项
    ///
    /// **示例值**: "true"
    #[serde(
        rename = "has_more",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub has_more: bool,
    /// 分页标记，当 has_more 为 true 时，会同时返回新的 page_token，否则不返回 page_token
    ///
    /// **示例值**: "1234452132"
    #[serde(
        rename = "page_token",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub page_token: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(default)]
pub struct PreHireQuerySubResp {
    /// 招聘投递 ID ，可以通过[获取投递列表](https://open.feishu.cn/document/ukTMukTMukTM/uMzM1YjLzMTN24yMzUjN/hire-v1/application/list)接口获取
    ///
    /// **示例值**: "4719168654814483759"
    #[serde(
        rename = "ats_application_id",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub ats_application_id: String,
    /// 待入职ID，可从[待入职列表](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/corehr-v2/pre_hire/search)接口获取
    ///
    /// **示例值**: "154545454"
    #[serde(
        rename = "id",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub id: String,
    /// 入职日期，格式："YYYY-MM-DD"
    ///
    /// **示例值**: "2020-01-01"
    #[serde(
        rename = "hire_date",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub hire_date: String,
    /// 雇佣类型
    ///
    /// **是否必填**: 是
    #[serde(
        rename = "employee_type",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub employee_type: EnumSubResp,
    /// 人员编号
    ///
    /// **示例值**: "1245646"
    #[serde(
        rename = "worker_id",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub worker_id: String,
    /// 人员类型，可通过[【批量查询人员类型】](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/corehr-v1/employee_type/list)接口获取
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "正式"
    #[serde(
        rename = "employee_type_id",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub employee_type_id: String,
    /// 个人信息 ID
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "656464648662"
    #[serde(
        rename = "person_id",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub person_id: String,
    /// 自定义字段
    #[serde(
        rename = "custom_fields",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub custom_fields: Vec<ObjectFieldDataSubResp>,
    /// 入职状态
    ///
    /// - `preboarding`：待入职
    ///
    /// - `day_one`：准备就绪
    ///
    /// - `completed`：已完成
    ///
    /// - `withdrawn`：已撤销
    ///
    /// - `deleted`：已删除，对应的系统操作是将待入职人员回退至 Offer 沟通阶段
    ///
    /// **是否必填**: 是
    #[serde(
        rename = "onboarding_status",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub onboarding_status: EnumSubResp,
    /// 成本中心分摊信息
    #[serde(
        rename = "cost_center_rate",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub cost_center_rate: Vec<SupportCostCenterItemSubResp>,
    /// 工作邮箱
    #[serde(
        rename = "work_email_list",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub work_email_list: Vec<EmailSubResp>,
    /// 部门ID，可通过[【批量查询部门】](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/corehr-v2/department/batch_get)接口获取
    ///
    /// **示例值**: "656464648662"
    #[serde(
        rename = "department_id",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub department_id: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(default)]
pub struct EnumSubResp {
    /// 枚举值
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "type_1"
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
pub struct ObjectFieldDataSubResp {
    /// 字段名
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "name"
    #[serde(
        rename = "field_name",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub field_name: String,
    /// 字段值，是json转义后的字符串，根据元数据定义不同，字段格式不同(如123, 123.23, "true", [\"id1\",\"id2\"], "2006-01-02 15:04:05")
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "\"Sandy\""
    #[serde(
        rename = "value",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub value: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(default)]
pub struct SupportCostCenterItemSubResp {
    /// 支持的成本中心id，详细信息可通过[【搜索成本中心信息】](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/corehr-v2/cost_center/search)接口查询获得
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
pub struct EmailSubResp {
    /// 邮箱号
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "12456@test.com"
    #[serde(
        rename = "email",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub email: String,
    /// 是否为主要邮箱
    ///
    /// **示例值**: "true"
    #[serde(
        rename = "is_primary",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub is_primary: bool,
    /// 是否为公开邮箱
    ///
    /// **示例值**: "true"
    #[serde(
        rename = "is_public",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub is_public: bool,
    /// 邮箱用途，枚举值可通过文档[【飞书人事枚举常量】](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/corehr-v1/feishu-people-enum-constant)邮箱用途（email_usage）枚举定义获得
    #[serde(
        rename = "email_usage",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub email_usage: EnumSubResp,
    /// 自定义字段
    #[serde(
        rename = "custom_fields",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub custom_fields: Vec<ObjectFieldDataSubResp>,
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
        Fn(GetCoreHrPreHireListReq) -> Result<(GetCoreHrPreHireListResp, CommonResponse), Error>
        + Send
        + Sync
        + 'static
    {
    }
    impl<
            T: Fn(
                    GetCoreHrPreHireListReq,
                ) -> Result<(GetCoreHrPreHireListResp, CommonResponse), Error>
                + Send
                + Sync
                + 'static,
        > MockFunc for T
    {
    }

    impl<'c, IStore: Store, IClient: HttpClient> CoreHrServiceMocker<'c, IStore, IClient> {
        pub fn mock_get_core_hr_pre_hire_list<F: MockFunc>(
            &self,
            f: F,
        ) -> MockerBuilder<
            Mocker,
            GetCoreHrPreHireListReq,
            GetCoreHrPreHireListResp,
            Arc<dyn MockFunc>,
        > {
            MockerBuilder::new(self.cli.instance_id, Arc::new(f))
        }

        pub(super) fn get_mock_get_core_hr_pre_hire_list(
            &self,
            req: &GetCoreHrPreHireListReq,
        ) -> Option<Arc<dyn MockFunc>> {
            do_mock::<Mocker, GetCoreHrPreHireListReq, GetCoreHrPreHireListResp, Arc<dyn MockFunc>>(
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
        api::gen::core_hr::get_core_hr_pre_hire_list::{
            GetCoreHrPreHireListReq, GetCoreHrPreHireListResp, GetCoreHrPreHireListRespInner,
        },
        core::{model::CommonResponse, Lark},
    };

    #[tokio::test]
    async fn test_mock() {
        let lark = Lark::new("".to_owned(), "".into());
        let mocker = lark
            .core_hr()
            .mock()
            .mock_get_core_hr_pre_hire_list(|_| {
                Ok((
                    GetCoreHrPreHireListResp::default(),
                    CommonResponse::default(),
                ))
            })
            .build();
        let res = lark
            .core_hr()
            .get_core_hr_pre_hire_list(GetCoreHrPreHireListReq::default())
            .await;
        assert!(res.is_ok());
        mocker.clear();
        let res = lark
            .core_hr()
            .get_core_hr_pre_hire_list(GetCoreHrPreHireListReq::default())
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
        "items": [
            {
                "ats_application_id": "4719168654814483759",
                "id": "154545454",
                "hire_date": "2020-01-01",
                "employee_type": {
                    "enum_name": "type_1",
                    "display": [
                        {
                            "lang": "zh-CN",
                            "value": "张三"
                        }
                    ]
                },
                "worker_id": "1245646",
                "employee_type_id": "正式",
                "person_id": "656464648662",
                "custom_fields": [
                    {
                        "field_name": "name",
                        "value": "\"Sandy\""
                    }
                ],
                "onboarding_status": {
                    "enum_name": "type_1",
                    "display": [
                        {
                            "lang": "zh-CN",
                            "value": "张三"
                        }
                    ]
                },
                "cost_center_rate": [
                    {
                        "cost_center_id": "6950635856373745165",
                        "rate": 100
                    }
                ],
                "work_email_list": [
                    {
                        "email": "12456@test.com",
                        "is_primary": true,
                        "is_public": true,
                        "email_usage": {
                            "enum_name": "type_1",
                            "display": [
                                {
                                    "lang": "zh-CN",
                                    "value": "张三"
                                }
                            ]
                        },
                        "custom_fields": [
                            {
                                "field_name": "name",
                                "value": "\"Sandy\""
                            }
                        ]
                    }
                ],
                "department_id": "656464648662"
            }
        ],
        "has_more": true,
        "page_token": "1234452132"
    }
}"#;
    #[test]
    fn test_resp() {
        let res = serde_json::from_str::<GetCoreHrPreHireListRespInner>(RESP);
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
