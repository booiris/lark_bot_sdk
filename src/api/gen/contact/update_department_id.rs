//! doc url: <https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/contact-v3/department/update_department_id>
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
    /// **api 版本: 2024-07-05T08:11:40+00:00**
    ///
    /// ## 更新部门ID
    ///
    /// 调用该接口可以更新部门的自定义 ID，即 department_id。
    ///

    ///
    /// doc: <https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/contact-v3/department/update_department_id>
    ///
    /// new doc: <https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/contact-v3/department/update_department_id>
    ///
    /// api url: <https://open.feishu.cn/document_portal/v1/document/get_detail?fullPath=%2FuAjLw4CM%2FukTMukTMukTM%2Freference%2Fcontact-v3%2Fdepartment%2Fupdate_department_id>
    pub async fn update_department_id(
        &self,
        req: UpdateDepartmentIdReq,
    ) -> Result<(UpdateDepartmentIdResp, CommonResponse), Error> {
        self.update_department_id_with_opt(req, Default::default())
            .await
    }

    /// 参见 [update_department_id](#method.update_department_id) 函数
    pub async fn update_department_id_with_opt(
        &self,
        req: UpdateDepartmentIdReq,
        method_option: MethodOption,
    ) -> Result<(UpdateDepartmentIdResp, CommonResponse), Error> {
        #[cfg(feature = "test-util")]
        {
            if let Some(f) = self.mock().get_mock_update_department_id(&req) {
                tracing::info!("[lark] Contact#UpdateDepartmentId **mocking** api");
                return f(req);
            }
        }

        tracing::info!("[lark] Contact#UpdateDepartmentId call api");

        let req = ApiRequest {
            scope: "Contact",
            api: "UpdateDepartmentId",
            method: http::Method::PATCH,
            url: String::new()
                + self.cli.open_base_url.as_ref()
                + "/open-apis/contact/v3/departments/:department_id/update_department_id",
            param_data: req.gen_param(),
            method_option,
            need_tenant_access_token: true,
            ..Default::default()
        };

        let (resp, common_resp): (UpdateDepartmentIdRespInner, _) = self.cli.do_req(req).await?;
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
pub struct UpdateDepartmentIdReq {
    /// 需要更新自定义 ID 的部门 ID，该 ID 类型需要与查询参数 department_id_type 的取值一致。ID 获取方式说明：
    ///
    /// - 调用[创建部门](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/contact-v3/department/create)接口后，可从返回结果中获取到部门 ID 信息。
    ///
    /// - 部门 API 提供了多种获取其他部门 ID 的方式，如[获取子部门列表](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/contact-v3/department/children)、[获取父部门信息](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/contact-v3/department/parent)、[搜索部门](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/contact-v3/department/search)，你可以选择合适的 API 进行查询。
    ///
    /// **示例值**: "od-d6b83d25c129775723a36f52495c4f81"
    ///
    /// **数据校验规则**：
    ///
    /// - **最大长度**: `64` 字符
    #[api(kind = "path", name = "department_id")]
    pub department_id: String,
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
    /// 新的自定义部门 ID，即部门的 department_id。
    ///
    /// **注意**：
    ///
    /// - 不能以 `od-` 开头。
    ///
    /// - 不能设置为 `0`。
    ///
    /// - 不能与其他未删除部门的 department_id 重复。
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "NewDevDepartID"
    ///
    /// **数据校验规则**：
    ///
    /// - **最大长度**: `128` 字符
    #[api(kind = "body", name = "new_department_id")]
    pub new_department_id: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, lark_bot_sdk_macros::ApiBaseResp)]
struct UpdateDepartmentIdRespInner {
    #[serde(flatten)]
    data: Option<UpdateDepartmentIdResp>,
    #[serde(flatten)]
    base: BaseResp,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct UpdateDepartmentIdResp {
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
        Fn(UpdateDepartmentIdReq) -> Result<(UpdateDepartmentIdResp, CommonResponse), Error>
        + Send
        + Sync
        + 'static
    {
    }
    impl<
            T: Fn(UpdateDepartmentIdReq) -> Result<(UpdateDepartmentIdResp, CommonResponse), Error>
                + Send
                + Sync
                + 'static,
        > MockFunc for T
    {
    }

    impl<'c, IStore: Store, IClient: HttpClient> ContactServiceMocker<'c, IStore, IClient> {
        pub fn mock_update_department_id<F: MockFunc>(
            &self,
            f: F,
        ) -> MockerBuilder<Mocker, UpdateDepartmentIdReq, UpdateDepartmentIdResp, Arc<dyn MockFunc>>
        {
            MockerBuilder::new(self.cli.instance_id, Arc::new(f))
        }

        pub(super) fn get_mock_update_department_id(
            &self,
            req: &UpdateDepartmentIdReq,
        ) -> Option<Arc<dyn MockFunc>> {
            do_mock::<Mocker, UpdateDepartmentIdReq, UpdateDepartmentIdResp, Arc<dyn MockFunc>>(
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
        api::gen::contact::update_department_id::{
            UpdateDepartmentIdReq, UpdateDepartmentIdResp, UpdateDepartmentIdRespInner,
        },
        core::{model::CommonResponse, Lark},
    };

    #[tokio::test]
    async fn test_mock() {
        let lark = Lark::new("".to_owned(), "".into());
        let mocker = lark
            .contact()
            .mock()
            .mock_update_department_id(|_| {
                Ok((UpdateDepartmentIdResp::default(), CommonResponse::default()))
            })
            .build();
        let res = lark
            .contact()
            .update_department_id(UpdateDepartmentIdReq::default())
            .await;
        assert!(res.is_ok());
        mocker.clear();
        let res = lark
            .contact()
            .update_department_id(UpdateDepartmentIdReq::default())
            .await;
        assert!(res.is_err());
    }

    const REQ: &str = r#"{
    "new_department_id": "NewDevDepartID"
}"#;

    #[test]
    fn test_req() {
        if REQ == "{}" {
            return;
        }
        if let Err(e) = serde_json::from_str::<super::UpdateDepartmentIdReqBody>(REQ) {
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
        let res = serde_json::from_str::<UpdateDepartmentIdRespInner>(RESP);
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
