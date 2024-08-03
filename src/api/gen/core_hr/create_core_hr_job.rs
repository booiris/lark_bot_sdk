//! doc url: <https://open.larkoffice.com/document/uAjLw4CM/ukTMukTMukTM/reference/corehr-v1/job/create>
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
    /// **api 版本: 2024-02-06T02:29:17+00:00**
    ///
    /// ## 创建职务
    ///
    /// 创建职务。
    ///

    ///
    /// doc: <https://open.larkoffice.com/document/uAjLw4CM/ukTMukTMukTM/reference/corehr-v1/job/create>
    ///
    /// new doc: <https://open.larkoffice.com/document/server-docs/corehr-v1/job-management/job/create>
    ///
    /// api url: <https://open.feishu.cn/document_portal/v1/document/get_detail?fullPath=%2Fserver-docs%2Fcorehr-v1%2Fjob-management%2Fjob%2Fcreate>
    pub async fn create_core_hr_job(
        &self,
        req: CreateCoreHrJobReq,
    ) -> Result<(CreateCoreHrJobResp, CommonResponse), Error> {
        self.create_core_hr_job_with_opt(req, Default::default())
            .await
    }

    /// 参见 [create_core_hr_job](#method.create_core_hr_job) 函数
    pub async fn create_core_hr_job_with_opt(
        &self,
        req: CreateCoreHrJobReq,
        method_option: MethodOption,
    ) -> Result<(CreateCoreHrJobResp, CommonResponse), Error> {
        #[cfg(feature = "test-util")]
        {
            if let Some(f) = self.mock().get_mock_create_core_hr_job(&req) {
                tracing::info!("[lark] CoreHr#CreateCoreHrJob **mocking** api");
                return f(req);
            }
        }

        tracing::info!("[lark] CoreHr#CreateCoreHrJob call api");

        let req = ApiRequest {
            scope: "CoreHr",
            api: "CreateCoreHrJob",
            method: http::Method::POST,
            url: String::new() + self.cli.open_base_url.as_ref() + "/open-apis/corehr/v1/jobs",
            param_data: req.gen_param(),
            method_option,
            need_tenant_access_token: true,
            ..Default::default()
        };

        let (resp, common_resp): (CreateCoreHrJobRespInner, _) = self.cli.do_req(req).await?;
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
pub struct CreateCoreHrJobReq {
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
    /// **示例值**: "JP422119"
    #[api(kind = "body", name = "code")]
    pub code: Option<String>,
    /// 名称
    ///
    /// **是否必填**: 是
    #[api(kind = "body", name = "name")]
    pub name: Vec<Option<I18nSubReq>>,
    /// 描述
    #[api(kind = "body", name = "description")]
    pub description: Vec<Option<I18nSubReq>>,
    /// 是否启用
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "true"
    #[api(kind = "body", name = "active")]
    pub active: bool,
    /// 职务头衔
    #[api(kind = "body", name = "job_title")]
    pub job_title: Vec<Option<I18nSubReq>>,
    /// 职务序列 ID 列表，枚举值及详细信息可通过【批量查询职务序列】接口查询获得
    ///
    /// **示例值**: "7373183781"
    #[api(kind = "body", name = "job_family_id_list")]
    pub job_family_id_list: Vec<Option<String>>,
    /// 职务级别 ID 列表，枚举值及详细信息可通过【批量查询职务级别】接口查询获得
    ///
    /// **示例值**: "316316317"
    #[api(kind = "body", name = "job_level_id_list")]
    pub job_level_id_list: Vec<Option<String>>,
    /// 工时制度 ID，枚举值及详细信息可通过【批量查询工时制度】接口查询获得
    ///
    /// **示例值**: "6890452208593372679"
    #[api(kind = "body", name = "working_hours_type_id")]
    pub working_hours_type_id: Option<String>,
    /// 生效时间
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "2020-01-01 00:00:00"
    #[api(kind = "body", name = "effective_time")]
    pub effective_time: String,
    /// 失效时间
    ///
    /// **示例值**: "2021-01-01 00:00:00"
    #[api(kind = "body", name = "expiration_time")]
    pub expiration_time: Option<String>,
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
struct CreateCoreHrJobRespInner {
    #[serde(flatten)]
    data: Option<CreateCoreHrJobResp>,
    #[serde(flatten)]
    base: BaseResp,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct CreateCoreHrJobResp {
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
    /// 创建成功的Job信息
    #[serde(
        rename = "job",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub job: JobSubResp,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(default)]
pub struct JobSubResp {
    /// 职务 ID
    ///
    /// **示例值**: "4698040628992333549"
    #[serde(
        rename = "id",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub id: String,
    /// 编码
    ///
    /// **示例值**: "JP422119"
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
    /// 职务头衔
    #[serde(
        rename = "job_title",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub job_title: Vec<I18nSubResp>,
    /// 职务序列 ID 列表，枚举值及详细信息可通过【批量查询职务序列】接口查询获得
    ///
    /// **示例值**: "7373183781"
    #[serde(
        rename = "job_family_id_list",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub job_family_id_list: Vec<String>,
    /// 职务级别 ID 列表，枚举值及详细信息可通过【批量查询职务级别】接口查询获得
    ///
    /// **示例值**: "316316317"
    #[serde(
        rename = "job_level_id_list",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub job_level_id_list: Vec<String>,
    /// 工时制度 ID，枚举值及详细信息可通过【批量查询工时制度】接口查询获得
    ///
    /// **示例值**: "6890452208593372679"
    #[serde(
        rename = "working_hours_type_id",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub working_hours_type_id: String,
    /// 生效时间
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "2020-01-01 00:00:00"
    #[serde(
        rename = "effective_time",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub effective_time: String,
    /// 失效时间
    ///
    /// **示例值**: "2021-01-01 00:00:00"
    #[serde(
        rename = "expiration_time",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub expiration_time: String,
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
        Fn(CreateCoreHrJobReq) -> Result<(CreateCoreHrJobResp, CommonResponse), Error>
        + Send
        + Sync
        + 'static
    {
    }
    impl<
            T: Fn(CreateCoreHrJobReq) -> Result<(CreateCoreHrJobResp, CommonResponse), Error>
                + Send
                + Sync
                + 'static,
        > MockFunc for T
    {
    }

    impl<'c, IStore: Store, IClient: HttpClient> CoreHrServiceMocker<'c, IStore, IClient> {
        pub fn mock_create_core_hr_job<F: MockFunc>(
            &self,
            f: F,
        ) -> MockerBuilder<Mocker, CreateCoreHrJobReq, CreateCoreHrJobResp, Arc<dyn MockFunc>>
        {
            MockerBuilder::new(self.cli.instance_id, Arc::new(f))
        }

        pub(super) fn get_mock_create_core_hr_job(
            &self,
            req: &CreateCoreHrJobReq,
        ) -> Option<Arc<dyn MockFunc>> {
            do_mock::<Mocker, CreateCoreHrJobReq, CreateCoreHrJobResp, Arc<dyn MockFunc>>(
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
        api::gen::core_hr::create_core_hr_job::{
            CreateCoreHrJobReq, CreateCoreHrJobResp, CreateCoreHrJobRespInner,
        },
        core::{model::CommonResponse, Lark},
    };

    #[tokio::test]
    async fn test_mock() {
        let lark = Lark::new("".to_owned(), "".into());
        let mocker = lark
            .core_hr()
            .mock()
            .mock_create_core_hr_job(|_| {
                Ok((CreateCoreHrJobResp::default(), CommonResponse::default()))
            })
            .build();
        let res = lark
            .core_hr()
            .create_core_hr_job(CreateCoreHrJobReq::default())
            .await;
        assert!(res.is_ok());
        mocker.clear();
        let res = lark
            .core_hr()
            .create_core_hr_job(CreateCoreHrJobReq::default())
            .await;
        assert!(res.is_err());
    }

    const REQ: &str = r#"{
    "code": "JP422119",
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
    "job_title": [
        {
            "lang": "zh-CN",
            "value": "张三"
        }
    ],
    "job_family_id_list": [
        "4719519211875096301"
    ],
    "job_level_id_list": [
        "4719519212005299950"
    ],
    "working_hours_type_id": "6890452208593372679",
    "effective_time": "2020-01-01 00:00:00",
    "expiration_time": "2021-01-01 00:00:00",
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
        if let Err(e) = serde_json::from_str::<super::CreateCoreHrJobReqBody>(REQ) {
            panic!("{}", e);
        }
    }

    const RESP: &str = r#"{
    "code": 0,
    "msg": "success",
    "data": {
        "job": {
            "id": "4698040628992333549",
            "code": "JP422119",
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
            "job_title": [
                {
                    "lang": "zh-CN",
                    "value": "张三"
                }
            ],
            "job_family_id_list": [
                "4719519211875096301"
            ],
            "job_level_id_list": [
                "4719519212005299950"
            ],
            "working_hours_type_id": "6890452208593372679",
            "effective_time": "2020-01-01 00:00:00",
            "expiration_time": "2021-01-01 00:00:00",
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
        let res = serde_json::from_str::<CreateCoreHrJobRespInner>(RESP);
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
