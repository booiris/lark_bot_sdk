//! doc url: <https://open.larkoffice.com/document/uAjLw4CM/ukTMukTMukTM/reference/calendar-v4/calendar-event-attendee/list>
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
    /// **api 版本: 2024-07-16T06:30:29+00:00**
    ///
    /// ## 获取日程参与人列表
    ///
    /// 调用该接口以当前身份（应用或用户）获取日程的参与人列表。
    ///
    /// - 当前身份由 Header Authorization 的 Token 类型决定。tenant_access_token 指应用身份，user_access_token 指用户身份。
    ///
    /// - 如果使用应用身份调用该接口，则需要确保应用开启了[机器人能力](https://open.feishu.cn/document/uAjLw4CM/ugTN1YjL4UTN24CO1UjN/trouble-shooting/how-to-enable-bot-ability)。
    ///
    /// - 当前身份必须对日历有 reader、writer 或 owner 权限。你可以调用[查询日历信息](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/calendar-v4/calendar/get)接口，获取当前身份对该日历的访问权限（role）。
    ///
    /// - 当前身份必须有权限查看日程的参与人列表，即当前身份需要是日程的组织者，或者是日程参与人且日程设置了**参与人可查看参与人列表**权限。你可以调用[获取日程](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/calendar-v4/calendar-event/get)接口，获取日程的参与人权限（attendee_ability）。
    ///
    /// - 如果你需要获取群组类型参与人的群成员，需要调用[获取参与人群成员列表](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/calendar-v4/calendar-event-attendee-chat_member/list)接口。
    ///
    /// doc: <https://open.larkoffice.com/document/uAjLw4CM/ukTMukTMukTM/reference/calendar-v4/calendar-event-attendee/list>
    ///
    /// new doc: <https://open.larkoffice.com/document/server-docs/calendar-v4/calendar-event-attendee/list-2>
    ///
    /// api url: <https://open.feishu.cn/document_portal/v1/document/get_detail?fullPath=%2Fserver-docs%2Fcalendar-v4%2Fcalendar-event-attendee%2Flist-2>
    pub async fn get_calendar_event_attendee_list(
        &self,
        req: GetCalendarEventAttendeeListReq,
    ) -> Result<(GetCalendarEventAttendeeListResp, CommonResponse), Error> {
        self.get_calendar_event_attendee_list_with_opt(req, Default::default())
            .await
    }

    /// 参见 [get_calendar_event_attendee_list](#method.get_calendar_event_attendee_list) 函数
    pub async fn get_calendar_event_attendee_list_with_opt(
        &self,
        req: GetCalendarEventAttendeeListReq,
        method_option: MethodOption,
    ) -> Result<(GetCalendarEventAttendeeListResp, CommonResponse), Error> {
        #[cfg(feature = "test-util")]
        {
            if let Some(f) = self.mock().get_mock_get_calendar_event_attendee_list(&req) {
                tracing::info!("[lark] Calendar#GetCalendarEventAttendeeList **mocking** api");
                return f(req);
            }
        }

        tracing::info!("[lark] Calendar#GetCalendarEventAttendeeList call api");

        let req = ApiRequest {
            scope: "Calendar",
            api: "GetCalendarEventAttendeeList",
            method: http::Method::GET,
            url: String::new()
                + self.cli.open_base_url.as_ref()
                + "/open-apis/calendar/v4/calendars/:calendar_id/events/:event_id/attendees",
            param_data: req.gen_param(),
            method_option,
            need_tenant_access_token: true,
            need_user_access_token: true,
            ..Default::default()
        };

        let (resp, common_resp): (GetCalendarEventAttendeeListRespInner, _) =
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
pub struct GetCalendarEventAttendeeListReq {
    /// 日程所在的日历 ID。关于日历 ID 可参见[日历 ID 说明](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/calendar-v4/calendar/introduction)。
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
    /// 是否需要会议室表单信息。
    ///
    /// **可选值有**：
    ///
    /// - true：需要
    ///
    /// - false（默认值）：不需要
    ///
    /// **注意**：当前身份需要有日程的编辑权限才会返回会议室表单信息，即当前身份需要是日程的组织者，或者是日程参与人且日程设置了**参与人可编辑日程**权限。你可以调用[获取日程](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/calendar-v4/calendar-event/get)接口，获取日程的参与人权限（attendee_ability）。
    ///
    /// **示例值**: "true"
    #[api(
        kind = "query",
        name = "need_resource_customization",
        v_type = "var",
        option = "false"
    )]
    pub need_resource_customization: bool,
    /// 分页标记，第一次请求不填，表示从头开始遍历；分页查询结果还有更多项时会同时返回新的 page_token，下次遍历可采用该 page_token 获取查询结果
    ///
    /// **示例值**: "780TRhwXXXXX"
    #[api(kind = "query", name = "page_token", v_type = "var", option = "false")]
    pub page_token: String,
    /// 一次请求返回的最大日程参与人数量。
    ///
    /// **示例值**: "10"
    #[api(kind = "query", name = "page_size", v_type = "var", option = "false")]
    pub page_size: i64,
}

