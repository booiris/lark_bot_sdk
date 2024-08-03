//! doc url: <https://open.larkoffice.com/document/uAjLw4CM/ukTMukTMukTM/reference/calendar-v4/freebusy/list>
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
    /// **api 版本: 2024-07-16T06:20:39+00:00**
    ///
    /// ## 查询主日历日程忙闲信息
    ///
    /// 调用该接口查询指定用户的主日历忙闲信息，或者查询指定会议室的忙闲信息。
    ///
    /// 如果使用应用身份调用该接口，则需要确保应用开启了[机器人能力](https://open.feishu.cn/document/uAjLw4CM/ugTN1YjL4UTN24CO1UjN/trouble-shooting/how-to-enable-bot-ability)。
    ///
    /// doc: <https://open.larkoffice.com/document/uAjLw4CM/ukTMukTMukTM/reference/calendar-v4/freebusy/list>
    ///
    /// new doc: <https://open.larkoffice.com/document/server-docs/calendar-v4/calendar/list>
    ///
    /// api url: <https://open.feishu.cn/document_portal/v1/document/get_detail?fullPath=%2Fserver-docs%2Fcalendar-v4%2Fcalendar%2Flist>
    pub async fn get_calendar_free_busy_list(
        &self,
        req: GetCalendarFreeBusyListReq,
    ) -> Result<(GetCalendarFreeBusyListResp, CommonResponse), Error> {
        self.get_calendar_free_busy_list_with_opt(req, Default::default())
            .await
    }

    /// 参见 [get_calendar_free_busy_list](#method.get_calendar_free_busy_list) 函数
    pub async fn get_calendar_free_busy_list_with_opt(
        &self,
        req: GetCalendarFreeBusyListReq,
        method_option: MethodOption,
    ) -> Result<(GetCalendarFreeBusyListResp, CommonResponse), Error> {
        #[cfg(feature = "test-util")]
        {
            if let Some(f) = self.mock().get_mock_get_calendar_free_busy_list(&req) {
                tracing::info!("[lark] Calendar#GetCalendarFreeBusyList **mocking** api");
                return f(req);
            }
        }

        tracing::info!("[lark] Calendar#GetCalendarFreeBusyList call api");

        let req = ApiRequest {
            scope: "Calendar",
            api: "GetCalendarFreeBusyList",
            method: http::Method::POST,
            url: String::new()
                + self.cli.open_base_url.as_ref()
                + "/open-apis/calendar/v4/freebusy/list",
            param_data: req.gen_param(),
            method_option,
            need_tenant_access_token: true,
            need_user_access_token: true,
            ..Default::default()
        };

        let (resp, common_resp): (GetCalendarFreeBusyListRespInner, _) =
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
pub struct GetCalendarFreeBusyListReq {
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
    /// 查询时段开始时间，[RFC 3339](https://datatracker.ietf.org/doc/html/rfc3339) date_time 格式。
    ///
    /// **注意**：time_min 与 time_max 之间的时间间隔不能大于 90 天。
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "2020-10-28T12:00:00+08:00"
    #[api(kind = "body", name = "time_min")]
    pub time_min: String,
    /// 查询时段结束时间，[RFC 3339](https://datatracker.ietf.org/doc/html/rfc3339) date_time 格式。
    ///
    /// **注意**：time_min 与 time_max 之间的时间间隔不能大于 90 天。
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "2020-12-28T12:00:00+08:00"
    #[api(kind = "body", name = "time_max")]
    pub time_max: String,
    /// 用户 ID，需要传入与查询参数 user_id_type 相匹配的 id。例如，`user_id_type=open_id` 时，需要传入用户的 open_id。了解用户 ID 参见[用户相关的 ID 概念](https://open.feishu.cn/document/home/user-identity-introduction/introduction)。
    ///
    /// **注意**：user_id 与 room_id 需要二选一传入，如果同时传入则只生效 user_id。
    ///
    /// **示例值**: "ou_xxxxxxxxxx"
    #[api(kind = "body", name = "user_id")]
    pub user_id: Option<String>,
    /// 会议室 room_id。你可以调用[查询会议室列表](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/vc-v1/room/list)接口或者[搜索会议室](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/vc-v1/room/search)接口，获取相应会议室的 room_id。
    ///
    /// **注意**：user_id 与 room_id 需要二选一传入，如果同时传入则只生效 user_id。
    ///
    /// **示例值**: "omm_xxxxxxxxxx"
    #[api(kind = "body", name = "room_id")]
    pub room_id: Option<String>,
    /// 是否包含绑定的三方日历中的日程。
    ///
    /// **取值**：
    ///
    /// - true（默认值）：包含
    ///
    /// - false：不包含
    ///
    /// **示例值**: "true"
    #[api(kind = "body", name = "include_external_calendar")]
    pub include_external_calendar: Option<bool>,
    /// 是否只查询忙碌日程信息。
    ///
    /// **取值**：
    ///
    /// - true（默认值）：是，查询结果不包含空闲日程。
    ///
    /// - false：否，查询结果包含空闲日程。
    ///
    /// **示例值**: "true"
    #[api(kind = "body", name = "only_busy")]
    pub only_busy: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize, Clone, lark_bot_sdk_macros::ApiBaseResp)]
