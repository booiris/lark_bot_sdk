//! doc url: <https://open.larkoffice.com/document/uAjLw4CM/ukTMukTMukTM/reference/vc-v1/meeting/kickout>
// Code generated by gen_api. DO NOT EDIT.

use serde::{Deserialize, Serialize};

use crate::api::BaseResp;
use crate::api::HasBaseResp;
use crate::core::model::{ApiRequest, CommonResponse, MethodOption, ReqParam, StreamReqParam};
use crate::{
    core::{http_client::HttpClient, store::Store},
    error::Error,
};

use crate::api::gen::vc::VcService;

impl<'c, IStore: Store, IClient: HttpClient> VcService<'c, IStore, IClient> {
    /// **api 版本: 2023-08-23T06:09:22+00:00**
    ///
    /// ## 移除参会人
    ///
    /// 将参会人从会议中移除。
    ///

    ///
    /// doc: <https://open.larkoffice.com/document/uAjLw4CM/ukTMukTMukTM/reference/vc-v1/meeting/kickout>
    ///
    /// new doc: <https://open.larkoffice.com/document/server-docs/vc-v1/meeting/kickout>
    ///
    /// api url: <https://open.feishu.cn/document_portal/v1/document/get_detail?fullPath=%2Fserver-docs%2Fvc-v1%2Fmeeting%2Fkickout>
    pub async fn kickout_vc_meeting(
        &self,
        req: KickoutVcMeetingReq,
    ) -> Result<(KickoutVcMeetingResp, CommonResponse), Error> {
        self.kickout_vc_meeting_with_opt(req, Default::default())
            .await
    }

    /// 参见 [kickout_vc_meeting](#method.kickout_vc_meeting) 函数
    pub async fn kickout_vc_meeting_with_opt(
        &self,
        req: KickoutVcMeetingReq,
        method_option: MethodOption,
    ) -> Result<(KickoutVcMeetingResp, CommonResponse), Error> {
        #[cfg(feature = "test-util")]
        {
            if let Some(f) = self.mock().get_mock_kickout_vc_meeting(&req) {
                tracing::info!("[lark] Vc#KickoutVcMeeting **mocking** api");
                return f(req);
            }
        }

        tracing::info!("[lark] Vc#KickoutVcMeeting call api");

        let req = ApiRequest {
            scope: "Vc",
            api: "KickoutVcMeeting",
            method: http::Method::POST,
            url: String::new()
                + self.cli.open_base_url.as_ref()
                + "/open-apis/vc/v1/meetings/:meeting_id/kickout",
            param_data: req.gen_param(),
            method_option,
            need_tenant_access_token: true,
            ..Default::default()
        };

        let (resp, common_resp): (KickoutVcMeetingRespInner, _) = self.cli.do_req(req).await?;
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
pub struct KickoutVcMeetingReq {
    /// 会议ID
    ///
    /// **示例值**: "6911188411932033028"
    #[api(kind = "path", name = "meeting_id")]
    pub meeting_id: String,
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
    /// 需移除的用户列表
    ///
    /// **是否必填**: 是
    #[api(kind = "body", name = "kickout_users")]
    pub kickout_users: Vec<Option<MeetingUserSubReq>>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(default)]
pub struct MeetingUserSubReq {
    /// 用户ID
    ///
    /// **示例值**: "ou_3ec3f6a28a0d08c45d895276e8e5e19b"
    #[serde(
        rename = "id",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub id: Option<String>,
    /// 用户类型
    ///
    /// **示例值**: "1"
    ///
    /// **可选值**:
    ///
    /// `lark_user`: 飞书用户
    ///
    /// `room_user`: rooms用户
    ///
    /// `doc_user`: 文档用户
    ///
    /// `neo_user`: neo单品用户
    ///
    /// `neo_guest_user`: neo单品游客用户
    ///
    /// `pstn_user`: pstn用户
    ///
    /// `sip_user`: sip用户
    #[serde(
        rename = "user_type",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub user_type: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize, Clone, lark_bot_sdk_macros::ApiBaseResp)]
struct KickoutVcMeetingRespInner {
    #[serde(flatten)]
    data: Option<KickoutVcMeetingResp>,
    #[serde(flatten)]
    base: BaseResp,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct KickoutVcMeetingResp {
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
    /// 移除结果
    #[serde(
        rename = "kickout_results",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub kickout_results: Vec<MeetingParticipantResultSubResp>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(default)]
pub struct MeetingParticipantResultSubResp {
    /// 用户ID
    ///
    /// **示例值**: "ou_3ec3f6a28a0d08c45d895276e8e5e19b"
    #[serde(
        rename = "id",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub id: String,
    /// 用户类型
    ///
    /// **示例值**: "1"
    ///
    /// **可选值**:
    ///
    /// `lark_user`: lark用户
    ///
    /// `room_user`: rooms用户
    ///
    /// `doc_user`: 文档用户
    ///
    /// `neo_user`: neo单品用户
    ///
    /// `neo_guest_user`: neo单品游客用户
    ///
    /// `pstn_user`: pstn用户
    ///
    /// `sip_user`: sip用户
    #[serde(
        rename = "user_type",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub user_type: i64,
    /// 移除结果
    ///
    /// **示例值**: "1"
    ///
    /// **可选值**:
    ///
    /// `ok`: 移除成功
    ///
    /// `failed`: 移除失败
    #[serde(
        rename = "result",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub result: i64,
}

#[cfg(feature = "test-util")]
mod test_utils {
    use std::sync::Arc;

