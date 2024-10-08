//! doc url: <https://open.larkoffice.com/document/uAjLw4CM/ukTMukTMukTM/helpdesk-v1/agent-schedules/get>
// Code generated by gen_api. DO NOT EDIT.

use serde::{Deserialize, Serialize};

use crate::api::BaseResp;
use crate::api::HasBaseResp;
use crate::core::model::{ApiRequest, CommonResponse, MethodOption, ReqParam, StreamReqParam};
use crate::{
    core::{http_client::HttpClient, store::Store},
    error::Error,
};

use crate::api::gen::helpdesk::HelpdeskService;

impl<'c, IStore: Store, IClient: HttpClient> HelpdeskService<'c, IStore, IClient> {
    /// **api 版本: 2023-08-15T07:34:19+00:00**
    ///
    /// ## 查询指定客服工作日程
    ///
    /// 该接口用于获取客服信息。
    ///

    ///
    /// doc: <https://open.larkoffice.com/document/uAjLw4CM/ukTMukTMukTM/helpdesk-v1/agent-schedules/get>
    ///
    /// new doc: <https://open.larkoffice.com/document/server-docs/helpdesk-v1/agent-function/agent-schedules/get>
    ///
    /// api url: <https://open.feishu.cn/document_portal/v1/document/get_detail?fullPath=%2Fserver-docs%2Fhelpdesk-v1%2Fagent-function%2Fagent-schedules%2Fget>
    pub async fn get_helpdesk_agent_schedule(
        &self,
        req: GetHelpdeskAgentScheduleReq,
    ) -> Result<(GetHelpdeskAgentScheduleResp, CommonResponse), Error> {
        self.get_helpdesk_agent_schedule_with_opt(req, Default::default())
            .await
    }

    /// 参见 [get_helpdesk_agent_schedule](#method.get_helpdesk_agent_schedule) 函数
    pub async fn get_helpdesk_agent_schedule_with_opt(
        &self,
        req: GetHelpdeskAgentScheduleReq,
        method_option: MethodOption,
    ) -> Result<(GetHelpdeskAgentScheduleResp, CommonResponse), Error> {
        #[cfg(feature = "test-util")]
        {
            if let Some(f) = self.mock().get_mock_get_helpdesk_agent_schedule(&req) {
                tracing::info!("[lark] Helpdesk#GetHelpdeskAgentSchedule **mocking** api");
                return f(req);
            }
        }

        tracing::info!("[lark] Helpdesk#GetHelpdeskAgentSchedule call api");

        let req = ApiRequest {
            scope: "Helpdesk",
            api: "GetHelpdeskAgentSchedule",
            method: http::Method::GET,
            url: String::new()
                + self.cli.open_base_url.as_ref()
                + "/open-apis/helpdesk/v1/agents/:agent_id/schedules",
            param_data: req.gen_param(),
            method_option,
            need_tenant_access_token: true,
            ..Default::default()
        };

        let (resp, common_resp): (GetHelpdeskAgentScheduleRespInner, _) =
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
pub struct GetHelpdeskAgentScheduleReq {
    /// 客服 id
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "	客服 id"
    #[api(kind = "path", name = "agent_id")]
    pub agent_id: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, lark_bot_sdk_macros::ApiBaseResp)]
struct GetHelpdeskAgentScheduleRespInner {
    #[serde(flatten)]
    data: Option<GetHelpdeskAgentScheduleResp>,
    #[serde(flatten)]
    base: BaseResp,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct GetHelpdeskAgentScheduleResp {
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
    /// 客服日程
    #[serde(
        rename = "agent_schedule",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub agent_schedule: AgentScheduleSubResp,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(default)]
pub struct AgentScheduleSubResp {
    /// 客服状态, 1 - online客服, 2 - offline(手动)客服, 3 - off duty(下班)自动处于非服务时间段
    ///
    /// **示例值**: "1"
    #[serde(
        rename = "status",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub status: i64,
    /// 客服信息
    #[serde(
        rename = "agent",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub agent: AgentUserSubResp,
    /// 工作日程列表
    #[serde(
        rename = "schedule",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub schedule: Vec<WeekdayScheduleSubResp>,
    /// 客服技能
    #[serde(
        rename = "agent_skills",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub agent_skills: Vec<AgentSkillLessInfoSubResp>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(default)]
pub struct AgentUserSubResp {
    /// 客服 id
    ///
    /// **示例值**: "ou_ea651a5c09e2d01af8acd34059f5359b"
    #[serde(
        rename = "id",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub id: String,
    /// avatar url
    ///
    /// **示例值**: "https://avatar-url.com/test.png"
    #[serde(
        rename = "avatar_url",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub avatar_url: String,
    /// 客服名字
    ///
    /// **示例值**: "test-user"
    #[serde(
        rename = "name",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub name: String,
    /// email
    ///
    /// **示例值**: "test@bytedance.com"
    #[serde(
        rename = "email",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub email: String,
    /// 部门
    ///
    /// **示例值**: "测试部门"
    #[serde(
        rename = "department",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub department: String,
    /// 公司名
    ///
    /// **示例值**: "test-company"
    #[serde(
        rename = "company_name",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub company_name: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(default)]
pub struct WeekdayScheduleSubResp {
    /// 开始时间, format 00:00 - 23:59
    ///
    /// **示例值**: "00:00"
    #[serde(
        rename = "start_time",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub start_time: String,
    /// 结束时间, format 00:00 - 23:59
    ///
    /// **示例值**: "24:00"
    #[serde(
        rename = "end_time",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub end_time: String,
    /// 星期几, 1 - Monday, 2 - Tuesday, 3 - Wednesday, 4 - Thursday, 5 - Friday, 6 - Saturday, 7 - Sunday, 9 - Everday, 10 - Weekday, 11 - Weekend
    ///
    /// **示例值**: "9"
    #[serde(
        rename = "weekday",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub weekday: i64,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(default)]
pub struct AgentSkillLessInfoSubResp {
    /// 客服技能 id
    ///
    /// **示例值**: "agent-skill-id"
    #[serde(
        rename = "id",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub id: String,
    /// 客服技能名
    ///
    /// **示例值**: "agent-skill"
    #[serde(
        rename = "name",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub name: String,
    /// 是默认技能
    ///
    /// **示例值**: "false"
    #[serde(
        rename = "is_default",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub is_default: bool,
}

#[cfg(feature = "test-util")]
mod test_utils {
    use std::sync::Arc;

