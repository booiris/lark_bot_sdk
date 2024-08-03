//! doc url: <https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/tenant-v2/tenant/query>
// Code generated by gen_api. DO NOT EDIT.

use serde::{Deserialize, Serialize};

use crate::api::BaseResp;
use crate::api::HasBaseResp;
use crate::core::model::{ApiRequest, CommonResponse, MethodOption, ReqParam, StreamReqParam};
use crate::{
    core::{http_client::HttpClient, store::Store},
    error::Error,
};

use crate::api::gen::tenant::TenantService;

impl<'c, IStore: Store, IClient: HttpClient> TenantService<'c, IStore, IClient> {
    /// **api 版本: 2023-11-27T07:07:12+00:00**
    ///
    /// ## 获取企业信息
    ///
    /// 获取企业名称、企业编号等企业信息
    ///

    ///
    /// doc: <https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/tenant-v2/tenant/query>
    ///
    /// new doc: <https://open.feishu.cn/document/server-docs/tenant-v2/query>
    ///
    /// api url: <https://open.feishu.cn/document_portal/v1/document/get_detail?fullPath=%2Fserver-docs%2Ftenant-v2%2Fquery>
    pub async fn get_tenant(
        &self,
        req: GetTenantReq,
    ) -> Result<(GetTenantResp, CommonResponse), Error> {
        self.get_tenant_with_opt(req, Default::default()).await
    }

    /// 参见 [get_tenant](#method.get_tenant) 函数
    pub async fn get_tenant_with_opt(
        &self,
        req: GetTenantReq,
        method_option: MethodOption,
    ) -> Result<(GetTenantResp, CommonResponse), Error> {
        #[cfg(feature = "test-util")]
        {
            if let Some(f) = self.mock().get_mock_get_tenant(&req) {
                tracing::info!("[lark] Tenant#GetTenant **mocking** api");
                return f(req);
            }
        }

        tracing::info!("[lark] Tenant#GetTenant call api");

        let req = ApiRequest {
            scope: "Tenant",
            api: "GetTenant",
            method: http::Method::GET,
            url: String::new()
                + self.cli.open_base_url.as_ref()
                + "/open-apis/tenant/v2/tenant/query",
            param_data: req.gen_param(),
            method_option,
            need_tenant_access_token: true,
            ..Default::default()
        };

        let (resp, common_resp): (GetTenantRespInner, _) = self.cli.do_req(req).await?;
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
pub struct GetTenantReq {}

#[derive(Debug, Serialize, Deserialize, Clone, lark_bot_sdk_macros::ApiBaseResp)]
struct GetTenantRespInner {
    #[serde(flatten)]
    data: Option<GetTenantResp>,
    #[serde(flatten)]
    base: BaseResp,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct GetTenantResp {
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
    /// 企业信息
    #[serde(
        rename = "tenant",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub tenant: TenantSubResp,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(default)]
pub struct TenantSubResp {
    /// 企业名称
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "企业名称"
    #[serde(
        rename = "name",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub name: String,
    /// 企业编号，平台内唯一
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "F123456789"
    #[serde(
        rename = "display_id",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub display_id: String,
    /// 个人版/团队版标志
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "0"
    ///
    /// **可选值**:
    ///
    /// `Standard`: 团队版
    ///
    /// `Simple`: 个人版
    #[serde(
        rename = "tenant_tag",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub tenant_tag: i64,
    /// 企业标识
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "abcdefghi"
    #[serde(
        rename = "tenant_key",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub tenant_key: String,
    /// 企业头像
    ///
    /// **是否必填**: 是
    #[serde(
        rename = "avatar",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub avatar: AvatarSubResp,
    /// 企业完整域名。企业域名可用于企业成员访问管理后台、云文档等含URL地址的网页。
    ///
    /// **示例值**: "example.feishu.cn"
    #[serde(
        rename = "domain",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub domain: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(default)]
pub struct AvatarSubResp {
    /// 企业头像
    ///
    /// **示例值**: "https://foo.icon.com/xxxx"
    #[serde(
        rename = "avatar_origin",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub avatar_origin: String,
    /// 企业头像 72x72
    ///
    /// **示例值**: "https://foo.icon.com/xxxx"
    #[serde(
        rename = "avatar_72",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub avatar_72: String,
    /// 企业头像 240x240
    ///
    /// **示例值**: "https://foo.icon.com/xxxx"
    #[serde(
        rename = "avatar_240",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub avatar_240: String,
    /// 企业头像 640x640
    ///
    /// **示例值**: "https://foo.icon.com/xxxx"
    #[serde(
        rename = "avatar_640",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub avatar_640: String,
}

#[cfg(feature = "test-util")]
mod test_utils {
    use std::sync::Arc;

    use self::gen::tenant::TenantServiceMocker;
    use crate::core::mocker::*;

    use super::*;
    use crate::api::*;
    use crate::core::model::*;

    pub struct Mocker;

    pub trait MockFunc:
        Fn(GetTenantReq) -> Result<(GetTenantResp, CommonResponse), Error> + Send + Sync + 'static
    {
    }
    impl<
            T: Fn(GetTenantReq) -> Result<(GetTenantResp, CommonResponse), Error>
                + Send
                + Sync
                + 'static,
        > MockFunc for T
    {
    }

    impl<'c, IStore: Store, IClient: HttpClient> TenantServiceMocker<'c, IStore, IClient> {
        pub fn mock_get_tenant<F: MockFunc>(
            &self,
            f: F,
        ) -> MockerBuilder<Mocker, GetTenantReq, GetTenantResp, Arc<dyn MockFunc>> {
            MockerBuilder::new(self.cli.instance_id, Arc::new(f))
        }

        pub(super) fn get_mock_get_tenant(&self, req: &GetTenantReq) -> Option<Arc<dyn MockFunc>> {
            do_mock::<Mocker, GetTenantReq, GetTenantResp, Arc<dyn MockFunc>>(
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
        api::gen::tenant::get_tenant::{GetTenantReq, GetTenantResp, GetTenantRespInner},
        core::{model::CommonResponse, Lark},
    };

    #[tokio::test]
    async fn test_mock() {
        let lark = Lark::new("".to_owned(), "".into());
        let mocker = lark
            .tenant()
            .mock()
            .mock_get_tenant(|_| Ok((GetTenantResp::default(), CommonResponse::default())))
            .build();
        let res = lark.tenant().get_tenant(GetTenantReq::default()).await;
        assert!(res.is_ok());
        mocker.clear();
        let res = lark.tenant().get_tenant(GetTenantReq::default()).await;
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
        "tenant": {
            "name": "企业名称",
            "display_id": "F123456789",
            "tenant_tag": 0,
            "tenant_key": "abcdefghi",
            "avatar": {
                "avatar_origin": "https://foo.icon.com/xxxx",
                "avatar_72": "https://foo.icon.com/xxxx",
                "avatar_240": "https://foo.icon.com/xxxx",
                "avatar_640": "https://foo.icon.com/xxxx"
            },
            "domain": "example.feishu.cn"
        }
    }
}"#;
    #[test]
    fn test_resp() {
        let res = serde_json::from_str::<GetTenantRespInner>(RESP);
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
