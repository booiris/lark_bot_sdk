//! doc url: <https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/calendar-v4/calendar-event/delete>
// Code generated by gen_api. DO NOT EDIT.

use serde::{Deserialize, Serialize};

use crate::api::BaseResp;
use crate::api::HasBaseResp;
use crate::core::model::{ApiRequest, CommonResponse, MethodOption, ReqParam, StreamReqParam};
use crate::{
    core::{http_client::HttpClient, store::Store},
    error::Error,
};

use crate::api::gen::calendar::CalendarService;

impl<'c, IStore: Store, IClient: HttpClient> CalendarService<'c, IStore, IClient> {
    /// **api 版本: 2024-07-16T06:26:39+00:00**
    ///
    /// ## 删除日程
    ///
    /// 调用该接口以当前身份（应用或用户）删除指定日历上的一个日程。
    ///
    /// - 当前身份由 Header Authorization 的 Token 类型决定。tenant_access_token 指应用身份，user_access_token 指用户身份。
    ///
    /// - 如果使用应用身份调用该接口，则需要确保应用开启了[机器人能力](https://open.feishu.cn/document/uAjLw4CM/ugTN1YjL4UTN24CO1UjN/trouble-shooting/how-to-enable-bot-ability)。
    ///
    /// - 当前身份必须对日历有 writer 或 owner 权限，并且日历的类型只能为 primary 或 shared。你可以调用[查询日历信息](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/calendar-v4/calendar/get)接口，获取日历类型以及当前身份对该日历的访问权限。
    ///
    /// - 当前身份必须是日程的组织者。
    ///
    /// doc: <https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/calendar-v4/calendar-event/delete>
    ///
    /// new doc: <https://open.feishu.cn/document/server-docs/calendar-v4/calendar-event/delete>
    ///
    /// api url: <https://open.feishu.cn/document_portal/v1/document/get_detail?fullPath=%2Fserver-docs%2Fcalendar-v4%2Fcalendar-event%2Fdelete>
    pub async fn delete_calendar_event(
        &self,
        req: DeleteCalendarEventReq,
    ) -> Result<(DeleteCalendarEventResp, CommonResponse), Error> {
        self.delete_calendar_event_with_opt(req, Default::default())
            .await
    }

    /// 参见 [delete_calendar_event](#method.delete_calendar_event) 函数
    pub async fn delete_calendar_event_with_opt(
        &self,
        req: DeleteCalendarEventReq,
        method_option: MethodOption,
    ) -> Result<(DeleteCalendarEventResp, CommonResponse), Error> {
        #[cfg(feature = "test-util")]
        {
            if let Some(f) = self.mock().get_mock_delete_calendar_event(&req) {
                tracing::info!("[lark] Calendar#DeleteCalendarEvent **mocking** api");
                return f(req);
            }
        }

        tracing::info!("[lark] Calendar#DeleteCalendarEvent call api");

        let req = ApiRequest {
            scope: "Calendar",
            api: "DeleteCalendarEvent",
            method: http::Method::DELETE,
            url: String::new()
                + self.cli.open_base_url.as_ref()
                + "/open-apis/calendar/v4/calendars/:calendar_id/events/:event_id",
            param_data: req.gen_param(),
            method_option,
            need_tenant_access_token: true,
            need_user_access_token: true,
            ..Default::default()
        };

        let (resp, common_resp): (DeleteCalendarEventRespInner, _) = self.cli.do_req(req).await?;
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
pub struct DeleteCalendarEventReq {
    /// 日程所在的日历 ID。了解更多，参见[日历 ID 说明](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/calendar-v4/calendar/introduction)。
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "feishu.cn_xxxxxxxxxx@group.calendar.feishu.cn"
    #[api(kind = "path", name = "calendar_id")]
    pub calendar_id: String,
    /// 日程 ID。
    ///
    /// 创建日程时会返回日程 ID。你也可以调用以下接口获取某一日历的 ID。
    ///
    /// - [获取日程列表](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/calendar-v4/calendar-event/list)
    ///
    /// - [搜索日程](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/calendar-v4/calendar-event/search)
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "xxxxxxxxx_0"
    #[api(kind = "path", name = "event_id")]
    pub event_id: String,
    /// 删除日程是否给日程参与人发送 Bot 通知。
    ///
    /// **默认值**：true
    ///
    /// **示例值**: "false"
    ///
    /// **可选值**:
    ///
    /// `true`: 发送
    ///
    /// `false`: 不发送
    #[api(
        kind = "query",
        name = "need_notification",
        v_type = "var",
        option = "false"
    )]
    pub need_notification: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, lark_bot_sdk_macros::ApiBaseResp)]
struct DeleteCalendarEventRespInner {
    #[serde(flatten)]
    data: Option<DeleteCalendarEventResp>,
    #[serde(flatten)]
    base: BaseResp,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct DeleteCalendarEventResp {
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

    use self::gen::calendar::CalendarServiceMocker;
    use crate::core::mocker::*;

    use super::*;
    use crate::api::*;
    use crate::core::model::*;

    pub struct Mocker;

    pub trait MockFunc:
        Fn(DeleteCalendarEventReq) -> Result<(DeleteCalendarEventResp, CommonResponse), Error>
        + Send
        + Sync
        + 'static
    {
    }
    impl<
            T: Fn(
                    DeleteCalendarEventReq,
                ) -> Result<(DeleteCalendarEventResp, CommonResponse), Error>
                + Send
                + Sync
                + 'static,
        > MockFunc for T
    {
    }

    impl<'c, IStore: Store, IClient: HttpClient> CalendarServiceMocker<'c, IStore, IClient> {
        pub fn mock_delete_calendar_event<F: MockFunc>(
            &self,
            f: F,
        ) -> MockerBuilder<Mocker, DeleteCalendarEventReq, DeleteCalendarEventResp, Arc<dyn MockFunc>>
        {
            MockerBuilder::new(self.cli.instance_id, Arc::new(f))
        }

        pub(super) fn get_mock_delete_calendar_event(
            &self,
            req: &DeleteCalendarEventReq,
        ) -> Option<Arc<dyn MockFunc>> {
            do_mock::<Mocker, DeleteCalendarEventReq, DeleteCalendarEventResp, Arc<dyn MockFunc>>(
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
        api::gen::calendar::delete_calendar_event::{
            DeleteCalendarEventReq, DeleteCalendarEventResp, DeleteCalendarEventRespInner,
        },
        core::{model::CommonResponse, Lark},
    };

    #[tokio::test]
    async fn test_mock() {
        let lark = Lark::new("".to_owned(), "".into());
        let mocker = lark
            .calendar()
            .mock()
            .mock_delete_calendar_event(|_| {
                Ok((
                    DeleteCalendarEventResp::default(),
                    CommonResponse::default(),
                ))
            })
            .build();
        let res = lark
            .calendar()
            .delete_calendar_event(DeleteCalendarEventReq::default())
            .await;
        assert!(res.is_ok());
        mocker.clear();
        let res = lark
            .calendar()
            .delete_calendar_event(DeleteCalendarEventReq::default())
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
    "data": {}
}"#;
    #[test]
    fn test_resp() {
        let res = serde_json::from_str::<DeleteCalendarEventRespInner>(RESP);
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
