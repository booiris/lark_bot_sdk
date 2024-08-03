//! doc url: <https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/calendar-v4/calendar-event/search>
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
    /// **api 版本: 2024-07-16T06:26:40+00:00**
    ///
    /// ## 搜索日程
    ///
    /// 调用该接口以用户身份搜索指定日历下的相关日程。
    ///
    /// 当前身份必须对日历有 reader、writer 或 owner 权限。你可以调用[查询日历信息](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/calendar-v4/calendar/get)接口，获取当前身份对该日历的访问权限。
    ///
    /// doc: <https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/calendar-v4/calendar-event/search>
    ///
    /// new doc: <https://open.feishu.cn/document/server-docs/calendar-v4/calendar-event/search>
    ///
    /// api url: <https://open.feishu.cn/document_portal/v1/document/get_detail?fullPath=%2Fserver-docs%2Fcalendar-v4%2Fcalendar-event%2Fsearch>
    pub async fn search_calendar_event(
        &self,
        req: SearchCalendarEventReq,
    ) -> Result<(SearchCalendarEventResp, CommonResponse), Error> {
        self.search_calendar_event_with_opt(req, Default::default())
            .await
    }

    /// 参见 [search_calendar_event](#method.search_calendar_event) 函数
    pub async fn search_calendar_event_with_opt(
        &self,
        req: SearchCalendarEventReq,
        method_option: MethodOption,
    ) -> Result<(SearchCalendarEventResp, CommonResponse), Error> {
        #[cfg(feature = "test-util")]
        {
            if let Some(f) = self.mock().get_mock_search_calendar_event(&req) {
                tracing::info!("[lark] Calendar#SearchCalendarEvent **mocking** api");
                return f(req);
            }
        }

        tracing::info!("[lark] Calendar#SearchCalendarEvent call api");

        let req = ApiRequest {
            scope: "Calendar",
            api: "SearchCalendarEvent",
            method: http::Method::POST,
            url: String::new()
                + self.cli.open_base_url.as_ref()
                + "/open-apis/calendar/v4/calendars/:calendar_id/events/search",
            param_data: req.gen_param(),
            method_option,
            need_tenant_access_token: true,
            need_user_access_token: true,
            ..Default::default()
        };

        let (resp, common_resp): (SearchCalendarEventRespInner, _) = self.cli.do_req(req).await?;
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
pub struct SearchCalendarEventReq {
    /// 日历 ID。关于日历 ID 可参见[日历 ID 说明](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/calendar-v4/calendar/introduction)。
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "feishu.cn_xxxxxxxxxx@group.calendar.feishu.cn"
    #[api(kind = "path", name = "calendar_id")]
    pub calendar_id: String,
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
    /// 分页标记，第一次请求不填，表示从头开始遍历；分页查询结果还有更多项时会同时返回新的 page_token，下次遍历可采用该 page_token 获取查询结果
    ///
    /// **示例值**: "xxxxx"
    #[api(kind = "query", name = "page_token", v_type = "var", option = "false")]
    pub page_token: String,
    /// 一次调用所返回的最大日程数量。
    ///
    /// **示例值**: "10"
    #[api(kind = "query", name = "page_size", v_type = "var", option = "false")]
    pub page_size: i64,
    /// 搜索关键字。用于模糊查询日程名称。
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "query words"
    ///
    /// **数据校验规则**：
    ///
    /// - **长度范围**: `0` 字符- `200` 字符
    #[api(kind = "body", name = "query")]
    pub query: String,
    /// 搜索过滤器。
    #[api(kind = "body", name = "filter")]
    pub filter: Option<EventSearchFilterSubReq>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(default)]
pub struct EventSearchFilterSubReq {
    /// 搜索过滤项，日程搜索区间的开始时间。不传值则表示不设置该过滤项。
    #[serde(
        rename = "start_time",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub start_time: Option<TimeInfoSubReq>,
    /// 搜索过滤项，日程搜索区间的结束时间。不传值则表示不设置该过滤项。
    #[serde(
        rename = "end_time",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub end_time: Option<TimeInfoSubReq>,
    /// 搜索过滤项，日程参与人的用户 ID 列表。设置该字段后，被搜索到的日程中至少包含其中一个参与人。
    ///
    /// **注意**：用户 ID 类型和 user_id_type 的值保持一致，关于用户 ID 可参见[用户相关的 ID 概念](https://open.feishu.cn/document/home/user-identity-introduction/introduction)。
    ///
    /// **默认值**：空，表示不设置该过滤项
    ///
    /// **示例值**: "xxxxx"
    #[serde(
        rename = "user_ids",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub user_ids: Vec<Option<String>>,
    /// 搜索过滤项，会议室 ID 列表。设置该字段后，被搜索到的日程中至少包含其中一个会议室。
    ///
    /// **默认值**：空，表示不设置该过滤项
    ///
    /// **示例值**: "xxxxx"
    #[serde(
        rename = "room_ids",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub room_ids: Vec<Option<String>>,
    /// 搜索过滤项，群 ID 列表。设置该字段后，被搜索到的日程中至少包含其中一个群。关于群 ID 可参见[群 ID 说明](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/im-v1/chat-id-description)。
    ///
    /// **默认值**：空，表示不设置该过滤项
    ///
    /// **示例值**: "xxxxx"
    #[serde(
        rename = "chat_ids",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub chat_ids: Vec<Option<String>>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(default)]
pub struct TimeInfoSubReq {
    /// 以天为最小单位指定开始时间，[RFC 3339](https://datatracker.ietf.org/doc/html/rfc3339) 格式，例如，2018-09-01。
    ///
    /// **注意**：该参数不能与 `timestamp` 同时指定。
    ///
    /// **示例值**: "2018-09-01"
    #[serde(
        rename = "date",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub date: Option<String>,
    /// 秒级时间戳，指具体的开始时间。例如，1602504000 表示 2020/10/12 20:00:00（UTC +8 时区）。
    ///
    /// **注意**：该参数不能与 `date` 同时指定。
    ///
    /// **示例值**: "1602504000"
    #[serde(
        rename = "timestamp",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub timestamp: Option<String>,
    /// 时区。使用 IANA Time Zone Database 标准，例如 Asia/Shanghai。
    ///
    /// - 全天时区固定为UTC +0
    ///
    /// - 非全天时区默认为 Asia/Shanghai
    ///
    /// **示例值**: "Asia/Shanghai"
    #[serde(
        rename = "timezone",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub timezone: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, lark_bot_sdk_macros::ApiBaseResp)]
