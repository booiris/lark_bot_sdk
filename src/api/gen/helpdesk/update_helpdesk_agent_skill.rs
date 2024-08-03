//! doc url: <https://open.larkoffice.com/document/uAjLw4CM/ukTMukTMukTM/helpdesk-v1/agent_skill/patch>
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
    /// **api 版本: 2024-03-06T11:33:59+00:00**
    ///
    /// ## 更新客服技能
    ///
    /// 该接口用于更新客服技能。
    ///

    ///
    /// doc: <https://open.larkoffice.com/document/uAjLw4CM/ukTMukTMukTM/helpdesk-v1/agent_skill/patch>
    ///
    /// new doc: <https://open.larkoffice.com/document/server-docs/helpdesk-v1/agent-function/agent_skill/patch>
    ///
    /// api url: <https://open.feishu.cn/document_portal/v1/document/get_detail?fullPath=%2Fserver-docs%2Fhelpdesk-v1%2Fagent-function%2Fagent_skill%2Fpatch>
    pub async fn update_helpdesk_agent_skill(
        &self,
        req: UpdateHelpdeskAgentSkillReq,
    ) -> Result<(UpdateHelpdeskAgentSkillResp, CommonResponse), Error> {
        self.update_helpdesk_agent_skill_with_opt(req, Default::default())
            .await
    }

    /// 参见 [update_helpdesk_agent_skill](#method.update_helpdesk_agent_skill) 函数
    pub async fn update_helpdesk_agent_skill_with_opt(
        &self,
        req: UpdateHelpdeskAgentSkillReq,
        method_option: MethodOption,
    ) -> Result<(UpdateHelpdeskAgentSkillResp, CommonResponse), Error> {
        #[cfg(feature = "test-util")]
        {
            if let Some(f) = self.mock().get_mock_update_helpdesk_agent_skill(&req) {
                tracing::info!("[lark] Helpdesk#UpdateHelpdeskAgentSkill **mocking** api");
                return f(req);
            }
        }

        tracing::info!("[lark] Helpdesk#UpdateHelpdeskAgentSkill call api");

        let req = ApiRequest {
            scope: "Helpdesk",
            api: "UpdateHelpdeskAgentSkill",
            method: http::Method::PATCH,
            url: String::new()
                + self.cli.open_base_url.as_ref()
                + "/open-apis/helpdesk/v1/agent_skills/:agent_skill_id",
            param_data: req.gen_param(),
            method_option,
            need_user_access_token: true,
            ..Default::default()
        };

        let (resp, common_resp): (UpdateHelpdeskAgentSkillRespInner, _) =
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
pub struct UpdateHelpdeskAgentSkillReq {
    /// agent skill id
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "test-skill-id"
    #[api(kind = "path", name = "agent_skill_id")]
    pub agent_skill_id: String,

    /// 更新技能
    #[api(kind = "body", name = "agent_skill")]
    pub agent_skill: Option<AgentSkillSubReq>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(default)]
pub struct AgentSkillSubReq {
    /// 技能名
    ///
    /// **示例值**: "skill-name"
    #[serde(
        rename = "name",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub name: Option<String>,
    /// 技能rules
    #[serde(
        rename = "rules",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub rules: Vec<Option<AgentSkillRuleSubReq>>,
    /// 具有此技能的客服ids
    ///
    /// **示例值**: "["ou_ea21d7f018e1155d960e40d33191f966"]"
    #[serde(
        rename = "agent_ids",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub agent_ids: Vec<Option<String>>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(default)]
pub struct AgentSkillRuleSubReq {
    /// rule id, 参考[获取客服技能rules](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/helpdesk-v1/agent_skill_rule/list) 用于获取rules options
    ///
    /// **示例值**: "test-skill-id"
    #[serde(
        rename = "id",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub id: Option<String>,
    /// 运算符比较, 参考[客服技能运算符选项](https://open.feishu.cn/document/ukTMukTMukTM/ucDOyYjL3gjM24yN4IjN/operator-options)
    ///
    /// **示例值**: "8"
    #[serde(
        rename = "selected_operator",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub selected_operator: Option<i64>,
    /// rule操作数value，[客服技能及运算符](https://open.feishu.cn/document/ukTMukTMukTM/ucDOyYjL3gjM24yN4IjN/operator-options)
    ///
    /// **示例值**: "[3]"
    #[serde(
        rename = "operator_options",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub operator_options: Vec<Option<i64>>,
    /// rule 操作数的值
    ///
    /// **示例值**: "{\"selected_departments\":[{\"id\":\"部门ID\",\"name\":\"IT\"}]}"
    #[serde(
        rename = "operand",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub operand: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, lark_bot_sdk_macros::ApiBaseResp)]
