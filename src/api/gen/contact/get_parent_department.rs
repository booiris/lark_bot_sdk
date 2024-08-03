//! doc url: <https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/contact-v3/department/parent>
// Code generated by gen_api. DO NOT EDIT.

use serde::{Deserialize, Serialize};

use crate::api::BaseResp;
use crate::api::HasBaseResp;
use crate::core::model::{ApiRequest, CommonResponse, MethodOption, ReqParam, StreamReqParam};
use crate::{
    core::{http_client::HttpClient, store::Store},
    error::Error,
};

use crate::api::gen::contact::ContactService;

impl<'c, IStore: Store, IClient: HttpClient> ContactService<'c, IStore, IClient> {
    /// **api 版本: 2024-07-05T08:11:27+00:00**
    ///
    /// ## 获取父部门信息
    ///
    /// 调用该接口递归获取指定部门的父部门信息，包括部门名称、ID、负责人以及状态等。
    ///

    ///
    /// doc: <https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/contact-v3/department/parent>
    ///
    /// new doc: <https://open.feishu.cn/document/server-docs/contact-v3/department/parent>
    ///
    /// api url: <https://open.feishu.cn/document_portal/v1/document/get_detail?fullPath=%2Fserver-docs%2Fcontact-v3%2Fdepartment%2Fparent>
    pub async fn get_parent_department(
        &self,
        req: GetParentDepartmentReq,
    ) -> Result<(GetParentDepartmentResp, CommonResponse), Error> {
        self.get_parent_department_with_opt(req, Default::default())
            .await
    }

    /// 参见 [get_parent_department](#method.get_parent_department) 函数
    pub async fn get_parent_department_with_opt(
        &self,
        req: GetParentDepartmentReq,
        method_option: MethodOption,
    ) -> Result<(GetParentDepartmentResp, CommonResponse), Error> {
        #[cfg(feature = "test-util")]
        {
            if let Some(f) = self.mock().get_mock_get_parent_department(&req) {
                tracing::info!("[lark] Contact#GetParentDepartment **mocking** api");
                return f(req);
            }
        }

        tracing::info!("[lark] Contact#GetParentDepartment call api");

        let req = ApiRequest {
            scope: "Contact",
            api: "GetParentDepartment",
            method: http::Method::GET,
            url: String::new()
                + self.cli.open_base_url.as_ref()
                + "/open-apis/contact/v3/departments/parent",
            param_data: req.gen_param(),
            method_option,
            need_tenant_access_token: true,
            need_user_access_token: true,
            ..Default::default()
        };

        let (resp, common_resp): (GetParentDepartmentRespInner, _) = self.cli.do_req(req).await?;
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
pub struct GetParentDepartmentReq {
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
    #[api(
        kind = "query",
        name = "user_id_type",
        v_type = "var",
        option = "false"
    )]
    pub user_id_type: String,
    /// 此次调用中的部门 ID 类型。关于部门 ID 的详细介绍，可参见[部门 ID 说明](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/contact-v3/department/field-overview#23857fe0)。
    ///
    /// **默认值**：open_department_id
    ///
    /// **示例值**: "open_department_id"
    ///
    /// **可选值**:
    ///
    /// `department_id`: 支持用户自定义配置的部门 ID。自定义配置时可复用已删除的 department_id，因此在未删除的部门范围内 department_id 具有唯一性。
    ///
    /// `open_department_id`: 由系统自动生成的部门 ID，ID 前缀固定为 `od-`，在租户内全局唯一。
    #[api(
        kind = "query",
        name = "department_id_type",
        v_type = "var",
        option = "false"
    )]
    pub department_id_type: String,
    /// 部门 ID。ID 类型需要与查询参数 department_id_type 的取值保持一致。
    ///
    /// 当你在创建部门时，可从返回结果中获取到部门 ID 信息，你也可以调用[搜索部门](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/contact-v3/department/search)接口，获取所需的部门 ID。
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "od-4e6ac4d14bcd5071a37a39de902c7141"
    #[api(
        kind = "query",
        name = "department_id",
        v_type = "var",
        option = "false"
    )]
    pub department_id: String,
    /// 分页标记，第一次请求不填，表示从头开始遍历；分页查询结果还有更多项时会同时返回新的 page_token，下次遍历可采用该 page_token 获取查询结果
    ///
    /// **示例值**: "AQD9/Rn9eij9Pm39ED40/RD/cIFmu77WxpxPB/2oHfQLZ%2BG8JG6tK7%2BZnHiT7COhD2hMSICh/eBl7cpzU6JEC3J7COKNe4jrQ8ExwBCR"
    #[api(kind = "query", name = "page_token", v_type = "var", option = "false")]
    pub page_token: String,
    /// 分页大小，用于限制一次请求所返回的数据条目数。
    ///
    /// **示例值**: "10"
    #[api(kind = "query", name = "page_size", v_type = "var", option = "false")]
    pub page_size: i64,
}

