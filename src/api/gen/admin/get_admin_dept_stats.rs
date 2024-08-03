//! doc url: <https://open.larkoffice.com/document/uAjLw4CM/ukTMukTMukTM/reference/admin-v1/admin_dept_stat/list>
// Code generated by gen_api. DO NOT EDIT.

use serde::{Deserialize, Serialize};

use crate::api::BaseResp;
use crate::api::HasBaseResp;
use crate::core::model::{ApiRequest, CommonResponse, MethodOption, ReqParam, StreamReqParam};
use crate::{
    core::{http_client::HttpClient, store::Store},
    error::Error,
};

use crate::api::gen::admin::AdminService;

impl<'c, IStore: Store, IClient: HttpClient> AdminService<'c, IStore, IClient> {
    /// **api 版本: 2024-07-09T06:31:04+00:00**
    ///
    /// ## 获取部门维度的用户活跃和功能使用数据
    ///
    /// 该接口用于获取部门维度的用户活跃和功能使用数据，即IM（即时通讯）、日历、云文档、音视频会议、邮箱功能的使用数据。
    ///
    /// - 只有企业自建应用才有权限调用此接口
    ///
    /// - 当天的数据会在第二天的早上九点半产出（UTC+8）
    ///
    /// - 数据权限范围配置：目前只支持给每个应用配置部门级别数据权限范围，默认包含子部门
    ///
    /// doc: <https://open.larkoffice.com/document/uAjLw4CM/ukTMukTMukTM/reference/admin-v1/admin_dept_stat/list>
    ///
    /// new doc: <https://open.larkoffice.com/document/server-docs/admin-v1/data-report-management/list>
    ///
    /// api url: <https://open.feishu.cn/document_portal/v1/document/get_detail?fullPath=%2Fserver-docs%2Fadmin-v1%2Fdata-report-management%2Flist>
    pub async fn get_admin_dept_stats(
        &self,
        req: GetAdminDeptStatsReq,
    ) -> Result<(GetAdminDeptStatsResp, CommonResponse), Error> {
        self.get_admin_dept_stats_with_opt(req, Default::default())
            .await
    }

    /// 参见 [get_admin_dept_stats](#method.get_admin_dept_stats) 函数
    pub async fn get_admin_dept_stats_with_opt(
        &self,
        req: GetAdminDeptStatsReq,
        method_option: MethodOption,
    ) -> Result<(GetAdminDeptStatsResp, CommonResponse), Error> {
        #[cfg(feature = "test-util")]
        {
            if let Some(f) = self.mock().get_mock_get_admin_dept_stats(&req) {
                tracing::info!("[lark] Admin#GetAdminDeptStats **mocking** api");
                return f(req);
            }
        }

        tracing::info!("[lark] Admin#GetAdminDeptStats call api");

        let req = ApiRequest {
            scope: "Admin",
            api: "GetAdminDeptStats",
            method: http::Method::GET,
            url: String::new()
                + self.cli.open_base_url.as_ref()
                + "/open-apis/admin/v1/admin_dept_stats",
            param_data: req.gen_param(),
            method_option,
            need_tenant_access_token: true,
            ..Default::default()
        };

        let (resp, common_resp): (GetAdminDeptStatsRespInner, _) = self.cli.do_req(req).await?;
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
pub struct GetAdminDeptStatsReq {
    /// 部门ID类型
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "open_department_id"
    ///
    /// **可选值**:
    ///
    /// `department_id`: 部门的 ID
    ///
    /// `open_department_id`: 部门的 Open ID
    #[api(
        kind = "query",
        name = "department_id_type",
        v_type = "var",
        option = "false"
    )]
    pub department_id_type: String,
    /// 起始日期（包含），格式是YYYY-mm-dd
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "2020-02-15"
    #[api(kind = "query", name = "start_date", v_type = "var", option = "false")]
    pub start_date: String,
    /// 终止日期（包含），格式是YYYY-mm-dd，起止日期之间相差不能超过91天（包含91天）
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "2020-02-15"
    #[api(kind = "query", name = "end_date", v_type = "var", option = "false")]
    pub end_date: String,
    /// 部门的 ID，取决于department_id_type，仅支持根部门及其下前4级子部门
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "od-382e2793cfc9471f892e8a672987654c"
    #[api(
        kind = "query",
        name = "department_id",
        v_type = "var",
        option = "false"
    )]
    pub department_id: String,
    /// 是否包含子部门，如果该值为false，则只查出本部门直属用户活跃和功能使用数据；如果该值为true，则查出该部门以及其子部门（子部门层级最多不超过根部门下的前4级）的用户活跃和功能使用数据
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "false"
    #[api(
        kind = "query",
        name = "contains_child_dept",
        v_type = "var",
        option = "false"
    )]
    pub contains_child_dept: bool,
    /// 分页大小
    ///
    /// **示例值**: "10"
    #[api(kind = "query", name = "page_size", v_type = "var", option = "false")]
    pub page_size: i64,
    /// 分页标记，第一次请求不填，表示从头开始遍历；分页查询结果还有更多项时会同时返回新的 page_token，下次遍历可采用该 page_token 获取查询结果
    ///
    /// **示例值**: "2"
    #[api(kind = "query", name = "page_token", v_type = "var", option = "false")]
    pub page_token: String,
    /// 需跨域访问的Geo数据，每个Geo仅包含本Geo数据，不传默认查本地数据，调用前需要先开通FG(cn、sg、jp、us)
    ///
    /// **示例值**: "cn"
    #[api(kind = "query", name = "target_geo", v_type = "var", option = "false")]
    pub target_geo: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, lark_bot_sdk_macros::ApiBaseResp)]
