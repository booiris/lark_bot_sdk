//! doc url: <https://open.larkoffice.com/document/uAjLw4CM/ukTMukTMukTM/reference/task-v1/task/batch_delete_follower>
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
    /// **api 版本: 2023-07-13T02:33:27+00:00**
    ///
    /// ## 批量删除关注人
    ///
    /// 该接口用于批量删除关注人。
    ///

    ///
    /// doc: <https://open.larkoffice.com/document/uAjLw4CM/ukTMukTMukTM/reference/task-v1/task/batch_delete_follower>
    ///
    /// new doc: <https://open.larkoffice.com/document/server-docs/task-v1/task-follower/batch_delete_follower>
    ///
    /// api url: <https://open.feishu.cn/document_portal/v1/document/get_detail?fullPath=%2Fserver-docs%2Ftask-v1%2Ftask-follower%2Fbatch_delete_follower>
    pub async fn batch_delete_task_follower(
        &self,
        req: BatchDeleteTaskFollowerReq,
    ) -> Result<(BatchDeleteTaskFollowerResp, CommonResponse), Error> {
        self.batch_delete_task_follower_with_opt(req, Default::default())
            .await
    }

    /// 参见 [batch_delete_task_follower](#method.batch_delete_task_follower) 函数
    pub async fn batch_delete_task_follower_with_opt(
        &self,
        req: BatchDeleteTaskFollowerReq,
        method_option: MethodOption,
    ) -> Result<(BatchDeleteTaskFollowerResp, CommonResponse), Error> {
        #[cfg(feature = "test-util")]
        {
            if let Some(f) = self.mock().get_mock_batch_delete_task_follower(&req) {
                tracing::info!("[lark] Task#BatchDeleteTaskFollower **mocking** api");
                return f(req);
            }
        }

        tracing::info!("[lark] Task#BatchDeleteTaskFollower call api");

        let req = ApiRequest {
            scope: "Task",
            api: "BatchDeleteTaskFollower",
            method: http::Method::POST,
            url: String::new()
                + self.cli.open_base_url.as_ref()
                + "/open-apis/task/v1/tasks/:task_id/batch_delete_follower",
            param_data: req.gen_param(),
            method_option,
            need_tenant_access_token: true,
            need_user_access_token: true,
            ..Default::default()
        };

        let (resp, common_resp): (BatchDeleteTaskFollowerRespInner, _) =
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
pub struct BatchDeleteTaskFollowerReq {
    /// 任务ID
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "83912691-2e43-47fc-94a4-d512e03984fa"
    #[api(kind = "path", name = "task_id")]
    pub task_id: String,
    /// 用户 ID 类型
    ///
    /// **示例值**: "open_id"
    ///
    /// **可选值**:
    ///
    /// `open_id`: 标识一个用户在某个应用中的身份。同一个用户在不同应用中的 Open ID 不同。[了解更多：如何获取 Open ID](https://open.feishu.cn/document/uAjLw4CM/ugTN1YjL4UTN24CO1UjN/trouble-shooting/how-to-obtain-openid)
    ///
    /// `union_id`: 标识一个用户在某个应用开发商下的身份。同一用户在同一开发商下的应用中的 Union ID 是相同的，在不同开发商下的应用中的 Union ID 是不同的。通过 Union ID，应用开发商可以把同个用户在多个应用中的身份关联起来。[了解更多：如何获取 Union ID？](https://open.feishu.cn/document/uAjLw4CM/ugTN1YjL4UTN24CO1UjN/trouble-shooting/how-to-obtain-union-id)
    ///
    /// `user_id`: 标识一个用户在某个租户内的身份。同一个用户在租户 A 和租户 B 内的 User ID 是不同的。在同一个租户内，一个用户的 User ID 在所有应用（包括商店应用）中都保持一致。User ID 主要用于在不同的应用间打通用户数据。[了解更多：如何获取 User ID？](https://open.feishu.cn/document/uAjLw4CM/ugTN1YjL4UTN24CO1UjN/trouble-shooting/how-to-obtain-user-id)
    #[api(
        kind = "query",
        name = "user_id_type",
        v_type = "var",
        option = "false"
    )]
    pub user_id_type: String,
    /// 要删除的关注人ID列表
    ///
    /// **示例值**: "[
    ///
    /// "ou_550cc75233d8b7b9fcbdad65f34433f4", "ou_d1e9d27cf3235b40ca9a67c67ef088b0"
    ///
    /// ]"
    #[api(kind = "body", name = "id_list")]
    pub id_list: Vec<Option<String>>,
}

