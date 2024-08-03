//! doc url: <https://open.feishu.cn/document/ukTMukTMukTM/uIzNzUjLyczM14iM3MTN/drive-v2/permission-public/patch>
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
    /// **api 版本: 2024-07-31T09:16:49+00:00**
    ///
    /// ## 更新云文档权限设置
    ///
    /// 该接口用于根据 filetoken 更新云文档的权限设置。
    ///

    ///
    /// doc: <https://open.feishu.cn/document/ukTMukTMukTM/uIzNzUjLyczM14iM3MTN/drive-v2/permission-public/patch>
    ///
    /// new doc: <https://open.feishu.cn/document/server-docs/docs/permission/permission-public/patch-2>
    ///
    /// api url: <https://open.feishu.cn/document_portal/v1/document/get_detail?fullPath=%2Fserver-docs%2Fdocs%2Fpermission%2Fpermission-public%2Fpatch-2>
    pub async fn update_drive_public_permission_v2(
        &self,
        req: UpdateDrivePublicPermissionV2Req,
    ) -> Result<(UpdateDrivePublicPermissionV2Resp, CommonResponse), Error> {
        self.update_drive_public_permission_v2_with_opt(req, Default::default())
            .await
    }

    /// 参见 [update_drive_public_permission_v2](#method.update_drive_public_permission_v2) 函数
    pub async fn update_drive_public_permission_v2_with_opt(
        &self,
        req: UpdateDrivePublicPermissionV2Req,
        method_option: MethodOption,
    ) -> Result<(UpdateDrivePublicPermissionV2Resp, CommonResponse), Error> {
        #[cfg(feature = "test-util")]
        {
            if let Some(f) = self.mock().get_mock_update_drive_public_permission_v2(&req) {
                tracing::info!("[lark] Drive#UpdateDrivePublicPermissionV2 **mocking** api");
                return f(req);
            }
        }

        tracing::info!("[lark] Drive#UpdateDrivePublicPermissionV2 call api");

        let req = ApiRequest {
            scope: "Drive",
            api: "UpdateDrivePublicPermissionV2",
            method: http::Method::PATCH,
            url: String::new()
                + self.cli.open_base_url.as_ref()
                + "/open-apis/drive/v2/permissions/:token/public",
            param_data: req.gen_param(),
            method_option,
            need_tenant_access_token: true,
            need_user_access_token: true,
            ..Default::default()
        };

        let (resp, common_resp): (UpdateDrivePublicPermissionV2RespInner, _) =
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
pub struct UpdateDrivePublicPermissionV2Req {
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
    /// `Doc`: 旧版文档
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
    /// 允许内容被分享到组织外
    ///
    /// **示例值**: "open"
    ///
    /// **可选值**:
    ///
    /// `Open`: 打开
    ///
    /// `Closed`: 关闭
    ///
    /// `AllowSharePartnerTenant`: 允许分享给关联组织（只有租户后台设置仅允许关联组织分享，才能设置为该值）
    #[api(kind = "body", name = "external_access_entity")]
    pub external_access_entity: Option<String>,
    /// 谁可以创建副本、打印、下载
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
    #[api(kind = "body", name = "security_entity")]
    pub security_entity: Option<String>,
    /// 谁可以评论
    ///
    /// **示例值**: "anyone_can_view"
    ///
    /// **可选值**:
    ///
    /// `AnyoneCanView`: 拥有可阅读权限的用户
    ///
    /// `AnyoneCanEdit`: 拥有可编辑权限的用户
    #[api(kind = "body", name = "comment_entity")]
    pub comment_entity: Option<String>,
    /// 谁可以添加和管理协作者-组织维度
    ///
    /// **示例值**: "anyone"
    ///
    /// **可选值**:
    ///
    /// `Anyone`: 所有可阅读或编辑此文档的用户
    ///
    /// `SameTenant`: 组织内所有可阅读或编辑此文档的用户
    #[api(kind = "body", name = "share_entity")]
    pub share_entity: Option<String>,
    /// 谁可以添加和管理协作者-协作者维度
    ///
    /// **示例值**: "collaborator_can_view"
    ///
    /// **可选值**:
    ///
    /// `CollaboratorCanView`: 拥有可阅读权限的协作者
    ///
    /// `CollaboratorCanEdit`: 拥有可编辑权限的协作者
    ///
    /// `CollaboratorFullAccess`: 拥有可管理权限（包括我）的协作者
    #[api(kind = "body", name = "manage_collaborator_entity")]
    pub manage_collaborator_entity: Option<String>,
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
    /// `PartnerTenantReadable`: 关联组织的人可阅读（只有租户后台设置仅允许关联组织分享，才能设置为该值）
    ///
    /// `PartnerTenantEditable`: 关联组织的人可编辑（只有租户后台设置仅允许关联组织分享，才能设置为该值）
    ///
    /// `AnyoneReadable`: 互联网上获得链接的任何人可阅读（仅external_access_entity=“open”时有效）
    ///
    /// `AnyoneEditable`: 互联网上获得链接的任何人可编辑（仅external_access_entity=“open”时有效）
    ///
    /// `Closed`: 关闭链接分享
    #[api(kind = "body", name = "link_share_entity")]
    pub link_share_entity: Option<String>,
    /// 谁可以复制内容
    ///
    /// **示例值**: "anyone_can_view"
    ///
    /// **可选值**:
    ///
    /// `AnyoneCanView`: 拥有可阅读权限的用户
    ///
    /// `AnyoneCanEdit`: 拥有可编辑权限的用户
    ///
    /// `OnlyFullAccess`: 拥有可管理权限（包括我）的协作者
    #[api(kind = "body", name = "copy_entity")]
    pub copy_entity: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, lark_bot_sdk_macros::ApiBaseResp)]