struct GetCalendarFreeBusyListRespInner {
    #[serde(flatten)]
    data: Option<GetCalendarFreeBusyListResp>,
    #[serde(flatten)]
    base: BaseResp,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct GetCalendarFreeBusyListResp {
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
    /// 在请求的时间区间内的忙碌时间段列表。
    #[serde(
        rename = "freebusy_list",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub freebusy_list: Vec<FreebusySubResp>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(default)]
pub struct FreebusySubResp {
    /// 忙闲信息开始时间，[RFC 3339](https://datatracker.ietf.org/doc/html/rfc3339) date_time 格式。
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "2020-10-28T22:30:00+08:00"
    #[serde(
        rename = "start_time",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub start_time: String,
    /// 忙闲信息结束时间，[RFC 3339](https://datatracker.ietf.org/doc/html/rfc3339) date_time 格式。
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "2020-10-28T22:45:00+08:00"
    #[serde(
        rename = "end_time",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub end_time: String,
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
            GetCalendarFreeBusyListReq,
        ) -> Result<(GetCalendarFreeBusyListResp, CommonResponse), Error>
        + Send
        + Sync
        + 'static
    {
    }
    impl<
            T: Fn(
                    GetCalendarFreeBusyListReq,
                ) -> Result<(GetCalendarFreeBusyListResp, CommonResponse), Error>
                + Send
                + Sync
                + 'static,
        > MockFunc for T
    {
    }

    impl<'c, IStore: Store, IClient: HttpClient> CalendarServiceMocker<'c, IStore, IClient> {
        pub fn mock_get_calendar_free_busy_list<F: MockFunc>(
            &self,
            f: F,
        ) -> MockerBuilder<
            Mocker,
            GetCalendarFreeBusyListReq,
            GetCalendarFreeBusyListResp,
            Arc<dyn MockFunc>,
        > {
            MockerBuilder::new(self.cli.instance_id, Arc::new(f))
        }

        pub(super) fn get_mock_get_calendar_free_busy_list(
            &self,
            req: &GetCalendarFreeBusyListReq,
        ) -> Option<Arc<dyn MockFunc>> {
            do_mock::<
                Mocker,
                GetCalendarFreeBusyListReq,
                GetCalendarFreeBusyListResp,
                Arc<dyn MockFunc>,
            >(self.cli.instance_id, req)
        }
    }
}

#[cfg(feature = "test-util")]
#[cfg(test)]
mod test {
    use crate::{
        api::gen::calendar::get_calendar_free_busy_list::{
            GetCalendarFreeBusyListReq, GetCalendarFreeBusyListResp,
            GetCalendarFreeBusyListRespInner,
        },
        core::{model::CommonResponse, Lark},
    };

    #[tokio::test]
    async fn test_mock() {
        let lark = Lark::new("".to_owned(), "".into());
        let mocker = lark
            .calendar()
            .mock()
            .mock_get_calendar_free_busy_list(|_| {
                Ok((
                    GetCalendarFreeBusyListResp::default(),
                    CommonResponse::default(),
                ))
            })
            .build();
        let res = lark
            .calendar()
            .get_calendar_free_busy_list(GetCalendarFreeBusyListReq::default())
            .await;
        assert!(res.is_ok());
        mocker.clear();
        let res = lark
            .calendar()
            .get_calendar_free_busy_list(GetCalendarFreeBusyListReq::default())
            .await;
        assert!(res.is_err());
    }

    const REQ: &str = r#"{
    "time_min": "2020-10-28T12:00:00+08:00",
    "time_max": "2020-12-28T12:00:00+08:00",
    "user_id": "ou_xxxxxxxxxx",
    "room_id": "omm_xxxxxxxxxx",
    "include_external_calendar": true,
    "only_busy": true
}"#;

    #[test]
    fn test_req() {
        if REQ == "{}" {
            return;
        }
        if let Err(e) = serde_json::from_str::<super::GetCalendarFreeBusyListReqBody>(REQ) {
            panic!("{}", e);
        }
    }

    const RESP: &str = r#"{
    "code": 0,
    "msg": "success",
    "data": {
        "freebusy_list": [
            {
                "start_time": "2020-10-28T22:30:00+08:00",
                "end_time": "2020-10-28T22:45:00+08:00"
            }
        ]
    }
}"#;
    #[test]
    fn test_resp() {
        let res = serde_json::from_str::<GetCalendarFreeBusyListRespInner>(RESP);
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
