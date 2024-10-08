//! doc url: <https://open.larkoffice.com/document/ukTMukTMukTM/uMzM1YjLzMTN24yMzUjN/hire-v1/eco_background_check_package/create>
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
    /// **api 版本: 2024-07-30T08:24:25+00:00**
    ///
    /// ## 创建背调套餐和附加调查项
    ///
    /// 在指定背调帐号下创建可用的背调套餐列表和附加调查项信息列表。该接口为增量创建，每次调用会在原有的套餐列表和附加调查项列表基础上新增。
    ///

    ///
    /// doc: <https://open.larkoffice.com/document/ukTMukTMukTM/uMzM1YjLzMTN24yMzUjN/hire-v1/eco_background_check_package/create>
    ///
    /// new doc: <https://open.larkoffice.com/document/server-docs/hire-v1/ecological-docking/eco_background_check_package/create>
    ///
    /// api url: <https://open.feishu.cn/document_portal/v1/document/get_detail?fullPath=%2Fserver-docs%2Fhire-v1%2Fecological-docking%2Feco_background_check_package%2Fcreate>
    pub async fn create_hire_eco_background_check_package(
        &self,
        req: CreateHireEcoBackgroundCheckPackageReq,
    ) -> Result<(CreateHireEcoBackgroundCheckPackageResp, CommonResponse), Error> {
        self.create_hire_eco_background_check_package_with_opt(req, Default::default())
            .await
    }

    /// 参见 [create_hire_eco_background_check_package](#method.create_hire_eco_background_check_package) 函数
    pub async fn create_hire_eco_background_check_package_with_opt(
        &self,
        req: CreateHireEcoBackgroundCheckPackageReq,
        method_option: MethodOption,
    ) -> Result<(CreateHireEcoBackgroundCheckPackageResp, CommonResponse), Error> {
        #[cfg(feature = "test-util")]
        {
            if let Some(f) = self
                .mock()
                .get_mock_create_hire_eco_background_check_package(&req)
            {
                tracing::info!("[lark] Hire#CreateHireEcoBackgroundCheckPackage **mocking** api");
                return f(req);
            }
        }

        tracing::info!("[lark] Hire#CreateHireEcoBackgroundCheckPackage call api");

        let req = ApiRequest {
            scope: "Hire",
            api: "CreateHireEcoBackgroundCheckPackage",
            method: http::Method::POST,
            url: String::new()
                + self.cli.open_base_url.as_ref()
                + "/open-apis/hire/v1/eco_background_check_packages",
            param_data: req.gen_param(),
            method_option,
            need_tenant_access_token: true,
            ..Default::default()
        };

        let (resp, common_resp): (CreateHireEcoBackgroundCheckPackageRespInner, _) =
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
pub struct CreateHireEcoBackgroundCheckPackageReq {
    /// 背调账号 ID，可通过[账号绑定](https://open.feishu.cn/document/ukTMukTMukTM/uMzM1YjLzMTN24yMzUjN/hire-v1/eco_account/events/created)事件获取
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "6995842370159937061"
    #[api(kind = "body", name = "account_id")]
    pub account_id: String,
    /// 背调套餐列表
    ///
    /// **是否必填**: 是
    #[api(kind = "body", name = "package_list")]
    pub package_list: Vec<Option<EcoBackgroundCheckPackageDataSubReq>>,
    /// 附加调查项列表
    #[api(kind = "body", name = "additional_item_list")]
    pub additional_item_list: Vec<Option<EcoBackgroundCheckPackageAdditionalItemSubReq>>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(default)]
pub struct EcoBackgroundCheckPackageDataSubReq {
    /// 套餐 ID。由调用方自定义<br>
    ///
    /// **注意**：长度不超过`36`字符
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "pkg001"
    #[serde(
        rename = "id",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub id: String,
    /// 套餐名称
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "基础套餐"
    ///
    /// **数据校验规则**：
    ///
    /// - **最大长度**: `100` 字符
    #[serde(
        rename = "name",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub name: String,
    /// 套餐描述
    ///
    /// **示例值**: "工作履历信息验证X1，工作表现鉴定评价X1，教育背景核实，公民身份信息验证，简历对比，民事诉讼调查"
    #[serde(
        rename = "description",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub description: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(default)]
pub struct EcoBackgroundCheckPackageAdditionalItemSubReq {
    /// 附件调查项 ID。由调用方自定义<br>
    ///
    /// **注意**：长度不超过`36`字符
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "ext001"
    #[serde(
        rename = "id",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub id: String,
    /// 附加调查项名称
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "工作履历信息验证X2"
    #[serde(
        rename = "name",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub name: String,
    /// 附加调查项描述
    ///
    /// **示例值**: "详细调查"
    #[serde(
        rename = "description",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub description: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, lark_bot_sdk_macros::ApiBaseResp)]
