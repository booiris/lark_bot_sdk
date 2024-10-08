//! doc url: <https://open.larkoffice.com/document/uAjLw4CM/ukTMukTMukTM/reference/calendar-v4/timeoff_event/delete>
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
    /// ## 删除请假日程
    ///
    /// 调用该接口删除一个指定的请假日程。请假日程删除后，用户个人签名页的请假信息也会消失。
    ///
    /// - 使用应用身份调用该接口，需要确保应用开启了[机器人能力](https://open.feishu.cn/document/uAjLw4CM/ugTN1YjL4UTN24CO1UjN/trouble-shooting/how-to-enable-bot-ability)。
    ///
    /// - 当前应用身份只能删除自己创建的请假日程。
    ///
    /// doc: <https://open.larkoffice.com/document/uAjLw4CM/ukTMukTMukTM/reference/calendar-v4/timeoff_event/delete>
    ///
    /// new doc: <https://open.larkoffice.com/document/server-docs/calendar-v4/timeoff_event/delete>
    ///
    /// api url: <https://open.feishu.cn/document_portal/v1/document/get_detail?fullPath=%2Fserver-docs%2Fcalendar-v4%2Ftimeoff_event%2Fdelete>
    pub async fn delete_calendar_timeoff_event(
        &self,
        req: DeleteCalendarTimeoffEventReq,
    ) -> Result<(DeleteCalendarTimeoffEventResp, CommonResponse), Error> {
        self.delete_calendar_timeoff_event_with_opt(req, Default::default())
            .await
    }

    /// 参见 [delete_calendar_timeoff_event](#method.delete_calendar_timeoff_event) 函数
    pub async fn delete_calendar_timeoff_event_with_opt(
        &self,
        req: DeleteCalendarTimeoffEventReq,
        method_option: MethodOption,
    ) -> Result<(DeleteCalendarTimeoffEventResp, CommonResponse), Error> {
        #[cfg(feature = "test-util")]
        {
            if let Some(f) = self.mock().get_mock_delete_calendar_timeoff_event(&req) {
                tracing::info!("[lark] Calendar#DeleteCalendarTimeoffEvent **mocking** api");
                return f(req);
            }
        }

        tracing::info!("[lark] Calendar#DeleteCalendarTimeoffEvent call api");

        let req = ApiRequest {
            scope: "Calendar",
            api: "DeleteCalendarTimeoffEvent",
            method: http::Method::DELETE,
            url: String::new()
                + self.cli.open_base_url.as_ref()
                + "/open-apis/calendar/v4/timeoff_events/:timeoff_event_id",
            param_data: req.gen_param(),
            method_option,
            need_tenant_access_token: true,
            ..Default::default()
        };

        let (resp, common_resp): (DeleteCalendarTimeoffEventRespInner, _) =
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
pub struct DeleteCalendarTimeoffEventReq {
    /// 请假日程 ID，在创建请假日程时从返回结果中获取。
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "timeoff:XXXXXX-XXXX-0917-1623-aa493d591a39"
    #[api(kind = "path", name = "timeoff_event_id")]
    pub timeoff_event_id: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, lark_bot_sdk_macros::ApiBaseResp)]
struct DeleteCalendarTimeoffEventRespInner {
    #[serde(flatten)]
    data: Option<DeleteCalendarTimeoffEventResp>,
    #[serde(flatten)]
    base: BaseResp,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct DeleteCalendarTimeoffEventResp {
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
        Fn(
            DeleteCalendarTimeoffEventReq,
        ) -> Result<(DeleteCalendarTimeoffEventResp, CommonResponse), Error>
        + Send
        + Sync
        + 'static
    {
    }
    impl<
            T: Fn(
                    DeleteCalendarTimeoffEventReq,
                )
                    -> Result<(DeleteCalendarTimeoffEventResp, CommonResponse), Error>
                + Send
                + Sync
                + 'static,
        > MockFunc for T
    {
    }

    impl<'c, IStore: Store, IClient: HttpClient> CalendarServiceMocker<'c, IStore, IClient> {
        pub fn mock_delete_calendar_timeoff_event<F: MockFunc>(
            &self,
            f: F,
        ) -> MockerBuilder<
            Mocker,
            DeleteCalendarTimeoffEventReq,
            DeleteCalendarTimeoffEventResp,
            Arc<dyn MockFunc>,
        > {
            MockerBuilder::new(self.cli.instance_id, Arc::new(f))
        }

        pub(super) fn get_mock_delete_calendar_timeoff_event(
            &self,
            req: &DeleteCalendarTimeoffEventReq,
        ) -> Option<Arc<dyn MockFunc>> {
            do_mock::<
                Mocker,
                DeleteCalendarTimeoffEventReq,
                DeleteCalendarTimeoffEventResp,
                Arc<dyn MockFunc>,
            >(self.cli.instance_id, req)
        }
    }
}

#[cfg(feature = "test-util")]
#[cfg(test)]
mod test {
    use crate::{
        api::gen::calendar::delete_calendar_timeoff_event::{
            DeleteCalendarTimeoffEventReq, DeleteCalendarTimeoffEventResp,
            DeleteCalendarTimeoffEventRespInner,
        },
        core::{model::CommonResponse, Lark},
    };

    #[tokio::test]
    async fn test_mock() {
        let lark = Lark::new("".to_owned(), "".into());
        let mocker = lark
            .calendar()
            .mock()
            .mock_delete_calendar_timeoff_event(|_| {
                Ok((
                    DeleteCalendarTimeoffEventResp::default(),
                    CommonResponse::default(),
                ))
            })
            .build();
        let res = lark
            .calendar()
            .delete_calendar_timeoff_event(DeleteCalendarTimeoffEventReq::default())
            .await;
        assert!(res.is_ok());
        mocker.clear();
        let res = lark
            .calendar()
            .delete_calendar_timeoff_event(DeleteCalendarTimeoffEventReq::default())
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
        let res = serde_json::from_str::<DeleteCalendarTimeoffEventRespInner>(RESP);
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
