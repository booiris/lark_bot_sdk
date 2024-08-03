//! doc url: <https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/contact-v3/user/resurrect>
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
    /// **api 版本: 2024-07-05T08:05:05+00:00**
    ///
    /// ## 恢复已删除用户
    ///
    /// 该接口用于恢复已删除用户（已离职的成员）。
    ///

    ///
    /// doc: <https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/contact-v3/user/resurrect>
    ///
    /// new doc: <https://open.feishu.cn/document/server-docs/contact-v3/user/resurrect>
    ///
    /// api url: <https://open.feishu.cn/document_portal/v1/document/get_detail?fullPath=%2Fserver-docs%2Fcontact-v3%2Fuser%2Fresurrect>
    pub async fn resurrect_user(
        &self,
        req: ResurrectUserReq,
    ) -> Result<(ResurrectUserResp, CommonResponse), Error> {
        self.resurrect_user_with_opt(req, Default::default()).await
    }

    /// 参见 [resurrect_user](#method.resurrect_user) 函数
    pub async fn resurrect_user_with_opt(
        &self,
        req: ResurrectUserReq,
        method_option: MethodOption,
    ) -> Result<(ResurrectUserResp, CommonResponse), Error> {
        #[cfg(feature = "test-util")]
        {
            if let Some(f) = self.mock().get_mock_resurrect_user(&req) {
                tracing::info!("[lark] Contact#ResurrectUser **mocking** api");
                return f(req);
            }
        }

        tracing::info!("[lark] Contact#ResurrectUser call api");

        let req = ApiRequest {
            scope: "Contact",
            api: "ResurrectUser",
            method: http::Method::POST,
            url: String::new()
                + self.cli.open_base_url.as_ref()
                + "/open-apis/contact/v3/users/:user_id/resurrect",
            param_data: req.gen_param(),
            method_option,
            need_tenant_access_token: true,
            ..Default::default()
        };

        let (resp, common_resp): (ResurrectUserRespInner, _) = self.cli.do_req(req).await?;
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
pub struct ResurrectUserReq {
    /// 用户 ID。ID 类型需要与查询参数中的 user_id_type类型保持一致。用户 ID 获取方式可参见[如何获取不同的用户 ID](https://open.feishu.cn/document/home/user-identity-introduction/open-id)。
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "ou_7dab8a3d3cdcc9da365777c7ad535d62"
    #[api(kind = "path", name = "user_id")]
    pub user_id: String,
    /// 用户 ID 类型
    ///
    /// **示例值**: "user_id"
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
    /// 此次调用中使用的部门 ID 类型。关于部门 ID 的详细介绍，可参见[部门 ID 说明](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/contact-v3/department/field-overview#23857fe0)。
    ///
    /// **示例值**: "department_id"
    ///
    /// **可选值**:
    ///
    /// `department_id`: 支持用户自定义配置的部门 ID。自定义配置时可复用已删除的 department_id，因此在未删除的部门范围内 department_id 具有唯一性。
    ///
    /// `open_department_id`: 由系统自动生成的部门 ID，ID 前缀固定为 od-，在租户内全局唯一。
    #[api(
        kind = "query",
        name = "department_id_type",
        v_type = "var",
        option = "false"
    )]
    pub department_id_type: String,
    /// 用户排序信息。用户可能存在多个部门中，且有不同的排序，该参数用于设置用户部门排序。
    ///
    /// **说明**：如果请求时不传入 departments 参数，则用户将恢复至原部门。
    ///
    /// **数据校验规则**：
    ///
    /// - **最大长度**: `50` 字符
    #[api(kind = "body", name = "departments")]
    pub departments: Vec<Option<UserDepartmentInfoSubReq>>,
    /// 如果用户正常状态时分配了席位，则可以通过该参数指定恢复后分配的席位 ID。
    ///
    /// **注意**：
    ///
    /// - 当在[混合license模式](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/tenant-v2/tenant-product_assign_info/query)下，该字段为必填。
    ///
    /// - 该字段需开通 **分配用户席位** 权限。
    ///
    /// **示例值**: "["7179609168571645971"]"
    #[api(kind = "body", name = "subscription_ids")]
    pub subscription_ids: Vec<Option<String>>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(default)]
