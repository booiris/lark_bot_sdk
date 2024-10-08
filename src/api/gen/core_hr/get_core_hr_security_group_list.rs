//! doc url: <https://open.larkoffice.com/document/uAjLw4CM/ukTMukTMukTM/reference/corehr-v1/security_group/list>
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
    /// **api 版本: 2024-07-29T07:17:31+00:00**
    ///
    /// ## 批量获取角色列表
    ///
    /// 用于查询飞书人事中的角色列表（对应[飞书人事管理后台](https://people.feishu.cn/people/) - 设置 - 权限设置 - 角色设置中的角色列表），列表内包含角色 ID、名称、状态以及描述等信息。
    ///

    ///
    /// doc: <https://open.larkoffice.com/document/uAjLw4CM/ukTMukTMukTM/reference/corehr-v1/security_group/list>
    ///
    /// new doc: <https://open.larkoffice.com/document/server-docs/corehr-v1/authorization/list>
    ///
    /// api url: <https://open.feishu.cn/document_portal/v1/document/get_detail?fullPath=%2Fserver-docs%2Fcorehr-v1%2Fauthorization%2Flist>
    pub async fn get_core_hr_security_group_list(
        &self,
        req: GetCoreHrSecurityGroupListReq,
    ) -> Result<(GetCoreHrSecurityGroupListResp, CommonResponse), Error> {
        self.get_core_hr_security_group_list_with_opt(req, Default::default())
            .await
    }

    /// 参见 [get_core_hr_security_group_list](#method.get_core_hr_security_group_list) 函数
    pub async fn get_core_hr_security_group_list_with_opt(
        &self,
        req: GetCoreHrSecurityGroupListReq,
        method_option: MethodOption,
    ) -> Result<(GetCoreHrSecurityGroupListResp, CommonResponse), Error> {
        #[cfg(feature = "test-util")]
        {
            if let Some(f) = self.mock().get_mock_get_core_hr_security_group_list(&req) {
                tracing::info!("[lark] CoreHr#GetCoreHrSecurityGroupList **mocking** api");
                return f(req);
            }
        }

        tracing::info!("[lark] CoreHr#GetCoreHrSecurityGroupList call api");

        let req = ApiRequest {
            scope: "CoreHr",
            api: "GetCoreHrSecurityGroupList",
            method: http::Method::GET,
            url: String::new()
                + self.cli.open_base_url.as_ref()
                + "/open-apis/corehr/v1/security_groups",
            param_data: req.gen_param(),
            method_option,
            need_tenant_access_token: true,
            ..Default::default()
        };

        let (resp, common_resp): (GetCoreHrSecurityGroupListRespInner, _) =
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
pub struct GetCoreHrSecurityGroupListReq {
    /// 分页标记，第一次请求不填，表示从头开始遍历；分页查询结果还有更多项时会同时返回新的 page_token，下次遍历可采用该 page_token 获取查询结果
    ///
    /// **示例值**: "10"
    #[api(kind = "query", name = "page_token", v_type = "var", option = "false")]
    pub page_token: String,
    /// 分页大小
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "100"
    #[api(kind = "query", name = "page_size", v_type = "var", option = "false")]
    pub page_size: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, lark_bot_sdk_macros::ApiBaseResp)]
