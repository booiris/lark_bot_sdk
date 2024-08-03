//! doc url: <https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/corehr-v1/contract/list>
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
    /// **api 版本: 2024-07-23T06:24:19+00:00**
    ///
    /// ## 批量查询合同
    ///
    /// 通过接口可以批量查询合同列表信息，目前暂不支持其他筛选条件且一次查询最多支持50条数据。
    ///
    /// 如果你只需要单一合同查询场景，建议通过[【查询单个合同接口】](/uAjLw4CM/ukTMukTMukTM/reference/corehr-v1/contract/get)获取合同信息。
    ///
    /// doc: <https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/corehr-v1/contract/list>
    ///
    /// new doc: <https://open.feishu.cn/document/server-docs/corehr-v1/contract/list>
    ///
    /// api url: <https://open.feishu.cn/document_portal/v1/document/get_detail?fullPath=%2Fserver-docs%2Fcorehr-v1%2Fcontract%2Flist>
    pub async fn get_core_hr_contract_list(
        &self,
        req: GetCoreHrContractListReq,
    ) -> Result<(GetCoreHrContractListResp, CommonResponse), Error> {
        self.get_core_hr_contract_list_with_opt(req, Default::default())
            .await
    }

    /// 参见 [get_core_hr_contract_list](#method.get_core_hr_contract_list) 函数
    pub async fn get_core_hr_contract_list_with_opt(
        &self,
        req: GetCoreHrContractListReq,
        method_option: MethodOption,
    ) -> Result<(GetCoreHrContractListResp, CommonResponse), Error> {
        #[cfg(feature = "test-util")]
        {
            if let Some(f) = self.mock().get_mock_get_core_hr_contract_list(&req) {
                tracing::info!("[lark] CoreHr#GetCoreHrContractList **mocking** api");
                return f(req);
            }
        }

        tracing::info!("[lark] CoreHr#GetCoreHrContractList call api");

        let req = ApiRequest {
            scope: "CoreHr",
            api: "GetCoreHrContractList",
            method: http::Method::GET,
            url: String::new() + self.cli.open_base_url.as_ref() + "/open-apis/corehr/v1/contracts",
            param_data: req.gen_param(),
            method_option,
            need_tenant_access_token: true,
            ..Default::default()
        };

        let (resp, common_resp): (GetCoreHrContractListRespInner, _) = self.cli.do_req(req).await?;
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
pub struct GetCoreHrContractListReq {
    /// 分页标记，第一次请求不填，表示从头开始遍历；分页查询结果还有更多项时会同时返回新的 page_token，下次遍历可采用该 page_token 获取查询结果
    ///
    /// **示例值**: "10"
    #[api(kind = "query", name = "page_token", v_type = "var", option = "false")]
    pub page_token: String,
    /// 分页大小；范围：0～50
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "10"
    #[api(kind = "query", name = "page_size", v_type = "var", option = "false")]
    pub page_size: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, lark_bot_sdk_macros::ApiBaseResp)]
struct GetCoreHrContractListRespInner {
    #[serde(flatten)]
    data: Option<GetCoreHrContractListResp>,
    #[serde(flatten)]
    base: BaseResp,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct GetCoreHrContractListResp {
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
    /// 查询的合同信息
    #[serde(
        rename = "items",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub items: Vec<ContractSubResp>,
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
    /// 雇佣 ID，详细信息可通过[【查询员工信息接口】](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/corehr-v2/employee/search)获取
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
    /// 合同主体ID,  详细信息可通过[【查询公司详情接口】](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/corehr-v1/company/get)接口查询获得
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
    /// 合同协议状态，枚举值可通过文档[【飞书人事枚举常量】](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/corehr-v1/feishu-people-enum-constant)合同协议状态（contract_status）枚举定义部分获得
    #[serde(
        rename = "contract_status",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub contract_status: EnumSubResp,
    /// 续签状态，枚举值可通过文档[【飞书人事枚举常量】](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/corehr-v1/feishu-people-enum-constant)续签状态（renewal_status）枚举定义部分获得
    #[serde(
        rename = "renewal_status",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub renewal_status: EnumSubResp,
    /// 第几次签署
    ///
    /// **示例值**: "1"
    #[serde(
        rename = "signing_times",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub signing_times: i64,
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
        Fn(GetCoreHrContractListReq) -> Result<(GetCoreHrContractListResp, CommonResponse), Error>
        + Send
        + Sync
        + 'static
    {
    }
    impl<
            T: Fn(
                    GetCoreHrContractListReq,
                ) -> Result<(GetCoreHrContractListResp, CommonResponse), Error>
                + Send
                + Sync
                + 'static,
        > MockFunc for T
    {
    }

    impl<'c, IStore: Store, IClient: HttpClient> CoreHrServiceMocker<'c, IStore, IClient> {
        pub fn mock_get_core_hr_contract_list<F: MockFunc>(
            &self,
            f: F,
        ) -> MockerBuilder<
            Mocker,
            GetCoreHrContractListReq,
            GetCoreHrContractListResp,
            Arc<dyn MockFunc>,
        > {
            MockerBuilder::new(self.cli.instance_id, Arc::new(f))
        }

        pub(super) fn get_mock_get_core_hr_contract_list(
            &self,
            req: &GetCoreHrContractListReq,
        ) -> Option<Arc<dyn MockFunc>> {
            do_mock::<Mocker, GetCoreHrContractListReq, GetCoreHrContractListResp, Arc<dyn MockFunc>>(
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
        api::gen::core_hr::get_core_hr_contract_list::{
            GetCoreHrContractListReq, GetCoreHrContractListResp, GetCoreHrContractListRespInner,
        },
        core::{model::CommonResponse, Lark},
    };

    #[tokio::test]
    async fn test_mock() {
        let lark = Lark::new("".to_owned(), "".into());
        let mocker = lark
            .core_hr()
            .mock()
            .mock_get_core_hr_contract_list(|_| {
                Ok((
                    GetCoreHrContractListResp::default(),
                    CommonResponse::default(),
                ))
            })
            .build();
        let res = lark
            .core_hr()
            .get_core_hr_contract_list(GetCoreHrContractListReq::default())
            .await;
        assert!(res.is_ok());
        mocker.clear();
        let res = lark
            .core_hr()
            .get_core_hr_contract_list(GetCoreHrContractListReq::default())
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
                    "enum_name": "fix_term",
                    "display": [
                        {
                            "lang": "zh-CN",
                            "value": "固定期限类型"
                        }
                    ]
                },
                "contract_end_date": "2006-01-02",
                "contract_number": "6919737965274990093",
                "signing_type": {
                    "enum_name": "new",
                    "display": [
                        {
                            "lang": "zh-CN",
                            "value": "新签"
                        }
                    ]
                },
                "contract_status": {
                    "enum_name": "to_be_effective",
                    "display": [
                        {
                            "lang": "zh-CN",
                            "value": "待生效"
                        }
                    ]
                },
                "renewal_status": {
                    "enum_name": "rejected",
                    "display": [
                        {
                            "lang": "zh-CN",
                            "value": "已拒绝"
                        }
                    ]
                },
                "signing_times": 1
            }
        ],
        "has_more": true,
        "page_token": "1234452132"
    }
}"#;
    #[test]
    fn test_resp() {
        let res = serde_json::from_str::<GetCoreHrContractListRespInner>(RESP);
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
