//! doc url: <https://open.larkoffice.com/document/uAjLw4CM/ukTMukTMukTM/reference/attendance-v1/user_setting/modify>
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
    /// **api 版本: 2024-07-26T07:16:34+00:00**
    ///
    /// ## 修改用户人脸识别信息
    ///
    /// 修改授权内员工的用户设置信息，包括人脸照片文件 ID。修改用户人脸识别信息目前只支持 API 方式修改，管理后台已无法修改。
    ///

    ///
    /// doc: <https://open.larkoffice.com/document/uAjLw4CM/ukTMukTMukTM/reference/attendance-v1/user_setting/modify>
    ///
    /// new doc: <https://open.larkoffice.com/document/server-docs/attendance-v1/user_setting/modify>
    ///
    /// api url: <https://open.feishu.cn/document_portal/v1/document/get_detail?fullPath=%2Fserver-docs%2Fattendance-v1%2Fuser_setting%2Fmodify>
    pub async fn update_attendance_user_setting(
        &self,
        req: UpdateAttendanceUserSettingReq,
    ) -> Result<(UpdateAttendanceUserSettingResp, CommonResponse), Error> {
        self.update_attendance_user_setting_with_opt(req, Default::default())
            .await
    }

    /// 参见 [update_attendance_user_setting](#method.update_attendance_user_setting) 函数
    pub async fn update_attendance_user_setting_with_opt(
        &self,
        req: UpdateAttendanceUserSettingReq,
        method_option: MethodOption,
    ) -> Result<(UpdateAttendanceUserSettingResp, CommonResponse), Error> {
        #[cfg(feature = "test-util")]
        {
            if let Some(f) = self.mock().get_mock_update_attendance_user_setting(&req) {
                tracing::info!("[lark] Attendance#UpdateAttendanceUserSetting **mocking** api");
                return f(req);
            }
        }

        tracing::info!("[lark] Attendance#UpdateAttendanceUserSetting call api");

        let req = ApiRequest {
            scope: "Attendance",
            api: "UpdateAttendanceUserSetting",
            method: http::Method::POST,
            url: String::new()
                + self.cli.open_base_url.as_ref()
                + "/open-apis/attendance/v1/user_settings/modify",
            param_data: req.gen_param(),
            method_option,
            need_tenant_access_token: true,
            ..Default::default()
        };

        let (resp, common_resp): (UpdateAttendanceUserSettingRespInner, _) =
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
pub struct UpdateAttendanceUserSettingReq {
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
    /// 用户设置
    #[api(kind = "body", name = "user_setting")]
    pub user_setting: Option<UserSettingSubReq>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(default)]
pub struct UserSettingSubReq {
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
    /// 人脸照片文件 ID，获取方式：[文件上传](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/attendance-v1/file/upload)
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "xxxxxb306842b1c189bc5212eefxxxxx"
    #[serde(
        rename = "face_key",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub face_key: String,
    /// 人脸照片更新时间，精确到秒的时间戳
    ///
    /// **示例值**: "1625681917"
    #[serde(
        rename = "face_key_update_time",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub face_key_update_time: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, lark_bot_sdk_macros::ApiBaseResp)]
struct UpdateAttendanceUserSettingRespInner {
    #[serde(flatten)]
    data: Option<UpdateAttendanceUserSettingResp>,
    #[serde(flatten)]
    base: BaseResp,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct UpdateAttendanceUserSettingResp {
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
    /// 用户设置
    #[serde(
        rename = "user_setting",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub user_setting: UserSettingSubResp,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(default)]
pub struct UserSettingSubResp {
    /// 用户 ID
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "abd754f7"
    #[serde(
        rename = "user_id",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub user_id: String,
    /// 人脸照片文件 ID，获取方式：[文件上传](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/attendance-v1/file/upload)
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "xxxxxb306842b1c189bc5212eefxxxxx"
    #[serde(
        rename = "face_key",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub face_key: String,
    /// 人脸照片更新时间，精确到秒的时间戳
    ///
    /// **示例值**: "1625681917"
    #[serde(
        rename = "face_key_update_time",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub face_key_update_time: String,
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
            UpdateAttendanceUserSettingReq,
        ) -> Result<(UpdateAttendanceUserSettingResp, CommonResponse), Error>
        + Send
        + Sync
        + 'static
    {
    }
    impl<
            T: Fn(
                    UpdateAttendanceUserSettingReq,
                )
                    -> Result<(UpdateAttendanceUserSettingResp, CommonResponse), Error>
                + Send
                + Sync
                + 'static,
        > MockFunc for T
    {
    }

    impl<'c, IStore: Store, IClient: HttpClient> AttendanceServiceMocker<'c, IStore, IClient> {
        pub fn mock_update_attendance_user_setting<F: MockFunc>(
            &self,
            f: F,
        ) -> MockerBuilder<
            Mocker,
            UpdateAttendanceUserSettingReq,
            UpdateAttendanceUserSettingResp,
            Arc<dyn MockFunc>,
        > {
            MockerBuilder::new(self.cli.instance_id, Arc::new(f))
        }

        pub(super) fn get_mock_update_attendance_user_setting(
            &self,
            req: &UpdateAttendanceUserSettingReq,
        ) -> Option<Arc<dyn MockFunc>> {
            do_mock::<
                Mocker,
                UpdateAttendanceUserSettingReq,
                UpdateAttendanceUserSettingResp,
                Arc<dyn MockFunc>,
            >(self.cli.instance_id, req)
        }
    }
}

#[cfg(feature = "test-util")]
#[cfg(test)]
mod test {
    use crate::{
        api::gen::attendance::update_attendance_user_setting::{
            UpdateAttendanceUserSettingReq, UpdateAttendanceUserSettingResp,
            UpdateAttendanceUserSettingRespInner,
        },
        core::{model::CommonResponse, Lark},
    };

    #[tokio::test]
    async fn test_mock() {
        let lark = Lark::new("".to_owned(), "".into());
        let mocker = lark
            .attendance()
            .mock()
            .mock_update_attendance_user_setting(|_| {
                Ok((
                    UpdateAttendanceUserSettingResp::default(),
                    CommonResponse::default(),
                ))
            })
            .build();
        let res = lark
            .attendance()
            .update_attendance_user_setting(UpdateAttendanceUserSettingReq::default())
            .await;
        assert!(res.is_ok());
        mocker.clear();
        let res = lark
            .attendance()
            .update_attendance_user_setting(UpdateAttendanceUserSettingReq::default())
            .await;
        assert!(res.is_err());
    }

    const REQ: &str = r#"{
    "user_setting": {
        "user_id": "abd754f7",
        "face_key": "xxxxxb306842b1c189bc5212eefxxxxx",
        "face_key_update_time": "1625681917"
    }
}"#;

    #[test]
    fn test_req() {
        if REQ == "{}" {
            return;
        }
        if let Err(e) = serde_json::from_str::<super::UpdateAttendanceUserSettingReqBody>(REQ) {
            panic!("{}", e);
        }
    }

    const RESP: &str = r#"{
    "code": 0,
    "msg": "success",
    "data": {
        "user_setting": {
            "user_id": "abd754f7",
            "face_key": "xxxxxb306842b1c189bc5212eefxxxxx",
            "face_key_update_time": "1625681917"
        }
    }
}"#;
    #[test]
    fn test_resp() {
        let res = serde_json::from_str::<UpdateAttendanceUserSettingRespInner>(RESP);
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
