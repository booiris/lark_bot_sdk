//! doc url: <https://open.larkoffice.com/document/uAjLw4CM/ukTMukTMukTM/reference/attendance-v1/user_task_remedy/query>
// Code generated by gen_api. DO NOT EDIT.

use serde::{Deserialize, Serialize};

use crate::api::BaseResp;
use crate::api::HasBaseResp;
use crate::core::model::{ApiRequest, CommonResponse, MethodOption, ReqParam, StreamReqParam};
use crate::{
    core::{http_client::HttpClient, store::Store},
    error::Error,
};

use crate::api::gen::attendance::AttendanceService;

impl<'c, IStore: Store, IClient: HttpClient> AttendanceService<'c, IStore, IClient> {
    /// **api 版本: 2024-07-25T08:30:16+00:00**
    ///
    /// ## 获取补卡记录
    ///
    /// 补卡：用户通过审批的方式，在某一次上/下班的打卡时间范围内，补充一条打卡记录，用以修正用户的考勤结果。本接口专用于获取员工的补卡记录（无页面功能对应）
    ///

    ///
    /// doc: <https://open.larkoffice.com/document/uAjLw4CM/ukTMukTMukTM/reference/attendance-v1/user_task_remedy/query>
    ///
    /// new doc: <https://open.larkoffice.com/document/server-docs/attendance-v1/user_task_remedy/query>
    ///
    /// api url: <https://open.feishu.cn/document_portal/v1/document/get_detail?fullPath=%2Fserver-docs%2Fattendance-v1%2Fuser_task_remedy%2Fquery>
    pub async fn get_attendance_user_task_remedy(
        &self,
        req: GetAttendanceUserTaskRemedyReq,
    ) -> Result<(GetAttendanceUserTaskRemedyResp, CommonResponse), Error> {
        self.get_attendance_user_task_remedy_with_opt(req, Default::default())
            .await
    }

    /// 参见 [get_attendance_user_task_remedy](#method.get_attendance_user_task_remedy) 函数
    pub async fn get_attendance_user_task_remedy_with_opt(
        &self,
        req: GetAttendanceUserTaskRemedyReq,
        method_option: MethodOption,
    ) -> Result<(GetAttendanceUserTaskRemedyResp, CommonResponse), Error> {
        #[cfg(feature = "test-util")]
        {
            if let Some(f) = self.mock().get_mock_get_attendance_user_task_remedy(&req) {
                tracing::info!("[lark] Attendance#GetAttendanceUserTaskRemedy **mocking** api");
                return f(req);
            }
        }

        tracing::info!("[lark] Attendance#GetAttendanceUserTaskRemedy call api");

        let req = ApiRequest {
            scope: "Attendance",
            api: "GetAttendanceUserTaskRemedy",
            method: http::Method::POST,
            url: String::new()
                + self.cli.open_base_url.as_ref()
                + "/open-apis/attendance/v1/user_task_remedys/query",
            param_data: req.gen_param(),
            method_option,
            need_tenant_access_token: true,
            ..Default::default()
        };

        let (resp, common_resp): (GetAttendanceUserTaskRemedyRespInner, _) =
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
pub struct GetAttendanceUserTaskRemedyReq {
    /// 请求体中的 user_ids 和响应体中的 user_id 的员工ID类型。如果没有后台管理权限，可使用[通过手机号或邮箱获取用户 ID](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/contact-v3/user/batch_get_id)
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "employee_id"
    ///
    /// **可选值**:
    ///
    /// `employee_id`: 员工 employee ID，即[飞书管理后台](https://example.feishu.cn/admin/contacts/departmentanduser) > 组织架构 > 成员与部门 > 成员详情中的用户 ID
    ///
    /// `employee_no`: 员工工号，即[飞书管理后台](https://example.feishu.cn/admin/contacts/departmentanduser) > 组织架构 > 成员与部门 > 成员详情中的工号
    #[api(
        kind = "query",
        name = "employee_type",
        v_type = "var",
        option = "false"
    )]
    pub employee_type: String,
    /// employee_no 或 employee_id 列表。传入的ID类型需要与employee_type的取值一致
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "["abd754f7"]"
    #[api(kind = "body", name = "user_ids")]
    pub user_ids: Vec<Option<String>>,
    /// 查询的起始时间，精确到秒的时间戳
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "1566641088"
    #[api(kind = "body", name = "check_time_from")]
    pub check_time_from: String,
    /// 查询的结束时间，精确到秒的时间戳
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "1592561088"
    #[api(kind = "body", name = "check_time_to")]
    pub check_time_to: String,
    /// 查询依据的时间类型（默认依据PeriodTime，如果使用非默认的，非特定租户不支持）
    ///
    /// **示例值**: "PeriodTime"
    ///
    /// **可选值**:
    ///
    /// `PeriodTime`: 单据作用时间
    ///
    /// `CreateTime`: 单据创建时间（目前暂不支持）
    ///
    /// `UpdateTime`: 单据状态更新时间（灰度中，暂不开放）
    #[api(kind = "body", name = "check_date_type")]
    pub check_date_type: Option<String>,
    /// 查询状态（不填默认查询已通过状态）
    ///
    /// **示例值**: "2"
    ///
    /// **可选值**:
    ///
    /// `Pending`: 待审批
    ///
    /// `Rejected`: 未通过
    ///
    /// `Pass`: 已通过
    ///
    /// `Cancel`: 已取消
    ///
    /// `Withdraw`: 已撤回
    #[api(kind = "body", name = "status")]
    pub status: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize, Clone, lark_bot_sdk_macros::ApiBaseResp)]
