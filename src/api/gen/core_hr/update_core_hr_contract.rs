//! doc url: <https://open.larkoffice.com/document/uAjLw4CM/ukTMukTMukTM/reference/corehr-v1/contract/patch>
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
    /// **api 版本: 2024-07-23T06:24:18+00:00**
    ///
    /// ## 更新合同
    ///
    /// 通过该接口可以更新员工合同相关信息，没有修改的参数会保留原值
    ///

    ///
    /// doc: <https://open.larkoffice.com/document/uAjLw4CM/ukTMukTMukTM/reference/corehr-v1/contract/patch>
    ///
    /// new doc: <https://open.larkoffice.com/document/server-docs/corehr-v1/contract/patch>
    ///
    /// api url: <https://open.feishu.cn/document_portal/v1/document/get_detail?fullPath=%2Fserver-docs%2Fcorehr-v1%2Fcontract%2Fpatch>
    pub async fn update_core_hr_contract(
        &self,
        req: UpdateCoreHrContractReq,
    ) -> Result<(UpdateCoreHrContractResp, CommonResponse), Error> {
        self.update_core_hr_contract_with_opt(req, Default::default())
            .await
    }

    /// 参见 [update_core_hr_contract](#method.update_core_hr_contract) 函数
    pub async fn update_core_hr_contract_with_opt(
        &self,
        req: UpdateCoreHrContractReq,
        method_option: MethodOption,
    ) -> Result<(UpdateCoreHrContractResp, CommonResponse), Error> {
        #[cfg(feature = "test-util")]
        {
            if let Some(f) = self.mock().get_mock_update_core_hr_contract(&req) {
                tracing::info!("[lark] CoreHr#UpdateCoreHrContract **mocking** api");
                return f(req);
            }
        }

        tracing::info!("[lark] CoreHr#UpdateCoreHrContract call api");

        let req = ApiRequest {
            scope: "CoreHr",
            api: "UpdateCoreHrContract",
            method: http::Method::PATCH,
            url: String::new()
                + self.cli.open_base_url.as_ref()
                + "/open-apis/corehr/v1/contracts/:contract_id",
            param_data: req.gen_param(),
            method_option,
            need_tenant_access_token: true,
            ..Default::default()
        };

        let (resp, common_resp): (UpdateCoreHrContractRespInner, _) = self.cli.do_req(req).await?;
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
pub struct UpdateCoreHrContractReq {
    /// 合同ID，该ID可以通过[【批量查询合同】](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/corehr-v1/contract/list)接口获取
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "7091849027838838316"
    #[api(kind = "path", name = "contract_id")]
    pub contract_id: String,
    /// 自定义值，根据client_token是否一致来判断是否为同一请求
    ///
    /// **示例值**: "227988d7-66da-4afb-9943-32e73d5cda8b"
    #[api(
        kind = "query",
        name = "client_token",
        v_type = "var",
        option = "false"
    )]
    pub client_token: String,
    /// 合同开始日期
    ///
    /// **示例值**: "2050-01-01 00:00:00"
    #[api(kind = "body", name = "effective_time")]
    pub effective_time: Option<String>,
    /// 合同实际结束日期，合同实际结束日期小于等于合同结束日期
    ///
    /// **示例值**: "9999-12-31 23:59:59"
    #[api(kind = "body", name = "expiration_time")]
    pub expiration_time: Option<String>,
    /// 雇员 ID，详细信息可通过[【搜索员工信息】](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/corehr-v2/employee/search)接口查询获得
    ///
    /// **示例值**: "6893013238632416776"
    #[api(kind = "body", name = "employment_id")]
    pub employment_id: Option<String>,
    /// 合同类型，枚举值可通过文档[【飞书人事枚举常量】](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/corehr-v1/feishu-people-enum-constant)合同类型（contract_type）枚举定义部分获得
    #[api(kind = "body", name = "contract_type")]
    pub contract_type: Option<EnumSubReq>,
    /// 甲方, 引用Company的ID，详细信息可通过[【查询单个公司】](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/corehr-v1/company/get)
    ///
    /// 接口查询获得
    ///
    /// **示例值**: "6892686614112241165"
    #[api(kind = "body", name = "first_party_company_id")]
    pub first_party_company_id: Option<String>,
    /// Person ID，详细信息可通过[【搜索员工信息】](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/corehr-v2/employee/search)接口查询获得
    ///
    /// **示例值**: "151515151"
    #[api(kind = "body", name = "person_id")]
    pub person_id: Option<String>,
    /// 自定义字段
    #[api(kind = "body", name = "custom_fields")]
    pub custom_fields: Vec<Option<ObjectFieldDataSubReq>>,
    /// 期限类型，枚举值可通过文档[【飞书人事枚举常量】](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/corehr-v1/feishu-people-enum-constant)合同期限类型（duration_type）枚举定义部分获得
    ///
    /// **示例值**: "fixed_term"
    #[api(kind = "body", name = "duration_type")]
    pub duration_type: Option<EnumSubReq>,
    /// 合同预计的结束日期
    ///
    /// **示例值**: "2006-01-02"
    #[api(kind = "body", name = "contract_end_date")]
    pub contract_end_date: Option<String>,
    /// 合同编号
    ///
    /// **示例值**: "6919737965274990093"
    #[api(kind = "body", name = "contract_number")]
    pub contract_number: Option<String>,
    /// 签订类型，枚举值可通过文档[【飞书人事枚举常量】](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/corehr-v1/feishu-people-enum-constant)签订类型（signing_type）枚举定义部分获得
    #[api(kind = "body", name = "signing_type")]
    pub signing_type: Option<EnumSubReq>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(default)]