struct GetCoreHrSecurityGroupListRespInner {
    #[serde(flatten)]
    data: Option<GetCoreHrSecurityGroupListResp>,
    #[serde(flatten)]
    base: BaseResp,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct GetCoreHrSecurityGroupListResp {
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
    /// 查询的用户角色信息
    #[serde(
        rename = "items",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub items: Vec<SecurityGroupSubResp>,
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
pub struct SecurityGroupSubResp {
    /// 角色ID
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "7034393015968122400"
    #[serde(
        rename = "id",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub id: String,
    /// 角色code，通常用于与其他系统进行交互
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "department_manager"
    #[serde(
        rename = "code",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub code: String,
    /// 角色名称
    #[serde(
        rename = "name",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub name: NameSubResp,
    /// 状态，可能值有：
    ///
    /// - 1 = Inactive / 停用
    ///
    /// - 2 = Active / 启用
    ///
    /// - 3 = TobeActivated / 待启用
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "1"
    #[serde(
        rename = "active_status",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub active_status: i64,
    /// 角色描述
    #[serde(
        rename = "description",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub description: NameSubResp,
    /// 角色类型
    ///
    /// - 3 = 组织类角色
    ///
    /// - 7 = 非组织类角色
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "1"
    #[serde(
        rename = "group_type",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub group_type: i64,
    /// 创建人
    ///
    /// - 返回"sys"时，表示角色是系统创建角色
    ///
    /// - 返回用户ID时，表示是角色是用户自定义角色，可以使用 [ID转换服务](https://open.larkoffice.com/document/uAjLw4CM/ukTMukTMukTM/reference/corehr-v1/common_data-id/convert) 换取 飞书人事的employment_id
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "6967639606963471902"
    #[serde(
        rename = "created_by",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub created_by: String,
    /// 更新时间
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "1"
    #[serde(
        rename = "update_time",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub update_time: String,
    /// 组织管理维度
    #[serde(
        rename = "org_truncation",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub org_truncation: Vec<OrgTruncationSubResp>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(default)]
pub struct NameSubResp {
    /// 中文
    ///
    /// **示例值**: "cn"
    #[serde(
        rename = "zh_cn",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub zh_cn: String,
    /// 英文
    ///
    /// **示例值**: "en"
    #[serde(
        rename = "en_us",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub en_us: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(default)]
pub struct OrgTruncationSubResp {
    /// 组织管理维度名称
    ///
    /// **示例值**: "department"
    #[serde(
        rename = "org_key",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub org_key: String,
    /// 下钻类型
    ///
    /// - 0 = 对当前管理维度及下级管理维度均有权限
    ///
    /// - 1 = 只对当前管理维度有权限，不包含其下级管理维度
    ///
    /// **示例值**: "0"
    #[serde(
        rename = "type",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub body_type: i64,
    /// 下钻深度
    ///
    /// **示例值**: "0"
    #[serde(
        rename = "depth",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub depth: i64,
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
            GetCoreHrSecurityGroupListReq,
        ) -> Result<(GetCoreHrSecurityGroupListResp, CommonResponse), Error>
        + Send
        + Sync
        + 'static
    {
    }
    impl<
            T: Fn(
                    GetCoreHrSecurityGroupListReq,
                )
                    -> Result<(GetCoreHrSecurityGroupListResp, CommonResponse), Error>
                + Send
                + Sync
                + 'static,
        > MockFunc for T
    {
    }

    impl<'c, IStore: Store, IClient: HttpClient> CoreHrServiceMocker<'c, IStore, IClient> {
        pub fn mock_get_core_hr_security_group_list<F: MockFunc>(
            &self,
            f: F,
        ) -> MockerBuilder<
            Mocker,
            GetCoreHrSecurityGroupListReq,
            GetCoreHrSecurityGroupListResp,
            Arc<dyn MockFunc>,
        > {
            MockerBuilder::new(self.cli.instance_id, Arc::new(f))
        }

        pub(super) fn get_mock_get_core_hr_security_group_list(
            &self,
            req: &GetCoreHrSecurityGroupListReq,
        ) -> Option<Arc<dyn MockFunc>> {
            do_mock::<
                Mocker,
                GetCoreHrSecurityGroupListReq,
                GetCoreHrSecurityGroupListResp,
                Arc<dyn MockFunc>,
            >(self.cli.instance_id, req)
        }
    }
}

#[cfg(feature = "test-util")]
#[cfg(test)]
mod test {
    use crate::{
        api::gen::core_hr::get_core_hr_security_group_list::{
            GetCoreHrSecurityGroupListReq, GetCoreHrSecurityGroupListResp,
            GetCoreHrSecurityGroupListRespInner,
        },
        core::{model::CommonResponse, Lark},
    };

    #[tokio::test]
    async fn test_mock() {
        let lark = Lark::new("".to_owned(), "".into());
        let mocker = lark
            .core_hr()
            .mock()
            .mock_get_core_hr_security_group_list(|_| {
                Ok((
                    GetCoreHrSecurityGroupListResp::default(),
                    CommonResponse::default(),
                ))
            })
            .build();
        let res = lark
            .core_hr()
            .get_core_hr_security_group_list(GetCoreHrSecurityGroupListReq::default())
            .await;
        assert!(res.is_ok());
        mocker.clear();
        let res = lark
            .core_hr()
            .get_core_hr_security_group_list(GetCoreHrSecurityGroupListReq::default())
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
                "id": "7034393015968122400",
                "code": "department_manager",
                "name": {
                    "zh_cn": "cn",
                    "en_us": "en"
                },
                "active_status": 1,
                "description": {
                    "zh_cn": "cn",
                    "en_us": "en"
                },
                "group_type": 1,
                "created_by": "6967639606963471902",
                "update_time": "1",
                "org_truncation": [
                    {
                        "org_key": "department",
                        "type": 0,
                        "depth": 0
                    }
                ]
            }
        ],
        "has_more": true,
        "page_token": "1234452132"
    }
}"#;
    #[test]
    fn test_resp() {
        let res = serde_json::from_str::<GetCoreHrSecurityGroupListRespInner>(RESP);
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