struct UpdateDrivePublicPermissionV2RespInner {
    #[serde(flatten)]
    data: Option<UpdateDrivePublicPermissionV2Resp>,
    #[serde(flatten)]
    base: BaseResp,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct UpdateDrivePublicPermissionV2Resp {
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
    /// 本次更新后文档公共设置
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
    /// **示例值**: "open"
    ///
    /// **可选值**:
    ///
    /// `Open`: 打开
    ///
    /// `Closed`: 关闭
    ///
    /// `AllowSharePartnerTenant`: 允许分享给关联组织
    #[serde(
        rename = "external_access_entity",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub external_access_entity: String,
    /// 谁可以创建副本、打印、下载
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
    /// 谁可以评论
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
    /// 谁可以添加和管理协作者-组织维度
    ///
    /// **示例值**: "anyone"
    ///
    /// **可选值**:
    ///
    /// `Anyone`: 所有可阅读或编辑此文档的用户
    ///
    /// `SameTenant`: 组织内所有可阅读或编辑此文档的用户
    #[serde(
        rename = "share_entity",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub share_entity: String,
    /// 谁可以添加和管理协作者-协作者维度
    ///
    /// **示例值**: "collaborator_can_view"
    ///
    /// **可选值**:
    ///
    /// `CollaboratorCanView`: 拥有可阅读权限的协作者
    ///
    /// `CollaboratorCanEdit`: 拥有可编辑权限的协作者
    ///
    /// `CollaboratorFullAccess`: 拥有可管理权限（包括我）的协作者
    #[serde(
        rename = "manage_collaborator_entity",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub manage_collaborator_entity: String,
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
    /// `PartnerTenantReadable`: 关联组织的人可阅读
    ///
    /// `PartnerTenantEditable`: 关联组织的人可编辑
    ///
    /// `AnyoneReadable`: 互联网上获得链接的任何人可阅读（仅external_access_entity=“open”时有效）
    ///
    /// `AnyoneEditable`: 互联网上获得链接的任何人可编辑（仅external_access_entity=“open”时有效）
    ///
    /// `Closed`: 关闭链接分享
    #[serde(
        rename = "link_share_entity",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub link_share_entity: String,
    /// 谁可以复制内容
    ///
    /// **示例值**: "anyone_can_view"
    ///
    /// **可选值**:
    ///
    /// `AnyoneCanView`: 拥有可阅读权限的用户
    ///
    /// `AnyoneCanEdit`: 拥有可编辑权限的用户
    ///
    /// `OnlyFullAccess`: 拥有可管理权限（包括我）的协作者
    #[serde(
        rename = "copy_entity",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub copy_entity: String,
    /// 节点是否已加锁，加锁之后不再继承父级页面的权限
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
            UpdateDrivePublicPermissionV2Req,
        ) -> Result<(UpdateDrivePublicPermissionV2Resp, CommonResponse), Error>
        + Send
        + Sync
        + 'static
    {
    }
    impl<
            T: Fn(
                    UpdateDrivePublicPermissionV2Req,
                )
                    -> Result<(UpdateDrivePublicPermissionV2Resp, CommonResponse), Error>
                + Send
                + Sync
                + 'static,
        > MockFunc for T
    {
    }