pub struct UserDepartmentInfoSubReq {
    /// 排序信息对应的部门 ID。表示用户所在的、且需要排序的部门。部门 ID 类型与查询参数 `department_id_type` 保持一致。
    ///
    /// 了解不同类型的部门 ID 以及获取部门 ID 的方式，可参见 [部门 ID 说明](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/contact-v3/department/field-overview#23857fe0)。
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "od-4e6ac4d14bcd5071a37a39de902c7141"
    #[serde(
        rename = "department_id",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub department_id: String,
    /// 用户在其直属部门内的排序。数值越大，排序越靠前。
    ///
    /// **示例值**: "0"
    #[serde(
        rename = "user_order",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub user_order: Option<i64>,
    /// 用户所属的多个部门之间的排序。数值越大，排序越靠前。
    ///
    /// **示例值**: "0"
    #[serde(
        rename = "department_order",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub department_order: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize, Clone, lark_bot_sdk_macros::ApiBaseResp)]
struct ResurrectUserRespInner {
    #[serde(flatten)]
    data: Option<ResurrectUserResp>,
    #[serde(flatten)]
    base: BaseResp,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct ResurrectUserResp {
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

    use self::gen::contact::ContactServiceMocker;
    use crate::core::mocker::*;

    use super::*;
    use crate::api::*;
    use crate::core::model::*;

    pub struct Mocker;

    pub trait MockFunc:
        Fn(ResurrectUserReq) -> Result<(ResurrectUserResp, CommonResponse), Error>
        + Send
        + Sync
        + 'static
    {
    }
    impl<
            T: Fn(ResurrectUserReq) -> Result<(ResurrectUserResp, CommonResponse), Error>
                + Send
                + Sync
                + 'static,
        > MockFunc for T
    {
    }

    impl<'c, IStore: Store, IClient: HttpClient> ContactServiceMocker<'c, IStore, IClient> {
        pub fn mock_resurrect_user<F: MockFunc>(
            &self,
            f: F,
        ) -> MockerBuilder<Mocker, ResurrectUserReq, ResurrectUserResp, Arc<dyn MockFunc>> {
            MockerBuilder::new(self.cli.instance_id, Arc::new(f))
        }

        pub(super) fn get_mock_resurrect_user(
            &self,
            req: &ResurrectUserReq,
        ) -> Option<Arc<dyn MockFunc>> {
            do_mock::<Mocker, ResurrectUserReq, ResurrectUserResp, Arc<dyn MockFunc>>(
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
        api::gen::contact::resurrect_user::{
            ResurrectUserReq, ResurrectUserResp, ResurrectUserRespInner,
        },
        core::{model::CommonResponse, Lark},
    };

    #[tokio::test]
    async fn test_mock() {
        let lark = Lark::new("".to_owned(), "".into());
        let mocker = lark
            .contact()
            .mock()
            .mock_resurrect_user(|_| Ok((ResurrectUserResp::default(), CommonResponse::default())))
            .build();
        let res = lark
            .contact()
            .resurrect_user(ResurrectUserReq::default())
            .await;
        assert!(res.is_ok());
        mocker.clear();
        let res = lark
            .contact()
            .resurrect_user(ResurrectUserReq::default())
            .await;
        assert!(res.is_err());
    }

    const REQ: &str = r#"{
    "departments": [
        {
            "department_id": "od-4e6ac4d14bcd5071a37a39de902c7141",
            "user_order": 0,
            "department_order": 0
        }
    ],
    "subscription_ids": [
        "23213213213123123"
    ]
}"#;

    #[test]
    fn test_req() {
        if REQ == "{}" {
            return;
        }
        if let Err(e) = serde_json::from_str::<super::ResurrectUserReqBody>(REQ) {
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
        let res = serde_json::from_str::<ResurrectUserRespInner>(RESP);
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
