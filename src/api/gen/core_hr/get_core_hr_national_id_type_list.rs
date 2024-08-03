//! doc url: <https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/corehr-v1/national_id_type/list>
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
    /// **api 版本: 2024-02-06T02:19:07+00:00**
    ///
    /// ## 批量查询国家证件类型
    ///
    /// 批量查询国家证件类型。
    ///

    ///
    /// doc: <https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/corehr-v1/national_id_type/list>
    ///
    /// new doc: <https://open.feishu.cn/document/server-docs/corehr-v1/basic-infomation/national_id_type/list>
    ///
    /// api url: <https://open.feishu.cn/document_portal/v1/document/get_detail?fullPath=%2Fserver-docs%2Fcorehr-v1%2Fbasic-infomation%2Fnational_id_type%2Flist>
    pub async fn get_core_hr_national_id_type_list(
        &self,
        req: GetCoreHrNationalIdTypeListReq,
    ) -> Result<(GetCoreHrNationalIdTypeListResp, CommonResponse), Error> {
        self.get_core_hr_national_id_type_list_with_opt(req, Default::default())
            .await
    }

    /// 参见 [get_core_hr_national_id_type_list](#method.get_core_hr_national_id_type_list) 函数
    pub async fn get_core_hr_national_id_type_list_with_opt(
        &self,
        req: GetCoreHrNationalIdTypeListReq,
        method_option: MethodOption,
    ) -> Result<(GetCoreHrNationalIdTypeListResp, CommonResponse), Error> {
        #[cfg(feature = "test-util")]
        {
            if let Some(f) = self.mock().get_mock_get_core_hr_national_id_type_list(&req) {
                tracing::info!("[lark] CoreHr#GetCoreHrNationalIdTypeList **mocking** api");
                return f(req);
            }
        }

        tracing::info!("[lark] CoreHr#GetCoreHrNationalIdTypeList call api");

        let req = ApiRequest {
            scope: "CoreHr",
            api: "GetCoreHrNationalIdTypeList",
            method: http::Method::GET,
            url: String::new()
                + self.cli.open_base_url.as_ref()
                + "/open-apis/corehr/v1/national_id_types",
            param_data: req.gen_param(),
            method_option,
            need_tenant_access_token: true,
            ..Default::default()
        };

        let (resp, common_resp): (GetCoreHrNationalIdTypeListRespInner, _) =
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
pub struct GetCoreHrNationalIdTypeListReq {
    /// 分页标记，第一次请求不填，表示从头开始遍历；分页查询结果还有更多项时会同时返回新的 page_token，下次遍历可采用该 page_token 获取查询结果
    ///
    /// **示例值**: "1231231987"
    #[api(kind = "query", name = "page_token", v_type = "var", option = "false")]
    pub page_token: String,
    /// 分页大小
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "100"
    #[api(kind = "query", name = "page_size", v_type = "var", option = "false")]
    pub page_size: String,
    /// 证件类型
    ///
    /// **示例值**: "regular_passport"
    #[api(
        kind = "query",
        name = "identification_type",
        v_type = "var",
        option = "false"
    )]
    pub identification_type: String,
    /// 证件类型编码
    ///
    /// **示例值**: "MYS-ID"
    #[api(kind = "query", name = "code", v_type = "var", option = "false")]
    pub code: String,
    /// 国家地区ID
    ///
    /// **示例值**: "6862995749043439111"
    #[api(
        kind = "query",
        name = "country_region_id",
        v_type = "var",
        option = "false"
    )]
    pub country_region_id: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, lark_bot_sdk_macros::ApiBaseResp)]