#[derive(Debug, Serialize, Deserialize, Clone, lark_bot_sdk_macros::ApiBaseResp)]
struct GetCalendarEventAttendeeListRespInner {
    #[serde(flatten)]
    data: Option<GetCalendarEventAttendeeListResp>,
    #[serde(flatten)]
    base: BaseResp,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct GetCalendarEventAttendeeListResp {
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
    /// 日程参与人列表。
    #[serde(
        rename = "items",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub items: Vec<CalendarEventAttendeeSubResp>,
    /// 是否还有更多项
    ///
    /// **示例值**: "true"
    #[serde(
        rename = "has_more",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub has_more: bool,
    /// 分页标记，当 has_more 为 true 时，会同时返回新的 page_token，否则不返回 page_token
    ///
    /// **示例值**: "38RTjheyXXXX"
    #[serde(
        rename = "page_token",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub page_token: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(default)]
pub struct CalendarEventAttendeeSubResp {
    /// 参与人类型。
    ///
    /// **示例值**: "user"
    ///
    /// **可选值**:
    ///
    /// `User`: 用户
    ///
    /// `Chat`: 群组
    ///
    /// `Resource`: 会议室
    ///
    /// `ThirdParty`: 外部邮箱
    #[serde(
        rename = "type",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub body_type: String,
    /// 参与人 ID。日程参与人在当前日程内的唯一标识，后续可通过该 ID 删除日程参与人，或用于查询群组类型参与人的群成员信息。
    ///
    /// **示例值**: "user_xxxxxx"
    #[serde(
        rename = "attendee_id",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub attendee_id: String,
    /// 参与人 RSVP 状态，即日程回复状态。
    ///
    /// **示例值**: "needs_action"
    ///
    /// **可选值**:
    ///
    /// `NeedsAction`: 参与人尚未回复状态，或表示会议室预约中
    ///
    /// `Accept`: 参与人回复接受，或表示会议室预约成功
    ///
    /// `Tentative`: 参与人回复待定
    ///
    /// `Decline`: 参与人回复拒绝，或表示会议室预约失败
    ///
    /// `Removed`: 参与人或会议室已经从日程中被移除
    #[serde(
        rename = "rsvp_status",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub rsvp_status: String,
    /// 参与人是否为可选参加，该参数值对群组的群成员不生效。
    ///
    /// **示例值**: "true"
    #[serde(
        rename = "is_optional",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub is_optional: bool,
    /// 参与人是否为日程组织者。
    ///
    /// **示例值**: "true"
    #[serde(
        rename = "is_organizer",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub is_organizer: bool,
    /// 参与人是否为外部参与人。外部参与人不支持编辑。
    ///
    /// **示例值**: "false"
    #[serde(
        rename = "is_external",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub is_external: bool,
    /// 参与人名称。
    ///
    /// **示例值**: "Zhang San"
    #[serde(
        rename = "display_name",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub display_name: String,
    /// 群成员信息。
    ///
    /// **注意**：该字段已废弃，如需获取群中的群成员，请使用 [获取参与人群成员列表](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/calendar-v4/calendar-event-attendee-chat_member/list)接口。
    #[serde(
        rename = "chat_members",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub chat_members: Vec<AttendeeChatMemberSubResp>,
    /// 用户类型参与人的用户 ID，ID 类型与 user_id_type 的值保持一致。关于用户 ID 可参见[用户相关的 ID 概念](https://open.feishu.cn/document/home/user-identity-introduction/introduction)。
    ///
    /// **注意**：当 is_external 返回为 true 时，此字段只会返回 open_id 或者 union_id。
    ///
    /// **示例值**: "ou_xxxxxxxx"
    #[serde(
        rename = "user_id",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub user_id: String,
    /// 群组类型参与人的群组 ID。关于群组 ID 可参见[群 ID 说明](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/im-v1/chat-id-description)。
    ///
    /// **示例值**: "oc_xxxxxxxxx"
    #[serde(
        rename = "chat_id",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub chat_id: String,
    /// 会议室类型参与人的会议室 ID。
    ///
    /// **示例值**: "omm_xxxxxxxx"
    #[serde(
        rename = "room_id",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub room_id: String,
    /// 外部邮箱类型参与人的邮箱地址。
    ///
    /// **示例值**: "wangwu@email.com"
    #[serde(
        rename = "third_party_email",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub third_party_email: String,
    /// 如果日程是使用应用身份创建的，在添加会议室时，指定的会议室联系人 ID。ID 类型与 user_id_type 的值保持一致。
    ///
    /// **示例值**: "ou_xxxxxxxx"
    #[serde(
        rename = "operate_id",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub operate_id: String,
    /// 会议室的个性化配置。
    #[serde(
        rename = "resource_customization",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub resource_customization: Vec<CalendarAttendeeResourceCustomizationSubResp>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(default)]
pub struct AttendeeChatMemberSubResp {
    /// 参与人 RSVP 状态。
    ///
    /// **示例值**: "needs_action"
    ///
    /// **可选值**:
    ///
    /// `NeedsAction`: 参与人尚未回复状态，或表示会议室预约中
    ///
    /// `Accept`: 参与人回复接受，或表示会议室预约成功
    ///
    /// `Tentative`: 参与人回复待定
    ///
    /// `Decline`: 参与人回复拒绝，或表示会议室预约失败
    ///
    /// `Removed`: 参与人或会议室已经从日程中被移除
    #[serde(
        rename = "rsvp_status",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub rsvp_status: String,
    /// 参与人是否为可选参加。
    ///
    /// **示例值**: "true"
    #[serde(
        rename = "is_optional",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub is_optional: bool,
    /// 参与人名称。
    ///
    /// **示例值**: "Group"
    #[serde(
        rename = "display_name",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub display_name: String,
    /// 参与人是否为日程组织者。
    ///
    /// **示例值**: "false"
    #[serde(
        rename = "is_organizer",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub is_organizer: bool,
    /// 参与人是否为外部参与人。
    ///
    /// **示例值**: "false"
    #[serde(
        rename = "is_external",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub is_external: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(default)]
pub struct CalendarAttendeeResourceCustomizationSubResp {
    /// 每个配置的唯一 ID。
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "16281481596100"
    #[serde(
        rename = "index_key",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub index_key: String,
    /// 填空类型的取值。
    ///
    /// **示例值**: "xxx"
    #[serde(
        rename = "input_content",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub input_content: String,
    /// 每个配置的选项。
    ///
    /// **示例值**: "无"
    #[serde(
        rename = "options",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub options: Vec<CustomizationOptionSubResp>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(default)]
pub struct CustomizationOptionSubResp {
    /// 每个选项的唯一 ID。
    ///
    /// **示例值**: "16281481596185"
    #[serde(
        rename = "option_key",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub option_key: String,
    /// 其他选项类型的取值。
    ///
    /// **示例值**: "xxx"
    #[serde(
        rename = "others_content",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub others_content: String,
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
            GetCalendarEventAttendeeListReq,
        ) -> Result<(GetCalendarEventAttendeeListResp, CommonResponse), Error>
        + Send
        + Sync
        + 'static
    {
    }
    impl<
            T: Fn(
                    GetCalendarEventAttendeeListReq,
                )
                    -> Result<(GetCalendarEventAttendeeListResp, CommonResponse), Error>
                + Send
                + Sync
                + 'static,
        > MockFunc for T
    {
    }

