//! doc url: <https://open.larkoffice.com/document/uAjLw4CM/ukTMukTMukTM/reference/vc-v1/meeting/list_by_no>
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
    /// **api 版本: 2024-05-20T08:34:10+00:00**
    ///
    /// ## 获取与会议号关联的会议列表
    ///
    /// 获取指定时间范围（90天内)会议号关联的会议简要信息列表。
    ///

    ///
    /// doc: <https://open.larkoffice.com/document/uAjLw4CM/ukTMukTMukTM/reference/vc-v1/meeting/list_by_no>
    ///
    /// new doc: <https://open.larkoffice.com/document/server-docs/vc-v1/meeting/list_by_no>
    ///
    /// api url: <https://open.feishu.cn/document_portal/v1/document/get_detail?fullPath=%2Fserver-docs%2Fvc-v1%2Fmeeting%2Flist_by_no>
    pub async fn list_vc_meeting_by_no(
        &self,
        req: ListVcMeetingByNoReq,
    ) -> Result<(ListVcMeetingByNoResp, CommonResponse), Error> {
        self.list_vc_meeting_by_no_with_opt(req, Default::default())
            .await
    }

    /// 参见 [list_vc_meeting_by_no](#method.list_vc_meeting_by_no) 函数
    pub async fn list_vc_meeting_by_no_with_opt(
        &self,
        req: ListVcMeetingByNoReq,
        method_option: MethodOption,
    ) -> Result<(ListVcMeetingByNoResp, CommonResponse), Error> {
        #[cfg(feature = "test-util")]
        {
            if let Some(f) = self.mock().get_mock_list_vc_meeting_by_no(&req) {
                tracing::info!("[lark] Vc#ListVcMeetingByNo **mocking** api");
                return f(req);
            }
        }

        tracing::info!("[lark] Vc#ListVcMeetingByNo call api");

        let req = ApiRequest {
            scope: "Vc",
            api: "ListVcMeetingByNo",
            method: http::Method::GET,
            url: String::new()
                + self.cli.open_base_url.as_ref()
                + "/open-apis/vc/v1/meetings/list_by_no",
            param_data: req.gen_param(),
            method_option,
            need_tenant_access_token: true,
            need_user_access_token: true,
            ..Default::default()
        };

        let (resp, common_resp): (ListVcMeetingByNoRespInner, _) = self.cli.do_req(req).await?;
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
pub struct ListVcMeetingByNoReq {
    /// 9位会议号
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "123456789"
    #[api(kind = "query", name = "meeting_no", v_type = "var", option = "false")]
    pub meeting_no: String,
    /// 查询开始时间（unix时间，单位sec）
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "1608888867"
    #[api(kind = "query", name = "start_time", v_type = "var", option = "false")]
    pub start_time: String,
    /// 查询结束时间（unix时间，单位sec）
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "1608888867"
    #[api(kind = "query", name = "end_time", v_type = "var", option = "false")]
    pub end_time: String,
    /// 分页标记，第一次请求不填，表示从头开始遍历；分页查询结果还有更多项时会同时返回新的 page_token，下次遍历可采用该 page_token 获取查询结果
    ///
    /// **示例值**: "5"
    #[api(kind = "query", name = "page_token", v_type = "var", option = "false")]
    pub page_token: String,
    /// 分页大小
    ///
    /// **示例值**: "10"
    #[api(kind = "query", name = "page_size", v_type = "var", option = "false")]
    pub page_size: i64,
}

#[derive(Debug, Serialize, Deserialize, Clone, lark_bot_sdk_macros::ApiBaseResp)]
struct ListVcMeetingByNoRespInner {
    #[serde(flatten)]
    data: Option<ListVcMeetingByNoResp>,
    #[serde(flatten)]
    base: BaseResp,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct ListVcMeetingByNoResp {
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
    /// 是否还有更多项
    ///
    /// **示例值**: "true"
    #[serde(
        rename = "has_more",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub has_more: bool,
    /// 分页标记，当 has_more 为 true 时，会同时返回新的 page_token，否则不返回 page_token
    ///
    /// **示例值**: "50"
    #[serde(
        rename = "page_token",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub page_token: String,
    /// 会议简要信息列表
    #[serde(
        rename = "meeting_briefs",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub meeting_briefs: Vec<MeetingSubResp>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(default)]
pub struct MeetingSubResp {
    /// 会议ID（视频会议的唯一标识，视频会议开始后才会产生）
    ///
    /// **示例值**: "6911188411934433028"
    #[serde(
        rename = "id",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub id: String,
    /// 会议主题
    ///
    /// **示例值**: "my meeting"
    #[serde(
        rename = "topic",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub topic: String,
    /// 会议链接（飞书用户可通过点击会议链接快捷入会）
    ///
    /// **示例值**: "https://vc.feishu.cn/j/337736498"
    #[serde(
        rename = "url",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub url: String,
    /// 会议号
    ///
    /// **示例值**: "123456789"
    #[serde(
        rename = "meeting_no",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub meeting_no: String,
    /// 会议密码
    ///
    /// **示例值**: "971024"
    #[serde(
        rename = "password",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub password: String,
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
        Fn(ListVcMeetingByNoReq) -> Result<(ListVcMeetingByNoResp, CommonResponse), Error>
        + Send
        + Sync
        + 'static
    {
    }
    impl<
            T: Fn(ListVcMeetingByNoReq) -> Result<(ListVcMeetingByNoResp, CommonResponse), Error>
                + Send
                + Sync
                + 'static,
        > MockFunc for T
    {
    }

    impl<'c, IStore: Store, IClient: HttpClient> VcServiceMocker<'c, IStore, IClient> {
        pub fn mock_list_vc_meeting_by_no<F: MockFunc>(
            &self,
            f: F,
        ) -> MockerBuilder<Mocker, ListVcMeetingByNoReq, ListVcMeetingByNoResp, Arc<dyn MockFunc>>
        {
            MockerBuilder::new(self.cli.instance_id, Arc::new(f))
        }

        pub(super) fn get_mock_list_vc_meeting_by_no(
            &self,
            req: &ListVcMeetingByNoReq,
        ) -> Option<Arc<dyn MockFunc>> {
            do_mock::<Mocker, ListVcMeetingByNoReq, ListVcMeetingByNoResp, Arc<dyn MockFunc>>(
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
        api::gen::vc::list_vc_meeting_by_no::{
            ListVcMeetingByNoReq, ListVcMeetingByNoResp, ListVcMeetingByNoRespInner,
        },
        core::{model::CommonResponse, Lark},
    };

    #[tokio::test]
    async fn test_mock() {
        let lark = Lark::new("".to_owned(), "".into());
        let mocker = lark
            .vc()
            .mock()
            .mock_list_vc_meeting_by_no(|_| {
                Ok((ListVcMeetingByNoResp::default(), CommonResponse::default()))
            })
            .build();
        let res = lark
            .vc()
            .list_vc_meeting_by_no(ListVcMeetingByNoReq::default())
            .await;
        assert!(res.is_ok());
        mocker.clear();
        let res = lark
            .vc()
            .list_vc_meeting_by_no(ListVcMeetingByNoReq::default())
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
    "data": {
        "has_more": true,
        "meeting_briefs": [
            {
                "id": "7011030664708603907",
                "meeting_no": "146453285",
                "password": "971024",
                "topic": "测试标题"
            },
            {
                "id": "7011031045899354115",
                "meeting_no": "146453285",
                "password": "971024",
                "topic": "测试标题"
            }
        ],
        "page_token": "2"
    },
    "msg": ""
}"#;
    #[test]
    fn test_resp() {
        let res = serde_json::from_str::<ListVcMeetingByNoRespInner>(RESP);
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
