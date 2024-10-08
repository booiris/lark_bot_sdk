//! doc url: <https://open.larkoffice.com/document/ukTMukTMukTM/uMzM1YjLzMTN24yMzUjN/hire-v1/eco_background_check/update_progress>
// Code generated by gen_api. DO NOT EDIT.

use serde::{Deserialize, Serialize};

use crate::api::BaseResp;
use crate::api::HasBaseResp;
use crate::core::model::{ApiRequest, CommonResponse, MethodOption, ReqParam, StreamReqParam};
use crate::{
    core::{http_client::HttpClient, store::Store},
    error::Error,
};

use crate::api::gen::hire::HireService;

impl<'c, IStore: Store, IClient: HttpClient> HireService<'c, IStore, IClient> {
    /// **api 版本: 2024-07-30T08:24:26+00:00**
    ///
    /// ## 更新背调订单进度
    ///
    /// 更新指定背调订单的进度信息和阶段性报告，进度信息将会被展示在「飞书招聘」-「投递详情页」-「背调卡片」上，告知用户目前背调订单的流转状态。
    ///
    /// 当订单状态已完成时，将无法通过此接口更新进度信息和阶段报告。
    ///
    /// doc: <https://open.larkoffice.com/document/ukTMukTMukTM/uMzM1YjLzMTN24yMzUjN/hire-v1/eco_background_check/update_progress>
    ///
    /// new doc: <https://open.larkoffice.com/document/server-docs/hire-v1/ecological-docking/eco_background_check/update_progress>
    ///
    /// api url: <https://open.feishu.cn/document_portal/v1/document/get_detail?fullPath=%2Fserver-docs%2Fhire-v1%2Fecological-docking%2Feco_background_check%2Fupdate_progress>
    pub async fn update_hire_eco_background_check_progress(
        &self,
        req: UpdateHireEcoBackgroundCheckProgressReq,
    ) -> Result<(UpdateHireEcoBackgroundCheckProgressResp, CommonResponse), Error> {
        self.update_hire_eco_background_check_progress_with_opt(req, Default::default())
            .await
    }