    use self::gen::vc::VcServiceMocker;
    use crate::core::mocker::*;

    use super::*;
    use crate::api::*;
    use crate::core::model::*;

    pub struct Mocker;

    pub trait MockFunc:
        Fn(KickoutVcMeetingReq) -> Result<(KickoutVcMeetingResp, CommonResponse), Error>
        + Send
        + Sync
        + 'static
    {
    }
    impl<
            T: Fn(KickoutVcMeetingReq) -> Result<(KickoutVcMeetingResp, CommonResponse), Error>
                + Send
                + Sync
                + 'static,
        > MockFunc for T
    {
    }

    impl<'c, IStore: Store, IClient: HttpClient> VcServiceMocker<'c, IStore, IClient> {
        pub fn mock_kickout_vc_meeting<F: MockFunc>(
            &self,
            f: F,
        ) -> MockerBuilder<Mocker, KickoutVcMeetingReq, KickoutVcMeetingResp, Arc<dyn MockFunc>>
        {
            MockerBuilder::new(self.cli.instance_id, Arc::new(f))
        }

        pub(super) fn get_mock_kickout_vc_meeting(
            &self,
            req: &KickoutVcMeetingReq,
        ) -> Option<Arc<dyn MockFunc>> {
            do_mock::<Mocker, KickoutVcMeetingReq, KickoutVcMeetingResp, Arc<dyn MockFunc>>(
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
        api::gen::vc::kickout_vc_meeting::{
            KickoutVcMeetingReq, KickoutVcMeetingResp, KickoutVcMeetingRespInner,
        },
        core::{model::CommonResponse, Lark},
    };

    #[tokio::test]
    async fn test_mock() {
        let lark = Lark::new("".to_owned(), "".into());
        let mocker = lark
            .vc()
            .mock()
            .mock_kickout_vc_meeting(|_| {
                Ok((KickoutVcMeetingResp::default(), CommonResponse::default()))
            })
            .build();
        let res = lark
            .vc()
            .kickout_vc_meeting(KickoutVcMeetingReq::default())
            .await;
        assert!(res.is_ok());
        mocker.clear();
        let res = lark
            .vc()
            .kickout_vc_meeting(KickoutVcMeetingReq::default())
            .await;
        assert!(res.is_err());
    }

    const REQ: &str = r#"{
    "kickout_users": [
        {
            "id": "ou_3ec3f6a28a0d08c45d895276e8e5e19b",
            "user_type": 1
        }
    ]
}"#;

    #[test]
    fn test_req() {
        if REQ == "{}" {
            return;
        }
        if let Err(e) = serde_json::from_str::<super::KickoutVcMeetingReqBody>(REQ) {
            panic!("{}", e);
        }
    }

    const RESP: &str = r#"{
    "code": 0,
    "msg": "success",
    "data": {
        "kickout_results": [
            {
                "id": "ou_3ec3f6a28a0d08c45d895276e8e5e19b",
                "user_type": 1,
                "result": 1
            }
        ]
    }
}"#;
    #[test]
    fn test_resp() {
        let res = serde_json::from_str::<KickoutVcMeetingRespInner>(RESP);
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
