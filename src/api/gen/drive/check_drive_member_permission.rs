//! doc url: <https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/drive-v1/permission-member/auth>
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
    /// **api 版本: 2024-07-23T07:32:27+00:00**
    ///
    /// ## 判断当前用户是否有某权限
    ///
    /// 该接口用于根据 filetoken 判断当前登录用户是否具有某权限。
    ///

    ///
    /// doc: <https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/drive-v1/permission-member/auth>
    ///
    /// new doc: <https://open.feishu.cn/document/server-docs/docs/permission/permission-member/auth>
    ///
    /// api url: <https://open.feishu.cn/document_portal/v1/document/get_detail?fullPath=%2Fserver-docs%2Fdocs%2Fpermission%2Fpermission-member%2Fauth>
    pub async fn check_drive_member_permission(
        &self,
        req: CheckDriveMemberPermissionReq,
    ) -> Result<(CheckDriveMemberPermissionResp, CommonResponse), Error> {
        self.check_drive_member_permission_with_opt(req, Default::default())
            .await
    }

    /// 参见 [check_drive_member_permission](#method.check_drive_member_permission) 函数
    pub async fn check_drive_member_permission_with_opt(
        &self,
        req: CheckDriveMemberPermissionReq,
        method_option: MethodOption,
    ) -> Result<(CheckDriveMemberPermissionResp, CommonResponse), Error> {
        #[cfg(feature = "test-util")]
        {
            if let Some(f) = self.mock().get_mock_check_drive_member_permission(&req) {
                tracing::info!("[lark] Drive#CheckDriveMemberPermission **mocking** api");
                return f(req);
            }
        }

        tracing::info!("[lark] Drive#CheckDriveMemberPermission call api");

        let req = ApiRequest {
            scope: "Drive",
            api: "CheckDriveMemberPermission",
            method: http::Method::GET,
            url: String::new()
                + self.cli.open_base_url.as_ref()
                + "/open-apis/drive/v1/permissions/:token/members/auth",
            param_data: req.gen_param(),
            method_option,
            need_tenant_access_token: true,
            need_user_access_token: true,
            ..Default::default()
        };

        let (resp, common_resp): (CheckDriveMemberPermissionRespInner, _) =
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
pub struct CheckDriveMemberPermissionReq {
    /// 文件的 token
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
    /// 需要判断的权限
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "view"
    ///
    /// **可选值**:
    ///
    /// `View`: 阅读
    ///
    /// `Edit`: 编辑
    ///
    /// `Share`: 分享
    ///
    /// `Comment`: 评论
    ///
    /// `Export`: 导出
    ///
    /// `Copy`: 拷贝
    ///
    /// `Print`: 打印
    ///
    /// `ManagePublic`: 管理权限设置
    #[api(kind = "query", name = "action", v_type = "var", option = "false")]
    pub action: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, lark_bot_sdk_macros::ApiBaseResp)]
struct CheckDriveMemberPermissionRespInner {
    #[serde(flatten)]
    data: Option<CheckDriveMemberPermissionResp>,
    #[serde(flatten)]
    base: BaseResp,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct CheckDriveMemberPermissionResp {
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
    /// 是否有权限
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "true"
    #[serde(
        rename = "auth_result",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub auth_result: bool,
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
            CheckDriveMemberPermissionReq,
        ) -> Result<(CheckDriveMemberPermissionResp, CommonResponse), Error>
        + Send
        + Sync
        + 'static
    {
    }
    impl<
            T: Fn(
                    CheckDriveMemberPermissionReq,
                )
                    -> Result<(CheckDriveMemberPermissionResp, CommonResponse), Error>
                + Send
                + Sync
                + 'static,
        > MockFunc for T
    {
    }

    impl<'c, IStore: Store, IClient: HttpClient> DriveServiceMocker<'c, IStore, IClient> {
        pub fn mock_check_drive_member_permission<F: MockFunc>(
            &self,
            f: F,
        ) -> MockerBuilder<
            Mocker,
            CheckDriveMemberPermissionReq,
            CheckDriveMemberPermissionResp,
            Arc<dyn MockFunc>,
        > {
            MockerBuilder::new(self.cli.instance_id, Arc::new(f))
        }

        pub(super) fn get_mock_check_drive_member_permission(
            &self,
            req: &CheckDriveMemberPermissionReq,
        ) -> Option<Arc<dyn MockFunc>> {
            do_mock::<
                Mocker,
                CheckDriveMemberPermissionReq,
                CheckDriveMemberPermissionResp,
                Arc<dyn MockFunc>,
            >(self.cli.instance_id, req)
        }
    }
}

#[cfg(feature = "test-util")]
#[cfg(test)]
mod test {
    use crate::{
        api::gen::drive::check_drive_member_permission::{
            CheckDriveMemberPermissionReq, CheckDriveMemberPermissionResp,
            CheckDriveMemberPermissionRespInner,
        },
        core::{model::CommonResponse, Lark},
    };

    #[tokio::test]
    async fn test_mock() {
        let lark = Lark::new("".to_owned(), "".into());
        let mocker = lark
            .drive()
            .mock()
            .mock_check_drive_member_permission(|_| {
                Ok((
                    CheckDriveMemberPermissionResp::default(),
                    CommonResponse::default(),
                ))
            })
            .build();
        let res = lark
            .drive()
            .check_drive_member_permission(CheckDriveMemberPermissionReq::default())
            .await;
        assert!(res.is_ok());
        mocker.clear();
        let res = lark
            .drive()
            .check_drive_member_permission(CheckDriveMemberPermissionReq::default())
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
        "auth_result": true
    }
}"#;
    #[test]
    fn test_resp() {
        let res = serde_json::from_str::<CheckDriveMemberPermissionRespInner>(RESP);
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