struct CreateHireEcoBackgroundCheckPackageRespInner {
    #[serde(flatten)]
    data: Option<CreateHireEcoBackgroundCheckPackageResp>,
    #[serde(flatten)]
    base: BaseResp,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct CreateHireEcoBackgroundCheckPackageResp {
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

    use self::gen::hire::HireServiceMocker;
    use crate::core::mocker::*;

    use super::*;
    use crate::api::*;
    use crate::core::model::*;

    pub struct Mocker;

    pub trait MockFunc:
        Fn(
            CreateHireEcoBackgroundCheckPackageReq,
        ) -> Result<(CreateHireEcoBackgroundCheckPackageResp, CommonResponse), Error>
        + Send
        + Sync
        + 'static
    {
    }
    impl<
            T: Fn(
                    CreateHireEcoBackgroundCheckPackageReq,
                )
                    -> Result<(CreateHireEcoBackgroundCheckPackageResp, CommonResponse), Error>
                + Send
                + Sync
                + 'static,
        > MockFunc for T
    {
    }

    impl<'c, IStore: Store, IClient: HttpClient> HireServiceMocker<'c, IStore, IClient> {
        pub fn mock_create_hire_eco_background_check_package<F: MockFunc>(
            &self,
            f: F,
        ) -> MockerBuilder<
            Mocker,
            CreateHireEcoBackgroundCheckPackageReq,
            CreateHireEcoBackgroundCheckPackageResp,
            Arc<dyn MockFunc>,
        > {
            MockerBuilder::new(self.cli.instance_id, Arc::new(f))
        }

        pub(super) fn get_mock_create_hire_eco_background_check_package(
            &self,
            req: &CreateHireEcoBackgroundCheckPackageReq,
        ) -> Option<Arc<dyn MockFunc>> {
            do_mock::<
                Mocker,
                CreateHireEcoBackgroundCheckPackageReq,
                CreateHireEcoBackgroundCheckPackageResp,
                Arc<dyn MockFunc>,
            >(self.cli.instance_id, req)
        }
    }
}

#[cfg(feature = "test-util")]
#[cfg(test)]
mod test {
    use crate::{
        api::gen::hire::create_hire_eco_background_check_package::{
            CreateHireEcoBackgroundCheckPackageReq, CreateHireEcoBackgroundCheckPackageResp,
            CreateHireEcoBackgroundCheckPackageRespInner,
        },
        core::{model::CommonResponse, Lark},
    };

    #[tokio::test]
    async fn test_mock() {
        let lark = Lark::new("".to_owned(), "".into());
        let mocker = lark
            .hire()
            .mock()
            .mock_create_hire_eco_background_check_package(|_| {
                Ok((
                    CreateHireEcoBackgroundCheckPackageResp::default(),
                    CommonResponse::default(),
                ))
            })
            .build();
        let res = lark
            .hire()
            .create_hire_eco_background_check_package(
                CreateHireEcoBackgroundCheckPackageReq::default(),
            )
            .await;
        assert!(res.is_ok());
        mocker.clear();
        let res = lark
            .hire()
            .create_hire_eco_background_check_package(
                CreateHireEcoBackgroundCheckPackageReq::default(),
            )
            .await;
        assert!(res.is_err());
    }

    const REQ: &str = r#"{
    "account_id": "6995842370159937061",
    "package_list": [
        {
            "id": "pkg001",
            "name": "基础套餐",
            "description": "工作履历信息验证X1，工作表现鉴定评价X1，教育背景核实，公民身份信息验证，简历对比，民事诉讼调查"
        }
    ],
    "additional_item_list": [
        {
            "id": "ext001",
            "name": "工作履历信息验证X2",
            "description": "详细调查"
        }
    ]
}"#;

    #[test]
    fn test_req() {
        if REQ == "{}" {
            return;
        }
        if let Err(e) =
            serde_json::from_str::<super::CreateHireEcoBackgroundCheckPackageReqBody>(REQ)
        {
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
        let res = serde_json::from_str::<CreateHireEcoBackgroundCheckPackageRespInner>(RESP);
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
