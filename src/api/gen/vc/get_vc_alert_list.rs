//! doc url: <https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/vc-v1/alert/list>
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
    /// **api 版本: 2024-07-23T07:33:02+00:00**
    ///
    /// ## 获取告警记录
    ///
    /// 获取特定条件下租户的设备告警记录。
    ///

    ///
    /// doc: <https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/vc-v1/alert/list>
    ///
    /// new doc: <https://open.feishu.cn/document/server-docs/vc-v1/alert/list>
    ///
    /// api url: <https://open.feishu.cn/document_portal/v1/document/get_detail?fullPath=%2Fserver-docs%2Fvc-v1%2Falert%2Flist>
    pub async fn get_vc_alert_list(
        &self,
        req: GetVcAlertListReq,
    ) -> Result<(GetVcAlertListResp, CommonResponse), Error> {
        self.get_vc_alert_list_with_opt(req, Default::default())
            .await
    }

    /// 参见 [get_vc_alert_list](#method.get_vc_alert_list) 函数
    pub async fn get_vc_alert_list_with_opt(
        &self,
        req: GetVcAlertListReq,
        method_option: MethodOption,
    ) -> Result<(GetVcAlertListResp, CommonResponse), Error> {
        #[cfg(feature = "test-util")]
        {
            if let Some(f) = self.mock().get_mock_get_vc_alert_list(&req) {
                tracing::info!("[lark] Vc#GetVcAlertList **mocking** api");
                return f(req);
            }
        }

        tracing::info!("[lark] Vc#GetVcAlertList call api");

        let req = ApiRequest {
            scope: "Vc",
            api: "GetVcAlertList",
            method: http::Method::GET,
            url: String::new() + self.cli.open_base_url.as_ref() + "/open-apis/vc/v1/alerts",
            param_data: req.gen_param(),
            method_option,
            need_tenant_access_token: true,
            ..Default::default()
        };

        let (resp, common_resp): (GetVcAlertListRespInner, _) = self.cli.do_req(req).await?;
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
pub struct GetVcAlertListReq {
    /// 开始时间（unix时间，单位秒）
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "1608888867"
    #[api(kind = "query", name = "start_time", v_type = "var", option = "false")]
    pub start_time: String,
    /// 结束时间（unix时间，单位秒）
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "1608888867"
    #[api(kind = "query", name = "end_time", v_type = "var", option = "false")]
    pub end_time: String,
    /// 查询对象类型，不填返回所有
    ///
    /// **示例值**: "1"
    ///
    /// **可选值**:
    ///
    /// `room`: 会议室
    ///
    /// `erc`: 企业会议室连接器
    ///
    /// `sip`: SIP会议室系统
    #[api(kind = "query", name = "query_type", v_type = "var", option = "false")]
    pub query_type: i64,
    /// 查询对象ID，会议室ID或企业会议室连接器ID
    ///
    /// **示例值**: "omm_4de32cf10a4358788ff4e09e37ebbf9b"
    #[api(kind = "query", name = "query_value", v_type = "var", option = "false")]
    pub query_value: String,
    /// 请求期望返回的告警记录数量，不足则返回全部，该值默认为 100，最大为 1000
    ///
    /// **示例值**: "100"
    #[api(kind = "query", name = "page_size", v_type = "var", option = "false")]
    pub page_size: i64,
    /// 分页标记，第一次请求不填，表示从头开始遍历；分页查询结果还有更多项时会同时返回新的 page_token，下次遍历可采用该 page_token 获取查询结果
    ///
    /// **示例值**: "100"
    #[api(kind = "query", name = "page_token", v_type = "var", option = "false")]
    pub page_token: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, lark_bot_sdk_macros::ApiBaseResp)]