    impl<'c, IStore: Store, IClient: HttpClient> DriveServiceMocker<'c, IStore, IClient> {
        pub fn mock_update_drive_public_permission_v2<F: MockFunc>(
            &self,
            f: F,
        ) -> MockerBuilder<
            Mocker,
            UpdateDrivePublicPermissionV2Req,
            UpdateDrivePublicPermissionV2Resp,
            Arc<dyn MockFunc>,
        > {
            MockerBuilder::new(self.cli.instance_id, Arc::new(f))
        }

        pub(super) fn get_mock_update_drive_public_permission_v2(
            &self,
            req: &UpdateDrivePublicPermissionV2Req,
        ) -> Option<Arc<dyn MockFunc>> {
            do_mock::<
                Mocker,
                UpdateDrivePublicPermissionV2Req,
                UpdateDrivePublicPermissionV2Resp,
                Arc<dyn MockFunc>,
            >(self.cli.instance_id, req)
        }
    }
}

#[cfg(feature = "test-util")]
#[cfg(test)]
mod test {
    use crate::{
        api::gen::drive::update_drive_public_permission_v2::{
            UpdateDrivePublicPermissionV2Req, UpdateDrivePublicPermissionV2Resp,
            UpdateDrivePublicPermissionV2RespInner,
        },
        core::{model::CommonResponse, Lark},
    };

    #[tokio::test]
    async fn test_mock() {
        let lark = Lark::new("".to_owned(), "".into());
        let mocker = lark
            .drive()
            .mock()
            .mock_update_drive_public_permission_v2(|_| {
                Ok((
                    UpdateDrivePublicPermissionV2Resp::default(),
                    CommonResponse::default(),
                ))
            })
            .build();
        let res = lark
            .drive()
            .update_drive_public_permission_v2(UpdateDrivePublicPermissionV2Req::default())
            .await;
        assert!(res.is_ok());
        mocker.clear();
        let res = lark
            .drive()
            .update_drive_public_permission_v2(UpdateDrivePublicPermissionV2Req::default())
            .await;
        assert!(res.is_err());
    }

    const REQ: &str = r#"{
    "external_access_entity": "open",
    "security_entity": "anyone_can_view",
    "comment_entity": "anyone_can_view",
    "share_entity": "anyone",
    "manage_collaborator_entity": "collaborator_can_view",
    "link_share_entity": "tenant_readable",
    "copy_entity": "anyone_can_view"
}"#;

    #[test]
    fn test_req() {
        if REQ == "{}" {
            return;
        }
        if let Err(e) = serde_json::from_str::<super::UpdateDrivePublicPermissionV2ReqBody>(REQ) {
            panic!("{}", e);
        }
    }

    const RESP: &str = r#"{
    "code": 0,
    "msg": "success",
    "data": {
        "permission_public": {
            "external_access_entity": "open",
            "security_entity": "anyone_can_view",
            "comment_entity": "anyone_can_view",
            "share_entity": "anyone",
            "manage_collaborator_entity": "collaborator_can_view",
            "link_share_entity": "tenant_readable",
            "copy_entity": "anyone_can_view",
            "lock_switch": false
        }
    }
}"#;
    #[test]
    fn test_resp() {
        let res = serde_json::from_str::<UpdateDrivePublicPermissionV2RespInner>(RESP);
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
