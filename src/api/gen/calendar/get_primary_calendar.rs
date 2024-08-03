//! doc url: <https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/calendar-v4/calendar/primary>
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
    /// **api 版本: 2024-07-16T06:20:38+00:00**
    ///
    /// ## 查询主日历信息
    ///
    /// 调用该接口获取当前身份（应用或用户）的主日历信息。
    ///
    /// **说明**
    ///
    /// - 当前身份由 Header Authorization 的 Token 类型决定。tenant_access_token 指应用身份，user_access_token 指用户身份。
    ///
    /// - 使用应用身份调用该接口前，需要确保该应用开启了[机器人能力](https://open.feishu.cn/document/uAjLw4CM/ugTN1YjL4UTN24CO1UjN/trouble-shooting/how-to-enable-bot-ability)。
    ///
    /// - 使用应用身份调用该接口时，查询参数 user_id_type 不能设置为 user_id。你可以选择 open_id 或者 union_id，在返回结果中，user_id 参数值会包含应用机器人对应的 open_id 或 union_id。
    ///
    /// doc: <https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/calendar-v4/calendar/primary>
    ///
    /// new doc: <https://open.feishu.cn/document/server-docs/calendar-v4/calendar/primary>
    ///
    /// api url: <https://open.feishu.cn/document_portal/v1/document/get_detail?fullPath=%2Fserver-docs%2Fcalendar-v4%2Fcalendar%2Fprimary>
    pub async fn get_primary_calendar(
        &self,
        req: GetPrimaryCalendarReq,
    ) -> Result<(GetPrimaryCalendarResp, CommonResponse), Error> {
        self.get_primary_calendar_with_opt(req, Default::default())
            .await
    }

    /// 参见 [get_primary_calendar](#method.get_primary_calendar) 函数
    pub async fn get_primary_calendar_with_opt(
        &self,
        req: GetPrimaryCalendarReq,
        method_option: MethodOption,
    ) -> Result<(GetPrimaryCalendarResp, CommonResponse), Error> {
        #[cfg(feature = "test-util")]
        {
            if let Some(f) = self.mock().get_mock_get_primary_calendar(&req) {
                tracing::info!("[lark] Calendar#GetPrimaryCalendar **mocking** api");
                return f(req);
            }
        }

        tracing::info!("[lark] Calendar#GetPrimaryCalendar call api");

        let req = ApiRequest {
            scope: "Calendar",
            api: "GetPrimaryCalendar",
            method: http::Method::POST,
            url: String::new()
                + self.cli.open_base_url.as_ref()
                + "/open-apis/calendar/v4/calendars/primary",
            param_data: req.gen_param(),
            method_option,
            need_tenant_access_token: true,
            need_user_access_token: true,
            ..Default::default()
        };

        let (resp, common_resp): (GetPrimaryCalendarRespInner, _) = self.cli.do_req(req).await?;
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
pub struct GetPrimaryCalendarReq {
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
struct GetPrimaryCalendarRespInner {
    #[serde(flatten)]
    data: Option<GetPrimaryCalendarResp>,
    #[serde(flatten)]
    base: BaseResp,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct GetPrimaryCalendarResp {
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
    /// 主日历列表。
    #[serde(
        rename = "calendars",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub calendars: Vec<UserCalendarSubResp>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(default)]
pub struct UserCalendarSubResp {
    /// 日历实体信息。
    #[serde(
        rename = "calendar",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub calendar: CalendarSubResp,
    /// 日历创建者的 User ID。了解用户不同类型的 ID，可参见[用户身份概述](https://open.feishu.cn/document/home/user-identity-introduction/introduction)。
    ///
    /// **示例值**: "ou_xxxxxx"
    #[serde(
        rename = "user_id",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub user_id: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(default)]
pub struct CalendarSubResp {
    /// 日历 ID。你可以通过该 ID 查询、更新主日历信息。更多信息可参见[日历 ID 字段说明](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/calendar-v4/calendar/introduction)。
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "feishu.cn_xxxxxxxxxx@group.calendar.feishu.cn"
    #[serde(
        rename = "calendar_id",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub calendar_id: String,
    /// 日历标题。
    ///
    /// **示例值**: "测试日历"
    ///
    /// **数据校验规则**：
    ///
    /// - **最大长度**: `255` 字符
    #[serde(
        rename = "summary",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub summary: String,
    /// 日历描述。
    ///
    /// **示例值**: "使用开放接口创建日历"
    ///
    /// **数据校验规则**：
    ///
    /// - **最大长度**: `255` 字符
    #[serde(
        rename = "description",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub description: String,
    /// 日历公开范围。
    ///
    /// **示例值**: "private"
    ///
    /// **可选值**:
    ///
    /// `Private`: 私密
    ///
    /// `ShowOnlyFreeBusy`: 仅展示忙闲信息
    ///
    /// `Public`: 公开，他人可查看日程详情
    #[serde(
        rename = "permissions",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub permissions: String,
    /// 日历颜色，由颜色 RGB 值的 int32 表示。实际在客户端展示时会映射到色板上最接近的一种颜色，且该字段仅对当前身份生效。
    ///
    /// **示例值**: "-1"
    #[serde(
        rename = "color",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub color: i64,
    /// 日历类型。
    ///
    /// **示例值**: "shared"
    ///
    /// **可选值**:
    ///
    /// `Unknown`: 未知类型
    ///
    /// `Primary`: 用户或应用的主日历
    ///
    /// `Shared`: 由用户或应用创建的共享日历
    ///
    /// `Google`: 用户绑定的谷歌日历
    ///
    /// `Resource`: 会议室日历
    ///
    /// `Exchange`: 用户绑定的 Exchange 日历
    #[serde(
        rename = "type",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub body_type: String,
    /// 日历备注名，仅对当前身份生效。
    ///
    /// **示例值**: "日历备注名"
    ///
    /// **数据校验规则**：
    ///
    /// - **最大长度**: `255` 字符
    #[serde(
        rename = "summary_alias",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub summary_alias: String,
    /// 对于当前身份，日历是否已经被标记为删除。
    ///
    /// **示例值**: "false"
    #[serde(
        rename = "is_deleted",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub is_deleted: bool,
    /// 当前日历是否是第三方数据。三方日历及日程只支持读，不支持写入。
    ///
    /// **示例值**: "false"
    #[serde(
        rename = "is_third_party",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub is_third_party: bool,
    /// 当前身份对于该日历的访问权限。
    ///
    /// **示例值**: "owner"
    ///
    /// **可选值**:
    ///
    /// `Unkonwn`: 未知权限
    ///
    /// `FreeBusyReader`: 游客，只能看到忙碌、空闲信息
    ///
    /// `Reader`: 订阅者，可查看所有日程详情
    ///
    /// `Writer`: 编辑者，可创建及修改日程
    ///
    /// `Owner`: 管理员，可管理日历及共享设置
    #[serde(
        rename = "role",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub role: String,
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
        Fn(GetPrimaryCalendarReq) -> Result<(GetPrimaryCalendarResp, CommonResponse), Error>
        + Send
        + Sync
        + 'static
    {
    }
    impl<
            T: Fn(GetPrimaryCalendarReq) -> Result<(GetPrimaryCalendarResp, CommonResponse), Error>
                + Send
                + Sync
                + 'static,
        > MockFunc for T
    {
    }

    impl<'c, IStore: Store, IClient: HttpClient> CalendarServiceMocker<'c, IStore, IClient> {
        pub fn mock_get_primary_calendar<F: MockFunc>(
            &self,
            f: F,
        ) -> MockerBuilder<Mocker, GetPrimaryCalendarReq, GetPrimaryCalendarResp, Arc<dyn MockFunc>>
        {
            MockerBuilder::new(self.cli.instance_id, Arc::new(f))
        }

        pub(super) fn get_mock_get_primary_calendar(
            &self,
            req: &GetPrimaryCalendarReq,
        ) -> Option<Arc<dyn MockFunc>> {
            do_mock::<Mocker, GetPrimaryCalendarReq, GetPrimaryCalendarResp, Arc<dyn MockFunc>>(
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
        api::gen::calendar::get_primary_calendar::{
            GetPrimaryCalendarReq, GetPrimaryCalendarResp, GetPrimaryCalendarRespInner,
        },
        core::{model::CommonResponse, Lark},
    };

    #[tokio::test]
    async fn test_mock() {
        let lark = Lark::new("".to_owned(), "".into());
        let mocker = lark
            .calendar()
            .mock()
            .mock_get_primary_calendar(|_| {
                Ok((GetPrimaryCalendarResp::default(), CommonResponse::default()))
            })
            .build();
        let res = lark
            .calendar()
            .get_primary_calendar(GetPrimaryCalendarReq::default())
            .await;
        assert!(res.is_ok());
        mocker.clear();
        let res = lark
            .calendar()
            .get_primary_calendar(GetPrimaryCalendarReq::default())
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
        "calendars": [
            {
                "calendar": {
                    "calendar_id": "feishu.cn_xxxxxxxxxx@group.calendar.feishu.cn",
                    "summary": "测试日历",
                    "description": "使用开放接口创建日历",
                    "permissions": "private",
                    "color": -1,
                    "type": "shared",
                    "summary_alias": "日历备注名",
                    "is_deleted": false,
                    "is_third_party": false,
                    "role": "owner"
                },
                "user_id": "ou_xxxxxx"
            }
        ]
    }
}"#;
    #[test]
    fn test_resp() {
        let res = serde_json::from_str::<GetPrimaryCalendarRespInner>(RESP);
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
