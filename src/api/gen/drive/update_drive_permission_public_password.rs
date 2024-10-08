//! doc url: <https://open.larkoffice.com/document/uAjLw4CM/ukTMukTMukTM/reference/drive-v1/permission-public-password/update>
// Code generated by gen_api. DO NOT EDIT.

use serde::{Deserialize, Serialize};

use crate::api::BaseResp;
use crate::api::HasBaseResp;
use crate::core::model::{ApiRequest, CommonResponse, MethodOption, ReqParam, StreamReqParam};
use crate::{
    core::{http_client::HttpClient, store::Store},
    error::Error,
};

use crate::api::gen::drive::DriveService;

impl<'c, IStore: Store, IClient: HttpClient> DriveService<'c, IStore, IClient> {
    /// **api 版本: 2024-04-09T07:36:38+00:00**
    ///
    /// ## 刷新密码
    ///
    /// 该接口用于根据 filetoken 刷新云文档的密码。
    ///
    /// 注意：刷新密码，需要先通过”云文档“-“权限”-“设置”-“更新云文档权限设置”的接口更新元文档为互联网上获得链接的任何人可阅读/编辑
    ///
    /// doc: <https://open.larkoffice.com/document/uAjLw4CM/ukTMukTMukTM/reference/drive-v1/permission-public-password/update>
    ///
    /// new doc: <https://open.larkoffice.com/document/server-docs/docs/permission/permission-public/permission-public-password/update>
    ///
    /// api url: <https://open.feishu.cn/document_portal/v1/document/get_detail?fullPath=%2Fserver-docs%2Fdocs%2Fpermission%2Fpermission-public%2Fpermission-public-password%2Fupdate>
    pub async fn update_drive_permission_public_password(
        &self,
        req: UpdateDrivePermissionPublicPasswordReq,
    ) -> Result<(UpdateDrivePermissionPublicPasswordResp, CommonResponse), Error> {
        self.update_drive_permission_public_password_with_opt(req, Default::default())
            .await
    }

    /// 参见 [update_drive_permission_public_password](#method.update_drive_permission_public_password) 函数
    pub async fn update_drive_permission_public_password_with_opt(
        &self,
        req: UpdateDrivePermissionPublicPasswordReq,
        method_option: MethodOption,
    ) -> Result<(UpdateDrivePermissionPublicPasswordResp, CommonResponse), Error> {
        #[cfg(feature = "test-util")]
        {
            if let Some(f) = self
                .mock()
                .get_mock_update_drive_permission_public_password(&req)
            {
                tracing::info!("[lark] Drive#UpdateDrivePermissionPublicPassword **mocking** api");
                return f(req);
            }
        }

        tracing::info!("[lark] Drive#UpdateDrivePermissionPublicPassword call api");

        let req = ApiRequest {
            scope: "Drive",
            api: "UpdateDrivePermissionPublicPassword",
            method: http::Method::PUT,
            url: String::new()
                + self.cli.open_base_url.as_ref()
                + "/open-apis/drive/v1/permissions/:token/public/password",
            param_data: req.gen_param(),
            method_option,
            need_tenant_access_token: true,
            need_user_access_token: true,
            ..Default::default()
        };

        let (resp, common_resp): (UpdateDrivePermissionPublicPasswordRespInner, _) =
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
pub struct UpdateDrivePermissionPublicPasswordReq {
    /// 文件的 token，获取方式见 [如何获取云文档资源相关 token](https://open.feishu.cn/document/ukTMukTMukTM/uczNzUjL3czM14yN3MTN#08bb5df6)
    ///
    /// **示例值**: "doccnBKgoMyY5OMbUG6FioTXuBe"
    #[api(kind = "path", name = "token")]
    pub token: String,
    /// 文件类型，需要与文件的 token 相匹配
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "doc"
    ///
    /// **可选值**:
    ///
    /// `Doc`: 文档
    ///
    /// `Sheet`: 电子表格
    ///
    /// `File`: 云空间文件
    ///
    /// `Wiki`: 知识库节点
    ///
    /// `Bitable`: 多维表格
    ///
    /// `Docx`: 新版文档
    ///
    /// `Mindnote`: 思维笔记
    ///
    /// `Minutes`: 妙计（暂不支持）
    ///
    /// `Slides`: 幻灯片
    #[api(kind = "query", name = "type", v_type = "var", option = "false")]
    pub query_type: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, lark_bot_sdk_macros::ApiBaseResp)]
struct UpdateDrivePermissionPublicPasswordRespInner {
    #[serde(flatten)]
    data: Option<UpdateDrivePermissionPublicPasswordResp>,
    #[serde(flatten)]
    base: BaseResp,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct UpdateDrivePermissionPublicPasswordResp {
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
    /// 密码
    ///
    /// **示例值**: "A8e6"
    #[serde(
        rename = "password",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub password: String,
}

#[cfg(feature = "test-util")]
mod test_utils {
    use std::sync::Arc;