#[derive(Debug, Serialize, Deserialize, Clone, lark_bot_sdk_macros::ApiBaseResp)]
struct GetParentDepartmentRespInner {
    #[serde(flatten)]
    data: Option<GetParentDepartmentResp>,
    #[serde(flatten)]
    base: BaseResp,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct GetParentDepartmentResp {
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
    /// **示例值**: "AQD9/Rn9eij9Pm39ED40/RD/cIFmu77WxpxPB/2oHfQLZ%2BG8JG6tK7%2BZnHiT7COhD2hMSICh/eBl7cpzU6JEC3J7COKNe4jrQ8ExwBCR"
    #[serde(
        rename = "page_token",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub page_token: String,
    /// 部门列表。
    #[serde(
        rename = "items",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub items: Vec<DepartmentSubResp>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(default)]
pub struct DepartmentSubResp {
    /// 部门名称。
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "DemoName"
    ///
    /// **数据校验规则**：
    ///
    /// - **最小长度**: `1` 字符
    #[serde(
        rename = "name",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub name: String,
    /// 部门名称的国际化配置。
    #[serde(
        rename = "i18n_name",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub i18n_name: DepartmentI18nNameSubResp,
    /// 父部门的部门 ID。
    ///
    /// - ID 类型与查询参数的 department_id_type 取值保持一致。
    ///
    /// -  当父部门为根部门时，该参数值为 `0`。
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "D067"
    #[serde(
        rename = "parent_department_id",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub parent_department_id: String,
    /// 自定义部门 ID。后续可以使用该 ID 删除、修改、查询部门信息。
    ///
    /// **示例值**: "D096"
    ///
    /// **数据校验规则**：
    ///
    /// - **最大长度**: `64` 字符
    #[serde(
        rename = "department_id",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub department_id: String,
    /// 部门的 open_department_id，由系统自动生成。后续可以使用该 ID 删除、修改、查询部门信息。
    ///
    /// **示例值**: "od-4e6ac4d14bcd5071a37a39de902c7141"
    #[serde(
        rename = "open_department_id",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub open_department_id: String,
    /// 部门主管的用户 ID，ID 类型与查询参数的 user_id_type 取值保持一致。
    ///
    /// **示例值**: "ou_7dab8a3d3cdcc9da365777c7ad535d62"
    #[serde(
        rename = "leader_user_id",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub leader_user_id: String,
    /// 部门群的群 ID。后续可以使用[获取群信息](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/im-v1/chat/get)，获取群的详细信息。
    ///
    /// **示例值**: "oc_5ad11d72b830411d72b836c20"
    #[serde(
        rename = "chat_id",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub chat_id: String,
    /// 部门的排序，即部门在其同级部门的展示顺序。取值越小排序越靠前。
    ///
    /// **示例值**: "100"
    #[serde(
        rename = "order",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub order: String,
    /// 部门绑定的单位自定义 ID 列表，当前只支持一个。
    #[serde(
        rename = "unit_ids",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub unit_ids: Vec<String>,
    /// 当前部门及其下属部门的用户（包含部门负责人）个数。
    ///
    /// **示例值**: "100"
    #[serde(
        rename = "member_count",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub member_count: i64,
    /// 部门状态。
    #[serde(
        rename = "status",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub status: DepartmentStatusSubResp,
    /// 部门负责人信息。
    #[serde(
        rename = "leaders",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub leaders: Vec<DepartmentLeaderSubResp>,
    /// 部门群的人员类型限制。人员类型的可能值如下：
    ///
    /// - 1：正式员工
    ///
    /// - 2：实习生
    ///
    /// - 3：外包
    ///
    /// - 4：劳务
    ///
    /// - 5：顾问
    ///
    /// 如果是自定义人员类型，则会返回对应的编号。你可以调用[查询人员类型](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/contact-v3/employee_type_enum/list)接口，获取相应编号（enum_value）对应的自定义人员类型信息。
    #[serde(
        rename = "group_chat_employee_types",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub group_chat_employee_types: Vec<i64>,
    /// 部门 HRBP 的用户 ID 列表。 ID 类型与查询参数 user_id_type 的取值保持一致。
    ///
    /// **数据校验规则**：
    ///
    /// - **最大长度**: `500` 字符
    #[serde(
        rename = "department_hrbps",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub department_hrbps: Vec<String>,
    /// 当前部门及其下属部门的主属成员（即成员的主部门为当前部门）的数量。
    ///
    /// **示例值**: "100"
    #[serde(
        rename = "primary_member_count",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub primary_member_count: i64,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(default)]
pub struct DepartmentI18nNameSubResp {
    /// 部门的中文名。
    ///
    /// **示例值**: "Demo名称"
    #[serde(
        rename = "zh_cn",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub zh_cn: String,
    /// 部门的日文名。
    ///
    /// **示例值**: "デモ名"
    #[serde(
        rename = "ja_jp",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub ja_jp: String,
    /// 部门的英文名。
    ///
    /// **示例值**: "Demo Name"
    #[serde(
        rename = "en_us",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub en_us: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(default)]
pub struct DepartmentStatusSubResp {
    /// 是否被删除。
    ///
    /// **可能值有：**
    ///
    /// - true：是
    ///
    /// - false：否
    ///
    /// **示例值**: "false"
    #[serde(
        rename = "is_deleted",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub is_deleted: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(default)]
pub struct DepartmentLeaderSubResp {
    /// 负责人类型。
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "1"
    ///
    /// **可选值**:
    ///
    /// `main`: 主负责人
    ///
    /// `deputy`: 副负责人
    #[serde(
        rename = "leaderType",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub leader_type: i64,
    /// 负责人的用户 ID，ID 类型与查询参数的 user_id_type 取值保持一致。
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "ou_7dab8a3d3cdcc9da365777c7ad535d62"
    #[serde(
        rename = "leaderID",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub leader_id: String,
}

#[cfg(feature = "test-util")]
mod test_utils {
    use std::sync::Arc;

