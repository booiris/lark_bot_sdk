//! doc url: <https://open.larkoffice.com/document/uAjLw4CM/ukTMukTMukTM/reference/task-v1/task-comment/delete>
// Code generated by gen_api. DO NOT EDIT.

use serde::{Deserialize, Serialize};

use crate::api::BaseResp;
use crate::api::HasBaseResp;
use crate::core::model::{ApiRequest, CommonResponse, MethodOption, ReqParam, StreamReqParam};
use crate::{
    core::{http_client::HttpClient, store::Store},
    error::Error,
};

use crate::api::gen::task::TaskService;

impl<'c, IStore: Store, IClient: HttpClient> TaskService<'c, IStore, IClient> {
    /// **api 版本: 2023-07-13T02:33:26+00:00**
    ///
    /// ## 删除评论
    ///
    /// 该接口用于通过评论ID删除评论。
    ///

    ///
    /// doc: <https://open.larkoffice.com/document/uAjLw4CM/ukTMukTMukTM/reference/task-v1/task-comment/delete>
    ///
    /// new doc: <https://open.larkoffice.com/document/server-docs/task-v1/task-comment/delete>
    ///
    /// api url: <https://open.feishu.cn/document_portal/v1/document/get_detail?fullPath=%2Fserver-docs%2Ftask-v1%2Ftask-comment%2Fdelete>
    pub async fn delete_task_comment(
        &self,
        req: DeleteTaskCommentReq,
    ) -> Result<(DeleteTaskCommentResp, CommonResponse), Error> {
        self.delete_task_comment_with_opt(req, Default::default())
            .await
    }

    /// 参见 [delete_task_comment](#method.delete_task_comment) 函数
    pub async fn delete_task_comment_with_opt(
        &self,
        req: DeleteTaskCommentReq,
        method_option: MethodOption,
    ) -> Result<(DeleteTaskCommentResp, CommonResponse), Error> {
        #[cfg(feature = "test-util")]
        {
            if let Some(f) = self.mock().get_mock_delete_task_comment(&req) {
                tracing::info!("[lark] Task#DeleteTaskComment **mocking** api");
                return f(req);
            }
        }

        tracing::info!("[lark] Task#DeleteTaskComment call api");

        let req = ApiRequest {
            scope: "Task",
            api: "DeleteTaskComment",
            method: http::Method::DELETE,
            url: String::new()
                + self.cli.open_base_url.as_ref()
                + "/open-apis/task/v1/tasks/:task_id/comments/:comment_id",
            param_data: req.gen_param(),
            method_option,
            need_tenant_access_token: true,
            need_user_access_token: true,
            ..Default::default()
        };

        let (resp, common_resp): (DeleteTaskCommentRespInner, _) = self.cli.do_req(req).await?;
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
pub struct DeleteTaskCommentReq {
    /// 任务ID
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "83912691-2e43-47fc-94a4-d512e03984fa"
    #[api(kind = "path", name = "task_id")]
    pub task_id: String,
    /// 评论ID
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "6937231762296684564"
    #[api(kind = "path", name = "comment_id")]
    pub comment_id: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, lark_bot_sdk_macros::ApiBaseResp)]
struct DeleteTaskCommentRespInner {
    #[serde(flatten)]
    data: Option<DeleteTaskCommentResp>,
    #[serde(flatten)]
    base: BaseResp,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct DeleteTaskCommentResp {
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

    use self::gen::task::TaskServiceMocker;
    use crate::core::mocker::*;

    use super::*;
    use crate::api::*;
    use crate::core::model::*;

    pub struct Mocker;

    pub trait MockFunc:
        Fn(DeleteTaskCommentReq) -> Result<(DeleteTaskCommentResp, CommonResponse), Error>
        + Send
        + Sync
        + 'static
    {
    }
    impl<
            T: Fn(DeleteTaskCommentReq) -> Result<(DeleteTaskCommentResp, CommonResponse), Error>
                + Send
                + Sync
                + 'static,
        > MockFunc for T
    {
    }

    impl<'c, IStore: Store, IClient: HttpClient> TaskServiceMocker<'c, IStore, IClient> {
        pub fn mock_delete_task_comment<F: MockFunc>(
            &self,
            f: F,
        ) -> MockerBuilder<Mocker, DeleteTaskCommentReq, DeleteTaskCommentResp, Arc<dyn MockFunc>>
        {
            MockerBuilder::new(self.cli.instance_id, Arc::new(f))
        }

        pub(super) fn get_mock_delete_task_comment(
            &self,
            req: &DeleteTaskCommentReq,
        ) -> Option<Arc<dyn MockFunc>> {
            do_mock::<Mocker, DeleteTaskCommentReq, DeleteTaskCommentResp, Arc<dyn MockFunc>>(
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
        api::gen::task::delete_task_comment::{
            DeleteTaskCommentReq, DeleteTaskCommentResp, DeleteTaskCommentRespInner,
        },
        core::{model::CommonResponse, Lark},
    };

    #[tokio::test]
    async fn test_mock() {
        let lark = Lark::new("".to_owned(), "".into());
        let mocker = lark
            .task()
            .mock()
            .mock_delete_task_comment(|_| {
                Ok((DeleteTaskCommentResp::default(), CommonResponse::default()))
            })
            .build();
        let res = lark
            .task()
            .delete_task_comment(DeleteTaskCommentReq::default())
            .await;
        assert!(res.is_ok());
        mocker.clear();
        let res = lark
            .task()
            .delete_task_comment(DeleteTaskCommentReq::default())
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
    "data": {},
    "msg": "success"
}"#;
    #[test]
    fn test_resp() {
        let res = serde_json::from_str::<DeleteTaskCommentRespInner>(RESP);
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
