//! doc url: <https://open.larkoffice.com/document/uAjLw4CM/ukTMukTMukTM/corehr-v2/bp/list>
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
    /// **api 版本: 2024-07-12T08:37:07+00:00**
    ///
    /// ## 获取 HRBP 列表
    ///
    /// 获取 HRBP 列表。列表中包含HRBP的ID以及部门ID信息。
    ///

    ///
    /// doc: <https://open.larkoffice.com/document/uAjLw4CM/ukTMukTMukTM/corehr-v2/bp/list>
    ///
    /// new doc: <https://open.larkoffice.com/document/uAjLw4CM/ukTMukTMukTM/corehr-v2/bp/list>
    ///
    /// api url: <https://open.feishu.cn/document_portal/v1/document/get_detail?fullPath=%2FuAjLw4CM%2FukTMukTMukTM%2Fcorehr-v2%2Fbp%2Flist>
    pub async fn get_core_hrbp_list(
        &self,
        req: GetCoreHrbpListReq,
    ) -> Result<(GetCoreHrbpListResp, CommonResponse), Error> {
        self.get_core_hrbp_list_with_opt(req, Default::default())
            .await
    }

    /// 参见 [get_core_hrbp_list](#method.get_core_hrbp_list) 函数
    pub async fn get_core_hrbp_list_with_opt(
        &self,
        req: GetCoreHrbpListReq,
        method_option: MethodOption,
    ) -> Result<(GetCoreHrbpListResp, CommonResponse), Error> {
        #[cfg(feature = "test-util")]
        {
            if let Some(f) = self.mock().get_mock_get_core_hrbp_list(&req) {
                tracing::info!("[lark] CoreHr#GetCoreHrbpList **mocking** api");
                return f(req);
            }
        }

        tracing::info!("[lark] CoreHr#GetCoreHrbpList call api");

        let req = ApiRequest {
            scope: "CoreHr",
            api: "GetCoreHrbpList",
            method: http::Method::GET,
            url: String::new() + self.cli.open_base_url.as_ref() + "/open-apis/corehr/v2/bps",
            param_data: req.gen_param(),
            method_option,
            need_tenant_access_token: true,
            ..Default::default()
        };

        let (resp, common_resp): (GetCoreHrbpListRespInner, _) = self.cli.do_req(req).await?;
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
pub struct GetCoreHrbpListReq {
    /// 分页大小，最大 500
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "500"
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
}

#[derive(Debug, Serialize, Deserialize, Clone, lark_bot_sdk_macros::ApiBaseResp)]
struct GetCoreHrbpListRespInner {
    #[serde(flatten)]
    data: Option<GetCoreHrbpListResp>,
    #[serde(flatten)]
    base: BaseResp,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct GetCoreHrbpListResp {
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
    /// HRBP 信息
    #[serde(
        rename = "items",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub items: Vec<BpSubResp>,
    /// 分页标记，当 has_more 为 true 时，会同时返回新的 page_token，否则不返回 page_token
    ///
    /// **示例值**: "eVQrYzJBNDNONlk4VFZBZVlSdzlKdFJ4bVVHVExENDNKVHoxaVdiVnViQT0="
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
pub struct BpSubResp {
    /// 部门 ID
    ///
    /// >
    ///
    /// 如想获取部门详细信息，可通过[搜索部门信息](https://open.larkoffice.com/document/server-docs/corehr-v1/organization-management/department/search)接口获取。
    ///
    /// **示例值**: "4719456877659520852"
    #[serde(
        rename = "department_id",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub department_id: String,
    /// 部门 HRBP 的雇佣 ID，不包括上级部门的 HRBP
    ///
    /// >
    ///
    /// 如想获取员工详细信息，可通过[搜索员工信息](https://open.larkoffice.com/document/server-docs/corehr-v1/employee/search)接口获取。
    ///
    /// **示例值**: "4719456877659520852"
    #[serde(
        rename = "hrbp_id",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub hrbp_id: String,
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
        Fn(GetCoreHrbpListReq) -> Result<(GetCoreHrbpListResp, CommonResponse), Error>
        + Send
        + Sync
        + 'static
    {
    }
    impl<
            T: Fn(GetCoreHrbpListReq) -> Result<(GetCoreHrbpListResp, CommonResponse), Error>
                + Send
                + Sync
                + 'static,
        > MockFunc for T
    {
    }

    impl<'c, IStore: Store, IClient: HttpClient> CoreHrServiceMocker<'c, IStore, IClient> {
        pub fn mock_get_core_hrbp_list<F: MockFunc>(
            &self,
            f: F,
        ) -> MockerBuilder<Mocker, GetCoreHrbpListReq, GetCoreHrbpListResp, Arc<dyn MockFunc>>
        {
            MockerBuilder::new(self.cli.instance_id, Arc::new(f))
        }

        pub(super) fn get_mock_get_core_hrbp_list(
            &self,
            req: &GetCoreHrbpListReq,
        ) -> Option<Arc<dyn MockFunc>> {
            do_mock::<Mocker, GetCoreHrbpListReq, GetCoreHrbpListResp, Arc<dyn MockFunc>>(
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
        api::gen::core_hr::get_core_hrbp_list::{
            GetCoreHrbpListReq, GetCoreHrbpListResp, GetCoreHrbpListRespInner,
        },
        core::{model::CommonResponse, Lark},
    };

    #[tokio::test]
    async fn test_mock() {
        let lark = Lark::new("".to_owned(), "".into());
        let mocker = lark
            .core_hr()
            .mock()
            .mock_get_core_hrbp_list(|_| {
                Ok((GetCoreHrbpListResp::default(), CommonResponse::default()))
            })
            .build();
        let res = lark
            .core_hr()
            .get_core_hrbp_list(GetCoreHrbpListReq::default())
            .await;
        assert!(res.is_ok());
        mocker.clear();
        let res = lark
            .core_hr()
            .get_core_hrbp_list(GetCoreHrbpListReq::default())
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
                "department_id": "4719456877659520852",
                "hrbp_id": "4719456877659520852"
            }
        ],
        "page_token": "eVQrYzJBNDNONlk4VFZBZVlSdzlKdFJ4bVVHVExENDNKVHoxaVdiVnViQT0=",
        "has_more": true
    }
}"#;
    #[test]
    fn test_resp() {
        let res = serde_json::from_str::<GetCoreHrbpListRespInner>(RESP);
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