struct GetAdminDeptStatsRespInner {
    #[serde(flatten)]
    data: Option<GetAdminDeptStatsResp>,
    #[serde(flatten)]
    base: BaseResp,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct GetAdminDeptStatsResp {
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
    /// **示例值**: "3"
    #[serde(
        rename = "page_token",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub page_token: String,
    /// 数据报表
    #[serde(
        rename = "items",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub items: Vec<AdminDeptStatSubResp>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(default)]
pub struct AdminDeptStatSubResp {
    /// 日期
    ///
    /// **示例值**: "2020-02-15"
    #[serde(
        rename = "date",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub date: String,
    /// 部门的department_id 或者open_department_id
    ///
    /// **示例值**: "od-382e2793cfc9471f892e8a672987654c"
    #[serde(
        rename = "department_id",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub department_id: String,
    /// 部门名字
    ///
    /// **示例值**: "subtestkkk"
    #[serde(
        rename = "department_name",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub department_name: String,
    /// 部门路径
    ///
    /// **示例值**: "testkkk/subtestkkk"
    #[serde(
        rename = "department_path",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub department_path: String,
    /// 部门总人数
    ///
    /// **示例值**: "2"
    #[serde(
        rename = "total_user_num",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub total_user_num: i64,
    /// 激活人数
    ///
    /// **示例值**: "0"
    #[serde(
        rename = "active_user_num",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub active_user_num: i64,
    /// 激活率
    ///
    /// **示例值**: "1.00"
    #[serde(
        rename = "active_user_rate",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub active_user_rate: String,
    /// 活跃人数
    ///
    /// **示例值**: "0"
    #[serde(
        rename = "suite_dau",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub suite_dau: i64,
    /// 活跃率
    ///
    /// **示例值**: "0.00"
    #[serde(
        rename = "suite_active_rate",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub suite_active_rate: String,
    /// 新用户数
    ///
    /// **示例值**: "0"
    #[serde(
        rename = "new_user_num",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub new_user_num: i64,
    /// 新激活数
    ///
    /// **示例值**: "0"
    #[serde(
        rename = "new_active_num",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub new_active_num: i64,
    /// 离职人数
    ///
    /// **示例值**: "0"
    #[serde(
        rename = "resign_user_num",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub resign_user_num: i64,
    /// 消息活跃人数
    ///
    /// **示例值**: "0"
    #[serde(
        rename = "im_dau",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub im_dau: i64,
    /// 发送消息人数
    ///
    /// **示例值**: "0"
    #[serde(
        rename = "send_messenger_user_num",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub send_messenger_user_num: i64,
    /// 发送消息数
    ///
    /// **示例值**: "0"
    #[serde(
        rename = "send_messenger_num",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub send_messenger_num: i64,
    /// 人均发送消息数
    ///
    /// **示例值**: "0.00"
    #[serde(
        rename = "avg_send_messenger_num",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub avg_send_messenger_num: String,
    /// 云文档活跃人数
    ///
    /// **示例值**: "0"
    #[serde(
        rename = "docs_dau",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub docs_dau: i64,
    /// 创建文件人数
    ///
    /// **示例值**: "0"
    #[serde(
        rename = "create_docs_user_num",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub create_docs_user_num: i64,
    /// 创建文件数
    ///
    /// **示例值**: "0"
    #[serde(
        rename = "create_docs_num",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub create_docs_num: i64,
    /// 人均创建文件数
    ///
    /// **示例值**: "0.00"
    #[serde(
        rename = "avg_create_docs_num",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub avg_create_docs_num: String,
    /// 日历活跃人数
    ///
    /// **示例值**: "0"
    #[serde(
        rename = "cal_dau",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub cal_dau: i64,
    /// 创建日程人数
    ///
    /// **示例值**: "0"
    #[serde(
        rename = "create_cal_user_num",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub create_cal_user_num: i64,
    /// 创建日程数
    ///
    /// **示例值**: "0"
    #[serde(
        rename = "create_cal_num",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub create_cal_num: i64,
    /// 人均创建日程数
    ///
    /// **示例值**: "0.00"
    #[serde(
        rename = "avg_create_cal_num",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub avg_create_cal_num: String,
    /// 音视频会议活跃人数
    ///
    /// **示例值**: "0"
    #[serde(
        rename = "vc_dau",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub vc_dau: i64,
    /// 会议时长：企业内员工参与通话与会议的总时长（分钟,不包括会议室的时长）
    ///
    /// **示例值**: "0"
    #[serde(
        rename = "vc_duration",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub vc_duration: i64,
    /// 人均会议时长（分钟，不包含会议室的时长）
    ///
    /// **示例值**: "0.00"
    #[serde(
        rename = "avg_vc_duration",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub avg_vc_duration: String,
    /// 人均飞书使用时长（分钟，私有化环境没值）
    ///
    /// **示例值**: "0.00"
    #[serde(
        rename = "avg_duration",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub avg_duration: String,
    /// 任务活跃人数
    ///
    /// **示例值**: "0"
    #[serde(
        rename = "task_dau",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub task_dau: i64,
    /// 创建任务人数
    ///
    /// **示例值**: "0"
    #[serde(
        rename = "create_task_user_num",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub create_task_user_num: i64,
    /// 创建任务数
    ///
    /// **示例值**: "0"
    #[serde(
        rename = "create_task_num",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub create_task_num: i64,
    /// 人均创建任务数
    ///
    /// **示例值**: "0.00"
    #[serde(
        rename = "avg_create_task_num",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub avg_create_task_num: String,
    /// 邮件总发件量
    ///
    /// **示例值**: "2"
    #[serde(
        rename = "email_send_count",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub email_send_count: String,
    /// 邮件总收件量
    ///
    /// **示例值**: "3"
    #[serde(
        rename = "email_receive_count",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub email_receive_count: String,
    /// 对外发件数
    ///
    /// **示例值**: "4"
    #[serde(
        rename = "email_send_ext_count",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub email_send_ext_count: String,
    /// 来自外部收件数
    ///
    /// **示例值**: "5"
    #[serde(
        rename = "email_receive_ext_count",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub email_receive_ext_count: String,
    /// 对内发件数
    ///
    /// **示例值**: "6"
    #[serde(
        rename = "email_send_in_count",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub email_send_in_count: String,
    /// 来自内部收件数
    ///
    /// **示例值**: "7"
    #[serde(
        rename = "email_receive_in_count",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub email_receive_in_count: String,
    /// 大搜搜索活跃人数
    ///
    /// **示例值**: "7"
    #[serde(
        rename = "search_active_dau",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub search_active_dau: String,
    /// 总搜索次数（在飞书主端搜索框发起过搜索请求的会话数）
    ///
    /// **示例值**: "7"
    #[serde(
        rename = "total_search_count",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub total_search_count: String,
    /// 综搜次数（在飞书主端搜索框的综合搜索发起过搜索请求的会话数）
    ///
    /// **示例值**: "7"
    #[serde(
        rename = "quick_search_count",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub quick_search_count: String,
    /// 垂搜次数（在飞书主端搜索框的垂类搜索tab（例如消息tab、云文档tab）发起过搜索请求的会话数）
    ///
    /// **示例值**: "7"
    #[serde(
        rename = "tab_search_count",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub tab_search_count: String,
}

#[cfg(feature = "test-util")]
mod test_utils {
    use std::sync::Arc;

