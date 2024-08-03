//! doc url: <https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/drive-v1/permission-public/get>
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
    /// **api 版本: 2024-07-31T09:16:05+00:00**
    ///
    /// ## 获取云文档权限设置
    ///
    /// 该接口用于根据 filetoken 获取云文档的权限设置。
    ///

    ///
    /// doc: <https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/drive-v1/permission-public/get>
    ///
    /// new doc: <https://open.feishu.cn/document/server-docs/docs/permission/permission-public/get>
    ///
    /// api url: <https://open.feishu.cn/document_portal/v1/document/get_detail?fullPath=%2Fserver-docs%2Fdocs%2Fpermission%2Fpermission-public%2Fget>
    pub async fn get_drive_public_permission(
        &self,
        req: GetDrivePublicPermissionReq,
    ) -> Result<(GetDrivePublicPermissionResp, CommonResponse), Error> {
        self.get_drive_public_permission_with_opt(req, Default::default())
            .await
    }

    /// 参见 [get_drive_public_permission](#method.get_drive_public_permission) 函数
    pub async fn get_drive_public_permission_with_opt(
        &self,
        req: GetDrivePublicPermissionReq,
        method_option: MethodOption,
    ) -> Result<(GetDrivePublicPermissionResp, CommonResponse), Error> {
        #[cfg(feature = "test-util")]
        {
            if let Some(f) = self.mock().get_mock_get_drive_public_permission(&req) {
                tracing::info!("[lark] Drive#GetDrivePublicPermission **mocking** api");
                return f(req);
            }
        }

        tracing::info!("[lark] Drive#GetDrivePublicPermission call api");

        let req = ApiRequest {
            scope: "Drive",
            api: "GetDrivePublicPermission",
            method: http::Method::GET,
            url: String::new()
                + self.cli.open_base_url.as_ref()
                + "/open-apis/drive/v1/permissions/:token/public",
            param_data: req.gen_param(),
            method_option,
            need_tenant_access_token: true,
            need_user_access_token: true,
            ..Default::default()
        };

        let (resp, common_resp): (GetDrivePublicPermissionRespInner, _) =
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
pub struct GetDrivePublicPermissionReq {
    /// 文件的 token，获取方式见 [如何获取云文档资源相关 token](https://open.feishu.cn/document/ukTMukTMukTM/uczNzUjL3czM14yN3MTN#08bb5df6)
    ///
    /// **是否必填**: 是
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
    /// `Minutes`: 妙记
    ///
    /// `Slides`: 幻灯片
    #[api(kind = "query", name = "type", v_type = "var", option = "false")]
    pub query_type: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, lark_bot_sdk_macros::ApiBaseResp)]
