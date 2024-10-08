//! doc url: <https://open.larkoffice.com/document/uAjLw4CM/ukTMukTMukTM/reference/attendance-v1/group/search>
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
    /// **api 版本: 2024-07-26T04:57:34+00:00**
    ///
    /// ## 按名称查询考勤组
    ///
    /// 按考勤组名称查询考勤组摘要信息。查询条件支持名称精确匹配和模糊匹配两种方式。查询结果按考勤组修改时间 desc 排序，且最大记录数为 10 条。对应页面设置-假勤设置-[考勤组](https://example.feishu.cn/people/workforce-management/setting/group/list)的名称搜索功能
    ///
    /// 该接口依赖的数据和考勤组主数据间存在数据同步延时（正常数据同步 2 秒以内），因此在使用该接口时需注意评估数据延迟潜在风险。
    ///
    /// doc: <https://open.larkoffice.com/document/uAjLw4CM/ukTMukTMukTM/reference/attendance-v1/group/search>
    ///
    /// new doc: <https://open.larkoffice.com/document/server-docs/attendance-v1/group/search>
    ///
    /// api url: <https://open.feishu.cn/document_portal/v1/document/get_detail?fullPath=%2Fserver-docs%2Fattendance-v1%2Fgroup%2Fsearch>
    pub async fn search_attendance_group(
        &self,
        req: SearchAttendanceGroupReq,
    ) -> Result<(SearchAttendanceGroupResp, CommonResponse), Error> {
        self.search_attendance_group_with_opt(req, Default::default())
            .await
    }

    /// 参见 [search_attendance_group](#method.search_attendance_group) 函数
    pub async fn search_attendance_group_with_opt(
        &self,
        req: SearchAttendanceGroupReq,
        method_option: MethodOption,
    ) -> Result<(SearchAttendanceGroupResp, CommonResponse), Error> {
        #[cfg(feature = "test-util")]
        {
            if let Some(f) = self.mock().get_mock_search_attendance_group(&req) {
                tracing::info!("[lark] Attendance#SearchAttendanceGroup **mocking** api");
                return f(req);
            }
        }

        tracing::info!("[lark] Attendance#SearchAttendanceGroup call api");

        let req = ApiRequest {
            scope: "Attendance",
            api: "SearchAttendanceGroup",
            method: http::Method::POST,
            url: String::new()
                + self.cli.open_base_url.as_ref()
                + "/open-apis/attendance/v1/groups/search",
            param_data: req.gen_param(),
            method_option,
            need_tenant_access_token: true,
            ..Default::default()
        };

        let (resp, common_resp): (SearchAttendanceGroupRespInner, _) = self.cli.do_req(req).await?;
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
pub struct SearchAttendanceGroupReq {
    /// 考勤组名称
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "考勤组1"
    #[api(kind = "body", name = "group_name")]
    pub group_name: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, lark_bot_sdk_macros::ApiBaseResp)]
struct SearchAttendanceGroupRespInner {
    #[serde(flatten)]
    data: Option<SearchAttendanceGroupResp>,
    #[serde(flatten)]
    base: BaseResp,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct SearchAttendanceGroupResp {
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
    /// 考勤组列表
    #[serde(
        rename = "group_list",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub group_list: Vec<GroupMetaSubResp>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(default)]
pub struct GroupMetaSubResp {
    /// 考勤组 ID，可用于[按 ID 查询考勤组](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/attendance-v1/group/get)
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "6919358128597097404"
    #[serde(
        rename = "group_id",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub group_id: String,
    /// 考勤组名称
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "考勤组1"
    #[serde(
        rename = "group_name",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub group_name: String,
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
        Fn(SearchAttendanceGroupReq) -> Result<(SearchAttendanceGroupResp, CommonResponse), Error>
        + Send
        + Sync
        + 'static
    {
    }
    impl<
            T: Fn(
                    SearchAttendanceGroupReq,
                ) -> Result<(SearchAttendanceGroupResp, CommonResponse), Error>
                + Send
                + Sync
                + 'static,
        > MockFunc for T
    {
    }

    impl<'c, IStore: Store, IClient: HttpClient> AttendanceServiceMocker<'c, IStore, IClient> {
        pub fn mock_search_attendance_group<F: MockFunc>(
            &self,
            f: F,
        ) -> MockerBuilder<
            Mocker,
            SearchAttendanceGroupReq,
            SearchAttendanceGroupResp,
            Arc<dyn MockFunc>,
        > {
            MockerBuilder::new(self.cli.instance_id, Arc::new(f))
        }

        pub(super) fn get_mock_search_attendance_group(
            &self,
            req: &SearchAttendanceGroupReq,
        ) -> Option<Arc<dyn MockFunc>> {
            do_mock::<Mocker, SearchAttendanceGroupReq, SearchAttendanceGroupResp, Arc<dyn MockFunc>>(
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
        api::gen::attendance::search_attendance_group::{
            SearchAttendanceGroupReq, SearchAttendanceGroupResp, SearchAttendanceGroupRespInner,
        },
        core::{model::CommonResponse, Lark},
    };

    #[tokio::test]
    async fn test_mock() {
        let lark = Lark::new("".to_owned(), "".into());
        let mocker = lark
            .attendance()
            .mock()
            .mock_search_attendance_group(|_| {
                Ok((
                    SearchAttendanceGroupResp::default(),
                    CommonResponse::default(),
                ))
            })
            .build();
        let res = lark
            .attendance()
            .search_attendance_group(SearchAttendanceGroupReq::default())
            .await;
        assert!(res.is_ok());
        mocker.clear();
        let res = lark
            .attendance()
            .search_attendance_group(SearchAttendanceGroupReq::default())
            .await;
        assert!(res.is_err());
    }

    const REQ: &str = r#"{
    "group_name": "考勤组1"
}"#;

    #[test]
    fn test_req() {
        if REQ == "{}" {
            return;
        }
        if let Err(e) = serde_json::from_str::<super::SearchAttendanceGroupReqBody>(REQ) {
            panic!("{}", e);
        }
    }

    const RESP: &str = r#"{
    "code": 0,
    "msg": "success",
    "data": {
        "group_list": [
            {
                "group_id": "6919358128597097404",
                "group_name": "考勤组1"
            }
        ]
    }
}"#;
    #[test]
    fn test_resp() {
        let res = serde_json::from_str::<SearchAttendanceGroupRespInner>(RESP);
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
