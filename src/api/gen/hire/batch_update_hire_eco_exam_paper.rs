//! doc url: <https://open.larkoffice.com/document/ukTMukTMukTM/uMzM1YjLzMTN24yMzUjN/hire-v1/eco_exam_paper/batch_update>
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
    /// **api 版本: 2023-11-02T08:52:04+00:00**
    ///
    /// ## 更新试卷
    ///
    /// 更新指定帐号可用的试卷列表
    ///

    ///
    /// doc: <https://open.larkoffice.com/document/ukTMukTMukTM/uMzM1YjLzMTN24yMzUjN/hire-v1/eco_exam_paper/batch_update>
    ///
    /// new doc: <https://open.larkoffice.com/document/server-docs/hire-v1/ecological-docking/eco_exam_paper/batch_update>
    ///
    /// api url: <https://open.feishu.cn/document_portal/v1/document/get_detail?fullPath=%2Fserver-docs%2Fhire-v1%2Fecological-docking%2Feco_exam_paper%2Fbatch_update>
    pub async fn batch_update_hire_eco_exam_paper(
        &self,
        req: BatchUpdateHireEcoExamPaperReq,
    ) -> Result<(BatchUpdateHireEcoExamPaperResp, CommonResponse), Error> {
        self.batch_update_hire_eco_exam_paper_with_opt(req, Default::default())
            .await
    }

    /// 参见 [batch_update_hire_eco_exam_paper](#method.batch_update_hire_eco_exam_paper) 函数
    pub async fn batch_update_hire_eco_exam_paper_with_opt(
        &self,
        req: BatchUpdateHireEcoExamPaperReq,
        method_option: MethodOption,
    ) -> Result<(BatchUpdateHireEcoExamPaperResp, CommonResponse), Error> {
        #[cfg(feature = "test-util")]
        {
            if let Some(f) = self.mock().get_mock_batch_update_hire_eco_exam_paper(&req) {
                tracing::info!("[lark] Hire#BatchUpdateHireEcoExamPaper **mocking** api");
                return f(req);
            }
        }

        tracing::info!("[lark] Hire#BatchUpdateHireEcoExamPaper call api");

        let req = ApiRequest {
            scope: "Hire",
            api: "BatchUpdateHireEcoExamPaper",
            method: http::Method::PATCH,
            url: String::new()
                + self.cli.open_base_url.as_ref()
                + "/open-apis/hire/v1/eco_exam_papers/batch_update",
            param_data: req.gen_param(),
            method_option,
            need_tenant_access_token: true,
            ..Default::default()
        };

        let (resp, common_resp): (BatchUpdateHireEcoExamPaperRespInner, _) =
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
pub struct BatchUpdateHireEcoExamPaperReq {
    /// 账号 ID，可在「账号绑定」事件中获取
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "7147998241542539527"
    #[api(kind = "body", name = "account_id")]
    pub account_id: String,
    /// 试卷列表
    ///
    /// **是否必填**: 是
    #[api(kind = "body", name = "paper_list")]
    pub paper_list: Vec<Option<EcoExamPaperDataSubReq>>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(default)]
pub struct EcoExamPaperDataSubReq {
    /// 试卷 ID
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "7147998241542539527"
    ///
    /// **数据校验规则**：
    ///
    /// - **最小长度**: `1` 字符
    #[serde(
        rename = "id",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub id: String,
    /// 试卷名称
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "春季测评"
    ///
    /// **数据校验规则**：
    ///
    /// - **最小长度**: `1` 字符
    #[serde(
        rename = "name",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub name: String,
    /// 考试时长（分钟）
    ///
    /// **示例值**: "30"
    #[serde(
        rename = "duration",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub duration: Option<i64>,
    /// 试卷题目数量
    ///
    /// **示例值**: "30"
    #[serde(
        rename = "question_count",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub question_count: Option<i64>,
    /// 开始时间，留空或不传表示不限制开始时间
    ///
    /// **示例值**: "1658676234053"
    #[serde(
        rename = "start_time",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub start_time: Option<String>,
    /// 结束时间，留空或不传表示不限制结束时间
    ///
    /// **示例值**: "1672444800000"
    #[serde(
        rename = "end_time",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub end_time: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, lark_bot_sdk_macros::ApiBaseResp)]
