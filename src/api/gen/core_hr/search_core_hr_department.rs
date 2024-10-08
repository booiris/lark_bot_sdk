//! doc url: <https://open.larkoffice.com/document/uAjLw4CM/ukTMukTMukTM/corehr-v2/department/search>
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
    /// **api 版本: 2024-03-22T02:02:11+00:00**
    ///
    /// ## 搜索部门信息
    ///
    /// 搜索部门信息
    ///

    ///
    /// doc: <https://open.larkoffice.com/document/uAjLw4CM/ukTMukTMukTM/corehr-v2/department/search>
    ///
    /// new doc: <https://open.larkoffice.com/document/server-docs/corehr-v1/organization-management/department/search>
    ///
    /// api url: <https://open.feishu.cn/document_portal/v1/document/get_detail?fullPath=%2Fserver-docs%2Fcorehr-v1%2Forganization-management%2Fdepartment%2Fsearch>
    pub async fn search_core_hr_department(
        &self,
        req: SearchCoreHrDepartmentReq,
    ) -> Result<(SearchCoreHrDepartmentResp, CommonResponse), Error> {
        self.search_core_hr_department_with_opt(req, Default::default())
            .await
    }

    /// 参见 [search_core_hr_department](#method.search_core_hr_department) 函数
    pub async fn search_core_hr_department_with_opt(
        &self,
        req: SearchCoreHrDepartmentReq,
        method_option: MethodOption,
    ) -> Result<(SearchCoreHrDepartmentResp, CommonResponse), Error> {
        #[cfg(feature = "test-util")]
        {
            if let Some(f) = self.mock().get_mock_search_core_hr_department(&req) {
                tracing::info!("[lark] CoreHr#SearchCoreHrDepartment **mocking** api");
                return f(req);
            }
        }

        tracing::info!("[lark] CoreHr#SearchCoreHrDepartment call api");

        let req = ApiRequest {
            scope: "CoreHr",
            api: "SearchCoreHrDepartment",
            method: http::Method::POST,
            url: String::new()
                + self.cli.open_base_url.as_ref()
                + "/open-apis/corehr/v2/departments/search",
            param_data: req.gen_param(),
            method_option,
            need_tenant_access_token: true,
            ..Default::default()
        };

        let (resp, common_resp): (SearchCoreHrDepartmentRespInner, _) =
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
pub struct SearchCoreHrDepartmentReq {
    /// 分页大小
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "100"
    #[api(kind = "query", name = "page_size", v_type = "var", option = "false")]
    pub page_size: i64,
    /// 分页标记，第一次请求不填，表示从头开始遍历；分页查询结果还有更多项时会同时返回新的 page_token，下次遍历可采用该 page_token 获取查询结果
    ///
    /// **示例值**: "6891251722631890445"
    #[api(kind = "query", name = "page_token", v_type = "var", option = "false")]
    pub page_token: String,
    /// 用户 ID 类型
    ///
    /// **示例值**: "open_id"
    ///
    /// **可选值**:
    ///
    /// `open_id`: 标识一个用户在某个应用中的身份。同一个用户在不同应用中的 Open ID 不同。[了解更多：如何获取 Open ID](https://open.feishu.cn/document/uAjLw4CM/ugTN1YjL4UTN24CO1UjN/trouble-shooting/how-to-obtain-openid)
    ///
    /// `union_id`: 标识一个用户在某个应用开发商下的身份。同一用户在同一开发商下的应用中的 Union ID 是相同的，在不同开发商下的应用中的 Union ID 是不同的。通过 Union ID，应用开发商可以把同个用户在多个应用中的身份关联起来。[了解更多：如何获取 Union ID？](https://open.feishu.cn/document/uAjLw4CM/ugTN1YjL4UTN24CO1UjN/trouble-shooting/how-to-obtain-union-id)
    ///
    /// `user_id`: 标识一个用户在某个租户内的身份。同一个用户在租户 A 和租户 B 内的 User ID 是不同的。在同一个租户内，一个用户的 User ID 在所有应用（包括商店应用）中都保持一致。User ID 主要用于在不同的应用间打通用户数据。[了解更多：如何获取 User ID？](https://open.feishu.cn/document/uAjLw4CM/ugTN1YjL4UTN24CO1UjN/trouble-shooting/how-to-obtain-user-id)
    ///
    /// `people_corehr_id`: 以飞书人事的 ID 来识别用户
    #[api(
        kind = "query",
        name = "user_id_type",
        v_type = "var",
        option = "false"
    )]
    pub user_id_type: String,
    /// 此次调用中使用的部门 ID 类型
    ///
    /// **示例值**: "open_department_id"
    ///
    /// **可选值**:
    ///
    /// `open_department_id`: 以 open_department_id 来标识部门
    ///
    /// `department_id`: 以 department_id 来标识部门
    ///
    /// `people_corehr_department_id`: 以 people_corehr_department_id 来标识部门
    #[api(
        kind = "query",
        name = "department_id_type",
        v_type = "var",
        option = "false"
    )]
    pub department_id_type: String,
    /// 是否启用
    ///
    /// **示例值**: "true"
    #[api(kind = "body", name = "active")]
    pub active: Option<bool>,
    /// 当通过上级部门 ID 查询时，填写 true 返回所有子部门，填写 false 只返回直接下级部门
    ///
    /// **示例值**: "false"
    #[api(kind = "body", name = "get_all_children")]
    pub get_all_children: Option<bool>,
    /// manager ID 列表，按部门直接负责人搜索
    ///
    /// <br><b>字段权限要求：</b>
    ///
    /// <br>按照部门负责人搜索 (corehr:department.manager.search:read)
    ///
    /// **数据校验规则**：
    ///
    /// - **最大长度**: `100` 字符
    #[api(kind = "body", name = "manager_list")]
    pub manager_list: Vec<Option<String>>,
    /// 部门 ID 列表
    #[api(kind = "body", name = "department_id_list")]
    pub department_id_list: Vec<Option<String>>,
    /// 部门名称列表，需精确匹配
    #[api(kind = "body", name = "name_list")]
    pub name_list: Vec<Option<String>>,
    /// 上级部门 ID ，可查询直接下级部门
    ///
    /// <br><b>字段权限要求：</b>
    ///
    /// <br>按照上级部门搜索(corehr:department.organize.search:read)
    ///
    /// **示例值**: "7094136522860922222"
    #[api(kind = "body", name = "parent_department_id")]
    pub parent_department_id: Option<String>,
    /// 部门 code 列表
    #[api(kind = "body", name = "code_list")]
    pub code_list: Vec<Option<String>>,
    /// 返回数据的字段列表，为空时不返回任何字段
    #[api(kind = "body", name = "fields")]
    pub fields: Vec<Option<String>>,
}