    /// 参见 [update_hire_eco_background_check_progress](#method.update_hire_eco_background_check_progress) 函数
    pub async fn update_hire_eco_background_check_progress_with_opt(
        &self,
        req: UpdateHireEcoBackgroundCheckProgressReq,
        method_option: MethodOption,
    ) -> Result<(UpdateHireEcoBackgroundCheckProgressResp, CommonResponse), Error> {
        #[cfg(feature = "test-util")]
        {
            if let Some(f) = self
                .mock()
                .get_mock_update_hire_eco_background_check_progress(&req)
            {
                tracing::info!("[lark] Hire#UpdateHireEcoBackgroundCheckProgress **mocking** api");
                return f(req);
            }
        }

        tracing::info!("[lark] Hire#UpdateHireEcoBackgroundCheckProgress call api");

        let req = ApiRequest {
            scope: "Hire",
            api: "UpdateHireEcoBackgroundCheckProgress",
            method: http::Method::POST,
            url: String::new()
                + self.cli.open_base_url.as_ref()
                + "/open-apis/hire/v1/eco_background_checks/update_progress",
            param_data: req.gen_param(),
            method_option,
            need_tenant_access_token: true,
            ..Default::default()
        };

        let (resp, common_resp): (UpdateHireEcoBackgroundCheckProgressRespInner, _) =
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
pub struct UpdateHireEcoBackgroundCheckProgressReq {
    /// 背调 ID。可通过[获取背调信息列表](https://open.feishu.cn/document/ukTMukTMukTM/uMzM1YjLzMTN24yMzUjN/hire-v1/background_check_order/list)获取
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "6931286400470354183"
    #[api(kind = "body", name = "background_check_id")]
    pub background_check_id: String,
    /// 阶段 ID。同一背调订单此 ID 不能重复，由调用方自定义
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "6931286400470354183"
    #[api(kind = "body", name = "stage_id")]
    pub stage_id: String,
    /// 背调阶段英文名称
    ///
    /// **示例值**: "stage report"
    #[api(kind = "body", name = "stage_en_name")]
    pub stage_en_name: Option<String>,
    /// 背调阶段名称
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "阶段报告"
    #[api(kind = "body", name = "stage_name")]
    pub stage_name: String,
    /// 阶段进度更新时间。 毫秒时间戳，每次调用此字段应严格递增
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "1660123456789"
    #[api(kind = "body", name = "stage_time")]
    pub stage_time: String,
    /// 背调结果（阶段性背调结果）。<br>
    ///
    /// **注意**：若需回传该字段，report_file_list为必填
    ///
    /// **示例值**: "通过"
    #[api(kind = "body", name = "result")]
    pub result: Option<String>,
    /// 报告列表
    #[api(kind = "body", name = "report_file_list")]
    pub report_file_list: Vec<Option<EcoBackgroundCheckReportFileSubReq>>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(default)]
pub struct EcoBackgroundCheckReportFileSubReq {
    /// 报告名称
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "阶段报告.pdf"
    #[serde(
        rename = "report_name",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub report_name: String,
    /// 报告地址；当report_url_type 为空或为 1 时需为可下载的 pdf 链接；为 2 时为预览型链接
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "https://xxxxx/xxxxxx/xxxx.pdf"
    #[serde(
        rename = "report_url",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub report_url: String,
    /// 报告地址类型；枚举值为空或 1 时为可下载的 pdf 链接，2 为预览型链接
    ///
    /// **示例值**: "1"
    ///
    /// **可选值**:
    ///
    /// `DownloadLink`: 可下载的链接
    ///
    /// `ExternalLink`: 预览型链接
    #[serde(
        rename = "report_url_type",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub report_url_type: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize, Clone, lark_bot_sdk_macros::ApiBaseResp)]
struct UpdateHireEcoBackgroundCheckProgressRespInner {
    #[serde(flatten)]
    data: Option<UpdateHireEcoBackgroundCheckProgressResp>,
    #[serde(flatten)]
    base: BaseResp,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct UpdateHireEcoBackgroundCheckProgressResp {
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

    use self::gen::hire::HireServiceMocker;
    use crate::core::mocker::*;

    use super::*;
    use crate::api::*;
    use crate::core::model::*;

    pub struct Mocker;

    pub trait MockFunc:
        Fn(
            UpdateHireEcoBackgroundCheckProgressReq,
        ) -> Result<(UpdateHireEcoBackgroundCheckProgressResp, CommonResponse), Error>
        + Send
        + Sync
        + 'static
    {
    }
    impl<
            T: Fn(
                    UpdateHireEcoBackgroundCheckProgressReq,
                )
                    -> Result<(UpdateHireEcoBackgroundCheckProgressResp, CommonResponse), Error>
                + Send
                + Sync
                + 'static,
        > MockFunc for T
    {
    }

    impl<'c, IStore: Store, IClient: HttpClient> HireServiceMocker<'c, IStore, IClient> {
        pub fn mock_update_hire_eco_background_check_progress<F: MockFunc>(
            &self,
            f: F,
        ) -> MockerBuilder<
            Mocker,
            UpdateHireEcoBackgroundCheckProgressReq,
            UpdateHireEcoBackgroundCheckProgressResp,
            Arc<dyn MockFunc>,
        > {
            MockerBuilder::new(self.cli.instance_id, Arc::new(f))
        }

        pub(super) fn get_mock_update_hire_eco_background_check_progress(
            &self,
            req: &UpdateHireEcoBackgroundCheckProgressReq,
        ) -> Option<Arc<dyn MockFunc>> {
            do_mock::<
                Mocker,
                UpdateHireEcoBackgroundCheckProgressReq,
                UpdateHireEcoBackgroundCheckProgressResp,
                Arc<dyn MockFunc>,
            >(self.cli.instance_id, req)
        }
    }
}

#[cfg(feature = "test-util")]
#[cfg(test)]
mod test {
    use crate::{
        api::gen::hire::update_hire_eco_background_check_progress::{
            UpdateHireEcoBackgroundCheckProgressReq, UpdateHireEcoBackgroundCheckProgressResp,
            UpdateHireEcoBackgroundCheckProgressRespInner,
        },
        core::{model::CommonResponse, Lark},
    };

    #[tokio::test]
    async fn test_mock() {
        let lark = Lark::new("".to_owned(), "".into());
        let mocker = lark
            .hire()
            .mock()
            .mock_update_hire_eco_background_check_progress(|_| {
                Ok((
                    UpdateHireEcoBackgroundCheckProgressResp::default(),
                    CommonResponse::default(),
                ))
            })
            .build();
        let res = lark
            .hire()
            .update_hire_eco_background_check_progress(
                UpdateHireEcoBackgroundCheckProgressReq::default(),
            )
            .await;
        assert!(res.is_ok());
        mocker.clear();
        let res = lark
            .hire()
            .update_hire_eco_background_check_progress(
                UpdateHireEcoBackgroundCheckProgressReq::default(),
            )
            .await;
        assert!(res.is_err());
    }

    const REQ: &str = r#"{
    "background_check_id": "6931286400470354183",
    "stage_id": "6931286400470354183",
    "stage_en_name": "stage report",
    "stage_name": "阶段报告",
    "stage_time": "1660123456789",
    "result": "通过",
    "report_file_list": [
        {
            "report_name": "阶段报告.pdf",
            "report_url": "https://xxxxx/xxxxxx/xxxx.pdf",
            "report_url_type": 1
        }
    ]
}"#;

    #[test]
    fn test_req() {
        if REQ == "{}" {
            return;
        }
        if let Err(e) =
            serde_json::from_str::<super::UpdateHireEcoBackgroundCheckProgressReqBody>(REQ)
        {
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
        let res = serde_json::from_str::<UpdateHireEcoBackgroundCheckProgressRespInner>(RESP);
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
