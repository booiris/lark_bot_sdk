//! doc url: <https://open.larkoffice.com/document/uAjLw4CM/ukTMukTMukTM/reference/vc-v1/reserve_config/patch>
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
    /// **api 版本: 2024-07-23T07:33:00+00:00**
    ///
    /// ## 更新会议室预定限制
    ///
    /// 更新会议室预定限制。
    ///

    ///
    /// doc: <https://open.larkoffice.com/document/uAjLw4CM/ukTMukTMukTM/reference/vc-v1/reserve_config/patch>
    ///
    /// new doc: <https://open.larkoffice.com/document/server-docs/vc-v1/scope_config/patch>
    ///
    /// api url: <https://open.feishu.cn/document_portal/v1/document/get_detail?fullPath=%2Fserver-docs%2Fvc-v1%2Fscope_config%2Fpatch>
    pub async fn update_vc_reserve_config(
        &self,
        req: UpdateVcReserveConfigReq,
    ) -> Result<(UpdateVcReserveConfigResp, CommonResponse), Error> {
        self.update_vc_reserve_config_with_opt(req, Default::default())
            .await
    }

    /// 参见 [update_vc_reserve_config](#method.update_vc_reserve_config) 函数
    pub async fn update_vc_reserve_config_with_opt(
        &self,
        req: UpdateVcReserveConfigReq,
        method_option: MethodOption,
    ) -> Result<(UpdateVcReserveConfigResp, CommonResponse), Error> {
        #[cfg(feature = "test-util")]
        {
            if let Some(f) = self.mock().get_mock_update_vc_reserve_config(&req) {
                tracing::info!("[lark] Vc#UpdateVcReserveConfig **mocking** api");
                return f(req);
            }
        }

        tracing::info!("[lark] Vc#UpdateVcReserveConfig call api");

        let req = ApiRequest {
            scope: "Vc",
            api: "UpdateVcReserveConfig",
            method: http::Method::PATCH,
            url: String::new()
                + self.cli.open_base_url.as_ref()
                + "/open-apis/vc/v1/reserve_configs/:reserve_config_id",
            param_data: req.gen_param(),
            method_option,
            need_tenant_access_token: true,
            ..Default::default()
        };

        let (resp, common_resp): (UpdateVcReserveConfigRespInner, _) = self.cli.do_req(req).await?;
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
pub struct UpdateVcReserveConfigReq {
    /// 会议室或层级id
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "omm_3c5dd7e09bac0c1758fcf9511bd1a771"
    #[api(kind = "path", name = "reserve_config_id")]
    pub reserve_config_id: String,
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
    /// 1 代表层级，2 代表会议室
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "2"
    #[api(kind = "body", name = "scope_type")]
    pub scope_type: String,
    /// 预定审批设置
    #[api(kind = "body", name = "approval_config")]
    pub approval_config: Option<ApprovalConfigSubReq>,
    /// 预定时间设置
    #[api(kind = "body", name = "time_config")]
    pub time_config: Option<TimeConfigSubReq>,
    /// 预定范围设置
    #[api(kind = "body", name = "reserve_scope_config")]
    pub reserve_scope_config: Option<ReserveScopeConfigSubReq>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(default)]
pub struct ApprovalConfigSubReq {
    /// 预定审批开关：0 代表关闭，1 代表打开。<br>
    ///
    /// <b>说明</b>：<br>
    ///
    /// 1.  未设置值时不更新原开关的值，但此时必填  approval_condition<br>
    ///
    /// 2.  设置值为 1 时，必填  approval_condition<br>
    ///
    /// 3.  设置值为 0 时整个
    ///
    /// approval_config 其他字段均可省略。
    ///
    /// **示例值**: "1"
    #[serde(
        rename = "approval_switch",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub approval_switch: Option<i64>,
    /// 预定审批条件：0 代表所有预定均需审批，1 代表满足条件的需审批<br>
    ///
    /// <b>说明</b>：为 1 时必填 meeting_duration
    ///
    /// **示例值**: "1"
    #[serde(
        rename = "approval_condition",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub approval_condition: Option<i64>,
    /// 超过 meeting_duration
    ///
    /// 的预定需要审批（单位：小时，取值范围[0.1-99]）
    ///
    /// <b>说明</b>：<br>
    ///
    /// 1.  当 approval_condition
    ///
    /// 为 0 ，更新时如果未设置值，默认更新为 99 .<br>
    ///
    /// 2.  传入的值小数点后超过 2 位，自动四舍五入保留两位。
    ///
    /// **示例值**: "3"
    #[serde(
        rename = "meeting_duration",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub meeting_duration: Option<f64>,
    /// 审批人列表，当打开审批开关时，至少需要设置一位审批人
    ///
    /// **示例值**: "[{user_id:"ou_e8bce6c3935ef1fc1b432992fd9d3db8"}]"
    #[serde(
        rename = "approvers",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub approvers: Vec<Option<SubscribeUserSubReq>>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(default)]
pub struct TimeConfigSubReq {
    /// 是否覆盖子层级及会议室
    ///
    /// **示例值**: "true"
    #[serde(
        rename = "if_cover_child_scope",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub if_cover_child_scope: Option<bool>,
    /// 预定时间开关：0 代表关闭，1 代表开启
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "1"
    #[serde(
        rename = "time_switch",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub time_switch: i64,
    /// 最早可提前
    ///
    /// days_in_advance 预定会议室（单位：天，取值范围[1-730]）<br>
    ///
    /// <b>说明</b>：不填写时，默认更新为 365
    ///
    /// **示例值**: "30"
    #[serde(
        rename = "days_in_advance",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub days_in_advance: Option<i64>,
    /// 开放当天可于
    ///
    /// opening_hour 开始预定（单位：秒，取值范围[0,86400]）<br>
    ///
    /// <b>说明</b>：<br>
    ///
    /// 1.  不填写时默认更新为
    ///
    /// 28800 <br>
    ///
    /// 2.  如果填写的值不是 60
    ///
    /// 的倍数，则自动会更新为离其最近的 60 整数倍的值。
    ///
    /// **示例值**: "27900"
    #[serde(
        rename = "opening_hour",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub opening_hour: Option<String>,
    /// 每日可预定时间范围的开始时间（单位：秒，取值范围[0,86400]）<br>
    ///
    /// <b>说明</b>：<br>
    ///
    /// 1.  不填写时，默认更新为 0 ，此时填写的  end_time 不得小于 30。<br>
    ///
    /// 2.  当 start_time 与
    ///
    /// end_time 均填写时，
    ///
    /// end_time 至少超过
    ///
    /// start_time 30 。<br>
    ///
    /// 3.  如果填写的值不是 60 的倍数，则自动会更新为离其最近的 60 整数倍的值。
    ///
    /// **示例值**: "0"
    #[serde(
        rename = "start_time",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub start_time: Option<String>,
    /// 每日可预定时间范围结束时间（单位：秒，取值范围[0,86400]）<br>
    ///
    /// <b>说明</b>：<br>
    ///
    /// 1.  不填写时，默认更新为 86400 ，此时填写的
    ///
    /// start_time 不得大于等于 86370 。<br>
    ///
    /// 2.  当 start_time 与
    ///
    /// end_time 均填写时，
    ///
    /// end_time 至少要超过
    ///
    /// start_time 30。<br>
    ///
    /// 3.  如果填写的值不是  60 的倍数，则自动会更新为离其最近的 60 整数倍的值。
    ///
    /// **示例值**: "86400"
    #[serde(
        rename = "end_time",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub end_time: Option<String>,
    /// 单次会议室可预定时长上限（单位：小时，取值范围[1,99]）<br>
    ///
    /// <b>说明</b>：不填写时默认更新为 2
    ///
    /// **示例值**: "24"
    #[serde(
        rename = "max_duration",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub max_duration: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(default)]
pub struct ReserveScopeConfigSubReq {
    /// 是否覆盖子层级及会议室
    ///
    /// **示例值**: "true"
    #[serde(
        rename = "if_cover_child_scope",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub if_cover_child_scope: Option<bool>,
    /// 可预定成员范围：0 代表部分成员，1 代表全部成员。<br>
    ///
    /// <b>说明</b>：<br>
    ///
    /// 1.  此值必填。
    ///
    /// 2.  当设置为 0 时，至少需要 1 个预定部门或预定人
    ///
    /// **示例值**: "0"
    #[serde(
        rename = "allow_all_users",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub allow_all_users: Option<i64>,
    /// 可预定成员列表
    ///
    /// **示例值**: "[{user_id:"ou_e8bce6c3935ef1fc1b432992fd9d3db8"}]"
    #[serde(
        rename = "allow_users",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub allow_users: Vec<Option<SubscribeUserSubReq>>,
    /// 可预定部门列表
    ///
    /// **示例值**: "[{department_id:"od-5c07f0c117cf8795f25610a69363ce31"}]"
    #[serde(
        rename = "allow_depts",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub allow_depts: Vec<Option<SubscribeDepartmentSubReq>>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(default)]
pub struct SubscribeUserSubReq {
    /// 预定管理员ID
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "ou_a27b07a9071d90577c0177bcec98f856"
    #[serde(
        rename = "user_id",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub user_id: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(default)]
pub struct SubscribeDepartmentSubReq {
    /// 预定管理部门ID，使用open_department_id
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "od-47d8b570b0a011e9679a755efcc5f61a"
    #[serde(
        rename = "department_id",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub department_id: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, lark_bot_sdk_macros::ApiBaseResp)]