struct GetCoreHrNationalIdTypeListRespInner {
    #[serde(flatten)]
    data: Option<GetCoreHrNationalIdTypeListResp>,
    #[serde(flatten)]
    base: BaseResp,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct GetCoreHrNationalIdTypeListResp {
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
    /// 查询的国家证件类型信息
    #[serde(
        rename = "items",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub items: Vec<NationalIdTypeSubResp>,
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
pub struct NationalIdTypeSubResp {
    /// 证件类型 ID
    ///
    /// **示例值**: "6888198886960137735"
    #[serde(
        rename = "id",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub id: String,
    /// 国家 / 地区
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "6862995747139225096"
    #[serde(
        rename = "country_region_id",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub country_region_id: String,
    /// 名称
    ///
    /// **是否必填**: 是
    #[serde(
        rename = "name",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub name: Vec<I18nSubResp>,
    /// 是否启用
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "true"
    #[serde(
        rename = "active",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub active: bool,
    /// 校验规则
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "^\d{9}$"
    #[serde(
        rename = "validation_rule",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub validation_rule: String,
    /// 校验规则描述
    #[serde(
        rename = "validation_rule_description",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub validation_rule_description: Vec<I18nSubResp>,
    /// 编码
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "AUS-TFN"
    #[serde(
        rename = "code",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub code: String,
    /// 证件类型
    ///
    /// **是否必填**: 是
    #[serde(
        rename = "identification_type",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub identification_type: EnumSubResp,
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
            GetCoreHrNationalIdTypeListReq,
        ) -> Result<(GetCoreHrNationalIdTypeListResp, CommonResponse), Error>
        + Send
        + Sync
        + 'static
    {
    }
    impl<
            T: Fn(
                    GetCoreHrNationalIdTypeListReq,
                )
                    -> Result<(GetCoreHrNationalIdTypeListResp, CommonResponse), Error>
                + Send
                + Sync
                + 'static,
        > MockFunc for T
    {
    }

    impl<'c, IStore: Store, IClient: HttpClient> CoreHrServiceMocker<'c, IStore, IClient> {
        pub fn mock_get_core_hr_national_id_type_list<F: MockFunc>(
            &self,
            f: F,
        ) -> MockerBuilder<
            Mocker,
            GetCoreHrNationalIdTypeListReq,
            GetCoreHrNationalIdTypeListResp,
            Arc<dyn MockFunc>,
        > {
            MockerBuilder::new(self.cli.instance_id, Arc::new(f))
        }

        pub(super) fn get_mock_get_core_hr_national_id_type_list(
            &self,
            req: &GetCoreHrNationalIdTypeListReq,
        ) -> Option<Arc<dyn MockFunc>> {
            do_mock::<
                Mocker,
                GetCoreHrNationalIdTypeListReq,
                GetCoreHrNationalIdTypeListResp,
                Arc<dyn MockFunc>,
            >(self.cli.instance_id, req)
        }
    }
}

#[cfg(feature = "test-util")]
#[cfg(test)]
mod test {
    use crate::{
        api::gen::core_hr::get_core_hr_national_id_type_list::{
            GetCoreHrNationalIdTypeListReq, GetCoreHrNationalIdTypeListResp,
            GetCoreHrNationalIdTypeListRespInner,
        },
        core::{model::CommonResponse, Lark},
    };

    #[tokio::test]
    async fn test_mock() {
        let lark = Lark::new("".to_owned(), "".into());
        let mocker = lark
            .core_hr()
            .mock()
            .mock_get_core_hr_national_id_type_list(|_| {
                Ok((
                    GetCoreHrNationalIdTypeListResp::default(),
                    CommonResponse::default(),
                ))
            })
            .build();
        let res = lark
            .core_hr()
            .get_core_hr_national_id_type_list(GetCoreHrNationalIdTypeListReq::default())
            .await;
        assert!(res.is_ok());
        mocker.clear();
        let res = lark
            .core_hr()
            .get_core_hr_national_id_type_list(GetCoreHrNationalIdTypeListReq::default())
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

    const RESP: &str = r#"{"code":0,
"msg":"success",
"data":{"items":[{"id":"6888198886960137735",
"country_region_id":"6862995747139225096",
"name":[{
    "lang": "zh-CN",
    "value": "张三"
}],
"active":true,
"validation_rule":"",
"validation_rule_description":[{
    "lang": "zh-CN",
    "value": "张三"
}],
"code":"AUS-TFN",
"identification_type":{"enum_name":"type_1",
"display":[{
    "lang": "zh-CN",
    "value": "张三"
}]},
"custom_fields":[{
    "field_name": "name",
    "value": "\"Sandy\""
}]}],
"has_more":true,
"page_token":"1234452132"}}"#;
    #[test]
    fn test_resp() {
        let res = serde_json::from_str::<GetCoreHrNationalIdTypeListRespInner>(RESP);
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