struct BatchUpdateHireEcoExamPaperRespInner {
    #[serde(flatten)]
    data: Option<BatchUpdateHireEcoExamPaperResp>,
    #[serde(flatten)]
    base: BaseResp,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct BatchUpdateHireEcoExamPaperResp {
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
            BatchUpdateHireEcoExamPaperReq,
        ) -> Result<(BatchUpdateHireEcoExamPaperResp, CommonResponse), Error>
        + Send
        + Sync
        + 'static
    {
    }
    impl<
            T: Fn(
                    BatchUpdateHireEcoExamPaperReq,
                )
                    -> Result<(BatchUpdateHireEcoExamPaperResp, CommonResponse), Error>
                + Send
                + Sync
                + 'static,
        > MockFunc for T
    {
    }

    impl<'c, IStore: Store, IClient: HttpClient> HireServiceMocker<'c, IStore, IClient> {
        pub fn mock_batch_update_hire_eco_exam_paper<F: MockFunc>(
            &self,
            f: F,
        ) -> MockerBuilder<
            Mocker,
            BatchUpdateHireEcoExamPaperReq,
            BatchUpdateHireEcoExamPaperResp,
            Arc<dyn MockFunc>,
        > {
            MockerBuilder::new(self.cli.instance_id, Arc::new(f))
        }

        pub(super) fn get_mock_batch_update_hire_eco_exam_paper(
            &self,
            req: &BatchUpdateHireEcoExamPaperReq,
        ) -> Option<Arc<dyn MockFunc>> {
            do_mock::<
                Mocker,
                BatchUpdateHireEcoExamPaperReq,
                BatchUpdateHireEcoExamPaperResp,
                Arc<dyn MockFunc>,
            >(self.cli.instance_id, req)
        }
    }
}

#[cfg(feature = "test-util")]
#[cfg(test)]
mod test {
    use crate::{
        api::gen::hire::batch_update_hire_eco_exam_paper::{
            BatchUpdateHireEcoExamPaperReq, BatchUpdateHireEcoExamPaperResp,
            BatchUpdateHireEcoExamPaperRespInner,
        },
        core::{model::CommonResponse, Lark},
    };

    #[tokio::test]
    async fn test_mock() {
        let lark = Lark::new("".to_owned(), "".into());
        let mocker = lark
            .hire()
            .mock()
            .mock_batch_update_hire_eco_exam_paper(|_| {
                Ok((
                    BatchUpdateHireEcoExamPaperResp::default(),
                    CommonResponse::default(),
                ))
            })
            .build();
        let res = lark
            .hire()
            .batch_update_hire_eco_exam_paper(BatchUpdateHireEcoExamPaperReq::default())
            .await;
        assert!(res.is_ok());
        mocker.clear();
        let res = lark
            .hire()
            .batch_update_hire_eco_exam_paper(BatchUpdateHireEcoExamPaperReq::default())
            .await;
        assert!(res.is_err());
    }

    const REQ: &str = r#"{
    "account_id": "7147998241542539527",
    "paper_list": [
        {
            "id": "7147998241542539527",
            "name": "春季测评",
            "duration": 30,
            "question_count": 30,
            "start_time": "1658676234053",
            "end_time": "1672444800000"
        }
    ]
}"#;

    #[test]
    fn test_req() {
        if REQ == "{}" {
            return;
        }
        if let Err(e) = serde_json::from_str::<super::BatchUpdateHireEcoExamPaperReqBody>(REQ) {
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
        let res = serde_json::from_str::<BatchUpdateHireEcoExamPaperRespInner>(RESP);
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