#[derive(Debug, Serialize, Deserialize, Clone, lark_bot_sdk_macros::ApiBaseResp)]
struct BatchDeleteTaskFollowerRespInner {
    #[serde(flatten)]
    data: Option<BatchDeleteTaskFollowerResp>,
    #[serde(flatten)]
    base: BaseResp,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct BatchDeleteTaskFollowerResp {
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
    /// 实际删除的关注人用户ID列表
    #[serde(
        rename = "followers",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub followers: Vec<String>,
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
        Fn(
            BatchDeleteTaskFollowerReq,
        ) -> Result<(BatchDeleteTaskFollowerResp, CommonResponse), Error>
        + Send
        + Sync
        + 'static
    {
    }
    impl<
            T: Fn(
                    BatchDeleteTaskFollowerReq,
                ) -> Result<(BatchDeleteTaskFollowerResp, CommonResponse), Error>
                + Send
                + Sync
                + 'static,
        > MockFunc for T
    {
    }

    impl<'c, IStore: Store, IClient: HttpClient> TaskServiceMocker<'c, IStore, IClient> {
        pub fn mock_batch_delete_task_follower<F: MockFunc>(
            &self,
            f: F,
        ) -> MockerBuilder<
            Mocker,
            BatchDeleteTaskFollowerReq,
            BatchDeleteTaskFollowerResp,
            Arc<dyn MockFunc>,
        > {
            MockerBuilder::new(self.cli.instance_id, Arc::new(f))
        }

        pub(super) fn get_mock_batch_delete_task_follower(
            &self,
            req: &BatchDeleteTaskFollowerReq,
        ) -> Option<Arc<dyn MockFunc>> {
            do_mock::<
                Mocker,
                BatchDeleteTaskFollowerReq,
                BatchDeleteTaskFollowerResp,
                Arc<dyn MockFunc>,
            >(self.cli.instance_id, req)
        }
    }
}

#[cfg(feature = "test-util")]
#[cfg(test)]
mod test {
    use crate::{
        api::gen::task::batch_delete_task_follower::{
            BatchDeleteTaskFollowerReq, BatchDeleteTaskFollowerResp,
            BatchDeleteTaskFollowerRespInner,
        },
        core::{model::CommonResponse, Lark},
    };

    #[tokio::test]
    async fn test_mock() {
        let lark = Lark::new("".to_owned(), "".into());
        let mocker = lark
            .task()
            .mock()
            .mock_batch_delete_task_follower(|_| {
                Ok((
                    BatchDeleteTaskFollowerResp::default(),
                    CommonResponse::default(),
                ))
            })
            .build();
        let res = lark
            .task()
            .batch_delete_task_follower(BatchDeleteTaskFollowerReq::default())
            .await;
        assert!(res.is_ok());
        mocker.clear();
        let res = lark
            .task()
            .batch_delete_task_follower(BatchDeleteTaskFollowerReq::default())
            .await;
        assert!(res.is_err());
    }

    const REQ: &str = r#"{
    "id_list": ["ou_13585843f02bc94923ed17a007cbc9b1", "ou_f4506885e436763c36e03c05ef1bd6f8"]
}"#;

    #[test]
    fn test_req() {
        if REQ == "{}" {
            return;
        }
        if let Err(e) = serde_json::from_str::<super::BatchDeleteTaskFollowerReqBody>(REQ) {
            panic!("{}", e);
        }
    }

    const RESP: &str = r#"{
    "code": 0,
    "data": {
        "followers": [
            "ou_13585843f02bc94923ed17a007cbc9b1",
            "ou_f4506885e436763c36e03c05ef1bd6f8"
        ]
    },
    "msg": ""
}"#;
    #[test]
    fn test_resp() {
        let res = serde_json::from_str::<BatchDeleteTaskFollowerRespInner>(RESP);
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
