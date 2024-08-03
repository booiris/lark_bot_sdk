//! doc url: <https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/attendance-v1/user_stats_data/query>
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
    /// **api 版本: 2024-07-24T07:41:52+00:00**
    ///
    /// ## 查询统计数据
    ///
    /// 查询日度统计或月度统计的统计数据。字段包含基本信息、考勤组信息、出勤统计、异常统计、请假统计、加班统计、打卡时间、考勤结果和自定义字段。具体报表可在考勤统计-[报表](https://example.feishu.cn/people/workforce-management/manage/statistics/report)中找到
    ///
    /// * 当天不在考勤组时没有相关统计数据
    ///
    /// doc: <https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/attendance-v1/user_stats_data/query>
    ///
    /// new doc: <https://open.feishu.cn/document/server-docs/attendance-v1/user_stats_data/query-3>
    ///
    /// api url: <https://open.feishu.cn/document_portal/v1/document/get_detail?fullPath=%2Fserver-docs%2Fattendance-v1%2Fuser_stats_data%2Fquery-3>
    pub async fn get_attendance_user_stats_data(
        &self,
        req: GetAttendanceUserStatsDataReq,
    ) -> Result<(GetAttendanceUserStatsDataResp, CommonResponse), Error> {
        self.get_attendance_user_stats_data_with_opt(req, Default::default())
            .await
    }

    /// 参见 [get_attendance_user_stats_data](#method.get_attendance_user_stats_data) 函数
    pub async fn get_attendance_user_stats_data_with_opt(
        &self,
        req: GetAttendanceUserStatsDataReq,
        method_option: MethodOption,
    ) -> Result<(GetAttendanceUserStatsDataResp, CommonResponse), Error> {
        #[cfg(feature = "test-util")]
        {
            if let Some(f) = self.mock().get_mock_get_attendance_user_stats_data(&req) {
                tracing::info!("[lark] Attendance#GetAttendanceUserStatsData **mocking** api");
                return f(req);
            }
        }

        tracing::info!("[lark] Attendance#GetAttendanceUserStatsData call api");

        let req = ApiRequest {
            scope: "Attendance",
            api: "GetAttendanceUserStatsData",
            method: http::Method::POST,
            url: String::new()
                + self.cli.open_base_url.as_ref()
                + "/open-apis/attendance/v1/user_stats_datas/query",
            param_data: req.gen_param(),
            method_option,
            need_tenant_access_token: true,
            ..Default::default()
        };

        let (resp, common_resp): (GetAttendanceUserStatsDataRespInner, _) =
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
pub struct GetAttendanceUserStatsDataReq {
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
    /// 语言类型
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "zh"
    ///
    /// **可选值**:
    ///
    /// `en`: 英语
    ///
    /// `ja`: 日语
    ///
    /// `zh`: 中文
    #[api(kind = "body", name = "locale")]
    pub locale: String,
    /// 统计类型
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "month"
    ///
    /// **可选值**:
    ///
    /// `daily`: 日度统计
    ///
    /// `month`: 月度统计
    #[api(kind = "body", name = "stats_type")]
    pub stats_type: String,
    /// 开始时间，格式yyyyMMdd
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "20210316"
    #[api(kind = "body", name = "start_date")]
    pub start_date: i64,
    /// 结束时间，格式yyyyMMdd
    ///
    /// （时间间隔不超过 31 天）
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "20210323"
    #[api(kind = "body", name = "end_date")]
    pub end_date: i64,
    /// 查询的用户 ID 列表，与employee_type对应
    ///
    /// （用户数量不超过 200）
    ///
    /// * 必填字段(已全部升级到新系统，新系统要求必填)
    ///
    /// **示例值**: "[
    ///
    /// "ec8ddg56",
    ///
    /// "4dbb52f2",
    ///
    /// "4167842e"
    ///
    /// ]"
    #[api(kind = "body", name = "user_ids")]
    pub user_ids: Vec<Option<String>>,
    /// 是否包含离职人员和转出人员，默认为false不包含
    ///
    /// **示例值**: "true"
    #[api(kind = "body", name = "need_history")]
    pub need_history: Option<bool>,
    /// * `true`：只展示员工当前所属考勤组数据
    ///
    /// * `false`：展示员工所有考勤组数据<br>默认值：false
    ///
    /// **示例值**: "true"
    #[api(kind = "body", name = "current_group_only")]
    pub current_group_only: Option<bool>,
    /// 操作者的 user_id。与employee_type对应
    ///
    /// * 不同的操作者（管理员）的每个报表可能有不同的字段设置，系统将根据 user_id 查询指定报表的统计数据。
    ///
    /// * 必填字段（已全部升级到新系统，新系统要求该字段必填）。
    ///
    /// **示例值**: "ec8ddg56"
    #[api(kind = "body", name = "user_id")]
    pub user_id: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, lark_bot_sdk_macros::ApiBaseResp)]