    use self::gen::helpdesk::HelpdeskServiceMocker;
    use crate::core::mocker::*;

    use super::*;
    use crate::api::*;
    use crate::core::model::*;

    pub struct Mocker;

    pub trait MockFunc:
        Fn(
            GetHelpdeskAgentScheduleReq,
        ) -> Result<(GetHelpdeskAgentScheduleResp, CommonResponse), Error>
        + Send
        + Sync
        + 'static
    {
    }
    impl<
            T: Fn(
                    GetHelpdeskAgentScheduleReq,
                ) -> Result<(GetHelpdeskAgentScheduleResp, CommonResponse), Error>
                + Send
                + Sync
                + 'static,
        > MockFunc for T
    {
    }

    impl<'c, IStore: Store, IClient: HttpClient> HelpdeskServiceMocker<'c, IStore, IClient> {
        pub fn mock_get_helpdesk_agent_schedule<F: MockFunc>(
            &self,
            f: F,
        ) -> MockerBuilder<
            Mocker,
            GetHelpdeskAgentScheduleReq,
            GetHelpdeskAgentScheduleResp,
            Arc<dyn MockFunc>,
        > {
            MockerBuilder::new(self.cli.instance_id, Arc::new(f))
        }

        pub(super) fn get_mock_get_helpdesk_agent_schedule(
            &self,
            req: &GetHelpdeskAgentScheduleReq,
        ) -> Option<Arc<dyn MockFunc>> {
            do_mock::<
                Mocker,
                GetHelpdeskAgentScheduleReq,
                GetHelpdeskAgentScheduleResp,
                Arc<dyn MockFunc>,
            >(self.cli.instance_id, req)
        }
    }
}

#[cfg(feature = "test-util")]
#[cfg(test)]
mod test {
    use crate::{
        api::gen::helpdesk::get_helpdesk_agent_schedule::{
            GetHelpdeskAgentScheduleReq, GetHelpdeskAgentScheduleResp,
            GetHelpdeskAgentScheduleRespInner,
        },
        core::{model::CommonResponse, Lark},
    };

    #[tokio::test]
    async fn test_mock() {
        let lark = Lark::new("".to_owned(), "".into());
        let mocker = lark
            .helpdesk()
            .mock()
            .mock_get_helpdesk_agent_schedule(|_| {
                Ok((
                    GetHelpdeskAgentScheduleResp::default(),
                    CommonResponse::default(),
                ))
            })
            .build();
        let res = lark
            .helpdesk()
            .get_helpdesk_agent_schedule(GetHelpdeskAgentScheduleReq::default())
            .await;
        assert!(res.is_ok());
        mocker.clear();
        let res = lark
            .helpdesk()
            .get_helpdesk_agent_schedule(GetHelpdeskAgentScheduleReq::default())
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
        "agent_schedule": {
            "status": 1,
            "agent": {
                "id": "ou_ea651a5c09e2d01af8acd34059f5359b",
                "avatar_url": "https://avatar-url.com/test.png",
                "name": "test-user",
                "email": "test@bytedance.com",
                "department": "测试部门",
                "company_name": "test-company"
            },
            "schedule": [
                {
                    "start_time": "00:00",
                    "end_time": "24:00",
                    "weekday": 9
                }
            ],
            "agent_skills": [
                {
                    "id": "agent-skill-id",
                    "name": "agent-skill",
                    "is_default": false
                }
            ]
        }
    }
}"#;
    #[test]
    fn test_resp() {
        let res = serde_json::from_str::<GetHelpdeskAgentScheduleRespInner>(RESP);
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
