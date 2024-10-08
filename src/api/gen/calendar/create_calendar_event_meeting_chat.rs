//! doc url: <https://open.larkoffice.com/document/uAjLw4CM/ukTMukTMukTM/reference/calendar-v4/calendar-event-meeting_chat/create>
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
    /// **api 版本: 2024-07-16T06:30:06+00:00**
    ///
    /// ## 创建会议群
    ///
    /// 调用该接口以当前身份（应用或用户）为指定日程创建一个会议群。
    ///
    /// - 当前身份由 Header Authorization 的 Token 类型决定。tenant_access_token 指应用身份，user_access_token 指用户身份。
    ///
    /// - 如果使用应用身份调用该接口，则需要确保应用开启了[机器人能力](https://open.feishu.cn/document/uAjLw4CM/ugTN1YjL4UTN24CO1UjN/trouble-shooting/how-to-enable-bot-ability)。
    ///
    /// - 日程所在的日历需要是当前身份的主日历，且具有日历的 writer 权限。你可以调用[查询主日历信息](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/calendar-v4/calendar/primary)接口，获取当前身份的主日历信息。
    ///
    /// - 日程需要添加了至少 2 个参与人，且不隐藏参与人列表。你可以调用[获取日程参与人列表](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/calendar-v4/calendar-event-attendee/list)接口获取日程的参与人情况；可以调用[获取日程](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/calendar-v4/calendar-event/get)接口，查看日程参与人权限信息（attendee_ability）。
    ///
    /// doc: <https://open.larkoffice.com/document/uAjLw4CM/ukTMukTMukTM/reference/calendar-v4/calendar-event-meeting_chat/create>
    ///
    /// new doc: <https://open.larkoffice.com/document/uAjLw4CM/ukTMukTMukTM/reference/calendar-v4/calendar-event-meeting_chat/create>
    ///
    /// api url: <https://open.feishu.cn/document_portal/v1/document/get_detail?fullPath=%2FuAjLw4CM%2FukTMukTMukTM%2Freference%2Fcalendar-v4%2Fcalendar-event-meeting_chat%2Fcreate>
    pub async fn create_calendar_event_meeting_chat(
        &self,
        req: CreateCalendarEventMeetingChatReq,
    ) -> Result<(CreateCalendarEventMeetingChatResp, CommonResponse), Error> {
        self.create_calendar_event_meeting_chat_with_opt(req, Default::default())
            .await
    }

    /// 参见 [create_calendar_event_meeting_chat](#method.create_calendar_event_meeting_chat) 函数
    pub async fn create_calendar_event_meeting_chat_with_opt(
        &self,
        req: CreateCalendarEventMeetingChatReq,
        method_option: MethodOption,
    ) -> Result<(CreateCalendarEventMeetingChatResp, CommonResponse), Error> {
        #[cfg(feature = "test-util")]
        {
            if let Some(f) = self
                .mock()
                .get_mock_create_calendar_event_meeting_chat(&req)
            {
                tracing::info!("[lark] Calendar#CreateCalendarEventMeetingChat **mocking** api");
                return f(req);
            }
        }

        tracing::info!("[lark] Calendar#CreateCalendarEventMeetingChat call api");

        let req = ApiRequest {
            scope: "Calendar",
            api: "CreateCalendarEventMeetingChat",
            method: http::Method::POST,
            url: String::new()
                + self.cli.open_base_url.as_ref()
                + "/open-apis/calendar/v4/calendars/:calendar_id/events/:event_id/meeting_chat",
            param_data: req.gen_param(),
            method_option,
            need_tenant_access_token: true,
            need_user_access_token: true,
            ..Default::default()
        };

        let (resp, common_resp): (CreateCalendarEventMeetingChatRespInner, _) =
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
pub struct CreateCalendarEventMeetingChatReq {
    /// 日程所在的日历 ID。了解更多，参见[日历 ID 说明](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/calendar-v4/calendar/introduction)。
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "feishu.cn_xxx@group.calendar.feishu.cn"
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
    /// **示例值**: "75d28f9b-e35c-4230-8a83-123_0"
    #[api(kind = "path", name = "event_id")]
    pub event_id: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, lark_bot_sdk_macros::ApiBaseResp)]
struct CreateCalendarEventMeetingChatRespInner {
    #[serde(flatten)]
    data: Option<CreateCalendarEventMeetingChatResp>,
    #[serde(flatten)]
    base: BaseResp,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct CreateCalendarEventMeetingChatResp {
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
    /// 会议群 ID。后续可用于解绑会议群。
    ///
    /// **示例值**: "oc_xxx"
    #[serde(
        rename = "meeting_chat_id",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub meeting_chat_id: String,
    /// 群分享链接。
    ///
    /// **示例值**: "https://example.cn?openChatId=oc_xxx"
    #[serde(
        rename = "applink",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub applink: String,
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
        Fn(
            CreateCalendarEventMeetingChatReq,
        ) -> Result<(CreateCalendarEventMeetingChatResp, CommonResponse), Error>
        + Send
        + Sync
        + 'static
    {
    }
    impl<
            T: Fn(
                    CreateCalendarEventMeetingChatReq,
                )
                    -> Result<(CreateCalendarEventMeetingChatResp, CommonResponse), Error>
                + Send
                + Sync
                + 'static,
        > MockFunc for T
    {
    }

    impl<'c, IStore: Store, IClient: HttpClient> CalendarServiceMocker<'c, IStore, IClient> {
        pub fn mock_create_calendar_event_meeting_chat<F: MockFunc>(
            &self,
            f: F,
        ) -> MockerBuilder<
            Mocker,
            CreateCalendarEventMeetingChatReq,
            CreateCalendarEventMeetingChatResp,
            Arc<dyn MockFunc>,
        > {
            MockerBuilder::new(self.cli.instance_id, Arc::new(f))
        }

        pub(super) fn get_mock_create_calendar_event_meeting_chat(
            &self,
            req: &CreateCalendarEventMeetingChatReq,
        ) -> Option<Arc<dyn MockFunc>> {
            do_mock::<
                Mocker,
                CreateCalendarEventMeetingChatReq,
                CreateCalendarEventMeetingChatResp,
                Arc<dyn MockFunc>,
            >(self.cli.instance_id, req)
        }
    }
}

#[cfg(feature = "test-util")]
#[cfg(test)]
mod test {
    use crate::{
        api::gen::calendar::create_calendar_event_meeting_chat::{
            CreateCalendarEventMeetingChatReq, CreateCalendarEventMeetingChatResp,
            CreateCalendarEventMeetingChatRespInner,
        },
        core::{model::CommonResponse, Lark},
    };

    #[tokio::test]
    async fn test_mock() {
        let lark = Lark::new("".to_owned(), "".into());
        let mocker = lark
            .calendar()
            .mock()
            .mock_create_calendar_event_meeting_chat(|_| {
                Ok((
                    CreateCalendarEventMeetingChatResp::default(),
                    CommonResponse::default(),
                ))
            })
            .build();
        let res = lark
            .calendar()
            .create_calendar_event_meeting_chat(CreateCalendarEventMeetingChatReq::default())
            .await;
        assert!(res.is_ok());
        mocker.clear();
        let res = lark
            .calendar()
            .create_calendar_event_meeting_chat(CreateCalendarEventMeetingChatReq::default())
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
        "meeting_chat_id": "oc_xxx",
        "applink": "https://example.cn?openChatId=oc_xxx"
    }
}"#;
    #[test]
    fn test_resp() {
        let res = serde_json::from_str::<CreateCalendarEventMeetingChatRespInner>(RESP);
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
