//! doc url: <https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/calendar-v4/exchange_binding/get>
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
    /// **api 版本: 2024-07-10T03:34:23+00:00**
    ///
    /// ## 查询 Exchange 账户的绑定状态
    ///
    /// 调用该接口获取 Exchange 账户的绑定状态，包括 Exchange 日历的同步状态。
    ///
    /// 当前身份需要是企业超级管理员。
    ///
    /// doc: <https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/calendar-v4/exchange_binding/get>
    ///
    /// new doc: <https://open.feishu.cn/document/server-docs/calendar-v4/exchange_binding/get>
    ///
    /// api url: <https://open.feishu.cn/document_portal/v1/document/get_detail?fullPath=%2Fserver-docs%2Fcalendar-v4%2Fexchange_binding%2Fget>
    pub async fn get_calendar_exchange_binding(
        &self,
        req: GetCalendarExchangeBindingReq,
    ) -> Result<(GetCalendarExchangeBindingResp, CommonResponse), Error> {
        self.get_calendar_exchange_binding_with_opt(req, Default::default())
            .await
    }

    /// 参见 [get_calendar_exchange_binding](#method.get_calendar_exchange_binding) 函数
    pub async fn get_calendar_exchange_binding_with_opt(
        &self,
        req: GetCalendarExchangeBindingReq,
        method_option: MethodOption,
    ) -> Result<(GetCalendarExchangeBindingResp, CommonResponse), Error> {
        #[cfg(feature = "test-util")]
        {
            if let Some(f) = self.mock().get_mock_get_calendar_exchange_binding(&req) {
                tracing::info!("[lark] Calendar#GetCalendarExchangeBinding **mocking** api");
                return f(req);
            }
        }

        tracing::info!("[lark] Calendar#GetCalendarExchangeBinding call api");

        let req = ApiRequest {
            scope: "Calendar",
            api: "GetCalendarExchangeBinding",
            method: http::Method::GET,
            url: String::new()
                + self.cli.open_base_url.as_ref()
                + "/open-apis/calendar/v4/exchange_bindings/:exchange_binding_id",
            param_data: req.gen_param(),
            method_option,
            need_user_access_token: true,
            ..Default::default()
        };

        let (resp, common_resp): (GetCalendarExchangeBindingRespInner, _) =
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
pub struct GetCalendarExchangeBindingReq {
    /// Exchange 绑定的唯一标识 ID。调用 [将 Exchange 账户绑定到飞书账户](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/calendar-v4/exchange_binding/create) 绑定时，可从返回结果中获取 exchange_binding_id。
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "ZW1haWxfYWRtaW5fZXhhbXBsZUBvdXRsb29rLmNvbSBlbWFpbF9hY2NvdW50X2V4YW1wbGVAb3V0bG9vay5jb20="
    #[api(kind = "path", name = "exchange_binding_id")]
    pub exchange_binding_id: String,
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
}

#[derive(Debug, Serialize, Deserialize, Clone, lark_bot_sdk_macros::ApiBaseResp)]
struct GetCalendarExchangeBindingRespInner {
    #[serde(flatten)]
    data: Option<GetCalendarExchangeBindingResp>,
    #[serde(flatten)]
    base: BaseResp,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct GetCalendarExchangeBindingResp {
    /// \-
    #[serde(
        rename = "data",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub data: ExchangeBindingSubResp,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(default)]
pub struct ExchangeBindingSubResp {
    /// Exchange 的 admin 账户。
    ///
    /// **示例值**: "email_admin_example@outlook.com"
    ///
    /// **数据校验规则**：
    ///
    /// - **长度范围**: `1` 字符- `500` 字符
    #[serde(
        rename = "admin_account",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub admin_account: String,
    /// 需绑定的 Exchange 账户。
    ///
    /// **示例值**: "email_account_example@outlook.com"
    ///
    /// **数据校验规则**：
    ///
    /// - **长度范围**: `1` 字符- `500` 字符
    #[serde(
        rename = "exchange_account",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub exchange_account: String,
    /// 用户 ID，即 Exchange 账户绑定的飞书账户 ID。关于用户 ID 可参见[用户相关的 ID 概念](https://open.feishu.cn/document/home/user-identity-introduction/introduction)。
    ///
    /// **示例值**: "ou_xxxxxxxxxxxxxxxxxx"
    #[serde(
        rename = "user_id",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub user_id: String,
    /// Exchange 账户的同步状态。
    ///
    /// **示例值**: "doing"
    ///
    /// **可选值**:
    ///
    /// `Doing`: 日历正在同步
    ///
    /// `Cal_done`: 日历同步完成
    ///
    /// `Timespan_done`: 近期时间段同步完成
    ///
    /// `Done`: 日程同步完成
    ///
    /// `Err`: 同步错误
    #[serde(
        rename = "status",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub status: String,
    /// Exchange 绑定的唯一标识 ID。
    ///
    /// **示例值**: "ZW1haWxfYWRtaW5fZXhhbXBsZUBvdXRsb29rLmNvbSBlbWFpbF9hY2NvdW50X2V4YW1wbGVAb3V0bG9vay5jb20="
    #[serde(
        rename = "exchange_binding_id",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub exchange_binding_id: String,
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
            GetCalendarExchangeBindingReq,
        ) -> Result<(GetCalendarExchangeBindingResp, CommonResponse), Error>
        + Send
        + Sync
        + 'static
    {
    }
    impl<
            T: Fn(
                    GetCalendarExchangeBindingReq,
                )
                    -> Result<(GetCalendarExchangeBindingResp, CommonResponse), Error>
                + Send
                + Sync
                + 'static,
        > MockFunc for T
    {
    }

    impl<'c, IStore: Store, IClient: HttpClient> CalendarServiceMocker<'c, IStore, IClient> {
        pub fn mock_get_calendar_exchange_binding<F: MockFunc>(
            &self,
            f: F,
        ) -> MockerBuilder<
            Mocker,
            GetCalendarExchangeBindingReq,
            GetCalendarExchangeBindingResp,
            Arc<dyn MockFunc>,
        > {
            MockerBuilder::new(self.cli.instance_id, Arc::new(f))
        }

        pub(super) fn get_mock_get_calendar_exchange_binding(
            &self,
            req: &GetCalendarExchangeBindingReq,
        ) -> Option<Arc<dyn MockFunc>> {
            do_mock::<
                Mocker,
                GetCalendarExchangeBindingReq,
                GetCalendarExchangeBindingResp,
                Arc<dyn MockFunc>,
            >(self.cli.instance_id, req)
        }
    }
}

#[cfg(feature = "test-util")]
#[cfg(test)]
mod test {
    use crate::{
        api::gen::calendar::get_calendar_exchange_binding::{
            GetCalendarExchangeBindingReq, GetCalendarExchangeBindingResp,
            GetCalendarExchangeBindingRespInner,
        },
        core::{model::CommonResponse, Lark},
    };

    #[tokio::test]
    async fn test_mock() {
        let lark = Lark::new("".to_owned(), "".into());
        let mocker = lark
            .calendar()
            .mock()
            .mock_get_calendar_exchange_binding(|_| {
                Ok((
                    GetCalendarExchangeBindingResp::default(),
                    CommonResponse::default(),
                ))
            })
            .build();
        let res = lark
            .calendar()
            .get_calendar_exchange_binding(GetCalendarExchangeBindingReq::default())
            .await;
        assert!(res.is_ok());
        mocker.clear();
        let res = lark
            .calendar()
            .get_calendar_exchange_binding(GetCalendarExchangeBindingReq::default())
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
        "admin_account": "email_admin_example@outlook.com",
        "exchange_account": "email_account_example@outlook.com",
        "user_id": "ou_xxxxxxxxxxxxxxxxxx",
        "status": "doing",
        "exchange_binding_id": "ZW1haWxfYWRtaW5fZXhhbXBsZUBvdXRsb29rLmNvbSBlbWFpbF9hY2NvdW50X2V4YW1wbGVAb3V0bG9vay5jb20="
    }
}"#;
    #[test]
    fn test_resp() {
        let res = serde_json::from_str::<GetCalendarExchangeBindingRespInner>(RESP);
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