pub struct EnumSubReq {
    /// 枚举值
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "labor_contract"
    #[serde(
        rename = "enum_name",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub enum_name: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(default)]
pub struct ObjectFieldDataSubReq {
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
    /// **示例值**: "Sandy"
    #[serde(
        rename = "value",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub value: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, lark_bot_sdk_macros::ApiBaseResp)]
struct UpdateCoreHrContractRespInner {
    #[serde(flatten)]
    data: Option<UpdateCoreHrContractResp>,
    #[serde(flatten)]
    base: BaseResp,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct UpdateCoreHrContractResp {
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
    /// 合同
    #[serde(
        rename = "contract",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub contract: ContractSubResp,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(default)]
pub struct ContractSubResp {
    /// 合同ID
    ///
    /// **示例值**: "6919737965274990093"
    #[serde(
        rename = "id",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub id: String,
    /// 合同开始日期
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "2050-01-01 00:00:00"
    #[serde(
        rename = "effective_time",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub effective_time: String,
    /// 实际结束日期
    ///
    /// **示例值**: "9999-12-31 23:59:59"
    #[serde(
        rename = "expiration_time",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub expiration_time: String,
    /// 雇员 ID，详细信息可通过[【批量查询员工信息】](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/corehr-v2/employee/batch_get)
    ///
    /// 接口查询获得
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "6893013238632416776"
    #[serde(
        rename = "employment_id",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub employment_id: String,
    /// 合同类型，枚举值可通过文档[【飞书人事枚举常量】](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/corehr-v1/feishu-people-enum-constant)合同类型（contract_type）枚举定义部分获得
    ///
    /// **是否必填**: 是
    #[serde(
        rename = "contract_type",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub contract_type: EnumSubResp,
    /// 合同主体,  详细信息可通过[【查询公司详情接口】](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/corehr-v1/company/get)接口查询获得
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "6892686614112241165"
    #[serde(
        rename = "first_party_company_id",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub first_party_company_id: String,
    /// Person ID，详细信息可通过接口文档[【查询个人信息接口】](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/corehr-v1/person/get)接口查询获得
    ///
    /// **示例值**: "151515151"
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
    /// 期限类型，枚举值可通过文档[【飞书人事枚举常量】](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/corehr-v1/feishu-people-enum-constant)合同期限类型（duration_type）枚举定义部分获得
    ///
    /// **示例值**: "fixed_term"
    #[serde(
        rename = "duration_type",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub duration_type: EnumSubResp,
    /// 合同结束日期
    ///
    /// **示例值**: "2006-01-02"
    #[serde(
        rename = "contract_end_date",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub contract_end_date: String,
    /// 合同编号
    ///
    /// **示例值**: "6919737965274990093"
    #[serde(
        rename = "contract_number",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub contract_number: String,
    /// 签订类型，枚举值可通过文档[【飞书人事枚举常量】](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/corehr-v1/feishu-people-enum-constant)签订类型（signing_type）枚举定义部分获得
    #[serde(
        rename = "signing_type",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub signing_type: EnumSubResp,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(default)]
pub struct EnumSubResp {
    /// 枚举值
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "labor_contract"
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
    /// **示例值**: "Sandy"
    #[serde(
        rename = "value",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub value: String,
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
    /// **示例值**: "劳动合同"
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
        Fn(UpdateCoreHrContractReq) -> Result<(UpdateCoreHrContractResp, CommonResponse), Error>
        + Send
        + Sync
        + 'static
    {
    }
    impl<
            T: Fn(
                    UpdateCoreHrContractReq,
                ) -> Result<(UpdateCoreHrContractResp, CommonResponse), Error>
                + Send
                + Sync
                + 'static,
        > MockFunc for T
    {
    }