struct GetAttendanceUserStatsDataRespInner {
    #[serde(flatten)]
    data: Option<GetAttendanceUserStatsDataResp>,
    #[serde(flatten)]
    base: BaseResp,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct GetAttendanceUserStatsDataResp {
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
    /// 用户统计数据（限制1000条，超过1000条会截断）
    #[serde(
        rename = "user_datas",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub user_datas: Vec<UserStatsDataSubResp>,
    /// 无权限获取的用户列表，与employee_type对应
    #[serde(
        rename = "invalid_user_list",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub invalid_user_list: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(default)]
pub struct UserStatsDataSubResp {
    /// 用户姓名
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "小李"
    #[serde(
        rename = "name",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub name: String,
    /// 用户 ID
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "ec8ddg56"
    #[serde(
        rename = "user_id",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub user_id: String,
    /// 用户的统计数据，code信息对应[查询统计表头](https://open.larkoffice.com/document/server-docs/attendance-v1/user_stats_data/query-2)
    #[serde(
        rename = "datas",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub datas: Vec<UserStatsDataCellSubResp>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(default)]
pub struct UserStatsDataCellSubResp {
    /// 字段编号
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "50102"
    #[serde(
        rename = "code",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub code: String,
    /// 数据值
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "无需打卡(-), 无需打卡(-)"
    #[serde(
        rename = "value",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub value: String,
    /// 数据属性
    #[serde(
        rename = "features",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub features: Vec<UserStatsDataFeatureSubResp>,
    /// 字段标题
    ///
    /// **示例值**: "姓名"
    #[serde(
        rename = "title",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub title: String,
    /// 时长，这个字段是一个map，key位时间单位，value为对应的时长值
    #[serde(
        rename = "duration_num",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub duration_num: UserStatsDataDurationSubResp,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(default)]
pub struct UserStatsDataDurationSubResp {
    /// 天
    ///
    /// **示例值**: "1"
    #[serde(
        rename = "day",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub day: String,
    /// 半天
    ///
    /// **示例值**: "1"
    #[serde(
        rename = "half_day",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub half_day: String,
    /// 小时
    ///
    /// **示例值**: "1"
    #[serde(
        rename = "hour",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub hour: String,
    /// 半小时
    ///
    /// **示例值**: "1"
    #[serde(
        rename = "half_hour",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub half_hour: String,
    /// 分钟
    ///
    /// **示例值**: "1"
    #[serde(
        rename = "minute",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub minute: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(default)]
pub struct UserStatsDataFeatureSubResp {
    /// 统计数据列附加属性的名称
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "Abnormal"
    #[serde(
        rename = "key",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub key: String,
    /// 统计数据列附加属性的值。
    ///
    /// * 先展示上下班的打卡结果，再展示假勤申请时间(如果有)
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "false"
    #[serde(
        rename = "value",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub value: String,
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
            GetAttendanceUserStatsDataReq,
        ) -> Result<(GetAttendanceUserStatsDataResp, CommonResponse), Error>
        + Send
        + Sync
        + 'static
    {
    }
    impl<
            T: Fn(
                    GetAttendanceUserStatsDataReq,
                )
                    -> Result<(GetAttendanceUserStatsDataResp, CommonResponse), Error>
                + Send
                + Sync
                + 'static,
        > MockFunc for T
    {
    }