#[derive(Debug, Serialize, Deserialize, Clone, lark_bot_sdk_macros::ApiBaseResp)]
struct SearchCoreHrDepartmentRespInner {
    #[serde(flatten)]
    data: Option<SearchCoreHrDepartmentResp>,
    #[serde(flatten)]
    base: BaseResp,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct SearchCoreHrDepartmentResp {
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
    /// 查询的部门信息
    #[serde(
        rename = "items",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub items: Vec<DepartmentSubResp>,
    /// 分页标记，当 has_more 为 true 时，会同时返回新的 page_token，否则不返回 page_token
    ///
    /// **示例值**: "eyJldV9uYyI6IlswLFwiNjk2MTI4Njg0NjA5Mzc4ODY4MC03MjExMDM0ODcxMjA3OTUzOTc1XCJdIn0"
    #[serde(
        rename = "page_token",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub page_token: String,
    /// 是否还有更多项
    ///
    /// **示例值**: "true"
    #[serde(
        rename = "has_more",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub has_more: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(default)]
pub struct DepartmentSubResp {
    /// 部门 ID
    ///
    /// **示例值**: "4719456877659520852"
    #[serde(
        rename = "id",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub id: String,
    /// 部门记录版本 ID
    ///
    /// **示例值**: "6890452208593372611"
    #[serde(
        rename = "version_id",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub version_id: String,
    /// 部门名称
    #[serde(
        rename = "department_name",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub department_name: Vec<I18nSubResp>,
    /// 部门类型，枚举值 api_name 可通过[【获取字段详情】](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/corehr-v1/custom_field/get_by_param)接口查询，查询参数如下：
    ///
    /// - object_api_name = "department"
    ///
    /// - custom_api_name = "subtype"
    #[serde(
        rename = "sub_type",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub sub_type: EnumSubResp,
    /// 上级部门 ID
    ///
    /// **示例值**: "4719456877659520111"
    #[serde(
        rename = "parent_department_id",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub parent_department_id: String,
    /// 部门负责人雇佣 ID，枚举值及详细信息可通过[【搜索员工信息】](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/corehr-v2/employee/search)接口查询获得
    ///
    /// **示例值**: "6893013238632416777"
    #[serde(
        rename = "manager",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub manager: String,
    /// 树形排序，代表同层级的部门排序序号
    ///
    /// **示例值**: "001000"
    #[serde(
        rename = "tree_order",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub tree_order: String,
    /// 列表排序，代表所有部门的混排序号
    ///
    /// **示例值**: "001000-001000"
    #[serde(
        rename = "list_order",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub list_order: String,
    /// 编码
    ///
    /// **示例值**: "D00000456"
    #[serde(
        rename = "code",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub code: String,
    /// 是否根部门
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "false"
    #[serde(
        rename = "is_root",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub is_root: bool,
    /// 是否保密
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "false"
    #[serde(
        rename = "is_confidential",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub is_confidential: bool,
    /// 生效日期
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "2020-05-01"
    #[serde(
        rename = "effective_date",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub effective_date: String,
    /// 失效日期
    ///
    /// **示例值**: "2020-05-02"
    #[serde(
        rename = "expiration_date",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub expiration_date: String,
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
    /// 描述
    #[serde(
        rename = "description",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub description: Vec<I18nSubResp>,
    /// 自定义字段
    #[serde(
        rename = "custom_fields",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub custom_fields: Vec<CustomFieldDataSubResp>,
    /// 是否使用职务
    ///
    /// （功能灰度中，暂未开放）
    #[serde(
        rename = "staffing_model",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub staffing_model: EnumSubResp,
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
        Fn(SearchCoreHrDepartmentReq) -> Result<(SearchCoreHrDepartmentResp, CommonResponse), Error>
        + Send
        + Sync
        + 'static
    {
    }
    impl<
            T: Fn(
                    SearchCoreHrDepartmentReq,
                ) -> Result<(SearchCoreHrDepartmentResp, CommonResponse), Error>
                + Send
                + Sync
                + 'static,
        > MockFunc for T
    {
    }