struct GetDrivePublicPermissionRespInner {
    #[serde(flatten)]
    data: Option<GetDrivePublicPermissionResp>,
    #[serde(flatten)]
    base: BaseResp,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct GetDrivePublicPermissionResp {
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
    /// 返回的文档权限设置
    #[serde(
        rename = "permission_public",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub permission_public: PermissionPublicSubResp,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(default)]
pub struct PermissionPublicSubResp {
    /// 允许内容被分享到组织外
    ///
    /// **可选值有：**
    ///
    /// - `true`: 允许
    ///
    /// - `false`: 不允许
    ///
    /// **示例值**: "true"
    #[serde(
        rename = "external_access",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub external_access: bool,
    /// 谁可以复制内容、创建副本、打印、下载
    ///
    /// **示例值**: "anyone_can_view"
    ///
    /// **可选值**:
    ///
    /// `AnyoneCanView`: 拥有可阅读权限的用户
    ///
    /// `AnyoneCanEdit`: 拥有可编辑权限的用户
    ///
    /// `OnlyFullAccess`: 拥有可管理权限（包括我）的用户
    #[serde(
        rename = "security_entity",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub security_entity: String,
    /// 可评论设置
    ///
    /// **示例值**: "anyone_can_view"
    ///
    /// **可选值**:
    ///
    /// `AnyoneCanView`: 拥有可阅读权限的用户
    ///
    /// `AnyoneCanEdit`: 拥有可编辑权限的用户
    #[serde(
        rename = "comment_entity",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub comment_entity: String,
    /// 谁可以添加和管理协作者
    ///
    /// **示例值**: "anyone"
    ///
    /// **可选值**:
    ///
    /// `Anyone`: 所有可阅读或编辑此文档的用户
    ///
    /// `SameTenant`: 组织内所有可阅读或编辑此文档的用户
    ///
    /// `OnlyFullAccess`: 拥有可管理权限（包括我）的用户
    #[serde(
        rename = "share_entity",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub share_entity: String,
    /// 链接分享设置
    ///
    /// **示例值**: "tenant_readable"
    ///
    /// **可选值**:
    ///
    /// `TenantReadable`: 组织内获得链接的人可阅读
    ///
    /// `TenantEditable`: 组织内获得链接的人可编辑
    ///
    /// `AnyoneReadable`: 互联网上获得链接的任何人可阅读
    ///
    /// `AnyoneEditable`: 互联网上获得链接的任何人可编辑
    ///
    /// `Closed`: 关闭链接分享
    #[serde(
        rename = "link_share_entity",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub link_share_entity: String,
    /// 允许非「可管理权限」的人分享到组织外
    ///
    /// **示例值**: "true"
    #[serde(
        rename = "invite_external",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub invite_external: bool,
    /// 节点加锁状态
    ///
    /// **示例值**: "false"
    #[serde(
        rename = "lock_switch",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub lock_switch: bool,
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
            GetDrivePublicPermissionReq,
        ) -> Result<(GetDrivePublicPermissionResp, CommonResponse), Error>
        + Send
        + Sync
        + 'static
    {
    }
    impl<
            T: Fn(
                    GetDrivePublicPermissionReq,
                ) -> Result<(GetDrivePublicPermissionResp, CommonResponse), Error>
                + Send
                + Sync
                + 'static,
        > MockFunc for T
    {
    }

    impl<'c, IStore: Store, IClient: HttpClient> DriveServiceMocker<'c, IStore, IClient> {
        pub fn mock_get_drive_public_permission<F: MockFunc>(
            &self,
            f: F,
        ) -> MockerBuilder<
            Mocker,
            GetDrivePublicPermissionReq,
            GetDrivePublicPermissionResp,
            Arc<dyn MockFunc>,
        > {
            MockerBuilder::new(self.cli.instance_id, Arc::new(f))
        }

        pub(super) fn get_mock_get_drive_public_permission(
            &self,
            req: &GetDrivePublicPermissionReq,
        ) -> Option<Arc<dyn MockFunc>> {
            do_mock::<
                Mocker,
                GetDrivePublicPermissionReq,
                GetDrivePublicPermissionResp,
                Arc<dyn MockFunc>,
            >(self.cli.instance_id, req)
        }
    }
}

#[cfg(feature = "test-util")]
#[cfg(test)]
mod test {
    use crate::{
        api::gen::drive::get_drive_public_permission::{
            GetDrivePublicPermissionReq, GetDrivePublicPermissionResp,
            GetDrivePublicPermissionRespInner,
        },
        core::{model::CommonResponse, Lark},
    };

    #[tokio::test]
    async fn test_mock() {
        let lark = Lark::new("".to_owned(), "".into());
        let mocker = lark
            .drive()
            .mock()
            .mock_get_drive_public_permission(|_| {
                Ok((
                    GetDrivePublicPermissionResp::default(),
                    CommonResponse::default(),
                ))
            })
            .build();
        let res = lark
            .drive()
            .get_drive_public_permission(GetDrivePublicPermissionReq::default())
            .await;
        assert!(res.is_ok());
        mocker.clear();
        let res = lark
            .drive()
            .get_drive_public_permission(GetDrivePublicPermissionReq::default())
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
        "permission_public": {
            "external_access": true,
            "security_entity": "anyone_can_view",
            "comment_entity": "anyone_can_view",
            "share_entity": "anyone",
            "link_share_entity": "tenant_readable",
            "invite_external": true,
            "lock_switch": false
        }
    }
}"#;
    #[test]
    fn test_resp() {
        let res = serde_json::from_str::<GetDrivePublicPermissionRespInner>(RESP);
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
