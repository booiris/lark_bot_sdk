//! doc url: <https://open.larkoffice.com/document/uAjLw4CM/ukTMukTMukTM/reference/bitable-v1/app-role-member/batch_delete>
// Code generated by gen_api. DO NOT EDIT.

use serde::{Deserialize, Serialize};

use crate::api::BaseResp;
use crate::api::HasBaseResp;
use crate::core::model::{ApiRequest, CommonResponse, MethodOption, ReqParam, StreamReqParam};
use crate::{
    core::{http_client::HttpClient, store::Store},
    error::Error,
};

use crate::api::gen::bitable::BitableService;

impl<'c, IStore: Store, IClient: HttpClient> BitableService<'c, IStore, IClient> {
    /// **api 版本: 2023-07-28T08:21:55+00:00**
    ///
    /// ## 批量删除协作者
    ///
    /// 批量删除自定义角色的协作者
    ///

    ///
    /// doc: <https://open.larkoffice.com/document/uAjLw4CM/ukTMukTMukTM/reference/bitable-v1/app-role-member/batch_delete>
    ///
    /// new doc: <https://open.larkoffice.com/document/server-docs/docs/bitable-v1/advanced-permission/app-role-member/batch_delete>
    ///
    /// api url: <https://open.feishu.cn/document_portal/v1/document/get_detail?fullPath=%2Fserver-docs%2Fdocs%2Fbitable-v1%2Fadvanced-permission%2Fapp-role-member%2Fbatch_delete>
    pub async fn batch_delete_bitable_app_role_member(
        &self,
        req: BatchDeleteBitableAppRoleMemberReq,
    ) -> Result<(BatchDeleteBitableAppRoleMemberResp, CommonResponse), Error> {
        self.batch_delete_bitable_app_role_member_with_opt(req, Default::default())
            .await
    }

    /// 参见 [batch_delete_bitable_app_role_member](#method.batch_delete_bitable_app_role_member) 函数
    pub async fn batch_delete_bitable_app_role_member_with_opt(
        &self,
        req: BatchDeleteBitableAppRoleMemberReq,
        method_option: MethodOption,
    ) -> Result<(BatchDeleteBitableAppRoleMemberResp, CommonResponse), Error> {
        #[cfg(feature = "test-util")]
        {
            if let Some(f) = self
                .mock()
                .get_mock_batch_delete_bitable_app_role_member(&req)
            {
                tracing::info!("[lark] Bitable#BatchDeleteBitableAppRoleMember **mocking** api");
                return f(req);
            }
        }

        tracing::info!("[lark] Bitable#BatchDeleteBitableAppRoleMember call api");

        let req = ApiRequest {
            scope: "Bitable",
            api: "BatchDeleteBitableAppRoleMember",
            method: http::Method::POST,
            url: String::new()
                + self.cli.open_base_url.as_ref()
                + "/open-apis/bitable/v1/apps/:app_token/roles/:role_id/members/batch_delete",
            param_data: req.gen_param(),
            method_option,
            need_tenant_access_token: true,
            need_user_access_token: true,
            ..Default::default()
        };

        let (resp, common_resp): (BatchDeleteBitableAppRoleMemberRespInner, _) =
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
pub struct BatchDeleteBitableAppRoleMemberReq {
    /// 多维表格文档 Token
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "bascnnKKvcoUblgmmhZkYqabcef"
    #[api(kind = "path", name = "app_token")]
    pub app_token: String,
    /// 自定义角色 ID
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "rolNGhPqks"
    #[api(kind = "path", name = "role_id")]
    pub role_id: String,

    /// 协作者列表
    ///
    /// **是否必填**: 是
    ///
    /// **数据校验规则**：
    ///
    /// - **最大长度**: `100` 字符
    #[api(kind = "body", name = "member_list")]
    pub member_list: Vec<Option<AppRoleMemberIdSubReq>>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(default)]
pub struct AppRoleMemberIdSubReq {
    /// 协作者 ID 类型
    ///
    /// **示例值**: "open_id"
    ///
    /// **可选值**:
    ///
    /// `OpenID`: 协作者 ID 类型为 open_id
    ///
    /// `UnionID`: 协作者 ID 类型为 union_id
    ///
    /// `UserID`: 协作者 ID 类型为 user_id
    ///
    /// `ChatID`: 协作者 ID 类型为 chat_id
    ///
    /// `DepartmentID`: 协作者 ID 类型为 department_id
    ///
    /// `OpenDepartmentID`: 协作者 ID 类型为 open_department_id
    #[serde(
        rename = "type",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub body_type: Option<String>,
    /// 协作者 ID
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "ou_35990a9d9052051a2fae9b2f1afabcef"
    #[serde(
        rename = "id",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub id: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, lark_bot_sdk_macros::ApiBaseResp)]
struct BatchDeleteBitableAppRoleMemberRespInner {
    #[serde(flatten)]
    data: Option<BatchDeleteBitableAppRoleMemberResp>,
    #[serde(flatten)]
    base: BaseResp,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct BatchDeleteBitableAppRoleMemberResp {
    /// \-
    #[serde(
        rename = "data",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub data: (),
}

#[cfg(feature = "test-util")]
mod test_utils {
    use std::sync::Arc;

