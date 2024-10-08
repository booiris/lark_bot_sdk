//! doc url: <https://open.larkoffice.com/document/uAjLw4CM/ukTMukTMukTM/reference/task-v1/task-reminder/list>
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
    /// **api 版本: 2023-07-13T02:33:25+00:00**
    ///
    /// ## 查询提醒时间列表
    ///
    /// 返回提醒时间列表，支持分页，最大值为50。
    ///

    ///
    /// doc: <https://open.larkoffice.com/document/uAjLw4CM/ukTMukTMukTM/reference/task-v1/task-reminder/list>
    ///
    /// new doc: <https://open.larkoffice.com/document/server-docs/task-v1/task-reminder/list>
    ///
    /// api url: <https://open.feishu.cn/document_portal/v1/document/get_detail?fullPath=%2Fserver-docs%2Ftask-v1%2Ftask-reminder%2Flist>
    pub async fn get_task_reminder_list(
        &self,
        req: GetTaskReminderListReq,
    ) -> Result<(GetTaskReminderListResp, CommonResponse), Error> {
        self.get_task_reminder_list_with_opt(req, Default::default())
            .await
    }

    /// 参见 [get_task_reminder_list](#method.get_task_reminder_list) 函数
    pub async fn get_task_reminder_list_with_opt(
        &self,
        req: GetTaskReminderListReq,
        method_option: MethodOption,
    ) -> Result<(GetTaskReminderListResp, CommonResponse), Error> {
        #[cfg(feature = "test-util")]
        {
            if let Some(f) = self.mock().get_mock_get_task_reminder_list(&req) {
                tracing::info!("[lark] Task#GetTaskReminderList **mocking** api");
                return f(req);
            }
        }

        tracing::info!("[lark] Task#GetTaskReminderList call api");

        let req = ApiRequest {
            scope: "Task",
            api: "GetTaskReminderList",
            method: http::Method::GET,
            url: String::new()
                + self.cli.open_base_url.as_ref()
                + "/open-apis/task/v1/tasks/:task_id/reminders",
            param_data: req.gen_param(),
            method_option,
            need_tenant_access_token: true,
            need_user_access_token: true,
            ..Default::default()
        };

        let (resp, common_resp): (GetTaskReminderListRespInner, _) = self.cli.do_req(req).await?;
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
pub struct GetTaskReminderListReq {
    /// 任务 ID
    ///
    /// **示例值**: "0d38e26e-190a-49e9-93a2-35067763ed1f"
    #[api(kind = "path", name = "task_id")]
    pub task_id: String,
    /// 分页大小
    ///
    /// **示例值**: "50"
    #[api(kind = "query", name = "page_size", v_type = "var", option = "false")]
    pub page_size: i64,
    /// 分页标记，第一次请求不填，表示从头开始遍历；分页查询结果还有更多项时会同时返回新的 page_token，下次遍历可采用该 page_token 获取查询结果
    ///
    /// **示例值**: "「填写上次返回的page_token」"
    #[api(kind = "query", name = "page_token", v_type = "var", option = "false")]
    pub page_token: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, lark_bot_sdk_macros::ApiBaseResp)]
struct GetTaskReminderListRespInner {
    #[serde(flatten)]
    data: Option<GetTaskReminderListResp>,
    #[serde(flatten)]
    base: BaseResp,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct GetTaskReminderListResp {
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
    /// 返回提醒时间设置列表
    #[serde(
        rename = "items",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub items: Vec<ReminderSubResp>,
    /// 分页标记，当 has_more 为 true 时，会同时返回新的 page_token，否则不返回 page_token
    ///
    /// **示例值**: "「填写上次返回的page_token」"
    #[serde(
        rename = "page_token",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub page_token: String,
    /// 是否还有更多项
    ///
    /// **示例值**: "false"
    #[serde(
        rename = "has_more",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub has_more: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(default)]
pub struct ReminderSubResp {
    /// 提醒时间设置的 ID（在删除时候需要使用这个）
    ///
    /// **示例值**: "1"
    #[serde(
        rename = "id",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub id: String,
    /// 相对于截止时间的提醒时间（如提前 30 分钟，截止时间后 30 分钟，则为 -30） 任务没有截止时间则为全天任务(截止时间为0)
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "30"
    #[serde(
        rename = "relative_fire_minute",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub relative_fire_minute: i64,
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
        Fn(GetTaskReminderListReq) -> Result<(GetTaskReminderListResp, CommonResponse), Error>
        + Send
        + Sync
        + 'static
    {
    }
    impl<
            T: Fn(
                    GetTaskReminderListReq,
                ) -> Result<(GetTaskReminderListResp, CommonResponse), Error>
                + Send
                + Sync
                + 'static,
        > MockFunc for T
    {
    }

    impl<'c, IStore: Store, IClient: HttpClient> TaskServiceMocker<'c, IStore, IClient> {
        pub fn mock_get_task_reminder_list<F: MockFunc>(
            &self,
            f: F,
        ) -> MockerBuilder<Mocker, GetTaskReminderListReq, GetTaskReminderListResp, Arc<dyn MockFunc>>
        {
            MockerBuilder::new(self.cli.instance_id, Arc::new(f))
        }

        pub(super) fn get_mock_get_task_reminder_list(
            &self,
            req: &GetTaskReminderListReq,
        ) -> Option<Arc<dyn MockFunc>> {
            do_mock::<Mocker, GetTaskReminderListReq, GetTaskReminderListResp, Arc<dyn MockFunc>>(
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
        api::gen::task::get_task_reminder_list::{
            GetTaskReminderListReq, GetTaskReminderListResp, GetTaskReminderListRespInner,
        },
        core::{model::CommonResponse, Lark},
    };

    #[tokio::test]
    async fn test_mock() {
        let lark = Lark::new("".to_owned(), "".into());
        let mocker = lark
            .task()
            .mock()
            .mock_get_task_reminder_list(|_| {
                Ok((
                    GetTaskReminderListResp::default(),
                    CommonResponse::default(),
                ))
            })
            .build();
        let res = lark
            .task()
            .get_task_reminder_list(GetTaskReminderListReq::default())
            .await;
        assert!(res.is_ok());
        mocker.clear();
        let res = lark
            .task()
            .get_task_reminder_list(GetTaskReminderListReq::default())
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
        "items": [
            {
                "id": "1",
                "relative_fire_minute": 30
            }
        ],
        "page_token": "「填写上次返回的page_token」",
        "has_more": false
    }
}"#;
    #[test]
    fn test_resp() {
        let res = serde_json::from_str::<GetTaskReminderListRespInner>(RESP);
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
