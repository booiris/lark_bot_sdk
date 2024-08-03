//! doc url: <https://open.larkoffice.com/document/uAjLw4CM/ukTMukTMukTM/reference/vc-v1/reserve/get>
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
    /// **api 版本: 2024-07-23T07:32:56+00:00**
    ///
    /// ## 获取预约
    ///
    /// 获取一个预约的详情。
    ///
    /// 只能获取归属于自己的预约
    ///
    /// doc: <https://open.larkoffice.com/document/uAjLw4CM/ukTMukTMukTM/reference/vc-v1/reserve/get>
    ///
    /// new doc: <https://open.larkoffice.com/document/server-docs/vc-v1/reserve/get>
    ///
    /// api url: <https://open.feishu.cn/document_portal/v1/document/get_detail?fullPath=%2Fserver-docs%2Fvc-v1%2Freserve%2Fget>
    pub async fn get_vc_reserve(
        &self,
        req: GetVcReserveReq,
    ) -> Result<(GetVcReserveResp, CommonResponse), Error> {
        self.get_vc_reserve_with_opt(req, Default::default()).await
    }

    /// 参见 [get_vc_reserve](#method.get_vc_reserve) 函数
    pub async fn get_vc_reserve_with_opt(
        &self,
        req: GetVcReserveReq,
        method_option: MethodOption,
    ) -> Result<(GetVcReserveResp, CommonResponse), Error> {
        #[cfg(feature = "test-util")]
        {
            if let Some(f) = self.mock().get_mock_get_vc_reserve(&req) {
                tracing::info!("[lark] Vc#GetVcReserve **mocking** api");
                return f(req);
            }
        }

        tracing::info!("[lark] Vc#GetVcReserve call api");

        let req = ApiRequest {
            scope: "Vc",
            api: "GetVcReserve",
            method: http::Method::GET,
            url: String::new()
                + self.cli.open_base_url.as_ref()
                + "/open-apis/vc/v1/reserves/:reserve_id",
            param_data: req.gen_param(),
            method_option,
            need_tenant_access_token: true,
            need_user_access_token: true,
            ..Default::default()
        };

        let (resp, common_resp): (GetVcReserveRespInner, _) = self.cli.do_req(req).await?;
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
pub struct GetVcReserveReq {
    /// 预约ID（预约的唯一标识）
    ///
    /// **示例值**: "6911188411932033028"
    #[api(kind = "path", name = "reserve_id")]
    pub reserve_id: String,
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
struct GetVcReserveRespInner {
    #[serde(flatten)]
    data: Option<GetVcReserveResp>,
    #[serde(flatten)]
    base: BaseResp,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct GetVcReserveResp {
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
    /// 预约数据
    #[serde(
        rename = "reserve",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub reserve: ReserveSubResp,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(default)]
pub struct ReserveSubResp {
    /// 预约ID（预约的唯一标识，非会议ID，会议ID仅在会议开始后才生成）
    ///
    /// **示例值**: "6911188411934973028"
    #[serde(
        rename = "id",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub id: String,
    /// 9位会议号（飞书用户可通过输入9位会议号快捷入会）
    ///
    /// **示例值**: "112000358"
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
    /// 会议链接（飞书用户可通过点击会议链接快捷入会）
    ///
    /// **示例值**: "https://vc.feishu.cn/j/337736498"
    #[serde(
        rename = "url",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub url: String,
    /// APPLink用于唤起飞书APP入会。"{?}"为占位符，用于配置入会参数，使用时需替换具体值：0表示关闭，1表示打开。preview为入会前的设置页，mic为麦克风，speaker为扬声器，camera为摄像头
    ///
    /// **示例值**: "https://applink.feishu.cn/client/videochat/open?source=openplatform&action=join&idtype=reservationid&id={?}&preview={?}&mic={?}&speaker={?}&camera={?}"
    #[serde(
        rename = "app_link",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub app_link: String,
    /// 会议转直播链接
    ///
    /// **示例值**: "https://meetings.feishu.cn/s/1gub381l4gglv"
    #[serde(
        rename = "live_link",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub live_link: String,
    /// 预约到期时间（unix时间，单位sec）
    ///
    /// **示例值**: "1608883322"
    #[serde(
        rename = "end_time",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub end_time: String,
    /// 过期状态
    ///
    /// **示例值**: "0"
    ///
    /// **可选值**:
    ///
    /// `not_expired`: 未过期
    ///
    /// `expired`: 已过期
    #[serde(
        rename = "expire_status",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub expire_status: i64,
    /// 预约人ID
    ///
    /// **示例值**: "ou_3ec3f6a28a0d08c45d895276e8e5e19b"
    #[serde(
        rename = "reserve_user_id",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub reserve_user_id: String,
    /// 会议设置
    #[serde(
        rename = "meeting_settings",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub meeting_settings: ReserveMeetingSettingSubResp,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(default)]
pub struct ReserveMeetingSettingSubResp {
    /// 会议主题
    ///
    /// **示例值**: "my meeting"
    #[serde(
        rename = "topic",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub topic: String,
    /// 会议权限配置列表，如果存在相同的权限配置项则它们之间为"逻辑或"的关系（即 有一个为true则拥有该权限）
    #[serde(
        rename = "action_permissions",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub action_permissions: Vec<ReserveActionPermissionSubResp>,
    /// 会议初始类型
    ///
    /// **示例值**: "1"
    ///
    /// **可选值**:
    ///
    /// `group_meeting`: 多人会议
    ///
    /// `call`: 1v1呼叫
    #[serde(
        rename = "meeting_initial_type",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub meeting_initial_type: i64,
    /// 1v1呼叫相关参数
    #[serde(
        rename = "call_setting",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub call_setting: ReserveCallSettingSubResp,
    /// 使用飞书视频会议时，是否开启自动录制，默认false
    ///
    /// **示例值**: "true"
    #[serde(
        rename = "auto_record",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub auto_record: bool,
    /// 指定主持人列表
    #[serde(
        rename = "assign_host_list",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub assign_host_list: Vec<ReserveAssignHostSubResp>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(default)]
pub struct ReserveCallSettingSubResp {
    /// 被呼叫的用户
    ///
    /// **是否必填**: 是
    #[serde(
        rename = "callee",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub callee: ReserveCalleeSubResp,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(default)]
pub struct ReserveActionPermissionSubResp {
    /// 权限项
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "1"
    ///
    /// **可选值**:
    ///
    /// `can_be_host`: 是否能成为主持人
    ///
    /// `can_invite`: 是否能邀请参会人
    ///
    /// `can_join`: 是否能加入会议
    #[serde(
        rename = "permission",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub permission: i64,
    /// 权限检查器列表，权限检查器之间为"逻辑或"的关系（即 有一个为true则拥有该权限）
    ///
    /// **是否必填**: 是
    #[serde(
        rename = "permission_checkers",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub permission_checkers: Vec<ReservePermissionCheckerSubResp>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(default)]
pub struct ReserveCalleeSubResp {
    /// 用户ID
    ///
    /// **示例值**: "ou_3ec3f6a28a0d08c45d895276e8e5e19b"
    #[serde(
        rename = "id",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub id: String,
    /// 用户类型，当前仅支持用户类型6(pstn用户)
    ///
    /// **是否必填**: 是
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
    /// pstn/sip信息
    #[serde(
        rename = "pstn_sip_info",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub pstn_sip_info: PstnSipInfoSubResp,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(default)]
pub struct ReserveAssignHostSubResp {
    /// 用户类型，仅支持设置同租户下的 Lark 用户
    ///
    /// **示例值**: "1"
    ///
    /// **可选值**:
    ///
    /// `lark_user`: 飞书用户
    #[serde(
        rename = "user_type",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub user_type: i64,
    /// 用户ID
    ///
    /// **示例值**: "ou_3ec3f6a28a0d08c45d895276e8e5e19b"
    #[serde(
        rename = "id",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub id: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(default)]
pub struct PstnSipInfoSubResp {
    /// 给pstn/sip用户设置的临时昵称
    ///
    /// **示例值**: "dodo"
    #[serde(
        rename = "nickname",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub nickname: String,
    /// pstn/sip主机号，格式为：[国际冠字]-[电话区号][电话号码]，当前仅支持国内手机及固定电话号码
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "+86-02187654321"
    #[serde(
        rename = "main_address",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub main_address: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(default)]
pub struct ReservePermissionCheckerSubResp {
    /// 检查字段类型
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "1"
    ///
    /// **可选值**:
    ///
    /// `user_id`: 用户ID（check_list填入用户ID）
    ///
    /// `user_type`: 用户类型（check_list可选值有
    ///
    /// "1"：飞书用户、
    ///
    /// "2"：rooms用户、
    ///
    /// "6"：pstn用户、
    ///
    /// "7"：sip用户）
    ///
    /// `tenant_id`: 租户ID（check_list填入租户tenant_key）
    #[serde(
        rename = "check_field",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub check_field: i64,
    /// 检查方式
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "1"
    ///
    /// **可选值**:
    ///
    /// `within`: 在check_list中为有权限（白名单）
    ///
    /// `without`: 不在check_list中为有权限（黑名单）
    #[serde(
        rename = "check_mode",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub check_mode: i64,
    /// 检查字段列表（根据check_field的类型填入对应内容）
    ///
    /// **是否必填**: 是
    #[serde(
        rename = "check_list",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub check_list: Vec<String>,
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
        Fn(GetVcReserveReq) -> Result<(GetVcReserveResp, CommonResponse), Error>
        + Send
        + Sync
        + 'static
    {
    }
    impl<
            T: Fn(GetVcReserveReq) -> Result<(GetVcReserveResp, CommonResponse), Error>
                + Send
                + Sync
                + 'static,
        > MockFunc for T
    {
    }

    impl<'c, IStore: Store, IClient: HttpClient> VcServiceMocker<'c, IStore, IClient> {
        pub fn mock_get_vc_reserve<F: MockFunc>(
            &self,
            f: F,
        ) -> MockerBuilder<Mocker, GetVcReserveReq, GetVcReserveResp, Arc<dyn MockFunc>> {
            MockerBuilder::new(self.cli.instance_id, Arc::new(f))
        }

        pub(super) fn get_mock_get_vc_reserve(
            &self,
            req: &GetVcReserveReq,
        ) -> Option<Arc<dyn MockFunc>> {
            do_mock::<Mocker, GetVcReserveReq, GetVcReserveResp, Arc<dyn MockFunc>>(
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
        api::gen::vc::get_vc_reserve::{GetVcReserveReq, GetVcReserveResp, GetVcReserveRespInner},
        core::{model::CommonResponse, Lark},
    };

    #[tokio::test]
    async fn test_mock() {
        let lark = Lark::new("".to_owned(), "".into());
        let mocker = lark
            .vc()
            .mock()
            .mock_get_vc_reserve(|_| Ok((GetVcReserveResp::default(), CommonResponse::default())))
            .build();
        let res = lark.vc().get_vc_reserve(GetVcReserveReq::default()).await;
        assert!(res.is_ok());
        mocker.clear();
        let res = lark.vc().get_vc_reserve(GetVcReserveReq::default()).await;
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
        "reserve": {
            "id": "6911188411934973028",
            "meeting_no": "112000358",
            "password": "971024",
            "url": "https://vc.feishu.cn/j/337736498",
            "app_link": "https://applink.feishu.cn/client/videochat/open?source=openplatform&action=join&idtype=reservationid&id={?}&preview={?}&mic={?}&speaker={?}&camera={?}",
            "live_link": "https://meetings.feishu.cn/s/1gub381l4gglv",
            "end_time": "1608883322",
            "expire_status": 0,
            "reserve_user_id": "ou_3ec3f6a28a0d08c45d895276e8e5e19b",
            "meeting_settings": {
                "topic": "my meeting",
                "action_permissions": [
                    {
                        "permission": 1,
                        "permission_checkers": [
                            {
                                "check_field": 1,
                                "check_mode": 1,
                                "check_list": [
                                    "ou_3ec3f6a28a0d08c45d895276e8e5e19b"
                                ]
                            }
                        ]
                    }
                ],
                "meeting_initial_type": 1,
                "call_setting": {
                    "callee": {
                        "id": "ou_3ec3f6a28a0d08c45d895276e8e5e19b",
                        "user_type": 1,
                        "pstn_sip_info": {
                            "nickname": "dodo",
                            "main_address": "+86-02187654321"
                        }
                    }
                },
                "auto_record": true,
                "assign_host_list": [
                    {
                        "user_type": 1,
                        "id": "ou_3ec3f6a28a0d08c45d895276e8e5e19b"
                    }
                ]
            }
        }
    }
}"#;
    #[test]
    fn test_resp() {
        let res = serde_json::from_str::<GetVcReserveRespInner>(RESP);
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