    use self::gen::drive::DriveServiceMocker;
    use crate::core::mocker::*;

    use super::*;
    use crate::api::*;
    use crate::core::model::*;

    pub struct Mocker;

    pub trait MockFunc:
        Fn(
            UpdateDrivePermissionPublicPasswordReq,
        ) -> Result<(UpdateDrivePermissionPublicPasswordResp, CommonResponse), Error>
        + Send
        + Sync
        + 'static
    {
    }
    impl<
            T: Fn(
                    UpdateDrivePermissionPublicPasswordReq,
                )
                    -> Result<(UpdateDrivePermissionPublicPasswordResp, CommonResponse), Error>
                + Send
                + Sync
                + 'static,
        > MockFunc for T
    {
    }

    impl<'c, IStore: Store, IClient: HttpClient> DriveServiceMocker<'c, IStore, IClient> {
        pub fn mock_update_drive_permission_public_password<F: MockFunc>(
            &self,
            f: F,
        ) -> MockerBuilder<
            Mocker,
            UpdateDrivePermissionPublicPasswordReq,
            UpdateDrivePermissionPublicPasswordResp,
            Arc<dyn MockFunc>,
        > {
            MockerBuilder::new(self.cli.instance_id, Arc::new(f))
        }

        pub(super) fn get_mock_update_drive_permission_public_password(
            &self,
            req: &UpdateDrivePermissionPublicPasswordReq,
        ) -> Option<Arc<dyn MockFunc>> {
            do_mock::<
                Mocker,
                UpdateDrivePermissionPublicPasswordReq,
                UpdateDrivePermissionPublicPasswordResp,
                Arc<dyn MockFunc>,
            >(self.cli.instance_id, req)
        }
    }
}

#[cfg(feature = "test-util")]
#[cfg(test)]
mod test {
    use crate::{
        api::gen::drive::update_drive_permission_public_password::{
            UpdateDrivePermissionPublicPasswordReq, UpdateDrivePermissionPublicPasswordResp,
            UpdateDrivePermissionPublicPasswordRespInner,
        },
        core::{model::CommonResponse, Lark},
    };

    #[tokio::test]
    async fn test_mock() {
        let lark = Lark::new("".to_owned(), "".into());
        let mocker = lark
            .drive()
            .mock()
            .mock_update_drive_permission_public_password(|_| {
                Ok((
                    UpdateDrivePermissionPublicPasswordResp::default(),
                    CommonResponse::default(),
                ))
            })
            .build();
        let res = lark
            .drive()
            .update_drive_permission_public_password(
                UpdateDrivePermissionPublicPasswordReq::default(),
            )
            .await;
        assert!(res.is_ok());
        mocker.clear();
        let res = lark
            .drive()
            .update_drive_permission_public_password(
                UpdateDrivePermissionPublicPasswordReq::default(),
            )
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
        "password": "A8e6"
    }
}"#;
    #[test]
    fn test_resp() {
        let res = serde_json::from_str::<UpdateDrivePermissionPublicPasswordRespInner>(RESP);
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