struct SearchCalendarEventRespInner {
    #[serde(flatten)]
    data: Option<SearchCalendarEventResp>,
    #[serde(flatten)]
    base: BaseResp,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct SearchCalendarEventResp {
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
    /// 搜索命中的日程列表。
    #[serde(
        rename = "items",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub items: Vec<CalendarEventSubResp>,
    /// 分页标记，当 has_more 为 true 时，会同时返回新的 page_token，否则不返回 page_token
    ///
    /// **示例值**: "xxxxx"
    #[serde(
        rename = "page_token",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub page_token: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(default)]
pub struct CalendarEventSubResp {
    /// 日程 ID。后续可通过该 ID 查询、更新或删除日程信息。更多信息可参见[日程 ID 说明](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/calendar-v4/calendar-event/introduction)。
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "00592a0e-7edf-4678-bc9d-1b77383ef08e_0"
    #[serde(
        rename = "event_id",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub event_id: String,
    /// 日程组织者的日历 ID。关于日历 ID 可参见[日历 ID 说明](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/calendar-v4/calendar/introduction)。
    ///
    /// **示例值**: "feishu.cn_xxxxxxxxxx@group.calendar.feishu.cn"
    #[serde(
        rename = "organizer_calendar_id",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub organizer_calendar_id: String,
    /// 日程标题。
    ///
    /// **示例值**: "日程标题"
    ///
    /// **数据校验规则**：
    ///
    /// - **最大长度**: `1000` 字符
    #[serde(
        rename = "summary",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub summary: String,
    /// 日程描述。
    ///
    /// **示例值**: "日程描述"
    ///
    /// **数据校验规则**：
    ///
    /// - **最大长度**: `40960` 字符
    #[serde(
        rename = "description",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub description: String,
    /// 日程开始时间。
    ///
    /// **是否必填**: 是
    #[serde(
        rename = "start_time",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub start_time: TimeInfoSubResp,
    /// 日程结束时间。
    ///
    /// **是否必填**: 是
    #[serde(
        rename = "end_time",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub end_time: TimeInfoSubResp,
    /// 日程公开范围。仅新建日程时对所有参与人生效，之后修改该属性仅对当前身份生效。
    ///
    /// **示例值**: "default"
    ///
    /// **可选值**:
    ///
    /// `Default`: 默认权限，跟随日历权限，默认仅向他人显示是否“忙碌”
    ///
    /// `Public`: 公开，显示日程详情
    ///
    /// `Private`: 私密，仅自己可见详情
    #[serde(
        rename = "visibility",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub visibility: String,
    /// 参与人权限。
    ///
    /// **示例值**: "can_see_others"
    ///
    /// **可选值**:
    ///
    /// `None`: 无法编辑日程、无法邀请其它参与人、无法查看参与人列表
    ///
    /// `CanSeeOthers`: 无法编辑日程、无法邀请其它参与人、可以查看参与人列表
    ///
    /// `CanInviteOthers`: 无法编辑日程、可以邀请其它参与人、可以查看参与人列表
    ///
    /// `CanModifyEvent`: 可以编辑日程、可以邀请其它参与人、可以查看参与人列表
    #[serde(
        rename = "attendee_ability",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub attendee_ability: String,
    /// 日程占用的忙闲状态。仅新建日程时对所有参与人生效，之后修改该属性仅对当前身份生效。
    ///
    /// **示例值**: "busy"
    ///
    /// **可选值**:
    ///
    /// `Busy`: 忙碌
    ///
    /// `Free`: 空闲
    #[serde(
        rename = "free_busy_status",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub free_busy_status: String,
    /// 日程地点。
    #[serde(
        rename = "location",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub location: EventLocationSubResp,
    /// 日程颜色，由颜色 RGB 值的 int32 表示。
    ///
    /// **说明**：
    ///
    /// - 仅对当前身份生效。
    ///
    /// - 取值为 0 或 -1 时，表示默认跟随日历颜色。
    ///
    /// - 客户端展示时会映射到色板上最接近的一种颜色。
    ///
    /// **示例值**: "-1"
    #[serde(
        rename = "color",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub color: i64,
    /// 日程提醒列表。
    #[serde(
        rename = "reminders",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub reminders: Vec<ReminderSubResp>,
    /// 重复日程的重复性规则，规则格式可参见 [rfc5545](https://datatracker.ietf.org/doc/html/rfc5545#section-3.3.10)。
    ///
    /// **示例值**: "FREQ=DAILY;INTERVAL=1"
    ///
    /// **数据校验规则**：
    ///
    /// - **最大长度**: `2000` 字符
    #[serde(
        rename = "recurrence",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub recurrence: String,
    /// 日程状态。
    ///
    /// **示例值**: "confirmed"
    ///
    /// **可选值**:
    ///
    /// `Tentative`: 未回应
    ///
    /// `Confirmed`: 已确认
    ///
    /// `Cancelled`: 日程已取消
    #[serde(
        rename = "status",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub status: String,
    /// 日程是否是一个重复日程的例外日程。了解例外日程，可参见[例外日程](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/calendar-v4/calendar-event/introduction#71c5ec78)。
    ///
    /// **示例值**: "false"
    #[serde(
        rename = "is_exception",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub is_exception: bool,
    /// 例外日程对应的原重复日程的 event_id。
    ///
    /// **示例值**: "1cd45aaa-fa70-4195-80b7-c93b2e208f45"
    #[serde(
        rename = "recurring_event_id",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub recurring_event_id: String,
    /// 日程组织者信息。
    #[serde(
        rename = "event_organizer",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub event_organizer: EventOrganizerSubResp,
    /// 日程的 app_link，跳转到具体的某个日程。
    ///
    /// **示例值**: "https://applink.larkoffice.com/client/calendar/event/detail?calendarId=7039673579105026066&key=aeac9c56-aeb1-4179-a21b-02f278f59048&originalTime=0&startTime=1700496000"
    #[serde(
        rename = "app_link",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub app_link: String,
    /// 日程附件
    #[serde(
        rename = "attachments",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub attachments: Vec<AttachmentSubResp>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(default)]
pub struct TimeInfoSubResp {
    /// 开始时间，仅全天日程使用该字段，[RFC 3339](https://datatracker.ietf.org/doc/html/rfc3339) 格式，例如，2018-09-01。
    ///
    /// **示例值**: "2018-09-01"
    #[serde(
        rename = "date",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub date: String,
    /// 秒级时间戳，指日程具体的开始时间。例如，1602504000 表示 2020/10/12 20:00:00（UTC +8 时区）。
    ///
    /// **示例值**: "1602504000"
    #[serde(
        rename = "timestamp",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub timestamp: String,
    /// 时区。使用 IANA Time Zone Database 标准。
    ///
    /// **示例值**: "Asia/Shanghai"
    #[serde(
        rename = "timezone",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub timezone: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(default)]
pub struct EventLocationSubResp {
    /// 地点名称。
    ///
    /// **示例值**: "地点名称"
    ///
    /// **数据校验规则**：
    ///
    /// - **长度范围**: `1` 字符- `512` 字符
    #[serde(
        rename = "name",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub name: String,
    /// 地点地址。
    ///
    /// **示例值**: "地点地址"
    ///
    /// **数据校验规则**：
    ///
    /// - **长度范围**: `1` 字符- `255` 字符
    #[serde(
        rename = "address",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub address: String,
    /// 地点坐标纬度信息。
    ///
    /// - 对于国内的地点，采用 GCJ-02 标准
    ///
    /// - 对于海外的地点，采用 WGS84 标准
    ///
    /// **示例值**: "1.100000023841858"
    #[serde(
        rename = "latitude",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub latitude: f64,
    /// 地点坐标经度信息。
    ///
    /// - 对于国内的地点，采用 GCJ-02 标准
    ///
    /// - 对于海外的地点，采用 WGS84 标准
    ///
    /// **示例值**: "2.200000047683716"
    #[serde(
        rename = "longitude",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub longitude: f64,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(default)]
pub struct EventOrganizerSubResp {
    /// 日程组织者 user ID。
    ///
    /// **示例值**: "ou_xxxxxx"
    #[serde(
        rename = "user_id",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub user_id: String,
    /// 日程组织者姓名。
    ///
    /// **示例值**: "孙二二"
    #[serde(
        rename = "display_name",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub display_name: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(default)]
pub struct ReminderSubResp {
    /// 日程提醒时间的偏移量。该参数仅对当前身份生效。
    ///
    /// - 正数时表示在日程开始前 X 分钟提醒。
    ///
    /// - 负数时表示在日程开始后 X 分钟提醒。
    ///
    /// **示例值**: "5"
    #[serde(
        rename = "minutes",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub minutes: i64,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(default)]
pub struct AttachmentSubResp {
    /// 附件token
    ///
    /// **示例值**: "xAAAAA"
    #[serde(
        rename = "file_token",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub file_token: String,
    /// 附件大小
    ///
    /// **示例值**: "2345"
    #[serde(
        rename = "file_size",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub file_size: String,
    /// 是否删除附件
    ///
    /// **示例值**: "true"
    #[serde(
        rename = "is_deleted",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub is_deleted: bool,
    /// 附件名称
    ///
    /// **示例值**: "附件.jpeg"
    #[serde(
        rename = "name",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub name: String,
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
        Fn(SearchCalendarEventReq) -> Result<(SearchCalendarEventResp, CommonResponse), Error>
        + Send
        + Sync
        + 'static
    {
    }
    impl<
            T: Fn(
                    SearchCalendarEventReq,
                ) -> Result<(SearchCalendarEventResp, CommonResponse), Error>
                + Send
                + Sync
                + 'static,
        > MockFunc for T
    {
    }