struct UpdateHelpdeskAgentSkillRespInner {
    #[serde(flatten)]
    data: Option<UpdateHelpdeskAgentSkillResp>,
    #[serde(flatten)]
    base: BaseResp,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct UpdateHelpdeskAgentSkillResp {
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

    use self::gen::helpdesk::HelpdeskServiceMocker;
    use crate::core::mocker::*;

    use super::*;
    use crate::api::*;
    use crate::core::model::*;

    pub struct Mocker;

    pub trait MockFunc:
        Fn(
            UpdateHelpdeskAgentSkillReq,
        ) -> Result<(UpdateHelpdeskAgentSkillResp, CommonResponse), Error>
        + Send
        + Sync
        + 'static
    {
    }
    impl<
            T: Fn(
                    UpdateHelpdeskAgentSkillReq,
                ) -> Result<(UpdateHelpdeskAgentSkillResp, CommonResponse), Error>
                + Send
                + Sync
                + 'static,
        > MockFunc for T
    {
    }

    impl<'c, IStore: Store, IClient: HttpClient> HelpdeskServiceMocker<'c, IStore, IClient> {
        pub fn mock_update_helpdesk_agent_skill<F: MockFunc>(
            &self,
            f: F,
        ) -> MockerBuilder<
            Mocker,
            UpdateHelpdeskAgentSkillReq,
            UpdateHelpdeskAgentSkillResp,
            Arc<dyn MockFunc>,
        > {
            MockerBuilder::new(self.cli.instance_id, Arc::new(f))
        }

        pub(super) fn get_mock_update_helpdesk_agent_skill(
            &self,
            req: &UpdateHelpdeskAgentSkillReq,
        ) -> Option<Arc<dyn MockFunc>> {
            do_mock::<
                Mocker,
                UpdateHelpdeskAgentSkillReq,
                UpdateHelpdeskAgentSkillResp,
                Arc<dyn MockFunc>,
            >(self.cli.instance_id, req)
        }
    }
}

#[cfg(feature = "test-util")]
#[cfg(test)]
mod test {
    use crate::{
        api::gen::helpdesk::update_helpdesk_agent_skill::{
            UpdateHelpdeskAgentSkillReq, UpdateHelpdeskAgentSkillResp,
            UpdateHelpdeskAgentSkillRespInner,
        },
        core::{model::CommonResponse, Lark},
    };

    #[tokio::test]
    async fn test_mock() {
        let lark = Lark::new("".to_owned(), "".into());
        let mocker = lark
            .helpdesk()
            .mock()
            .mock_update_helpdesk_agent_skill(|_| {
                Ok((
                    UpdateHelpdeskAgentSkillResp::default(),
                    CommonResponse::default(),
                ))
            })
            .build();
        let res = lark
            .helpdesk()
            .update_helpdesk_agent_skill(UpdateHelpdeskAgentSkillReq::default())
            .await;
        assert!(res.is_ok());
        mocker.clear();
        let res = lark
            .helpdesk()
            .update_helpdesk_agent_skill(UpdateHelpdeskAgentSkillReq::default())
            .await;
        assert!(res.is_err());
    }

    const REQ: &str = r#"{
    "agent_skill": {
        "name": "skill-name",
        "rules": [
            {
                "id": "test-skill-id",
                "selected_operator": 8,
                "operator_options": [
                    7,
                    8
                ],
                "operand": "{\n\"selected_departments\":[\n{\n\"id\":\"部门ID\",\n\"name\":\"IT\"\n}\n]\n}"
            }
        ],
        "agent_ids": [
            "ou_ea21d7f018e1155d960e40d33191f966"
        ]
    }
}"#;

    #[test]
    fn test_req() {
        if REQ == "{}" {
            return;
        }
        if let Err(e) = serde_json::from_str::<super::UpdateHelpdeskAgentSkillReqBody>(REQ) {
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
        let res = serde_json::from_str::<UpdateHelpdeskAgentSkillRespInner>(RESP);
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