    use self::gen::bitable::BitableServiceMocker;
    use crate::core::mocker::*;

    use super::*;
    use crate::api::*;
    use crate::core::model::*;

    pub struct Mocker;

    pub trait MockFunc:
        Fn(
            BatchDeleteBitableAppRoleMemberReq,
        ) -> Result<(BatchDeleteBitableAppRoleMemberResp, CommonResponse), Error>
        + Send
        + Sync
        + 'static
    {
    }
    impl<
            T: Fn(
                    BatchDeleteBitableAppRoleMemberReq,
                )
                    -> Result<(BatchDeleteBitableAppRoleMemberResp, CommonResponse), Error>
                + Send
                + Sync
                + 'static,
        > MockFunc for T
    {
    }

    impl<'c, IStore: Store, IClient: HttpClient> BitableServiceMocker<'c, IStore, IClient> {
        pub fn mock_batch_delete_bitable_app_role_member<F: MockFunc>(
            &self,
            f: F,
        ) -> MockerBuilder<
            Mocker,
            BatchDeleteBitableAppRoleMemberReq,
            BatchDeleteBitableAppRoleMemberResp,
            Arc<dyn MockFunc>,
        > {
            MockerBuilder::new(self.cli.instance_id, Arc::new(f))
        }

        pub(super) fn get_mock_batch_delete_bitable_app_role_member(
            &self,
            req: &BatchDeleteBitableAppRoleMemberReq,
        ) -> Option<Arc<dyn MockFunc>> {
            do_mock::<
                Mocker,
                BatchDeleteBitableAppRoleMemberReq,
                BatchDeleteBitableAppRoleMemberResp,
                Arc<dyn MockFunc>,
            >(self.cli.instance_id, req)
        }
    }
}

#[cfg(feature = "test-util")]
#[cfg(test)]
mod test {
    use crate::{
        api::gen::bitable::batch_delete_bitable_app_role_member::{
            BatchDeleteBitableAppRoleMemberReq, BatchDeleteBitableAppRoleMemberResp,
            BatchDeleteBitableAppRoleMemberRespInner,
        },
        core::{model::CommonResponse, Lark},
    };

    #[tokio::test]
    async fn test_mock() {
        let lark = Lark::new("".to_owned(), "".into());
        let mocker = lark
            .bitable()
            .mock()
            .mock_batch_delete_bitable_app_role_member(|_| {
                Ok((
                    BatchDeleteBitableAppRoleMemberResp::default(),
                    CommonResponse::default(),
                ))
            })
            .build();
        let res = lark
            .bitable()
            .batch_delete_bitable_app_role_member(BatchDeleteBitableAppRoleMemberReq::default())
            .await;
        assert!(res.is_ok());
        mocker.clear();
        let res = lark
            .bitable()
            .batch_delete_bitable_app_role_member(BatchDeleteBitableAppRoleMemberReq::default())
            .await;
        assert!(res.is_err());
    }

    const REQ: &str = r#"{
    "member_list": [
        {
            "type": "open_id",
            "id": "ou_35990a9d9052051a2fae9b2f1afabcef"
        }
    ]
}"#;

    #[test]
    fn test_req() {
        if REQ == "{}" {
            return;
        }
        if let Err(e) = serde_json::from_str::<super::BatchDeleteBitableAppRoleMemberReqBody>(REQ) {
            panic!("{}", e);
        }
    }

    const RESP: &str = r#"{
    "code": 0,
    "msg": "success",
    "data": {}
}"#;
    #[test]
    fn test_resp() {
        let res = serde_json::from_str::<BatchDeleteBitableAppRoleMemberRespInner>(RESP);
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