struct GetAttendanceUserTaskRemedyRespInner {
    #[serde(flatten)]
    data: Option<GetAttendanceUserTaskRemedyResp>,
    #[serde(flatten)]
    base: BaseResp,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct GetAttendanceUserTaskRemedyResp {
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
    /// 补卡记录列表
    #[serde(
        rename = "user_remedys",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub user_remedys: Vec<UserTaskRemedySubResp>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(default)]
pub struct UserTaskRemedySubResp {
    /// 用户 ID，对应employee_type
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "abd754f7"
    #[serde(
        rename = "user_id",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub user_id: String,
    /// 补卡日期，格式为yyyyMMdd
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "20210701"
    #[serde(
        rename = "remedy_date",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub remedy_date: i64,
    /// 第几次上下班，0：第 1 次上下班，1：第 2 次上下班，2：第 3 次上下班，自由班制填 0
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "0"
    #[serde(
        rename = "punch_no",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub punch_no: i64,
    /// 上班 / 下班，1：上班，2：下班，自由班制为 1
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "1"
    #[serde(
        rename = "work_type",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub work_type: i64,
    /// 审批 ID，可用于[通知审批状态更新](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/attendance-v1/approval_info/process)
    ///
    /// **示例值**: "6737202939523236113"
    #[serde(
        rename = "approval_id",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub approval_id: String,
    /// 补卡时间，时间格式为 yyyy-MM-dd HH:mm
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "2021-07-01 08:00"
    #[serde(
        rename = "remedy_time",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub remedy_time: String,
    /// 补卡状态
    ///
    /// **示例值**: "2"
    ///
    /// **可选值**:
    ///
    /// `Pending`: 审批中
    ///
    /// `Rejected`: 未通过
    ///
    /// `Pass`: 已通过
    ///
    /// `Cancel`: 已取消
    ///
    /// `Withdraw`: 通过后撤回
    #[serde(
        rename = "status",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub status: i64,
    /// 补卡原因
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "忘记打卡"
    #[serde(
        rename = "reason",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub reason: String,
    /// 补卡时间，精确到秒的时间戳
    ///
    /// **示例值**: "1611476284"
    #[serde(
        rename = "time",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub time: String,
    /// 补卡时考勤组时区
    ///
    /// **示例值**: "Asia/Shanghai"
    #[serde(
        rename = "time_zone",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub time_zone: String,
    /// 补卡发起时间，精确到秒的时间戳
    ///
    /// **示例值**: "1611476284"
    #[serde(
        rename = "create_time",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub create_time: String,
    /// 补卡状态更新时间，精确到秒的时间戳
    ///
    /// **示例值**: "1611476284"
    #[serde(
        rename = "update_time",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub update_time: String,
}

#[cfg(feature = "test-util")]
mod test_utils {
    use std::sync::Arc;

