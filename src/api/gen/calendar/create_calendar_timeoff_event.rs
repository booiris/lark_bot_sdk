//! doc url: <https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/calendar-v4/timeoff_event/create>
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
    /// ## 创建请假日程
    ///
    /// 调用该接口为指定用户创建一个请假日程。请假日程分为普通日程和全天日程。创建请假日程后，在请假时间内，用户个人签名页会展示请假信息。
    ///
    /// 使用应用身份调用该接口，需要确保应用开启了[机器人能力](https://open.feishu.cn/document/uAjLw4CM/ugTN1YjL4UTN24CO1UjN/trouble-shooting/how-to-enable-bot-ability)。
    ///
    /// doc: <https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/calendar-v4/timeoff_event/create>
    ///
    /// new doc: <https://open.feishu.cn/document/server-docs/calendar-v4/timeoff_event/create>
    ///
    /// api url: <https://open.feishu.cn/document_portal/v1/document/get_detail?fullPath=%2Fserver-docs%2Fcalendar-v4%2Ftimeoff_event%2Fcreate>
    pub async fn create_calendar_timeoff_event(
        &self,
        req: CreateCalendarTimeoffEventReq,
    ) -> Result<(CreateCalendarTimeoffEventResp, CommonResponse), Error> {
        self.create_calendar_timeoff_event_with_opt(req, Default::default())
            .await
    }

    /// 参见 [create_calendar_timeoff_event](#method.create_calendar_timeoff_event) 函数
    pub async fn create_calendar_timeoff_event_with_opt(
        &self,
        req: CreateCalendarTimeoffEventReq,
        method_option: MethodOption,
    ) -> Result<(CreateCalendarTimeoffEventResp, CommonResponse), Error> {
        #[cfg(feature = "test-util")]
        {
            if let Some(f) = self.mock().get_mock_create_calendar_timeoff_event(&req) {
                tracing::info!("[lark] Calendar#CreateCalendarTimeoffEvent **mocking** api");
                return f(req);
            }
        }

        tracing::info!("[lark] Calendar#CreateCalendarTimeoffEvent call api");

        let req = ApiRequest {
            scope: "Calendar",
            api: "CreateCalendarTimeoffEvent",
            method: http::Method::POST,
            url: String::new()
                + self.cli.open_base_url.as_ref()
                + "/open-apis/calendar/v4/timeoff_events",
            param_data: req.gen_param(),
            method_option,
            need_tenant_access_token: true,
            ..Default::default()
        };

        let (resp, common_resp): (CreateCalendarTimeoffEventRespInner, _) =
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
pub struct CreateCalendarTimeoffEventReq {
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
    /// 用户 ID。ID 类型需要与 user_id_type 的值保持一致。关于用户 ID 可参见[用户相关的 ID 概念](https://open.feishu.cn/document/home/user-identity-introduction/introduction)。
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "ou_XXXXXXXXXX"
    #[api(kind = "body", name = "user_id")]
    pub user_id: String,
    /// 时区信息。
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "Asia/Shanghai"
    #[api(kind = "body", name = "timezone")]
    pub timezone: String,
    /// 请假开始时间。支持以下任一格式：
    ///
    /// - 秒级时间戳：通过时间戳设置的请假日程为普通日程，即按小时请假。取值示例 `1609430400`
    ///
    /// - 日期格式：通过日期设置的请假日程为全天日程。取值示例 `2021-01-01`
    ///
    /// **注意**：start_time 和 end_time 所选用的时间格式必须保持一致，否则无效。
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "2021-01-01"
    #[api(kind = "body", name = "start_time")]
    pub start_time: String,
    /// 请假结束时间。支持以下任一格式：
    ///
    /// - 秒级时间戳：通过时间戳设置的请假日程为普通日程，即按小时请假。取值示例 `1609430400`
    ///
    /// - 日期格式：通过日期设置的请假日程为全天日程。取值示例 `2021-01-01`
    ///
    /// **注意**：start_time 和 end_time 所选用的时间格式必须保持一致，否则无效。
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "2021-01-01"
    #[api(kind = "body", name = "end_time")]
    pub end_time: String,
    /// 自定义请假日程标题。
    ///
    /// **默认值**：空，使用默认标题
    ///
    /// **示例值**: "请假中(全天) / 1-Day Time Off"
    #[api(kind = "body", name = "title")]
    pub title: Option<String>,
    /// 自定义请假日程描述。
    ///
    /// **默认值**：空，使用默认描述
    ///
    /// **示例值**: "若删除此日程，飞书中相应的“请假”标签将自动消失，而请假系统中的休假申请不会被撤销。"
    #[api(kind = "body", name = "description")]
    pub description: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, lark_bot_sdk_macros::ApiBaseResp)]