    impl<'c, IStore: Store, IClient: HttpClient> CalendarServiceMocker<'c, IStore, IClient> {
        pub fn mock_search_calendar_event<F: MockFunc>(
            &self,
            f: F,
        ) -> MockerBuilder<Mocker, SearchCalendarEventReq, SearchCalendarEventResp, Arc<dyn MockFunc>>
        {
            MockerBuilder::new(self.cli.instance_id, Arc::new(f))
        }

        pub(super) fn get_mock_search_calendar_event(
            &self,
            req: &SearchCalendarEventReq,
        ) -> Option<Arc<dyn MockFunc>> {
            do_mock::<Mocker, SearchCalendarEventReq, SearchCalendarEventResp, Arc<dyn MockFunc>>(
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
        api::gen::calendar::search_calendar_event::{
            SearchCalendarEventReq, SearchCalendarEventResp, SearchCalendarEventRespInner,
        },
        core::{model::CommonResponse, Lark},
    };

    #[tokio::test]
    async fn test_mock() {
        let lark = Lark::new("".to_owned(), "".into());
        let mocker = lark
            .calendar()
            .mock()
            .mock_search_calendar_event(|_| {
                Ok((
                    SearchCalendarEventResp::default(),
                    CommonResponse::default(),
                ))
            })
            .build();
        let res = lark
            .calendar()
            .search_calendar_event(SearchCalendarEventReq::default())
            .await;
        assert!(res.is_ok());
        mocker.clear();
        let res = lark
            .calendar()
            .search_calendar_event(SearchCalendarEventReq::default())
            .await;
        assert!(res.is_err());
    }

    const REQ: &str = r#"{
    "query": "query words",
    "filter": {
        "start_time": {
            "date": "2018-09-01",
            "timestamp": "1602504000",
            "timezone": "Asia/Shanghai"
        },
        "end_time": {
            "date": "2018-09-01",
            "timestamp": "1602504000",
            "timezone": "Asia/Shanghai"
        },
        "user_ids": [
            "ou_e051986ab19f80d16b7b8d74f3f1235"
        ],
        "room_ids": [
            "omm_eada1d61a550955240c28757e7dec3af"
        ],
        "chat_ids": [
            "oc_a0553eda9014c201e6969b478895c230"
        ]
    }
}"#;

