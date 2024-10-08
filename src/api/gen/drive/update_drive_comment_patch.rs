//! doc url: <https://open.larkoffice.com/document/uAjLw4CM/ukTMukTMukTM/reference/drive-v1/file-comment/patch>
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
    /// **api 版本: 2024-04-09T11:08:32+00:00**
    ///
    /// ## 解决/恢复评论
    ///
    /// 解决或恢复云文档中的评论。
    ///

    ///
    /// doc: <https://open.larkoffice.com/document/uAjLw4CM/ukTMukTMukTM/reference/drive-v1/file-comment/patch>
    ///
    /// new doc: <https://open.larkoffice.com/document/server-docs/docs/CommentAPI/patch>
    ///
    /// api url: <https://open.feishu.cn/document_portal/v1/document/get_detail?fullPath=%2Fserver-docs%2Fdocs%2FCommentAPI%2Fpatch>
    pub async fn update_drive_comment_patch(
        &self,
        req: UpdateDriveCommentPatchReq,
    ) -> Result<(UpdateDriveCommentPatchResp, CommonResponse), Error> {
        self.update_drive_comment_patch_with_opt(req, Default::default())
            .await
    }

    /// 参见 [update_drive_comment_patch](#method.update_drive_comment_patch) 函数
    pub async fn update_drive_comment_patch_with_opt(
        &self,
        req: UpdateDriveCommentPatchReq,
        method_option: MethodOption,
    ) -> Result<(UpdateDriveCommentPatchResp, CommonResponse), Error> {
        #[cfg(feature = "test-util")]
        {
            if let Some(f) = self.mock().get_mock_update_drive_comment_patch(&req) {
                tracing::info!("[lark] Drive#UpdateDriveCommentPatch **mocking** api");
                return f(req);
            }
        }

        tracing::info!("[lark] Drive#UpdateDriveCommentPatch call api");

        let req = ApiRequest {
            scope: "Drive",
            api: "UpdateDriveCommentPatch",
            method: http::Method::PATCH,
            url: String::new()
                + self.cli.open_base_url.as_ref()
                + "/open-apis/drive/v1/files/:file_token/comments/:comment_id",
            param_data: req.gen_param(),
            method_option,
            need_tenant_access_token: true,
            need_user_access_token: true,
            ..Default::default()
        };

        let (resp, common_resp): (UpdateDriveCommentPatchRespInner, _) =
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
pub struct UpdateDriveCommentPatchReq {
    /// 文档token
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "doccnGp4UK1UskrOEJwBXd3****"
    #[api(kind = "path", name = "file_token")]
    pub file_token: String,
    /// 评论ID
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "6916106822734578184"
    #[api(kind = "path", name = "comment_id")]
    pub comment_id: String,
    /// 文档类型
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "doc"
    ///
    /// **可选值**:
    ///
    /// `doc`: 文档
    ///
    /// `sheet`: 表格
    ///
    /// `file`: 文件
    ///
    /// `docx`: 新版文档
    #[api(kind = "query", name = "file_type", v_type = "var", option = "false")]
    pub file_type: String,
    /// 评论解决标志
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "true"
    #[api(kind = "body", name = "is_solved")]
    pub is_solved: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone, lark_bot_sdk_macros::ApiBaseResp)]
struct UpdateDriveCommentPatchRespInner {
    #[serde(flatten)]
    data: Option<UpdateDriveCommentPatchResp>,
    #[serde(flatten)]
    base: BaseResp,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct UpdateDriveCommentPatchResp {
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

    use self::gen::drive::DriveServiceMocker;
    use crate::core::mocker::*;

    use super::*;
    use crate::api::*;
    use crate::core::model::*;

    pub struct Mocker;

    pub trait MockFunc:
        Fn(
            UpdateDriveCommentPatchReq,
        ) -> Result<(UpdateDriveCommentPatchResp, CommonResponse), Error>
        + Send
        + Sync
        + 'static
    {
    }
    impl<
            T: Fn(
                    UpdateDriveCommentPatchReq,
                ) -> Result<(UpdateDriveCommentPatchResp, CommonResponse), Error>
                + Send
                + Sync
                + 'static,
        > MockFunc for T
    {
    }

    impl<'c, IStore: Store, IClient: HttpClient> DriveServiceMocker<'c, IStore, IClient> {
        pub fn mock_update_drive_comment_patch<F: MockFunc>(
            &self,
            f: F,
        ) -> MockerBuilder<
            Mocker,
            UpdateDriveCommentPatchReq,
            UpdateDriveCommentPatchResp,
            Arc<dyn MockFunc>,
        > {
            MockerBuilder::new(self.cli.instance_id, Arc::new(f))
        }

        pub(super) fn get_mock_update_drive_comment_patch(
            &self,
            req: &UpdateDriveCommentPatchReq,
        ) -> Option<Arc<dyn MockFunc>> {
            do_mock::<
                Mocker,
                UpdateDriveCommentPatchReq,
                UpdateDriveCommentPatchResp,
                Arc<dyn MockFunc>,
            >(self.cli.instance_id, req)
        }
    }
}

#[cfg(feature = "test-util")]
#[cfg(test)]
mod test {
    use crate::{
        api::gen::drive::update_drive_comment_patch::{
            UpdateDriveCommentPatchReq, UpdateDriveCommentPatchResp,
            UpdateDriveCommentPatchRespInner,
        },
        core::{model::CommonResponse, Lark},
    };

    #[tokio::test]
    async fn test_mock() {
        let lark = Lark::new("".to_owned(), "".into());
        let mocker = lark
            .drive()
            .mock()
            .mock_update_drive_comment_patch(|_| {
                Ok((
                    UpdateDriveCommentPatchResp::default(),
                    CommonResponse::default(),
                ))
            })
            .build();
        let res = lark
            .drive()
            .update_drive_comment_patch(UpdateDriveCommentPatchReq::default())
            .await;
        assert!(res.is_ok());
        mocker.clear();
        let res = lark
            .drive()
            .update_drive_comment_patch(UpdateDriveCommentPatchReq::default())
            .await;
        assert!(res.is_err());
    }

    const REQ: &str = r#"{
    "is_solved": true
}"#;

    #[test]
    fn test_req() {
        if REQ == "{}" {
            return;
        }
        if let Err(e) = serde_json::from_str::<super::UpdateDriveCommentPatchReqBody>(REQ) {
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
        let res = serde_json::from_str::<UpdateDriveCommentPatchRespInner>(RESP);
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