    use self::gen::attendance::AttendanceServiceMocker;
    use crate::core::mocker::*;

    use super::*;
    use crate::api::*;
    use crate::core::model::*;

    pub struct Mocker;

    pub trait MockFunc:
        Fn(
            GetAttendanceUserTaskRemedyReq,
        ) -> Result<(GetAttendanceUserTaskRemedyResp, CommonResponse), Error>
        + Send
        + Sync
        + 'static
    {
    }
    impl<
            T: Fn(
                    GetAttendanceUserTaskRemedyReq,
                )
                    -> Result<(GetAttendanceUserTaskRemedyResp, CommonResponse), Error>
                + Send
                + Sync
                + 'static,
        > MockFunc for T
    {
    }

    impl<'c, IStore: Store, IClient: HttpClient> AttendanceServiceMocker<'c, IStore, IClient> {
        pub fn mock_get_attendance_user_task_remedy<F: MockFunc>(
            &self,
            f: F,
        ) -> MockerBuilder<
            Mocker,
            GetAttendanceUserTaskRemedyReq,
            GetAttendanceUserTaskRemedyResp,
            Arc<dyn MockFunc>,
        > {
            MockerBuilder::new(self.cli.instance_id, Arc::new(f))
        }

        pub(super) fn get_mock_get_attendance_user_task_remedy(
            &self,
            req: &GetAttendanceUserTaskRemedyReq,
        ) -> Option<Arc<dyn MockFunc>> {
            do_mock::<
                Mocker,
                GetAttendanceUserTaskRemedyReq,
                GetAttendanceUserTaskRemedyResp,
                Arc<dyn MockFunc>,
            >(self.cli.instance_id, req)
        }
    }
}

#[cfg(feature = "test-util")]
#[cfg(test)]
mod test {
    use crate::{
        api::gen::attendance::get_attendance_user_task_remedy::{
            GetAttendanceUserTaskRemedyReq, GetAttendanceUserTaskRemedyResp,
            GetAttendanceUserTaskRemedyRespInner,
        },
        core::{model::CommonResponse, Lark},
    };

    #[tokio::test]
    async fn test_mock() {
        let lark = Lark::new("".to_owned(), "".into());
        let mocker = lark
            .attendance()
            .mock()
            .mock_get_attendance_user_task_remedy(|_| {
                Ok((
                    GetAttendanceUserTaskRemedyResp::default(),
                    CommonResponse::default(),
                ))
            })
            .build();
        let res = lark
            .attendance()
            .get_attendance_user_task_remedy(GetAttendanceUserTaskRemedyReq::default())
            .await;
        assert!(res.is_ok());
        mocker.clear();
        let res = lark
            .attendance()
            .get_attendance_user_task_remedy(GetAttendanceUserTaskRemedyReq::default())
            .await;
        assert!(res.is_err());
    }

    const REQ: &str = r#"{
    "user_ids": [
        "abd754f7"
    ],
    "check_time_from": "1566641088",
    "check_time_to": "1592561088",
    "status": 2
}"#;

    #[test]
    fn test_req() {
        if REQ == "{}" {
            return;
        }
        if let Err(e) = serde_json::from_str::<super::GetAttendanceUserTaskRemedyReqBody>(REQ) {
            panic!("{}", e);
        }
    }

    const RESP: &str = r#"{
    "code": 0,
    "msg": "success",
    "data": {
        "user_remedys": [
            {
                "user_id": "abd754f7",
                "remedy_date": 20210701,
                "punch_no": 0,
                "work_type": 1,
                "approval_id": "6737202939523236113",
                "remedy_time": "2021-07-01 08:00",
                "status": 2,
                "reason": "忘记打卡",
                "time": "1611476284",
                "time_zone": "Asia/Shanghai",
                "create_time": "1611476284",
                "update_time": "1611476284"
            }
        ]
    }
}"#;
    #[test]
    fn test_resp() {
        let res = serde_json::from_str::<GetAttendanceUserTaskRemedyRespInner>(RESP);
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