    #[test]
    fn test_req() {
        if REQ == "{}" {
            return;
        }
        if let Err(e) = serde_json::from_str::<super::SearchCalendarEventReqBody>(REQ) {
            panic!("{}", e);
        }
    }

    const RESP: &str = r#"{
    "code": 0,
    "msg": "success",
    "data": {
        "items": [
            {
                "event_id": "00592a0e-7edf-4678-bc9d-1b77383ef08e_0",
                "organizer_calendar_id": "feishu.cn_xxxxxxxxxx@group.calendar.feishu.cn",
                "summary": "日程标题",
                "description": "日程描述",
                "start_time": {
                    "date": "2018-09-01",
                    "timestamp": "1602504000",
                    "timezone": "Asia/Shanghai"
                },
                "end_time": {
                    "date": "2018-09-01",
                    "timestamp": "1602504000",
                    "timezone": "Asia/Shanghai"
                },
                "visibility": "default",
                "attendee_ability": "can_see_others",
                "free_busy_status": "busy",
                "location": {
                    "name": "地点名称",
                    "address": "地点地址",
                    "latitude": 1.100000023841858,
                    "longitude": 2.200000047683716
                },
                "color": -1,
                "reminders": [
                    {
                        "minutes": 5
                    }
                ],
                "recurrence": "FREQ=DAILY;INTERVAL=1",
                "status": "confirmed",
                "is_exception": false,
                "recurring_event_id": "1cd45aaa-fa70-4195-80b7-c93b2e208f45",
                "event_organizer": {
                    "user_id": "ou_xxxxxx",
                    "display_name": "孙二二"
                },
                "app_link": "https://applink.larkoffice.com/client/calendar/event/detail?calendarId=7039673579105026066&key=aeac9c56-aeb1-4179-a21b-02f278f59048&originalTime=0&startTime=1700496000",
                "attachments": [
                    {
                        "file_token": "xAAAAA",
                        "file_size": "2345",
                        "is_deleted": true,
                        "name": "附件.jpeg"
                    }
                ]
            }
        ],
        "page_token": "xxxxx"
    }
}"#;
    #[test]
    fn test_resp() {
        let res = serde_json::from_str::<SearchCalendarEventRespInner>(RESP);
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