    impl<'c, IStore: Store, IClient: HttpClient> CalendarServiceMocker<'c, IStore, IClient> {
        pub fn mock_get_calendar_event_attendee_list<F: MockFunc>(
            &self,
            f: F,
        ) -> MockerBuilder<
            Mocker,
            GetCalendarEventAttendeeListReq,
            GetCalendarEventAttendeeListResp,
            Arc<dyn MockFunc>,
        > {
            MockerBuilder::new(self.cli.instance_id, Arc::new(f))
        }

        pub(super) fn get_mock_get_calendar_event_attendee_list(
            &self,
            req: &GetCalendarEventAttendeeListReq,
        ) -> Option<Arc<dyn MockFunc>> {
            do_mock::<
                Mocker,
                GetCalendarEventAttendeeListReq,
                GetCalendarEventAttendeeListResp,
                Arc<dyn MockFunc>,
            >(self.cli.instance_id, req)
        }
    }
}

#[cfg(feature = "test-util")]
#[cfg(test)]
mod test {
    use crate::{
        api::gen::calendar::get_calendar_event_attendee_list::{
            GetCalendarEventAttendeeListReq, GetCalendarEventAttendeeListResp,
            GetCalendarEventAttendeeListRespInner,
        },
        core::{model::CommonResponse, Lark},
    };

