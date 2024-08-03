//! doc url: <https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/corehr-v1/job/get>
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
    /// **api 版本: 2024-02-06T02:30:00+00:00**
    ///
    /// ## 查询单个职务
    ///
    /// 根据 ID 查询单个职务。
    ///

    ///
    /// doc: <https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/corehr-v1/job/get>
    ///
    /// new doc: <https://open.feishu.cn/document/server-docs/corehr-v1/job-management/job/get>
    ///
    /// api url: <https://open.feishu.cn/document_portal/v1/document/get_detail?fullPath=%2Fserver-docs%2Fcorehr-v1%2Fjob-management%2Fjob%2Fget>
    pub async fn get_core_hr_job(
        &self,
        req: GetCoreHrJobReq,
    ) -> Result<(GetCoreHrJobResp, CommonResponse), Error> {
        self.get_core_hr_job_with_opt(req, Default::default()).await
    }

    /// 参见 [get_core_hr_job](#method.get_core_hr_job) 函数
    pub async fn get_core_hr_job_with_opt(
        &self,
        req: GetCoreHrJobReq,
        method_option: MethodOption,
    ) -> Result<(GetCoreHrJobResp, CommonResponse), Error> {
        #[cfg(feature = "test-util")]
        {
            if let Some(f) = self.mock().get_mock_get_core_hr_job(&req) {
                tracing::info!("[lark] CoreHr#GetCoreHrJob **mocking** api");
                return f(req);
            }
        }

        tracing::info!("[lark] CoreHr#GetCoreHrJob call api");

        let req = ApiRequest {
            scope: "CoreHr",
            api: "GetCoreHrJob",
            method: http::Method::GET,
            url: String::new()
                + self.cli.open_base_url.as_ref()
                + "/open-apis/corehr/v1/jobs/:job_id",
            param_data: req.gen_param(),
            method_option,
            need_tenant_access_token: true,
            ..Default::default()
        };

        let (resp, common_resp): (GetCoreHrJobRespInner, _) = self.cli.do_req(req).await?;
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
pub struct GetCoreHrJobReq {
    /// 职务 ID
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "151515"
    #[api(kind = "path", name = "job_id")]
    pub job_id: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, lark_bot_sdk_macros::ApiBaseResp)]
struct GetCoreHrJobRespInner {
    #[serde(flatten)]
    data: Option<GetCoreHrJobResp>,
    #[serde(flatten)]
    base: BaseResp,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct GetCoreHrJobResp {
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
    /// 职务信息
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
    #[serde(
        rename = "job_family_id_list",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub job_family_id_list: Vec<String>,
    /// 职务级别 ID 列表，枚举值及详细信息可通过【批量查询职务级别】接口查询获得
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
        Fn(GetCoreHrJobReq) -> Result<(GetCoreHrJobResp, CommonResponse), Error>
        + Send
        + Sync
        + 'static
    {
    }
    impl<
            T: Fn(GetCoreHrJobReq) -> Result<(GetCoreHrJobResp, CommonResponse), Error>
                + Send
                + Sync
                + 'static,
        > MockFunc for T
    {
    }

    impl<'c, IStore: Store, IClient: HttpClient> CoreHrServiceMocker<'c, IStore, IClient> {
        pub fn mock_get_core_hr_job<F: MockFunc>(
            &self,
            f: F,
        ) -> MockerBuilder<Mocker, GetCoreHrJobReq, GetCoreHrJobResp, Arc<dyn MockFunc>> {
            MockerBuilder::new(self.cli.instance_id, Arc::new(f))
        }

        pub(super) fn get_mock_get_core_hr_job(
            &self,
            req: &GetCoreHrJobReq,
        ) -> Option<Arc<dyn MockFunc>> {
            do_mock::<Mocker, GetCoreHrJobReq, GetCoreHrJobResp, Arc<dyn MockFunc>>(
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
        api::gen::core_hr::get_core_hr_job::{
            GetCoreHrJobReq, GetCoreHrJobResp, GetCoreHrJobRespInner,
        },
        core::{model::CommonResponse, Lark},
    };

    #[tokio::test]
    async fn test_mock() {
        let lark = Lark::new("".to_owned(), "".into());
        let mocker = lark
            .core_hr()
            .mock()
            .mock_get_core_hr_job(|_| Ok((GetCoreHrJobResp::default(), CommonResponse::default())))
            .build();
        let res = lark
            .core_hr()
            .get_core_hr_job(GetCoreHrJobReq::default())
            .await;
        assert!(res.is_ok());
        mocker.clear();
        let res = lark
            .core_hr()
            .get_core_hr_job(GetCoreHrJobReq::default())
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
        let res = serde_json::from_str::<GetCoreHrJobRespInner>(RESP);
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
