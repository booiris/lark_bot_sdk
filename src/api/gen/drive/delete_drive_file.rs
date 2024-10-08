//! doc url: <https://open.larkoffice.com/document/uAjLw4CM/ukTMukTMukTM/reference/drive-v1/file/delete>
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
    /// **api 版本: 2024-05-29T08:44:06+00:00**
    ///
    /// ## 删除文件或文件夹
    ///
    /// 删除用户在云空间内的文件或者文件夹。文件或文件夹被删除后，会进入回收站中。
    ///
    /// 如果你删除的是文件夹，该接口将异步执行，同时返回该异步任务的 ID。你可使用[查询异步任务状态](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/drive-v1/file/task_check)接口查询任务执行的状态。
    ///
    /// doc: <https://open.larkoffice.com/document/uAjLw4CM/ukTMukTMukTM/reference/drive-v1/file/delete>
    ///
    /// new doc: <https://open.larkoffice.com/document/server-docs/docs/drive-v1/file/delete>
    ///
    /// api url: <https://open.feishu.cn/document_portal/v1/document/get_detail?fullPath=%2Fserver-docs%2Fdocs%2Fdrive-v1%2Ffile%2Fdelete>
    pub async fn delete_drive_file(
        &self,
        req: DeleteDriveFileReq,
    ) -> Result<(DeleteDriveFileResp, CommonResponse), Error> {
        self.delete_drive_file_with_opt(req, Default::default())
            .await
    }

    /// 参见 [delete_drive_file](#method.delete_drive_file) 函数
    pub async fn delete_drive_file_with_opt(
        &self,
        req: DeleteDriveFileReq,
        method_option: MethodOption,
    ) -> Result<(DeleteDriveFileResp, CommonResponse), Error> {
        #[cfg(feature = "test-util")]
        {
            if let Some(f) = self.mock().get_mock_delete_drive_file(&req) {
                tracing::info!("[lark] Drive#DeleteDriveFile **mocking** api");
                return f(req);
            }
        }

        tracing::info!("[lark] Drive#DeleteDriveFile call api");

        let req = ApiRequest {
            scope: "Drive",
            api: "DeleteDriveFile",
            method: http::Method::DELETE,
            url: String::new()
                + self.cli.open_base_url.as_ref()
                + "/open-apis/drive/v1/files/:file_token",
            param_data: req.gen_param(),
            method_option,
            need_tenant_access_token: true,
            need_user_access_token: true,
            ..Default::default()
        };

        let (resp, common_resp): (DeleteDriveFileRespInner, _) = self.cli.do_req(req).await?;
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
pub struct DeleteDriveFileReq {
    /// 需要删除的文件或文件夹 token。
    ///
    /// 了解如何获取文件 token，参考[文件概述](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/drive-v1/file/file-overview)。
    ///
    /// 了解如何获取文件夹 token，参考[文件夹概述](https://open.feishu.cn/document/ukTMukTMukTM/ugTNzUjL4UzM14CO1MTN/folder-overview)。
    ///
    /// **示例值**: "boxcnrHpsg1QDqXAAAyachabcef"
    #[api(kind = "path", name = "file_token")]
    pub file_token: String,
    /// 被删除文件的类型
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "file"
    ///
    /// **可选值**:
    ///
    /// `file`: 文件类型
    ///
    /// `Docx`: 新版文档类型
    ///
    /// `Bitable`: 多维表格类型
    ///
    /// `Folder`: 文件夹类型
    ///
    /// `Doc`: 文档类型
    ///
    /// `Sheet`: 电子表格类型
    ///
    /// `Mindnote`: 思维笔记类型
    ///
    /// `Shortcut`: 快捷方式类型
    ///
    /// `Slides`: 幻灯片
    #[api(kind = "query", name = "type", v_type = "var", option = "false")]
    pub query_type: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, lark_bot_sdk_macros::ApiBaseResp)]
struct DeleteDriveFileRespInner {
    #[serde(flatten)]
    data: Option<DeleteDriveFileResp>,
    #[serde(flatten)]
    base: BaseResp,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct DeleteDriveFileResp {
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
    /// 异步任务 ID，删除文件夹时返回。你可继续使用[查询异步任务状态](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/drive-v1/file/task_check)接口查询任务执行状态
    ///
    /// **示例值**: "7360595374803812356"
    #[serde(
        rename = "task_id",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub task_id: String,
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
        Fn(DeleteDriveFileReq) -> Result<(DeleteDriveFileResp, CommonResponse), Error>
        + Send
        + Sync
        + 'static
    {
    }
    impl<
            T: Fn(DeleteDriveFileReq) -> Result<(DeleteDriveFileResp, CommonResponse), Error>
                + Send
                + Sync
                + 'static,
        > MockFunc for T
    {
    }

    impl<'c, IStore: Store, IClient: HttpClient> DriveServiceMocker<'c, IStore, IClient> {
        pub fn mock_delete_drive_file<F: MockFunc>(
            &self,
            f: F,
        ) -> MockerBuilder<Mocker, DeleteDriveFileReq, DeleteDriveFileResp, Arc<dyn MockFunc>>
        {
            MockerBuilder::new(self.cli.instance_id, Arc::new(f))
        }

        pub(super) fn get_mock_delete_drive_file(
            &self,
            req: &DeleteDriveFileReq,
        ) -> Option<Arc<dyn MockFunc>> {
            do_mock::<Mocker, DeleteDriveFileReq, DeleteDriveFileResp, Arc<dyn MockFunc>>(
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
        api::gen::drive::delete_drive_file::{
            DeleteDriveFileReq, DeleteDriveFileResp, DeleteDriveFileRespInner,
        },
        core::{model::CommonResponse, Lark},
    };

    #[tokio::test]
    async fn test_mock() {
        let lark = Lark::new("".to_owned(), "".into());
        let mocker = lark
            .drive()
            .mock()
            .mock_delete_drive_file(|_| {
                Ok((DeleteDriveFileResp::default(), CommonResponse::default()))
            })
            .build();
        let res = lark
            .drive()
            .delete_drive_file(DeleteDriveFileReq::default())
            .await;
        assert!(res.is_ok());
        mocker.clear();
        let res = lark
            .drive()
            .delete_drive_file(DeleteDriveFileReq::default())
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
        "task_id": "7360595374803812356"
    }
}"#;
    #[test]
    fn test_resp() {
        let res = serde_json::from_str::<DeleteDriveFileRespInner>(RESP);
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