    impl<'c, IStore: Store, IClient: HttpClient> CoreHrServiceMocker<'c, IStore, IClient> {
        pub fn mock_search_core_hr_department<F: MockFunc>(
            &self,
            f: F,
        ) -> MockerBuilder<
            Mocker,
            SearchCoreHrDepartmentReq,
            SearchCoreHrDepartmentResp,
            Arc<dyn MockFunc>,
        > {
            MockerBuilder::new(self.cli.instance_id, Arc::new(f))
        }

        pub(super) fn get_mock_search_core_hr_department(
            &self,
            req: &SearchCoreHrDepartmentReq,
        ) -> Option<Arc<dyn MockFunc>> {
            do_mock::<
                Mocker,
                SearchCoreHrDepartmentReq,
                SearchCoreHrDepartmentResp,
                Arc<dyn MockFunc>,
            >(self.cli.instance_id, req)
        }
    }
}

#[cfg(feature = "test-util")]
#[cfg(test)]
mod test {
    use crate::{
        api::gen::core_hr::search_core_hr_department::{
            SearchCoreHrDepartmentReq, SearchCoreHrDepartmentResp, SearchCoreHrDepartmentRespInner,
        },
        core::{model::CommonResponse, Lark},
    };

    #[tokio::test]
    async fn test_mock() {
        let lark = Lark::new("".to_owned(), "".into());
        let mocker = lark
            .core_hr()
            .mock()
            .mock_search_core_hr_department(|_| {
                Ok((
                    SearchCoreHrDepartmentResp::default(),
                    CommonResponse::default(),
                ))
            })
            .build();
        let res = lark
            .core_hr()
            .search_core_hr_department(SearchCoreHrDepartmentReq::default())
            .await;
        assert!(res.is_ok());
        mocker.clear();
        let res = lark
            .core_hr()
            .search_core_hr_department(SearchCoreHrDepartmentReq::default())
            .await;
        assert!(res.is_err());
    }

    const REQ: &str = r#"{
    "active": true,
    "get_all_children": false,
    "manager_list": [
        "7094136522860922112"
    ],
    "department_id_list": [
        "7094136522860922111"
    ],
    "name_list": [
        "后端研发部"
    ],
    "parent_department_id": "7094136522860922222",
    "code_list": [
        "D00000123"
    ],
    "fields": [
        "department_name"
    ]
}"#;

    #[test]
    fn test_req() {
        if REQ == "{}" {
            return;
        }
        if let Err(e) = serde_json::from_str::<super::SearchCoreHrDepartmentReqBody>(REQ) {
            panic!("{}", e);
        }
    }

    const RESP: &str = r#"{
    "code": 0,
    "msg": "success",
    "data": {
        "items": [
            {
                "id": "4719456877659520852",
                "version_id": "6890452208593372611",
                "department_name": [
                    {
                        "lang": "zh-CN",
                        "value": "张三"
                    }
                ],
                "sub_type": {
                    "enum_name": "phone_type",
                    "display": [
                        {
                            "lang": "zh-CN",
                            "value": "张三"
                        }
                    ]
                },
                "parent_department_id": "4719456877659520111",
                "manager": "6893013238632416777",
                "tree_order": "001000",
                "list_order": "001000-001000",
                "code": "D00000456",
                "is_root": false,
                "is_confidential": false,
                "effective_date": "2020-05-01",
                "expiration_date": "2020-05-02",
                "active": true,
                "description": [
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
                ],
                "staffing_model": {
                    "enum_name": "phone_type",
                    "display": [
                        {
                            "lang": "zh-CN",
                            "value": "张三"
                        }
                    ]
                }
            }
        ],
        "page_token": "eyJldV9uYyI6IlswLFwiNjk2MTI4Njg0NjA5Mzc4ODY4MC03MjExMDM0ODcxMjA3OTUzOTc1XCJdIn0",
        "has_more": true
    }
}"#;
    #[test]
    fn test_resp() {
        let res = serde_json::from_str::<SearchCoreHrDepartmentRespInner>(RESP);
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
