//! doc url: <https://open.larkoffice.com/document/ukTMukTMukTM/uMzM1YjLzMTN24yMzUjN/hire-v1/eco_exam_paper/batch_delete>
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
    /// ## 删除试卷
    ///
    /// 删除指定帐号的指定试卷列表，删除不影响已创建的笔试，删除不存在的试卷时不会报错
    ///

    ///
    /// doc: <https://open.larkoffice.com/document/ukTMukTMukTM/uMzM1YjLzMTN24yMzUjN/hire-v1/eco_exam_paper/batch_delete>
    ///
    /// new doc: <https://open.larkoffice.com/document/server-docs/hire-v1/ecological-docking/eco_exam_paper/batch_delete>
    ///
    /// api url: <https://open.feishu.cn/document_portal/v1/document/get_detail?fullPath=%2Fserver-docs%2Fhire-v1%2Fecological-docking%2Feco_exam_paper%2Fbatch_delete>
    pub async fn batch_delete_hire_eco_exam_paper(
        &self,
        req: BatchDeleteHireEcoExamPaperReq,
    ) -> Result<(BatchDeleteHireEcoExamPaperResp, CommonResponse), Error> {
        self.batch_delete_hire_eco_exam_paper_with_opt(req, Default::default())
            .await
    }

    /// 参见 [batch_delete_hire_eco_exam_paper](#method.batch_delete_hire_eco_exam_paper) 函数
    pub async fn batch_delete_hire_eco_exam_paper_with_opt(
        &self,
        req: BatchDeleteHireEcoExamPaperReq,
        method_option: MethodOption,
    ) -> Result<(BatchDeleteHireEcoExamPaperResp, CommonResponse), Error> {
        #[cfg(feature = "test-util")]
        {
            if let Some(f) = self.mock().get_mock_batch_delete_hire_eco_exam_paper(&req) {
                tracing::info!("[lark] Hire#BatchDeleteHireEcoExamPaper **mocking** api");
                return f(req);
            }
        }

        tracing::info!("[lark] Hire#BatchDeleteHireEcoExamPaper call api");

        let req = ApiRequest {
            scope: "Hire",
            api: "BatchDeleteHireEcoExamPaper",
            method: http::Method::POST,
            url: String::new()
                + self.cli.open_base_url.as_ref()
                + "/open-apis/hire/v1/eco_exam_papers/batch_delete",
            param_data: req.gen_param(),
            method_option,
            need_tenant_access_token: true,
            ..Default::default()
        };

        let (resp, common_resp): (BatchDeleteHireEcoExamPaperRespInner, _) =
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
pub struct BatchDeleteHireEcoExamPaperReq {
    /// 背调账号 ID，可在「账号绑定」事件中获取
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "7147998241542539527"
    #[api(kind = "body", name = "account_id")]
    pub account_id: String,
    /// 试卷 ID 列表
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "["7147998241542539512"]"
    #[api(kind = "body", name = "paper_id_list")]
    pub paper_id_list: Vec<Option<String>>,
}

#[derive(Debug, Serialize, Deserialize, Clone, lark_bot_sdk_macros::ApiBaseResp)]
struct BatchDeleteHireEcoExamPaperRespInner {
    #[serde(flatten)]
    data: Option<BatchDeleteHireEcoExamPaperResp>,
    #[serde(flatten)]
    base: BaseResp,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct BatchDeleteHireEcoExamPaperResp {
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
            BatchDeleteHireEcoExamPaperReq,
        ) -> Result<(BatchDeleteHireEcoExamPaperResp, CommonResponse), Error>
        + Send
        + Sync
        + 'static
    {
    }
    impl<
            T: Fn(
                    BatchDeleteHireEcoExamPaperReq,
                )
                    -> Result<(BatchDeleteHireEcoExamPaperResp, CommonResponse), Error>
                + Send
                + Sync
                + 'static,
        > MockFunc for T
    {
    }

    impl<'c, IStore: Store, IClient: HttpClient> HireServiceMocker<'c, IStore, IClient> {
        pub fn mock_batch_delete_hire_eco_exam_paper<F: MockFunc>(
            &self,
            f: F,
        ) -> MockerBuilder<
            Mocker,
            BatchDeleteHireEcoExamPaperReq,
            BatchDeleteHireEcoExamPaperResp,
            Arc<dyn MockFunc>,
        > {
            MockerBuilder::new(self.cli.instance_id, Arc::new(f))
        }

        pub(super) fn get_mock_batch_delete_hire_eco_exam_paper(
            &self,
            req: &BatchDeleteHireEcoExamPaperReq,
        ) -> Option<Arc<dyn MockFunc>> {
            do_mock::<
                Mocker,
                BatchDeleteHireEcoExamPaperReq,
                BatchDeleteHireEcoExamPaperResp,
                Arc<dyn MockFunc>,
            >(self.cli.instance_id, req)
        }
    }
}

#[cfg(feature = "test-util")]
#[cfg(test)]
mod test {
    use crate::{
        api::gen::hire::batch_delete_hire_eco_exam_paper::{
            BatchDeleteHireEcoExamPaperReq, BatchDeleteHireEcoExamPaperResp,
            BatchDeleteHireEcoExamPaperRespInner,
        },
        core::{model::CommonResponse, Lark},
    };

    #[tokio::test]
    async fn test_mock() {
        let lark = Lark::new("".to_owned(), "".into());
        let mocker = lark
            .hire()
            .mock()
            .mock_batch_delete_hire_eco_exam_paper(|_| {
                Ok((
                    BatchDeleteHireEcoExamPaperResp::default(),
                    CommonResponse::default(),
                ))
            })
            .build();
        let res = lark
            .hire()
            .batch_delete_hire_eco_exam_paper(BatchDeleteHireEcoExamPaperReq::default())
            .await;
        assert!(res.is_ok());
        mocker.clear();
        let res = lark
            .hire()
            .batch_delete_hire_eco_exam_paper(BatchDeleteHireEcoExamPaperReq::default())
            .await;
        assert!(res.is_err());
    }

    const REQ: &str = r#"{
    "account_id": "7147998241542539527",
    "paper_id_list": [
        "7147998241542539526"
    ]
}"#;

    #[test]
    fn test_req() {
        if REQ == "{}" {
            return;
        }
        if let Err(e) = serde_json::from_str::<super::BatchDeleteHireEcoExamPaperReqBody>(REQ) {
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
        let res = serde_json::from_str::<BatchDeleteHireEcoExamPaperRespInner>(RESP);
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