struct CreateCalendarTimeoffEventRespInner {
    #[serde(flatten)]
    data: Option<CreateCalendarTimeoffEventResp>,
    #[serde(flatten)]
    base: BaseResp,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct CreateCalendarTimeoffEventResp {
    /// \-
    #[serde(
        rename = "data",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub data: TimeoffEventSubResp,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(default)]
pub struct TimeoffEventSubResp {
    /// 请假日程 ID。后续可以使用该 ID 删除请假日程。
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "timeoff:XXXXXX-XXXX-0917-1623-aa493d591a39-XXXXXX"
    #[serde(
        rename = "timeoff_event_id",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub timeoff_event_id: String,
    /// 用户 ID。关于用户 ID 可参见[用户相关的 ID 概念](https://open.feishu.cn/document/home/user-identity-introduction/introduction)。
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "ou_XXXXXXXXXX"
    #[serde(
        rename = "user_id",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub user_id: String,
    /// 时区信息。
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "Asia/Shanghai"
    #[serde(
        rename = "timezone",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub timezone: String,
    /// 请假开始时间。可能返回的时间格式：
    ///
    /// - 秒级时间戳，例如 `1609430400`
    ///
    /// - 日期，例如 `2021-01-01`
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "2021-01-01"
    #[serde(
        rename = "start_time",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub start_time: String,
    /// 请假结束时间。可能返回的时间格式：
    ///
    /// - 秒级时间戳，例如 `1609430400`
    ///
    /// - 日期，例如 `2021-01-01`
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "2021-01-01"
    #[serde(
        rename = "end_time",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub end_time: String,
    /// 请假日程标题。
    ///
    /// **示例值**: "请假中(全天) / 1-Day Time Off"
    #[serde(
        rename = "title",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub title: String,
    /// 请假日程描述。
    ///
    /// **示例值**: "若删除此日程，飞书中相应的“请假”标签将自动消失，而请假系统中的休假申请不会被撤销。"
    #[serde(
        rename = "description",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub description: String,
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
            CreateCalendarTimeoffEventReq,
        ) -> Result<(CreateCalendarTimeoffEventResp, CommonResponse), Error>
        + Send
        + Sync
        + 'static
    {
    }
    impl<
            T: Fn(
                    CreateCalendarTimeoffEventReq,
                )
                    -> Result<(CreateCalendarTimeoffEventResp, CommonResponse), Error>
                + Send
                + Sync
                + 'static,
        > MockFunc for T
    {
    }

    impl<'c, IStore: Store, IClient: HttpClient> CalendarServiceMocker<'c, IStore, IClient> {
        pub fn mock_create_calendar_timeoff_event<F: MockFunc>(
            &self,
            f: F,
        ) -> MockerBuilder<
            Mocker,
            CreateCalendarTimeoffEventReq,
            CreateCalendarTimeoffEventResp,
            Arc<dyn MockFunc>,
        > {
            MockerBuilder::new(self.cli.instance_id, Arc::new(f))
        }

        pub(super) fn get_mock_create_calendar_timeoff_event(
            &self,
            req: &CreateCalendarTimeoffEventReq,
        ) -> Option<Arc<dyn MockFunc>> {
            do_mock::<
                Mocker,
                CreateCalendarTimeoffEventReq,
                CreateCalendarTimeoffEventResp,
                Arc<dyn MockFunc>,
            >(self.cli.instance_id, req)
        }
    }
}

#[cfg(feature = "test-util")]
#[cfg(test)]
mod test {
    use crate::{
        api::gen::calendar::create_calendar_timeoff_event::{
            CreateCalendarTimeoffEventReq, CreateCalendarTimeoffEventResp,
            CreateCalendarTimeoffEventRespInner,
        },
        core::{model::CommonResponse, Lark},
    };

    #[tokio::test]
    async fn test_mock() {
        let lark = Lark::new("".to_owned(), "".into());
        let mocker = lark
            .calendar()
            .mock()
            .mock_create_calendar_timeoff_event(|_| {
                Ok((
                    CreateCalendarTimeoffEventResp::default(),
                    CommonResponse::default(),
                ))
            })
            .build();
        let res = lark
            .calendar()
            .create_calendar_timeoff_event(CreateCalendarTimeoffEventReq::default())
            .await;
        assert!(res.is_ok());
        mocker.clear();
        let res = lark
            .calendar()
            .create_calendar_timeoff_event(CreateCalendarTimeoffEventReq::default())
            .await;
        assert!(res.is_err());
    }

    const REQ: &str = r#"{
    "user_id": "ou_XXXXXXXXXX",
    "timezone": "Asia/Shanghai",
    "start_time": "2021-01-01",
    "end_time": "2021-01-01",
    "title": "请假中(全天) / 1-Day Time Off",
    "description": "若删除此日程，飞书中相应的“请假”标签将自动消失，而请假系统中的休假申请不会被撤销。"
}"#;

    #[test]
    fn test_req() {
        if REQ == "{}" {
            return;
        }
        if let Err(e) = serde_json::from_str::<super::CreateCalendarTimeoffEventReqBody>(REQ) {
            panic!("{}", e);
        }
    }

    const RESP: &str = r#"{
    "code": 0,
    "msg": "success",
    "data": {
        "timeoff_event_id": "timeoff:XXXXXX-XXXX-0917-1623-aa493d591a39-XXXXXX",
        "user_id": "ou_XXXXXXXXXX",
        "timezone": "Asia/Shanghai",
        "start_time": "2021-01-01",
        "end_time": "2021-01-01",
        "title": "请假中(全天) / 1-Day Time Off",
        "description": "若删除此日程，飞书中相应的“请假”标签将自动消失，而请假系统中的休假申请不会被撤销。"
    }
}"#;
    #[test]
    fn test_resp() {
        let res = serde_json::from_str::<CreateCalendarTimeoffEventRespInner>(RESP);
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
