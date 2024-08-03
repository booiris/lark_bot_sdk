//! doc url: <https://open.larkoffice.com/document/uAjLw4CM/ukTMukTMukTM/reference/corehr-v1/working_hours_type/patch>
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
    /// **api 版本: 2024-02-06T02:19:08+00:00**
    ///
    /// ## 更新工时制度
    ///
    /// 更新工时制度。
    ///

    ///
    /// doc: <https://open.larkoffice.com/document/uAjLw4CM/ukTMukTMukTM/reference/corehr-v1/working_hours_type/patch>
    ///
    /// new doc: <https://open.larkoffice.com/document/server-docs/corehr-v1/basic-infomation/working_hours_type/patch>
    ///
    /// api url: <https://open.feishu.cn/document_portal/v1/document/get_detail?fullPath=%2Fserver-docs%2Fcorehr-v1%2Fbasic-infomation%2Fworking_hours_type%2Fpatch>
    pub async fn update_core_hr_working_hours_type(
        &self,
        req: UpdateCoreHrWorkingHoursTypeReq,
    ) -> Result<(UpdateCoreHrWorkingHoursTypeResp, CommonResponse), Error> {
        self.update_core_hr_working_hours_type_with_opt(req, Default::default())
            .await
    }

    /// 参见 [update_core_hr_working_hours_type](#method.update_core_hr_working_hours_type) 函数
    pub async fn update_core_hr_working_hours_type_with_opt(
        &self,
        req: UpdateCoreHrWorkingHoursTypeReq,
        method_option: MethodOption,
    ) -> Result<(UpdateCoreHrWorkingHoursTypeResp, CommonResponse), Error> {
        #[cfg(feature = "test-util")]
        {
            if let Some(f) = self.mock().get_mock_update_core_hr_working_hours_type(&req) {
                tracing::info!("[lark] CoreHr#UpdateCoreHrWorkingHoursType **mocking** api");
                return f(req);
            }
        }

        tracing::info!("[lark] CoreHr#UpdateCoreHrWorkingHoursType call api");

        let req = ApiRequest {
            scope: "CoreHr",
            api: "UpdateCoreHrWorkingHoursType",
            method: http::Method::PATCH,
            url: String::new()
                + self.cli.open_base_url.as_ref()
                + "/open-apis/corehr/v1/working_hours_types/:working_hours_type_id",
            param_data: req.gen_param(),
            method_option,
            need_tenant_access_token: true,
            ..Default::default()
        };

        let (resp, common_resp): (UpdateCoreHrWorkingHoursTypeRespInner, _) =
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
pub struct UpdateCoreHrWorkingHoursTypeReq {
    /// 工时制度ID
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "1616161616"
    #[api(kind = "path", name = "working_hours_type_id")]
    pub working_hours_type_id: String,
    /// 根据client_token是否一致来判断是否为同一请求
    ///
    /// **示例值**: "12454646"
    #[api(
        kind = "query",
        name = "client_token",
        v_type = "var",
        option = "false"
    )]
    pub client_token: String,
    /// 编码
    ///
    /// **示例值**: "1"
    #[api(kind = "body", name = "code")]
    pub code: Option<String>,
    /// 名称
    #[api(kind = "body", name = "name")]
    pub name: Vec<Option<I18nSubReq>>,
    /// 国家/地区 ID 列表
    ///
    /// **示例值**: "创建成功的工时制度信息"
    #[api(kind = "body", name = "country_region_id_list")]
    pub country_region_id_list: Vec<Option<String>>,
    /// 职务默认值
    ///
    /// **示例值**: "true"
    #[api(kind = "body", name = "default_for_job")]
    pub default_for_job: Option<bool>,
    /// 是否启用
    ///
    /// **示例值**: "true"
    #[api(kind = "body", name = "active")]
    pub active: Option<bool>,
    /// 自定义字段
    #[api(kind = "body", name = "custom_fields")]
    pub custom_fields: Vec<Option<ObjectFieldDataSubReq>>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(default)]
pub struct I18nSubReq {
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
    /// **示例值**: "\"Sandy\""
    #[serde(
        rename = "value",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub value: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, lark_bot_sdk_macros::ApiBaseResp)]
