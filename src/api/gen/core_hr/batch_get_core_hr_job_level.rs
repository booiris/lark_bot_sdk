//! doc url: <https://open.larkoffice.com/document/uAjLw4CM/ukTMukTMukTM/corehr-v2/job_level/batch_get>
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
    /// **api 版本: 2024-02-06T02:28:53+00:00**
    ///
    /// ## 通过职级 ID 批量获取职级信息
    ///
    /// 通过职级 ID 批量获取职级信息
    ///

    ///
    /// doc: <https://open.larkoffice.com/document/uAjLw4CM/ukTMukTMukTM/corehr-v2/job_level/batch_get>
    ///
    /// new doc: <https://open.larkoffice.com/document/uAjLw4CM/ukTMukTMukTM/corehr-v2/job_level/batch_get>
    ///
    /// api url: <https://open.feishu.cn/document_portal/v1/document/get_detail?fullPath=%2FuAjLw4CM%2FukTMukTMukTM%2Fcorehr-v2%2Fjob_level%2Fbatch_get>
    pub async fn batch_get_core_hr_job_level(
        &self,
        req: BatchGetCoreHrJobLevelReq,
    ) -> Result<(BatchGetCoreHrJobLevelResp, CommonResponse), Error> {
        self.batch_get_core_hr_job_level_with_opt(req, Default::default())
            .await
    }

    /// 参见 [batch_get_core_hr_job_level](#method.batch_get_core_hr_job_level) 函数
    pub async fn batch_get_core_hr_job_level_with_opt(
        &self,
        req: BatchGetCoreHrJobLevelReq,
        method_option: MethodOption,
    ) -> Result<(BatchGetCoreHrJobLevelResp, CommonResponse), Error> {
        #[cfg(feature = "test-util")]
        {
            if let Some(f) = self.mock().get_mock_batch_get_core_hr_job_level(&req) {
                tracing::info!("[lark] CoreHr#BatchGetCoreHrJobLevel **mocking** api");
                return f(req);
            }
        }

        tracing::info!("[lark] CoreHr#BatchGetCoreHrJobLevel call api");

        let req = ApiRequest {
            scope: "CoreHr",
            api: "BatchGetCoreHrJobLevel",
            method: http::Method::POST,
            url: String::new()
                + self.cli.open_base_url.as_ref()
                + "/open-apis/corehr/v2/job_levels/batch_get",
            param_data: req.gen_param(),
            method_option,
            need_tenant_access_token: true,
            ..Default::default()
        };

        let (resp, common_resp): (BatchGetCoreHrJobLevelRespInner, _) =
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
pub struct BatchGetCoreHrJobLevelReq {
    /// 职级 ID 列表
    ///
    /// **是否必填**: 是
    ///
    /// **数据校验规则**：
    ///
    /// - **长度范围**: `1` 字符- `100` 字符
    #[api(kind = "body", name = "job_level_ids")]
    pub job_level_ids: Vec<Option<String>>,
}

#[derive(Debug, Serialize, Deserialize, Clone, lark_bot_sdk_macros::ApiBaseResp)]
struct BatchGetCoreHrJobLevelRespInner {
    #[serde(flatten)]
    data: Option<BatchGetCoreHrJobLevelResp>,
    #[serde(flatten)]
    base: BaseResp,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct BatchGetCoreHrJobLevelResp {
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
    /// 查询的职级信息
    #[serde(
        rename = "items",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub items: Vec<JobLevelSubResp>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(default)]
pub struct JobLevelSubResp {
    /// 职级 ID
    ///
    /// **示例值**: "4692446793125560154"
    #[serde(
        rename = "job_level_id",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub job_level_id: String,
    /// 职级数值
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "9999"
    #[serde(
        rename = "level_order",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub level_order: i64,
    /// 编码
    ///
    /// **示例值**: "VQzo/BSonp8l6PmcZ+VlDhkd2595LMkhyBAGX6HAlCY="
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
    /// 描述
    #[serde(
        rename = "description",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub description: Vec<I18nSubResp>,
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
    /// 自定义字段
    #[serde(
        rename = "custom_fields",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub custom_fields: Vec<CustomFieldDataSubResp>,
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
        Fn(BatchGetCoreHrJobLevelReq) -> Result<(BatchGetCoreHrJobLevelResp, CommonResponse), Error>
        + Send
        + Sync
        + 'static
    {
    }
    impl<
            T: Fn(
                    BatchGetCoreHrJobLevelReq,
                ) -> Result<(BatchGetCoreHrJobLevelResp, CommonResponse), Error>
                + Send
                + Sync
                + 'static,
        > MockFunc for T
    {
    }

    impl<'c, IStore: Store, IClient: HttpClient> CoreHrServiceMocker<'c, IStore, IClient> {
        pub fn mock_batch_get_core_hr_job_level<F: MockFunc>(
            &self,
            f: F,
        ) -> MockerBuilder<
            Mocker,
            BatchGetCoreHrJobLevelReq,
            BatchGetCoreHrJobLevelResp,
            Arc<dyn MockFunc>,
        > {
            MockerBuilder::new(self.cli.instance_id, Arc::new(f))
        }

        pub(super) fn get_mock_batch_get_core_hr_job_level(
            &self,
            req: &BatchGetCoreHrJobLevelReq,
        ) -> Option<Arc<dyn MockFunc>> {
            do_mock::<
                Mocker,
                BatchGetCoreHrJobLevelReq,
                BatchGetCoreHrJobLevelResp,
                Arc<dyn MockFunc>,
            >(self.cli.instance_id, req)
        }
    }
}

#[cfg(feature = "test-util")]
#[cfg(test)]
mod test {
    use crate::{
        api::gen::core_hr::batch_get_core_hr_job_level::{
            BatchGetCoreHrJobLevelReq, BatchGetCoreHrJobLevelResp, BatchGetCoreHrJobLevelRespInner,
        },
        core::{model::CommonResponse, Lark},
    };

    #[tokio::test]
    async fn test_mock() {
        let lark = Lark::new("".to_owned(), "".into());
        let mocker = lark
            .core_hr()
            .mock()
            .mock_batch_get_core_hr_job_level(|_| {
                Ok((
                    BatchGetCoreHrJobLevelResp::default(),
                    CommonResponse::default(),
                ))
            })
            .build();
        let res = lark
            .core_hr()
            .batch_get_core_hr_job_level(BatchGetCoreHrJobLevelReq::default())
            .await;
        assert!(res.is_ok());
        mocker.clear();
        let res = lark
            .core_hr()
            .batch_get_core_hr_job_level(BatchGetCoreHrJobLevelReq::default())
            .await;
        assert!(res.is_err());
    }

    const REQ: &str = r#"{
    "job_level_ids": [
        "1515"
    ]
}"#;

    #[test]
    fn test_req() {
        if REQ == "{}" {
            return;
        }
        if let Err(e) = serde_json::from_str::<super::BatchGetCoreHrJobLevelReqBody>(REQ) {
            panic!("{}", e);
        }
    }

    const RESP: &str = r#"{
    "code": 0,
    "msg": "success",
    "data": {
        "items": [
            {
                "job_level_id": "4692446793125560154",
                "level_order": 9999,
                "code": "VQzo/BSonp8l6PmcZ+VlDhkd2595LMkhyBAGX6HAlCY=",
                "name": [
                    {
                        "lang": "zh-CN",
                        "value": "张三"
                    }
                ],
                "description": [
                    {
                        "lang": "zh-CN",
                        "value": "张三"
                    }
                ],
                "active": true,
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
        let res = serde_json::from_str::<BatchGetCoreHrJobLevelRespInner>(RESP);
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
