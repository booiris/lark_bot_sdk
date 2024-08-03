//! doc url: <https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/corehr-v2/company/batch_get>
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
    /// **api 版本: 2024-02-06T02:23:53+00:00**
    ///
    /// ## 通过公司 ID 批量获取公司信息
    ///
    /// 通过公司 ID 批量获取公司信息
    ///

    ///
    /// doc: <https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/corehr-v2/company/batch_get>
    ///
    /// new doc: <https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/corehr-v2/company/batch_get>
    ///
    /// api url: <https://open.feishu.cn/document_portal/v1/document/get_detail?fullPath=%2FuAjLw4CM%2FukTMukTMukTM%2Fcorehr-v2%2Fcompany%2Fbatch_get>
    pub async fn batch_get_core_hr_company(
        &self,
        req: BatchGetCoreHrCompanyReq,
    ) -> Result<(BatchGetCoreHrCompanyResp, CommonResponse), Error> {
        self.batch_get_core_hr_company_with_opt(req, Default::default())
            .await
    }

    /// 参见 [batch_get_core_hr_company](#method.batch_get_core_hr_company) 函数
    pub async fn batch_get_core_hr_company_with_opt(
        &self,
        req: BatchGetCoreHrCompanyReq,
        method_option: MethodOption,
    ) -> Result<(BatchGetCoreHrCompanyResp, CommonResponse), Error> {
        #[cfg(feature = "test-util")]
        {
            if let Some(f) = self.mock().get_mock_batch_get_core_hr_company(&req) {
                tracing::info!("[lark] CoreHr#BatchGetCoreHrCompany **mocking** api");
                return f(req);
            }
        }

        tracing::info!("[lark] CoreHr#BatchGetCoreHrCompany call api");

        let req = ApiRequest {
            scope: "CoreHr",
            api: "BatchGetCoreHrCompany",
            method: http::Method::POST,
            url: String::new()
                + self.cli.open_base_url.as_ref()
                + "/open-apis/corehr/v2/companies/batch_get",
            param_data: req.gen_param(),
            method_option,
            need_tenant_access_token: true,
            ..Default::default()
        };

        let (resp, common_resp): (BatchGetCoreHrCompanyRespInner, _) = self.cli.do_req(req).await?;
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
pub struct BatchGetCoreHrCompanyReq {
    /// 公司 ID 列表
    ///
    /// **是否必填**: 是
    ///
    /// **数据校验规则**：
    ///
    /// - **长度范围**: `1` 字符- `100` 字符
    #[api(kind = "body", name = "company_ids")]
    pub company_ids: Vec<Option<String>>,
}

#[derive(Debug, Serialize, Deserialize, Clone, lark_bot_sdk_macros::ApiBaseResp)]
struct BatchGetCoreHrCompanyRespInner {
    #[serde(flatten)]
    data: Option<BatchGetCoreHrCompanyResp>,
    #[serde(flatten)]
    base: BaseResp,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct BatchGetCoreHrCompanyResp {
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
    /// 查询的公司信息
    #[serde(
        rename = "items",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub items: Vec<CompanySubResp>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(default)]
pub struct CompanySubResp {
    /// 公司 ID
    ///
    /// **示例值**: "4692472714243080020"
    #[serde(
        rename = "company_id",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub company_id: String,
    /// 公司基本信息
    ///
    /// **是否必填**: 是
    #[serde(
        rename = "hiberarchy_common",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub hiberarchy_common: HiberarchyCommonSubResp,
    /// 性质
    #[serde(
        rename = "type",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub body_type: EnumSubResp,
    /// 行业
    #[serde(
        rename = "industry_list",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub industry_list: Vec<EnumSubResp>,
    /// 法定代表人
    #[serde(
        rename = "legal_representative",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub legal_representative: Vec<I18nSubResp>,
    /// 邮编
    ///
    /// **示例值**: "邮编"
    #[serde(
        rename = "post_code",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub post_code: String,
    /// 纳税人识别号
    ///
    /// **示例值**: "123456840"
    #[serde(
        rename = "tax_payer_id",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub tax_payer_id: String,
    /// 是否保密
    ///
    /// **示例值**: "true"
    #[serde(
        rename = "confidential",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub confidential: bool,
    /// 主体类型
    #[serde(
        rename = "sub_type_list",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub sub_type_list: Vec<EnumSubResp>,
    /// 是否为分公司
    ///
    /// **示例值**: "true"
    #[serde(
        rename = "branch_company",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub branch_company: bool,
    /// 主要负责人
    #[serde(
        rename = "primary_manager",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub primary_manager: Vec<I18nSubResp>,
    /// 默认币种
    #[serde(
        rename = "currency",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub currency: CurrencySubResp,
    /// 电话
    #[serde(
        rename = "phone",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub phone: PhoneNumberAndAreaCodeSubResp,
    /// 传真
    #[serde(
        rename = "fax",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub fax: PhoneNumberAndAreaCodeSubResp,
    /// 注册地址
    #[serde(
        rename = "registered_office_address",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub registered_office_address: Vec<I18nSubResp>,
    /// 办公地址
    #[serde(
        rename = "office_address",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub office_address: Vec<I18nSubResp>,
    /// 自定义字段
    #[serde(
        rename = "custom_fields",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub custom_fields: Vec<CustomFieldDataSubResp>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(default)]
pub struct HiberarchyCommonSubResp {
    /// 上级
    ///
    /// **示例值**: "4719168654814483759"
    #[serde(
        rename = "parent_id",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub parent_id: String,
    /// 名称
    ///
    /// **是否必填**: 是
    #[serde(
        rename = "name",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub name: Vec<I18nSubResp>,
    /// 类型
    ///
    /// **是否必填**: 是
    #[serde(
        rename = "type",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub body_type: EnumSubResp,
    /// 启用
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "true"
    #[serde(
        rename = "active",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub active: bool,
    /// 生效时间
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
    /// 编码
    ///
    /// **示例值**: "12456"
    #[serde(
        rename = "code",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub code: String,
    /// 描述
    #[serde(
        rename = "description",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub description: Vec<I18nSubResp>,
    /// 树形排序
    ///
    /// **示例值**: "123"
    #[serde(
        rename = "tree_order",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub tree_order: String,
    /// 列表排序
    ///
    /// **示例值**: "123"
    #[serde(
        rename = "list_order",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub list_order: String,
    /// 自定义字段
    #[serde(
        rename = "custom_fields",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub custom_fields: Vec<ObjectFieldDataSubResp>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(default)]
pub struct EnumSubResp {
    /// 枚举值
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "phone_type"
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
pub struct CurrencySubResp {
    /// 货币 ID
    ///
    /// **示例值**: "6893114062142064111"
    #[serde(
        rename = "currency_id",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub currency_id: String,
    /// 货币所属国家/地区 ID，详细信息可通过[【查询国家/地区信息】](https://open.feishu.cn/document/server-docs/corehr-v1/basic-infomation/location_data/list)接口查询获得
    ///
    /// **示例值**: "6893114162142064111"
    #[serde(
        rename = "country_region_id",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub country_region_id: String,
    /// 货币名称
    #[serde(
        rename = "currency_name",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub currency_name: Vec<I18nSubResp>,
    /// 数字代码
    ///
    /// **示例值**: "156"
    #[serde(
        rename = "numeric_code",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub numeric_code: i64,
    /// 三位字母代码
    ///
    /// **示例值**: "CNY"
    #[serde(
        rename = "currency_alpha_3_code",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub currency_alpha_3_code: String,
    /// 状态
    ///
    /// **示例值**: "1"
    ///
    /// **可选值**:
    ///
    /// `Effective`: 生效
    ///
    /// `Expiration`: 失效
    #[serde(
        rename = "status",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub status: i64,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(default)]
pub struct PhoneNumberAndAreaCodeSubResp {
    /// 区号
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "123123"
    #[serde(
        rename = "area_code",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub area_code: EnumSubResp,
    /// 号码
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "213213"
    #[serde(
        rename = "phone_number",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub phone_number: String,
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

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(default)]
pub struct CustomFieldDataSubResp {
    /// 自定义字段 apiname，即自定义字段的唯一标识
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "name"
    #[serde(
        rename = "custom_api_name",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub custom_api_name: String,
    /// 自定义字段名称
    #[serde(
        rename = "name",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub name: CustomNameSubResp,
    /// 自定义字段类型
    ///
    /// **示例值**: "1"
    #[serde(
        rename = "type",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub body_type: i64,
    /// 字段值，是 json 转义后的字符串，根据元数据定义不同，字段格式不同（如 123, 123.23, "true", ["id1","id2"], "2006-01-02 15:04:05"）
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "\"231\""
    #[serde(
        rename = "value",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub value: String,
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
    /// 字段值，是json转义后的字符串，根据元数据定义不同，字段格式不同(123, 123.23, true, [\"id1\",\"id2\], 2006-01-02 15:04:05])
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
pub struct CustomNameSubResp {
    /// 中文
    ///
    /// **示例值**: "自定义姓名"
    #[serde(
        rename = "zh_cn",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub zh_cn: String,
    /// 英文
    ///
    /// **示例值**: "Custom Name"
    #[serde(
        rename = "en_us",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub en_us: String,
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
        Fn(BatchGetCoreHrCompanyReq) -> Result<(BatchGetCoreHrCompanyResp, CommonResponse), Error>
        + Send
        + Sync
        + 'static
    {
    }
    impl<
            T: Fn(
                    BatchGetCoreHrCompanyReq,
                ) -> Result<(BatchGetCoreHrCompanyResp, CommonResponse), Error>
                + Send
                + Sync
                + 'static,
        > MockFunc for T
    {
    }

    impl<'c, IStore: Store, IClient: HttpClient> CoreHrServiceMocker<'c, IStore, IClient> {
        pub fn mock_batch_get_core_hr_company<F: MockFunc>(
            &self,
            f: F,
        ) -> MockerBuilder<
            Mocker,
            BatchGetCoreHrCompanyReq,
            BatchGetCoreHrCompanyResp,
            Arc<dyn MockFunc>,
        > {
            MockerBuilder::new(self.cli.instance_id, Arc::new(f))
        }

        pub(super) fn get_mock_batch_get_core_hr_company(
            &self,
            req: &BatchGetCoreHrCompanyReq,
        ) -> Option<Arc<dyn MockFunc>> {
            do_mock::<Mocker, BatchGetCoreHrCompanyReq, BatchGetCoreHrCompanyResp, Arc<dyn MockFunc>>(
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
        api::gen::core_hr::batch_get_core_hr_company::{
            BatchGetCoreHrCompanyReq, BatchGetCoreHrCompanyResp, BatchGetCoreHrCompanyRespInner,
        },
        core::{model::CommonResponse, Lark},
    };

    #[tokio::test]
    async fn test_mock() {
        let lark = Lark::new("".to_owned(), "".into());
        let mocker = lark
            .core_hr()
            .mock()
            .mock_batch_get_core_hr_company(|_| {
                Ok((
                    BatchGetCoreHrCompanyResp::default(),
                    CommonResponse::default(),
                ))
            })
            .build();
        let res = lark
            .core_hr()
            .batch_get_core_hr_company(BatchGetCoreHrCompanyReq::default())
            .await;
        assert!(res.is_ok());
        mocker.clear();
        let res = lark
            .core_hr()
            .batch_get_core_hr_company(BatchGetCoreHrCompanyReq::default())
            .await;
        assert!(res.is_err());
    }

    const REQ: &str = r#"{
    "company_ids": [
        "151515"
    ]
}"#;

    #[test]
    fn test_req() {
        if REQ == "{}" {
            return;
        }
        if let Err(e) = serde_json::from_str::<super::BatchGetCoreHrCompanyReqBody>(REQ) {
            panic!("{}", e);
        }
    }

    const RESP: &str = r#"{
    "code": 0,
    "msg": "success",
    "data": {
        "items": [
            {
                "company_id": "4692472714243080020",
                "hiberarchy_common": {
                    "parent_id": "4719168654814483759",
                    "name": [
                        {
                            "lang": "zh-CN",
                            "value": "张三"
                        }
                    ],
                    "type": {
                        "enum_name": "phone_type",
                        "display": [
                            {
                                "lang": "zh-CN",
                                "value": "张三"
                            }
                        ]
                    },
                    "active": true,
                    "effective_time": "2020-05-01 00:00:00",
                    "expiration_time": "2020-05-02 00:00:00",
                    "code": "12456",
                    "description": [
                        {
                            "lang": "zh-CN",
                            "value": "张三"
                        }
                    ],
                    "tree_order": "123",
                    "list_order": "123",
                    "custom_fields": [
                        {
                            "field_name": "name",
                            "value": "Sandy"
                        }
                    ]
                },
                "type": {
                    "enum_name": "phone_type",
                    "display": [
                        {
                            "lang": "zh-CN",
                            "value": "张三"
                        }
                    ]
                },
                "industry_list": [
                    {
                        "enum_name": "phone_type",
                        "display": [
                            {
                                "lang": "zh-CN",
                                "value": "张三"
                            }
                        ]
                    }
                ],
                "legal_representative": [
                    {
                        "lang": "zh-CN",
                        "value": "张三"
                    }
                ],
                "post_code": "邮编",
                "tax_payer_id": "123456840",
                "confidential": true,
                "sub_type_list": [
                    {
                        "enum_name": "phone_type",
                        "display": [
                            {
                                "lang": "zh-CN",
                                "value": "张三"
                            }
                        ]
                    }
                ],
                "branch_company": true,
                "primary_manager": [
                    {
                        "lang": "zh-CN",
                        "value": "张三"
                    }
                ],
                "currency": {
                    "currency_id": "6893114062142064111",
                    "country_region_id": "6893114162142064111",
                    "currency_name": [
                        {
                            "lang": "zh-CN",
                            "value": "张三"
                        }
                    ],
                    "numeric_code": 156,
                    "currency_alpha_3_code": "CNY",
                    "status": 1
                },
                "phone": {
                    "area_code": {
                        "enum_name": "phone_type",
                        "display": [
                            {
                                "lang": "zh-CN",
                                "value": "张三"
                            }
                        ]
                    },
                    "phone_number": "213213"
                },
                "fax": {
                    "area_code": {
                        "enum_name": "phone_type",
                        "display": [
                            {
                                "lang": "zh-CN",
                                "value": "张三"
                            }
                        ]
                    },
                    "phone_number": "213213"
                },
                "registered_office_address": [
                    {
                        "lang": "zh-CN",
                        "value": "张三"
                    }
                ],
                "office_address": [
                    {
                        "lang": "zh-CN",
                        "value": "张三"
                    }
                ],
                "custom_fields": [
                    {
                        "custom_api_name": "name",
                        "name": {
                            "zh_cn": "自定义姓名",
                            "en_us": "Custom Name"
                        },
                        "type": 1,
                        "value": "\"231\""
                    }
                ]
            }
        ]
    }
}"#;
    #[test]
    fn test_resp() {
        let res = serde_json::from_str::<BatchGetCoreHrCompanyRespInner>(RESP);
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
