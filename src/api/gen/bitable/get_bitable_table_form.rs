//! doc url: <https://open.larkoffice.com/document/uAjLw4CM/ukTMukTMukTM/reference/bitable-v1/app-table-form/get>
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
    /// **api 版本: 2024-02-01T13:06:39+00:00**
    ///
    /// ## 获取表单元数据
    ///
    /// 获取表单的所有元数据项
    ///

    ///
    /// doc: <https://open.larkoffice.com/document/uAjLw4CM/ukTMukTMukTM/reference/bitable-v1/app-table-form/get>
    ///
    /// new doc: <https://open.larkoffice.com/document/server-docs/docs/bitable-v1/form/get>
    ///
    /// api url: <https://open.feishu.cn/document_portal/v1/document/get_detail?fullPath=%2Fserver-docs%2Fdocs%2Fbitable-v1%2Fform%2Fget>
    pub async fn get_bitable_table_form(
        &self,
        req: GetBitableTableFormReq,
    ) -> Result<(GetBitableTableFormResp, CommonResponse), Error> {
        self.get_bitable_table_form_with_opt(req, Default::default())
            .await
    }

    /// 参见 [get_bitable_table_form](#method.get_bitable_table_form) 函数
    pub async fn get_bitable_table_form_with_opt(
        &self,
        req: GetBitableTableFormReq,
        method_option: MethodOption,
    ) -> Result<(GetBitableTableFormResp, CommonResponse), Error> {
        #[cfg(feature = "test-util")]
        {
            if let Some(f) = self.mock().get_mock_get_bitable_table_form(&req) {
                tracing::info!("[lark] Bitable#GetBitableTableForm **mocking** api");
                return f(req);
            }
        }

        tracing::info!("[lark] Bitable#GetBitableTableForm call api");

        let req = ApiRequest {
            scope: "Bitable",
            api: "GetBitableTableForm",
            method: http::Method::GET,
            url: String::new()
                + self.cli.open_base_url.as_ref()
                + "/open-apis/bitable/v1/apps/:app_token/tables/:table_id/forms/:form_id",
            param_data: req.gen_param(),
            method_option,
            need_tenant_access_token: true,
            need_user_access_token: true,
            ..Default::default()
        };

        let (resp, common_resp): (GetBitableTableFormRespInner, _) = self.cli.do_req(req).await?;
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
pub struct GetBitableTableFormReq {
    /// 多维表格文档 Token
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "bascnv1jIEppJdTCn3jOosabcef"
    #[api(kind = "path", name = "app_token")]
    pub app_token: String,
    /// 表格 ID
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "tblz8nadEUdxNMt5"
    #[api(kind = "path", name = "table_id")]
    pub table_id: String,
    /// 表单 ID
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "vew6oMbAa4"
    #[api(kind = "path", name = "form_id")]
    pub form_id: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, lark_bot_sdk_macros::ApiBaseResp)]
struct GetBitableTableFormRespInner {
    #[serde(flatten)]
    data: Option<GetBitableTableFormResp>,
    #[serde(flatten)]
    base: BaseResp,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct GetBitableTableFormResp {
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
    /// 表单元数据信息
    ///
    /// **是否必填**: 是
    #[serde(
        rename = "form",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub form: AppTableFormSubResp,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(default)]
pub struct AppTableFormSubResp {
    /// 表单名称
    ///
    /// **示例值**: "表单"
    #[serde(
        rename = "name",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub name: String,
    /// 表单描述
    ///
    /// **示例值**: "表单描述"
    #[serde(
        rename = "description",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub description: String,
    /// 是否开启共享
    ///
    /// **示例值**: "true"
    #[serde(
        rename = "shared",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub shared: bool,
    /// 分享 URL
    ///
    /// **示例值**: "https://example.feishu.cn/share/base/shrcnCy1KAlpahNotmhRn1abcde"
    #[serde(
        rename = "shared_url",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub shared_url: String,
    /// 分享范围限制
    ///
    /// **示例值**: "tenant_editable"
    ///
    /// **可选值**:
    ///
    /// `Off`: 仅邀请的人可填写
    ///
    /// `TenantEditable`: 组织内获得链接的人可填写
    ///
    /// `AnyoneEditable`: 互联网上获得链接的人可填写
    #[serde(
        rename = "shared_limit",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub shared_limit: String,
    /// 填写次数限制一次
    ///
    /// **示例值**: "true"
    #[serde(
        rename = "submit_limit_once",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub submit_limit_once: bool,
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
        Fn(GetBitableTableFormReq) -> Result<(GetBitableTableFormResp, CommonResponse), Error>
        + Send
        + Sync
        + 'static
    {
    }
    impl<
            T: Fn(
                    GetBitableTableFormReq,
                ) -> Result<(GetBitableTableFormResp, CommonResponse), Error>
                + Send
                + Sync
                + 'static,
        > MockFunc for T
    {
    }

    impl<'c, IStore: Store, IClient: HttpClient> BitableServiceMocker<'c, IStore, IClient> {
        pub fn mock_get_bitable_table_form<F: MockFunc>(
            &self,
            f: F,
        ) -> MockerBuilder<Mocker, GetBitableTableFormReq, GetBitableTableFormResp, Arc<dyn MockFunc>>
        {
            MockerBuilder::new(self.cli.instance_id, Arc::new(f))
        }

        pub(super) fn get_mock_get_bitable_table_form(
            &self,
            req: &GetBitableTableFormReq,
        ) -> Option<Arc<dyn MockFunc>> {
            do_mock::<Mocker, GetBitableTableFormReq, GetBitableTableFormResp, Arc<dyn MockFunc>>(
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
        api::gen::bitable::get_bitable_table_form::{
            GetBitableTableFormReq, GetBitableTableFormResp, GetBitableTableFormRespInner,
        },
        core::{model::CommonResponse, Lark},
    };

    #[tokio::test]
    async fn test_mock() {
        let lark = Lark::new("".to_owned(), "".into());
        let mocker = lark
            .bitable()
            .mock()
            .mock_get_bitable_table_form(|_| {
                Ok((
                    GetBitableTableFormResp::default(),
                    CommonResponse::default(),
                ))
            })
            .build();
        let res = lark
            .bitable()
            .get_bitable_table_form(GetBitableTableFormReq::default())
            .await;
        assert!(res.is_ok());
        mocker.clear();
        let res = lark
            .bitable()
            .get_bitable_table_form(GetBitableTableFormReq::default())
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
        "form": {
            "name": "表单",
            "description": "表单描述",
            "shared": true,
            "shared_url": "https://example.feishu.cn/share/base/shrcnCy1KAlpahNotmhRn1abcde",
            "shared_limit": "tenant_editable",
            "submit_limit_once": true
        }
    }
}"#;
    #[test]
    fn test_resp() {
        let res = serde_json::from_str::<GetBitableTableFormRespInner>(RESP);
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