    use self::gen::admin::AdminServiceMocker;
    use crate::core::mocker::*;

    use super::*;
    use crate::api::*;
    use crate::core::model::*;

    pub struct Mocker;

    pub trait MockFunc:
        Fn(GetAdminDeptStatsReq) -> Result<(GetAdminDeptStatsResp, CommonResponse), Error>
        + Send
        + Sync
        + 'static
    {
    }
    impl<
            T: Fn(GetAdminDeptStatsReq) -> Result<(GetAdminDeptStatsResp, CommonResponse), Error>
                + Send
                + Sync
                + 'static,
        > MockFunc for T
    {
    }

    impl<'c, IStore: Store, IClient: HttpClient> AdminServiceMocker<'c, IStore, IClient> {
        pub fn mock_get_admin_dept_stats<F: MockFunc>(
            &self,
            f: F,
        ) -> MockerBuilder<Mocker, GetAdminDeptStatsReq, GetAdminDeptStatsResp, Arc<dyn MockFunc>>
        {
            MockerBuilder::new(self.cli.instance_id, Arc::new(f))
        }

        pub(super) fn get_mock_get_admin_dept_stats(
            &self,
            req: &GetAdminDeptStatsReq,
        ) -> Option<Arc<dyn MockFunc>> {
            do_mock::<Mocker, GetAdminDeptStatsReq, GetAdminDeptStatsResp, Arc<dyn MockFunc>>(
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
        api::gen::admin::get_admin_dept_stats::{
            GetAdminDeptStatsReq, GetAdminDeptStatsResp, GetAdminDeptStatsRespInner,
        },
        core::{model::CommonResponse, Lark},
    };

    #[tokio::test]
    async fn test_mock() {
        let lark = Lark::new("".to_owned(), "".into());
        let mocker = lark
            .admin()
            .mock()
            .mock_get_admin_dept_stats(|_| {
                Ok((GetAdminDeptStatsResp::default(), CommonResponse::default()))
            })
            .build();
        let res = lark
            .admin()
            .get_admin_dept_stats(GetAdminDeptStatsReq::default())
            .await;
        assert!(res.is_ok());
        mocker.clear();
        let res = lark
            .admin()
            .get_admin_dept_stats(GetAdminDeptStatsReq::default())
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
        "page_token": "3",
        "items": [
            {
                "date": "2020-02-15",
                "department_id": "od-382e2793cfc9471f892e8a672987654c",
                "department_name": "subtestkkk",
                "department_path": "testkkk/subtestkkk",
                "total_user_num": 2,
                "active_user_num": 0,
                "active_user_rate": "1.00",
                "suite_dau": 0,
                "suite_active_rate": "0.00",
                "new_user_num": 0,
                "new_active_num": 0,
                "resign_user_num": 0,
                "im_dau": 0,
                "send_messenger_user_num": 0,
                "send_messenger_num": 0,
                "avg_send_messenger_num": "0.00",
                "docs_dau": 0,
                "create_docs_user_num": 0,
                "create_docs_num": 0,
                "avg_create_docs_num": "0.00",
                "cal_dau": 0,
                "create_cal_user_num": 0,
                "create_cal_num": 0,
                "avg_create_cal_num": "0.00",
                "vc_dau": 0,
                "vc_duration": 0,
                "avg_vc_duration": "0.00",
                "avg_duration": "0.00",
                "task_dau": 0,
                "create_task_user_num": 0,
                "create_task_num": 0,
                "avg_create_task_num": "0.00",
                "email_send_count": "2",
                "email_receive_count": "3",
                "email_send_ext_count": "4",
                "email_receive_ext_count": "5",
                "email_send_in_count": "6",
                "email_receive_in_count": "7",
                "search_active_dau": "7",
                "total_search_count": "7",
                "quick_search_count": "7",
                "tab_search_count": "7"
            }
        ]
    }
}"#;
    #[test]
    fn test_resp() {
        let res = serde_json::from_str::<GetAdminDeptStatsRespInner>(RESP);
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