    impl<'c, IStore: Store, IClient: HttpClient> AttendanceServiceMocker<'c, IStore, IClient> {
        pub fn mock_get_attendance_user_stats_data<F: MockFunc>(
            &self,
            f: F,
        ) -> MockerBuilder<
            Mocker,
            GetAttendanceUserStatsDataReq,
            GetAttendanceUserStatsDataResp,
            Arc<dyn MockFunc>,
        > {
            MockerBuilder::new(self.cli.instance_id, Arc::new(f))
        }

        pub(super) fn get_mock_get_attendance_user_stats_data(
            &self,
            req: &GetAttendanceUserStatsDataReq,
        ) -> Option<Arc<dyn MockFunc>> {
            do_mock::<
                Mocker,
                GetAttendanceUserStatsDataReq,
                GetAttendanceUserStatsDataResp,
                Arc<dyn MockFunc>,
            >(self.cli.instance_id, req)
        }
    }
}

#[cfg(feature = "test-util")]
#[cfg(test)]
mod test {
    use crate::{
        api::gen::attendance::get_attendance_user_stats_data::{
            GetAttendanceUserStatsDataReq, GetAttendanceUserStatsDataResp,
            GetAttendanceUserStatsDataRespInner,
        },
        core::{model::CommonResponse, Lark},
    };

    #[tokio::test]
    async fn test_mock() {
        let lark = Lark::new("".to_owned(), "".into());
        let mocker = lark
            .attendance()
            .mock()
            .mock_get_attendance_user_stats_data(|_| {
                Ok((
                    GetAttendanceUserStatsDataResp::default(),
                    CommonResponse::default(),
                ))
            })
            .build();
        let res = lark
            .attendance()
            .get_attendance_user_stats_data(GetAttendanceUserStatsDataReq::default())
            .await;
        assert!(res.is_ok());
        mocker.clear();
        let res = lark
            .attendance()
            .get_attendance_user_stats_data(GetAttendanceUserStatsDataReq::default())
            .await;
        assert!(res.is_err());
    }

    const REQ: &str = r#"{
    "locale": "zh",
    "stats_type": "month",
    "start_date": 20210316,
    "end_date": 20210323,
    "user_ids": [
        "ec8ddg56",
        "af5ddg73"
    ],
    "need_history": true,
    "current_group_only": true,
    "user_id": "ec8ddg56"
}"#;

    #[test]
    fn test_req() {
        if REQ == "{}" {
            return;
        }
        if let Err(e) = serde_json::from_str::<super::GetAttendanceUserStatsDataReqBody>(REQ) {
            panic!("{}", e);
        }
    }

    const RESP: &str = r#"{
    "code": 0,
    "msg": "success",
    "data": {
        "user_datas": [
            {
                "name": "小李",
                "user_id": "ec8ddg56",
                "datas": [
                    {
                        "code": "50102",
                        "value": "无需打卡(-), 无需打卡(-)",
                        "features": [
                            {
                                "key": "Abnormal",
                                "value": "false"
                            }
                        ],
                        "title": "姓名",
                        "duration_num": {
                            "day": "1",
                            "half_day": "1",
                            "hour": "1",
                            "half_hour": "1",
                            "minute": "1"
                        }
                    }
                ]
            }
        ],
        "invalid_user_list": [
            "af5ddg73"
        ]
    }
}"#;
    #[test]
    fn test_resp() {
        let res = serde_json::from_str::<GetAttendanceUserStatsDataRespInner>(RESP);
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