struct UpdateVcReserveConfigRespInner {
    #[serde(flatten)]
    data: Option<UpdateVcReserveConfigResp>,
    #[serde(flatten)]
    base: BaseResp,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct UpdateVcReserveConfigResp {
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

    use self::gen::vc::VcServiceMocker;
    use crate::core::mocker::*;

    use super::*;
    use crate::api::*;
    use crate::core::model::*;

    pub struct Mocker;

    pub trait MockFunc:
        Fn(UpdateVcReserveConfigReq) -> Result<(UpdateVcReserveConfigResp, CommonResponse), Error>
        + Send
        + Sync
        + 'static
    {
    }
    impl<
            T: Fn(
                    UpdateVcReserveConfigReq,
                ) -> Result<(UpdateVcReserveConfigResp, CommonResponse), Error>
                + Send
                + Sync
                + 'static,
        > MockFunc for T
    {
    }

    impl<'c, IStore: Store, IClient: HttpClient> VcServiceMocker<'c, IStore, IClient> {
        pub fn mock_update_vc_reserve_config<F: MockFunc>(
            &self,
            f: F,
        ) -> MockerBuilder<
            Mocker,
            UpdateVcReserveConfigReq,
            UpdateVcReserveConfigResp,
            Arc<dyn MockFunc>,
        > {
            MockerBuilder::new(self.cli.instance_id, Arc::new(f))
        }

        pub(super) fn get_mock_update_vc_reserve_config(
            &self,
            req: &UpdateVcReserveConfigReq,
        ) -> Option<Arc<dyn MockFunc>> {
            do_mock::<Mocker, UpdateVcReserveConfigReq, UpdateVcReserveConfigResp, Arc<dyn MockFunc>>(
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
        api::gen::vc::update_vc_reserve_config::{
            UpdateVcReserveConfigReq, UpdateVcReserveConfigResp, UpdateVcReserveConfigRespInner,
        },
        core::{model::CommonResponse, Lark},
    };

    #[tokio::test]
    async fn test_mock() {
        let lark = Lark::new("".to_owned(), "".into());
        let mocker = lark
            .vc()
            .mock()
            .mock_update_vc_reserve_config(|_| {
                Ok((
                    UpdateVcReserveConfigResp::default(),
                    CommonResponse::default(),
                ))
            })
            .build();
        let res = lark
            .vc()
            .update_vc_reserve_config(UpdateVcReserveConfigReq::default())
            .await;
        assert!(res.is_ok());
        mocker.clear();
        let res = lark
            .vc()
            .update_vc_reserve_config(UpdateVcReserveConfigReq::default())
            .await;
        assert!(res.is_err());
    }

    const REQ: &str = r#"{
    "scope_type": "2",
    "approval_config": {
        "approval_switch": 1,
        "approval_condition": 1,
        "meeting_duration": 3,
        "approvers": [
            {
                "user_id": "ou_a27b07a9071d90577c0177bcec98f856"
            }
        ]
    },
    "time_config": {
        "if_cover_child_scope": true,
        "time_switch": 1,
        "days_in_advance": 30,
        "opening_hour": "27900",
        "start_time": "0",
        "end_time": "86400",
        "max_duration": 24
    },
    "reserve_scope_config": {
        "if_cover_child_scope": true,
        "allow_all_users": 0,
        "allow_users": [
            {
                "user_id": "ou_a27b07a9071d90577c0177bcec98f856"
            }
        ],
        "allow_depts": [
            {
                "department_id": "od-47d8b570b0a011e9679a755efcc5f61a"
            }
        ]
    }
}"#;

    #[test]
    fn test_req() {
        if REQ == "{}" {
            return;
        }
        if let Err(e) = serde_json::from_str::<super::UpdateVcReserveConfigReqBody>(REQ) {
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
        let res = serde_json::from_str::<UpdateVcReserveConfigRespInner>(RESP);
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