struct GetVcAlertListRespInner {
    #[serde(flatten)]
    data: Option<GetVcAlertListResp>,
    #[serde(flatten)]
    base: BaseResp,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct GetVcAlertListResp {
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
    /// 告警记录
    #[serde(
        rename = "items",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub items: Vec<AlertSubResp>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(default)]
pub struct AlertSubResp {
    /// 告警ID
    ///
    /// **示例值**: "7115030004018184212"
    #[serde(
        rename = "alert_id",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub alert_id: String,
    /// 触发告警规则的会议室/服务器具体的名称
    ///
    /// **示例值**: "XX层级"
    #[serde(
        rename = "resource_scope",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub resource_scope: String,
    /// 触发告警规则的监控对象
    ///
    /// **示例值**: "2"
    ///
    /// **可选值**:
    ///
    /// `rooms`: 飞书会议室
    ///
    /// `checkboard`: 飞书会议室签到板
    ///
    /// `roombox`: 飞书投屏盒子
    ///
    /// `room_tv_share`: 飞书投屏
    ///
    /// `sip`: sip会议室系统
    ///
    /// `erc`: erc节点
    ///
    /// `room_sensor`: 飞书传感器
    #[serde(
        rename = "monitor_target",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub monitor_target: i64,
    /// 告警规则的规则描述
    ///
    /// **示例值**: "连续1个周期（共1分钟），控制器电量 < 50%，则告警"
    #[serde(
        rename = "alert_strategy",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub alert_strategy: String,
    /// 告警通知发生时间（unix时间，单位秒）
    ///
    /// **示例值**: "1656914944"
    #[serde(
        rename = "alert_time",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub alert_time: String,
    /// 告警等级：严重/警告/提醒
    ///
    /// **示例值**: "2"
    ///
    /// **可选值**:
    ///
    /// `notify`: 提醒
    ///
    /// `warning`: 警告
    ///
    /// `serious`: 严重
    #[serde(
        rename = "alert_level",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub alert_level: i64,
    /// 告警联系人
    #[serde(
        rename = "contacts",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub contacts: Vec<ContactSubResp>,
    /// 通知方式
    ///
    /// **示例值**: "[0,1]"
    #[serde(
        rename = "notifyMethods",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub notify_methods: Vec<i64>,
    /// 规则名称
    ///
    /// **示例值**: "签到板断开连接"
    #[serde(
        rename = "alertRule",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub alert_rule: String,
    /// 处理时间
    ///
    /// **示例值**: "1656914944"
    #[serde(
        rename = "process_time",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub process_time: String,
    /// 恢复时间
    ///
    /// **示例值**: "1656914944"
    #[serde(
        rename = "recover_time",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub recover_time: String,
    /// 处理状态：待处理/处理中/已恢复
    ///
    /// **示例值**: "2"
    ///
    /// **可选值**:
    ///
    /// `wait_process`: 待处理（deprecated）
    ///
    /// `wait_process`: 待处理
    ///
    /// `processing`: 处理中
    ///
    /// `recover`: 已恢复（deprecated）
    ///
    /// `recover`: 已恢复
    #[serde(
        rename = "process_status",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub process_status: i64,
    /// 告警规则ID
    ///
    /// **示例值**: "100"
    #[serde(
        rename = "alert_rule_id",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub alert_rule_id: String,
    /// 触发告警规则的会议室ID，当触发告警规则的是会议室时返回该信息
    ///
    /// **示例值**: "omm_4de32cf10a4358788ff4e09e37ebbf9b"
    #[serde(
        rename = "monitor_target_room_id",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub monitor_target_room_id: String,
    /// 触发告警规则的会议室主机Mac地址，当monitor_target=1时返回该信息
    ///
    /// **示例值**: "52:60:19:9c:97:21"
    #[serde(
        rename = "monitor_target_room_mac",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub monitor_target_room_mac: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(default)]
pub struct ContactSubResp {
    /// 联系人类型
    ///
    /// **示例值**: "1"
    ///
    /// **可选值**:
    ///
    /// `user`: 用户
    ///
    /// `user_group`: 用户组
    ///
    /// `department`: 部门
    #[serde(
        rename = "contact_type",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub contact_type: i64,
    /// 联系人名
    ///
    /// **示例值**: "张三"
    #[serde(
        rename = "contact_name",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub contact_name: String,
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
        Fn(GetVcAlertListReq) -> Result<(GetVcAlertListResp, CommonResponse), Error>
        + Send
        + Sync
        + 'static
    {
    }
    impl<
            T: Fn(GetVcAlertListReq) -> Result<(GetVcAlertListResp, CommonResponse), Error>
                + Send
                + Sync
                + 'static,
        > MockFunc for T
    {
    }

    impl<'c, IStore: Store, IClient: HttpClient> VcServiceMocker<'c, IStore, IClient> {
        pub fn mock_get_vc_alert_list<F: MockFunc>(
            &self,
            f: F,
        ) -> MockerBuilder<Mocker, GetVcAlertListReq, GetVcAlertListResp, Arc<dyn MockFunc>>
        {
            MockerBuilder::new(self.cli.instance_id, Arc::new(f))
        }

        pub(super) fn get_mock_get_vc_alert_list(
            &self,
            req: &GetVcAlertListReq,
        ) -> Option<Arc<dyn MockFunc>> {
            do_mock::<Mocker, GetVcAlertListReq, GetVcAlertListResp, Arc<dyn MockFunc>>(
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
        api::gen::vc::get_vc_alert_list::{
            GetVcAlertListReq, GetVcAlertListResp, GetVcAlertListRespInner,
        },
        core::{model::CommonResponse, Lark},
    };

    #[tokio::test]
    async fn test_mock() {
        let lark = Lark::new("".to_owned(), "".into());
        let mocker = lark
            .vc()
            .mock()
            .mock_get_vc_alert_list(|_| {
                Ok((GetVcAlertListResp::default(), CommonResponse::default()))
            })
            .build();
        let res = lark
            .vc()
            .get_vc_alert_list(GetVcAlertListReq::default())
            .await;
        assert!(res.is_ok());
        mocker.clear();
        let res = lark
            .vc()
            .get_vc_alert_list(GetVcAlertListReq::default())
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
        "has_more": true,
        "page_token": "50",
        "items": [
            {
                "alert_id": "7115030004018184212",
                "resource_scope": "XX层级",
                "monitor_target": 2,
                "alert_strategy": "连续1个周期（共1分钟），控制器电量 < 50%，则告警",
                "alert_time": "1656914944",
                "alert_level": 2,
                "contacts": [
                    {
                        "contact_type": 1,
                        "contact_name": "张三"
                    }
                ],
                "notifyMethods": [
                        0,
                        1
                ],
                "alertRule": "签到板断开连接",
                "process_time": "1656914944",
                "recover_time": "1656914944",
                "process_status": 2,
                "alert_rule_id": "100",
                "monitor_target_room_id": "omm_4de32cf10a4358788ff4e09e37ebbf9b",
                "monitor_target_room_mac": "52:60:19:9c:97:21"
            }
        ]
    }
}"#;
    #[test]
    fn test_resp() {
        let res = serde_json::from_str::<GetVcAlertListRespInner>(RESP);
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