    impl<'c, IStore: Store, IClient: HttpClient> CoreHrServiceMocker<'c, IStore, IClient> {
        pub fn mock_update_core_hr_contract<F: MockFunc>(
            &self,
            f: F,
        ) -> MockerBuilder<
            Mocker,
            UpdateCoreHrContractReq,
            UpdateCoreHrContractResp,
            Arc<dyn MockFunc>,
        > {
            MockerBuilder::new(self.cli.instance_id, Arc::new(f))
        }

        pub(super) fn get_mock_update_core_hr_contract(
            &self,
            req: &UpdateCoreHrContractReq,
        ) -> Option<Arc<dyn MockFunc>> {
            do_mock::<Mocker, UpdateCoreHrContractReq, UpdateCoreHrContractResp, Arc<dyn MockFunc>>(
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
        api::gen::core_hr::update_core_hr_contract::{
            UpdateCoreHrContractReq, UpdateCoreHrContractResp, UpdateCoreHrContractRespInner,
        },
        core::{model::CommonResponse, Lark},
    };

    #[tokio::test]
    async fn test_mock() {
        let lark = Lark::new("".to_owned(), "".into());
        let mocker = lark
            .core_hr()
            .mock()
            .mock_update_core_hr_contract(|_| {
                Ok((
                    UpdateCoreHrContractResp::default(),
                    CommonResponse::default(),
                ))
            })
            .build();
        let res = lark
            .core_hr()
            .update_core_hr_contract(UpdateCoreHrContractReq::default())
            .await;
        assert!(res.is_ok());
        mocker.clear();
        let res = lark
            .core_hr()
            .update_core_hr_contract(UpdateCoreHrContractReq::default())
            .await;
        assert!(res.is_err());
    }

    const REQ: &str = r#"{
    "effective_time": "2050-01-01 00:00:00",
    "expiration_time": "9999-12-31 23:59:59",
    "employment_id": "6893013238632416776",
    "contract_type": {
        "enum_name": "labor_contract"
    },
    "first_party_company_id": "6892686614112241165",
    "person_id": "151515151",
    "custom_fields": [
        {
            "field_name": "name",
            "value": "Sandy"
        }
    ],
    "duration_type": {
        "enum_name": "open_ended"
    },
    "contract_end_date": "2006-01-02",
    "contract_number": "6919737965274990093",
    "signing_type": {
        "enum_name": "new"
    }
}"#;

    #[test]
    fn test_req() {
        if REQ == "{}" {
            return;
        }
        if let Err(e) = serde_json::from_str::<super::UpdateCoreHrContractReqBody>(REQ) {
            panic!("{}", e);
        }
    }

    const RESP: &str = r#"{
    "code": 0,
    "msg": "success",
    "data": {
        "contract": {
            "id": "6919737965274990093",
            "effective_time": "2050-01-01 00:00:00",
            "expiration_time": "9999-12-31 23:59:59",
            "employment_id": "6893013238632416776",
            "contract_type": {
                "enum_name": "labor_contract",
                "display": [
                    {
                        "lang": "zh-CN",
                        "value": "劳动合同"
                    }
                ]
            },
            "first_party_company_id": "6892686614112241165",
            "person_id": "151515151",
            "custom_fields": [
                {
                    "field_name": "name",
                    "value": "Sandy"
                }
            ],
            "duration_type": {
                "enum_name": "fixed_term",
                "display": [
                    {
                        "lang": "zh-CN",
                        "value": "固定期限类型合同"
                    }
                ]
            },
            "contract_end_date": "2006-01-02",
            "contract_number": "6919737965274990093",
            "signing_type": {
                "enum_name": "changed",
                "display": [
                    {
                        "lang": "zh-CN",
                        "value": "变更"
                    }
                ]
            }
        }
    }
}"#;
    #[test]
    fn test_resp() {
        let res = serde_json::from_str::<UpdateCoreHrContractRespInner>(RESP);
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