    #[tokio::test]
    async fn test_mock() {
        let lark = Lark::new("".to_owned(), "".into());
        let mocker = lark
            .calendar()
            .mock()
            .mock_get_calendar_event_attendee_list(|_| {
                Ok((
                    GetCalendarEventAttendeeListResp::default(),
                    CommonResponse::default(),
                ))
            })
            .build();
        let res = lark
            .calendar()
            .get_calendar_event_attendee_list(GetCalendarEventAttendeeListReq::default())
            .await;
        assert!(res.is_ok());
        mocker.clear();
        let res = lark
            .calendar()
            .get_calendar_event_attendee_list(GetCalendarEventAttendeeListReq::default())
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
                "type": "user",
                "attendee_id": "user_xxxxxx",
                "rsvp_status": "needs_action",
                "is_optional": true,
                "is_organizer": true,
                "is_external": false,
                "display_name": "Zhang San",
                "chat_members": [
                    {
                        "rsvp_status": "needs_action",
                        "is_optional": true,
                        "display_name": "Group",
                        "is_organizer": false,
                        "is_external": false
                    }
                ],
                "user_id": "ou_xxxxxxxx",
                "chat_id": "oc_xxxxxxxxx",
                "room_id": "omm_xxxxxxxx",
                "third_party_email": "wangwu@email.com",
                "operate_id": "ou_xxxxxxxx",
                "resource_customization": [
                    {
                        "index_key": "16281481596100",
                        "input_content": "xxx",
                        "options": [
                            {
                                "option_key": "16281481596185",
                                "others_content": "xxx"
                            }
                        ]
                    }
                ]
            }
        ],
        "has_more": true,
        "page_token": "38RTjheyXXXX"
    }
}"#;
    #[test]
    fn test_resp() {
        let res = serde_json::from_str::<GetCalendarEventAttendeeListRespInner>(RESP);
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
