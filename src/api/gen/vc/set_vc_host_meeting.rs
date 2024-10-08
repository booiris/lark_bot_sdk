//! doc url: <https://open.larkoffice.com/document/uAjLw4CM/ukTMukTMukTM/reference/vc-v1/meeting/set_host>
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
    /// ## 设置主持人
    ///
    /// 设置会议的主持人。
    ///
    /// 发起设置主持人的操作者必须具有相应的权限（如果操作者为用户，必须是会中当前主持人）；该操作使用CAS并发安全机制，需传入会中当前主持人，如果操作失败可使用返回的最新数据重试
    ///
    /// doc: <https://open.larkoffice.com/document/uAjLw4CM/ukTMukTMukTM/reference/vc-v1/meeting/set_host>
    ///
    /// new doc: <https://open.larkoffice.com/document/server-docs/vc-v1/meeting/set_host>
    ///
    /// api url: <https://open.feishu.cn/document_portal/v1/document/get_detail?fullPath=%2Fserver-docs%2Fvc-v1%2Fmeeting%2Fset_host>
    pub async fn set_vc_host_meeting(
        &self,
        req: SetVcHostMeetingReq,
    ) -> Result<(SetVcHostMeetingResp, CommonResponse), Error> {
        self.set_vc_host_meeting_with_opt(req, Default::default())
            .await
    }

    /// 参见 [set_vc_host_meeting](#method.set_vc_host_meeting) 函数
    pub async fn set_vc_host_meeting_with_opt(
        &self,
        req: SetVcHostMeetingReq,
        method_option: MethodOption,
    ) -> Result<(SetVcHostMeetingResp, CommonResponse), Error> {
        #[cfg(feature = "test-util")]
        {
            if let Some(f) = self.mock().get_mock_set_vc_host_meeting(&req) {
                tracing::info!("[lark] Vc#SetVcHostMeeting **mocking** api");
                return f(req);
            }
        }

        tracing::info!("[lark] Vc#SetVcHostMeeting call api");

        let req = ApiRequest {
            scope: "Vc",
            api: "SetVcHostMeeting",
            method: http::Method::PATCH,
            url: String::new()
                + self.cli.open_base_url.as_ref()
                + "/open-apis/vc/v1/meetings/:meeting_id/set_host",
            param_data: req.gen_param(),
            method_option,
            need_tenant_access_token: true,
            need_user_access_token: true,
            ..Default::default()
        };

        let (resp, common_resp): (SetVcHostMeetingRespInner, _) = self.cli.do_req(req).await?;
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
pub struct SetVcHostMeetingReq {
    /// 会议ID（视频会议的唯一标识，视频会议开始后才会产生）
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
    /// 将要设置的主持人
    ///
    /// **是否必填**: 是
    #[api(kind = "body", name = "host_user")]
    pub host_user: MeetingUserSubReq,
    /// 当前主持人（CAS并发安全：如果和会中当前主持人不符则会设置失败，可使用返回的最新数据重新设置）
    #[api(kind = "body", name = "old_host_user")]
    pub old_host_user: Option<MeetingUserSubReq>,
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
struct SetVcHostMeetingRespInner {
    #[serde(flatten)]
    data: Option<SetVcHostMeetingResp>,
    #[serde(flatten)]
    base: BaseResp,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct SetVcHostMeetingResp {
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
    /// 会中当前主持人
    #[serde(
        rename = "host_user",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub host_user: MeetingUserSubResp,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(default)]
pub struct MeetingUserSubResp {
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
    pub user_type: i64,
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
        Fn(SetVcHostMeetingReq) -> Result<(SetVcHostMeetingResp, CommonResponse), Error>
        + Send
        + Sync
        + 'static
    {
    }
    impl<
            T: Fn(SetVcHostMeetingReq) -> Result<(SetVcHostMeetingResp, CommonResponse), Error>
                + Send
                + Sync
                + 'static,
        > MockFunc for T
    {
    }

    impl<'c, IStore: Store, IClient: HttpClient> VcServiceMocker<'c, IStore, IClient> {
        pub fn mock_set_vc_host_meeting<F: MockFunc>(
            &self,
            f: F,
        ) -> MockerBuilder<Mocker, SetVcHostMeetingReq, SetVcHostMeetingResp, Arc<dyn MockFunc>>
        {
            MockerBuilder::new(self.cli.instance_id, Arc::new(f))
        }

        pub(super) fn get_mock_set_vc_host_meeting(
            &self,
            req: &SetVcHostMeetingReq,
        ) -> Option<Arc<dyn MockFunc>> {
            do_mock::<Mocker, SetVcHostMeetingReq, SetVcHostMeetingResp, Arc<dyn MockFunc>>(
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
        api::gen::vc::set_vc_host_meeting::{
            SetVcHostMeetingReq, SetVcHostMeetingResp, SetVcHostMeetingRespInner,
        },
        core::{model::CommonResponse, Lark},
    };

    #[tokio::test]
    async fn test_mock() {
        let lark = Lark::new("".to_owned(), "".into());
        let mocker = lark
            .vc()
            .mock()
            .mock_set_vc_host_meeting(|_| {
                Ok((SetVcHostMeetingResp::default(), CommonResponse::default()))
            })
            .build();
        let res = lark
            .vc()
            .set_vc_host_meeting(SetVcHostMeetingReq::default())
            .await;
        assert!(res.is_ok());
        mocker.clear();
        let res = lark
            .vc()
            .set_vc_host_meeting(SetVcHostMeetingReq::default())
            .await;
        assert!(res.is_err());
    }

    const REQ: &str = r#"{
    "host_user": {
        "id": "ou_3ec3f6a28a0d08c45d895276e8e5e19b",
        "user_type": 1
    },
    "old_host_user": {
        "id": "ou_3ec3f6a28a0d08c45d895276e8e5e19b",
        "user_type": 1
    }
}"#;

    #[test]
    fn test_req() {
        if REQ == "{}" {
            return;
        }
        if let Err(e) = serde_json::from_str::<super::SetVcHostMeetingReqBody>(REQ) {
            panic!("{}", e);
        }
    }

    const RESP: &str = r#"{
    "code": 0,
    "msg": "success",
    "data": {
        "host_user": {
            "id": "ou_3ec3f6a28a0d08c45d895276e8e5e19b",
            "user_type": 1
        }
    }
}"#;
    #[test]
    fn test_resp() {
        let res = serde_json::from_str::<SetVcHostMeetingRespInner>(RESP);
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
