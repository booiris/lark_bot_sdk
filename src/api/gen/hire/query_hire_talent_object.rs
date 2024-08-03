//! doc url: <https://open.feishu.cn/document/ukTMukTMukTM/uMzM1YjLzMTN24yMzUjN/hire-v1/talent_object/query>
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
    /// **api 版本: 2024-07-10T14:31:11+00:00**
    ///
    /// ## 获取人才字段
    ///
    /// 获取人才字段。
    ///

    ///
    /// doc: <https://open.feishu.cn/document/ukTMukTMukTM/uMzM1YjLzMTN24yMzUjN/hire-v1/talent_object/query>
    ///
    /// new doc: <https://open.feishu.cn/document/server-docs/hire-v1/candidate-management/talent/query>
    ///
    /// api url: <https://open.feishu.cn/document_portal/v1/document/get_detail?fullPath=%2Fserver-docs%2Fhire-v1%2Fcandidate-management%2Ftalent%2Fquery>
    pub async fn query_hire_talent_object(
        &self,
        req: QueryHireTalentObjectReq,
    ) -> Result<(QueryHireTalentObjectResp, CommonResponse), Error> {
        self.query_hire_talent_object_with_opt(req, Default::default())
            .await
    }

    /// 参见 [query_hire_talent_object](#method.query_hire_talent_object) 函数
    pub async fn query_hire_talent_object_with_opt(
        &self,
        req: QueryHireTalentObjectReq,
        method_option: MethodOption,
    ) -> Result<(QueryHireTalentObjectResp, CommonResponse), Error> {
        #[cfg(feature = "test-util")]
        {
            if let Some(f) = self.mock().get_mock_query_hire_talent_object(&req) {
                tracing::info!("[lark] Hire#QueryHireTalentObject **mocking** api");
                return f(req);
            }
        }

        tracing::info!("[lark] Hire#QueryHireTalentObject call api");

        let req = ApiRequest {
            scope: "Hire",
            api: "QueryHireTalentObject",
            method: http::Method::GET,
            url: String::new()
                + self.cli.open_base_url.as_ref()
                + "/open-apis/hire/v1/talent_objects/query",
            param_data: req.gen_param(),
            method_option,
            need_tenant_access_token: true,
            ..Default::default()
        };

        let (resp, common_resp): (QueryHireTalentObjectRespInner, _) = self.cli.do_req(req).await?;
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
pub struct QueryHireTalentObjectReq {}

#[derive(Debug, Serialize, Deserialize, Clone, lark_bot_sdk_macros::ApiBaseResp)]
struct QueryHireTalentObjectRespInner {
    #[serde(flatten)]
    data: Option<QueryHireTalentObjectResp>,
    #[serde(flatten)]
    base: BaseResp,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct QueryHireTalentObjectResp {
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
    /// 数据列表
    #[serde(
        rename = "items",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub items: Vec<CommonSchemaSubResp>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(default)]
pub struct CommonSchemaSubResp {
    /// 模块 ID
    ///
    /// **示例值**: "6949805467799537964"
    #[serde(
        rename = "id",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub id: String,
    /// 模块名称
    #[serde(
        rename = "name",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub name: I18nSubResp,
    /// 模块描述
    #[serde(
        rename = "description",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub description: I18nSubResp,
    /// 模块信息
    #[serde(
        rename = "setting",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub setting: CommonSchemaSettingSubResp,
    /// 是否是自定义模块
    ///
    /// **示例值**: "true"
    #[serde(
        rename = "is_customized",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub is_customized: bool,
    /// 是否必填
    ///
    /// **示例值**: "false"
    #[serde(
        rename = "is_required",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub is_required: bool,
    /// 是否启用
    ///
    /// **示例值**: "1"
    ///
    /// **可选值**:
    ///
    /// `Active`: 已启用
    ///
    /// `Disable`: 已停用
    #[serde(
        rename = "active_status",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub active_status: i64,
    /// 字段列表
    #[serde(
        rename = "children_list",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub children_list: Vec<CommonSchemaChildSubResp>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(default)]
pub struct I18nSubResp {
    /// 中文
    ///
    /// **示例值**: "测试"
    #[serde(
        rename = "zh_cn",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub zh_cn: String,
    /// 英文
    ///
    /// **示例值**: "test"
    #[serde(
        rename = "en_us",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub en_us: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(default)]
pub struct CommonSchemaSettingSubResp {
    /// 字段类型
    ///
    /// **示例值**: "1"
    ///
    /// **可选值**:
    ///
    /// `Text`: 单行文本
    ///
    /// `LongText`: 多行文本
    ///
    /// `Select`: 单选
    ///
    /// `MultiSelect`: 多选
    ///
    /// `DateSelect`: 日期
    ///
    /// `MonthSelect`: 月份选择
    ///
    /// `YearSelect`: 年份选择
    ///
    /// `DateRange`: 时间段
    ///
    /// `Number`: 数字
    ///
    /// `Default`: 默认字段
    ///
    /// `Group`: 模块
    ///
    /// `Attachment`: 附件
    #[serde(
        rename = "object_type",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub object_type: i64,
    /// 配置信息
    #[serde(
        rename = "config",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub config: CommonSchemaConfigSubResp,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(default)]
pub struct CommonSchemaConfigSubResp {
    /// 选项信息
    #[serde(
        rename = "options",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub options: Vec<CommonSchemaOptionSubResp>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(default)]
pub struct CommonSchemaChildSubResp {
    /// 字段 ID
    ///
    /// **示例值**: "6949805467799537964"
    #[serde(
        rename = "id",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub id: String,
    /// 字段名称
    #[serde(
        rename = "name",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub name: I18nSubResp,
    /// 字段描述
    #[serde(
        rename = "description",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub description: I18nSubResp,
    /// 字段信息
    #[serde(
        rename = "setting",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub setting: CommonSchemaSettingSubResp,
    /// 所属模块 ID
    ///
    /// **示例值**: "6949805467799537964"
    #[serde(
        rename = "parent_id",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub parent_id: String,
    /// 是否是自定义字段
    ///
    /// **示例值**: "true"
    #[serde(
        rename = "is_customized",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub is_customized: bool,
    /// 是否必填
    ///
    /// **示例值**: "false"
    #[serde(
        rename = "is_required",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub is_required: bool,
    /// 是否启用
    ///
    /// **示例值**: "1"
    ///
    /// **可选值**:
    ///
    /// `Active`: 已启用
    ///
    /// `Disable`: 已停用
    #[serde(
        rename = "active_status",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub active_status: i64,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(default)]
pub struct CommonSchemaOptionSubResp {
    /// 选项 ID
    ///
    /// **示例值**: "test"
    #[serde(
        rename = "key",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub key: String,
    /// 选项名称
    #[serde(
        rename = "name",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub name: I18nSubResp,
    /// 选项描述
    #[serde(
        rename = "description",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub description: I18nSubResp,
    /// 是否启用
    ///
    /// **示例值**: "1"
    ///
    /// **可选值**:
    ///
    /// `Active`: 已启用
    ///
    /// `Disable`: 已停用
    #[serde(
        rename = "active_status",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub active_status: i64,
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
        Fn(QueryHireTalentObjectReq) -> Result<(QueryHireTalentObjectResp, CommonResponse), Error>
        + Send
        + Sync
        + 'static
    {
    }
    impl<
            T: Fn(
                    QueryHireTalentObjectReq,
                ) -> Result<(QueryHireTalentObjectResp, CommonResponse), Error>
                + Send
                + Sync
                + 'static,
        > MockFunc for T
    {
    }

    impl<'c, IStore: Store, IClient: HttpClient> HireServiceMocker<'c, IStore, IClient> {
        pub fn mock_query_hire_talent_object<F: MockFunc>(
            &self,
            f: F,
        ) -> MockerBuilder<
            Mocker,
            QueryHireTalentObjectReq,
            QueryHireTalentObjectResp,
            Arc<dyn MockFunc>,
        > {
            MockerBuilder::new(self.cli.instance_id, Arc::new(f))
        }

        pub(super) fn get_mock_query_hire_talent_object(
            &self,
            req: &QueryHireTalentObjectReq,
        ) -> Option<Arc<dyn MockFunc>> {
            do_mock::<Mocker, QueryHireTalentObjectReq, QueryHireTalentObjectResp, Arc<dyn MockFunc>>(
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
        api::gen::hire::query_hire_talent_object::{
            QueryHireTalentObjectReq, QueryHireTalentObjectResp, QueryHireTalentObjectRespInner,
        },
        core::{model::CommonResponse, Lark},
    };

    #[tokio::test]
    async fn test_mock() {
        let lark = Lark::new("".to_owned(), "".into());
        let mocker = lark
            .hire()
            .mock()
            .mock_query_hire_talent_object(|_| {
                Ok((
                    QueryHireTalentObjectResp::default(),
                    CommonResponse::default(),
                ))
            })
            .build();
        let res = lark
            .hire()
            .query_hire_talent_object(QueryHireTalentObjectReq::default())
            .await;
        assert!(res.is_ok());
        mocker.clear();
        let res = lark
            .hire()
            .query_hire_talent_object(QueryHireTalentObjectReq::default())
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
                "id": "6949805467799537964",
                "name": {
                    "zh_cn": "测试",
                    "en_us": "test"
                },
                "description": {
                    "zh_cn": "测试",
                    "en_us": "test"
                },
                "setting": {
                    "object_type": 1,
                    "config": {
                        "options": [
                            {
                                "key": "test",
                                "name": {
                                    "zh_cn": "测试",
                                    "en_us": "test"
                                },
                                "description": {
                                    "zh_cn": "测试",
                                    "en_us": "test"
                                },
                                "active_status": 1
                            }
                        ]
                    }
                },
                "is_customized": true,
                "is_required": false,
                "active_status": 1,
                "children_list": [
                    {
                        "id": "6949805467799537964",
                        "name": {
                            "zh_cn": "测试",
                            "en_us": "test"
                        },
                        "description": {
                            "zh_cn": "测试",
                            "en_us": "test"
                        },
                        "setting": {
                            "object_type": 1,
                            "config": {
                                "options": [
                                    {
                                        "key": "test",
                                        "name": {
                                            "zh_cn": "测试",
                                            "en_us": "test"
                                        },
                                        "description": {
                                            "zh_cn": "测试",
                                            "en_us": "test"
                                        },
                                        "active_status": 1
                                    }
                                ]
                            }
                        },
                        "parent_id": "6949805467799537964",
                        "is_customized": true,
                        "is_required": false,
                        "active_status": 1
                    }
                ]
            }
        ]
    }
}"#;
    #[test]
    fn test_resp() {
        let res = serde_json::from_str::<QueryHireTalentObjectRespInner>(RESP);
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