    use self::gen::contact::ContactServiceMocker;
    use crate::core::mocker::*;

    use super::*;
    use crate::api::*;
    use crate::core::model::*;

    pub struct Mocker;

    pub trait MockFunc:
        Fn(GetParentDepartmentReq) -> Result<(GetParentDepartmentResp, CommonResponse), Error>
        + Send
        + Sync
        + 'static
    {
    }
    impl<
            T: Fn(
                    GetParentDepartmentReq,
                ) -> Result<(GetParentDepartmentResp, CommonResponse), Error>
                + Send
                + Sync
                + 'static,
        > MockFunc for T
    {
    }

    impl<'c, IStore: Store, IClient: HttpClient> ContactServiceMocker<'c, IStore, IClient> {
        pub fn mock_get_parent_department<F: MockFunc>(
            &self,
            f: F,
        ) -> MockerBuilder<Mocker, GetParentDepartmentReq, GetParentDepartmentResp, Arc<dyn MockFunc>>
        {
            MockerBuilder::new(self.cli.instance_id, Arc::new(f))
        }

        pub(super) fn get_mock_get_parent_department(
            &self,
            req: &GetParentDepartmentReq,
        ) -> Option<Arc<dyn MockFunc>> {
            do_mock::<Mocker, GetParentDepartmentReq, GetParentDepartmentResp, Arc<dyn MockFunc>>(
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
        api::gen::contact::get_parent_department::{
            GetParentDepartmentReq, GetParentDepartmentResp, GetParentDepartmentRespInner,
        },
        core::{model::CommonResponse, Lark},
    };

    #[tokio::test]
    async fn test_mock() {
        let lark = Lark::new("".to_owned(), "".into());
        let mocker = lark
            .contact()
            .mock()
            .mock_get_parent_department(|_| {
                Ok((
                    GetParentDepartmentResp::default(),
                    CommonResponse::default(),
                ))
            })
            .build();
        let res = lark
            .contact()
            .get_parent_department(GetParentDepartmentReq::default())
            .await;
        assert!(res.is_ok());
        mocker.clear();
        let res = lark
            .contact()
            .get_parent_department(GetParentDepartmentReq::default())
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
        "has_more": true,
        "page_token": "AQD9/Rn9eij9Pm39ED40/RD/cIFmu77WxpxPB/2oHfQLZ%2BG8JG6tK7%2BZnHiT7COhD2hMSICh/eBl7cpzU6JEC3J7COKNe4jrQ8ExwBCR",
        "items": [
            {
                "name": "DemoName",
                "i18n_name": {
                    "zh_cn": "Demo名称",
                    "ja_jp": "デモ名",
                    "en_us": "Demo Name"
                },
                "parent_department_id": "D067",
                "department_id": "D096",
                "open_department_id": "od-4e6ac4d14bcd5071a37a39de902c7141",
                "leader_user_id": "ou_7dab8a3d3cdcc9da365777c7ad535d62",
                "chat_id": "oc_5ad11d72b830411d72b836c20",
                "order": "100",
                "unit_ids": [
                    "custom_unit_id"
                ],
                "member_count": 100,
                "status": {
                    "is_deleted": false
                },
                "leaders": [
                    {
                        "leaderType": 1,
                        "leaderID": "ou_7dab8a3d3cdcc9da365777c7ad535d62"
                    }
                ],
                "group_chat_employee_types": [
                    1
                ],
                "department_hrbps": [
                    "ou_7dab8a3d3cdcc9da365777c7ad535d62"
                ],
                "primary_member_count": 100
            }
        ]
    }
}"#;
    #[test]
    fn test_resp() {
        let res = serde_json::from_str::<GetParentDepartmentRespInner>(RESP);
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