struct UpdateCoreHrWorkingHoursTypeRespInner {
    #[serde(flatten)]
    data: Option<UpdateCoreHrWorkingHoursTypeResp>,
    #[serde(flatten)]
    base: BaseResp,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct UpdateCoreHrWorkingHoursTypeResp {
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
    /// 工时制度
    #[serde(
        rename = "working_hours_type",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub working_hours_type: WorkingHoursTypeSubResp,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(default)]
pub struct WorkingHoursTypeSubResp {
    /// 工时制度 ID
    ///
    /// **示例值**: "6890452208593372679"
    #[serde(
        rename = "id",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub id: String,
    /// 编码
    ///
    /// **示例值**: "1"
    #[serde(
        rename = "code",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub code: String,
    /// 名称
    ///
    /// **是否必填**: 是
    #[serde(
        rename = "name",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub name: Vec<I18nSubResp>,
    /// 国家/地区 ID 列表
    ///
    /// **示例值**: "创建成功的工时制度信息"
    #[serde(
        rename = "country_region_id_list",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub country_region_id_list: Vec<String>,
    /// 职务默认值
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "true"
    #[serde(
        rename = "default_for_job",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub default_for_job: bool,
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
            UpdateCoreHrWorkingHoursTypeReq,
        ) -> Result<(UpdateCoreHrWorkingHoursTypeResp, CommonResponse), Error>
        + Send
        + Sync
        + 'static
    {
    }
    impl<
            T: Fn(
                    UpdateCoreHrWorkingHoursTypeReq,
                )
                    -> Result<(UpdateCoreHrWorkingHoursTypeResp, CommonResponse), Error>
                + Send
                + Sync
                + 'static,
        > MockFunc for T
    {
    }

    impl<'c, IStore: Store, IClient: HttpClient> CoreHrServiceMocker<'c, IStore, IClient> {
        pub fn mock_update_core_hr_working_hours_type<F: MockFunc>(
            &self,
            f: F,
        ) -> MockerBuilder<
            Mocker,
            UpdateCoreHrWorkingHoursTypeReq,
            UpdateCoreHrWorkingHoursTypeResp,
            Arc<dyn MockFunc>,
        > {
            MockerBuilder::new(self.cli.instance_id, Arc::new(f))
        }

        pub(super) fn get_mock_update_core_hr_working_hours_type(
            &self,
            req: &UpdateCoreHrWorkingHoursTypeReq,
        ) -> Option<Arc<dyn MockFunc>> {
            do_mock::<
                Mocker,
                UpdateCoreHrWorkingHoursTypeReq,
                UpdateCoreHrWorkingHoursTypeResp,
                Arc<dyn MockFunc>,
            >(self.cli.instance_id, req)
        }
    }
}

#[cfg(feature = "test-util")]
#[cfg(test)]
mod test {
    use crate::{
        api::gen::core_hr::update_core_hr_working_hours_type::{
            UpdateCoreHrWorkingHoursTypeReq, UpdateCoreHrWorkingHoursTypeResp,
            UpdateCoreHrWorkingHoursTypeRespInner,
        },
        core::{model::CommonResponse, Lark},
    };

    #[tokio::test]
    async fn test_mock() {
        let lark = Lark::new("".to_owned(), "".into());
        let mocker = lark
            .core_hr()
            .mock()
            .mock_update_core_hr_working_hours_type(|_| {
                Ok((
                    UpdateCoreHrWorkingHoursTypeResp::default(),
                    CommonResponse::default(),
                ))
            })
            .build();
        let res = lark
            .core_hr()
            .update_core_hr_working_hours_type(UpdateCoreHrWorkingHoursTypeReq::default())
            .await;
        assert!(res.is_ok());
        mocker.clear();
        let res = lark
            .core_hr()
            .update_core_hr_working_hours_type(UpdateCoreHrWorkingHoursTypeReq::default())
            .await;
        assert!(res.is_err());
    }

    const REQ: &str = r#"{
    "code": "1",
    "name": [
        {
            "lang": "zh-CN",
            "value": "张三"
        }
    ],
    "country_region_id_list": [
        "6890452208593356295"
    ],
    "default_for_job": true,
    "active": true,
    "custom_fields": [
        {
            "field_name": "name",
            "value": "\"Sandy\""
        }
    ]
}"#;

    #[test]
    fn test_req() {
        if REQ == "{}" {
            return;
        }
        if let Err(e) = serde_json::from_str::<super::UpdateCoreHrWorkingHoursTypeReqBody>(REQ) {
            panic!("{}", e);
        }
    }

    const RESP: &str = r#"{
    "code": 0,
    "msg": "success",
    "data": {
        "working_hours_type": {
            "id": "6890452208593372679",
            "code": "1",
            "name": [
                {
                    "lang": "zh-CN",
                    "value": "张三"
                }
            ],
            "country_region_id_list": [
                "6890452208593356295"
            ],
            "default_for_job": true,
            "active": true,
            "custom_fields": [
                {
                    "field_name": "name",
                    "value": "\"Sandy\""
                }
            ]
        }
    }
}"#;
    #[test]
    fn test_resp() {
        let res = serde_json::from_str::<UpdateCoreHrWorkingHoursTypeRespInner>(RESP);
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
