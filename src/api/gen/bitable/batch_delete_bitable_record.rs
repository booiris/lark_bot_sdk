//! doc url: <https://open.larkoffice.com/document/uAjLw4CM/ukTMukTMukTM/reference/bitable-v1/app-table-record/batch_delete>
// Code generated by gen_api. DO NOT EDIT.

use serde::{Deserialize, Serialize};

use crate::api::BaseResp;
use crate::api::HasBaseResp;
use crate::core::model::{ApiRequest, CommonResponse, MethodOption, ReqParam, StreamReqParam};
use crate::{
    core::{http_client::HttpClient, store::Store},
    error::Error,
};

use crate::api::gen::bitable::BitableService;

impl<'c, IStore: Store, IClient: HttpClient> BitableService<'c, IStore, IClient> {
    /// **api 版本: 2023-07-28T08:21:53+00:00**
    ///
    /// ## 删除多条记录
    ///
    /// 该接口用于删除数据表中现有的多条记录，单次调用中最多删除 500 条记录。
    ///

    ///
    /// doc: <https://open.larkoffice.com/document/uAjLw4CM/ukTMukTMukTM/reference/bitable-v1/app-table-record/batch_delete>
    ///
    /// new doc: <https://open.larkoffice.com/document/server-docs/docs/bitable-v1/app-table-record/batch_delete>
    ///
    /// api url: <https://open.feishu.cn/document_portal/v1/document/get_detail?fullPath=%2Fserver-docs%2Fdocs%2Fbitable-v1%2Fapp-table-record%2Fbatch_delete>
    pub async fn batch_delete_bitable_record(
        &self,
        req: BatchDeleteBitableRecordReq,
    ) -> Result<(BatchDeleteBitableRecordResp, CommonResponse), Error> {
        self.batch_delete_bitable_record_with_opt(req, Default::default())
            .await
    }

    /// 参见 [batch_delete_bitable_record](#method.batch_delete_bitable_record) 函数
    pub async fn batch_delete_bitable_record_with_opt(
        &self,
        req: BatchDeleteBitableRecordReq,
        method_option: MethodOption,
    ) -> Result<(BatchDeleteBitableRecordResp, CommonResponse), Error> {
        #[cfg(feature = "test-util")]
        {
            if let Some(f) = self.mock().get_mock_batch_delete_bitable_record(&req) {
                tracing::info!("[lark] Bitable#BatchDeleteBitableRecord **mocking** api");
                return f(req);
            }
        }

        tracing::info!("[lark] Bitable#BatchDeleteBitableRecord call api");

        let req = ApiRequest {
            scope: "Bitable",
            api: "BatchDeleteBitableRecord",
            method: http::Method::POST,
            url: String::new()
                + self.cli.open_base_url.as_ref()
                + "/open-apis/bitable/v1/apps/:app_token/tables/:table_id/records/batch_delete",
            param_data: req.gen_param(),
            method_option,
            need_tenant_access_token: true,
            need_user_access_token: true,
            ..Default::default()
        };

        let (resp, common_resp): (BatchDeleteBitableRecordRespInner, _) =
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
pub struct BatchDeleteBitableRecordReq {
    /// base app token
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "appbcbWCzen6D8dezhoCH2RpMAh"
    #[api(kind = "path", name = "app_token")]
    pub app_token: String,
    /// table id
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "tblsRc9GRRXKqhvW"
    #[api(kind = "path", name = "table_id")]
    pub table_id: String,

    /// 删除的多条记录id列表
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "[
    ///
    /// "recIcJBbvC",
    ///
    /// "recvmiCORa"
    ///
    /// ]"
    #[api(kind = "body", name = "records")]
    pub records: Vec<Option<String>>,
}

#[derive(Debug, Serialize, Deserialize, Clone, lark_bot_sdk_macros::ApiBaseResp)]
struct BatchDeleteBitableRecordRespInner {
    #[serde(flatten)]
    data: Option<BatchDeleteBitableRecordResp>,
    #[serde(flatten)]
    base: BaseResp,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct BatchDeleteBitableRecordResp {
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
    /// 记录
    #[serde(
        rename = "records",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub records: Vec<DeleteRecordSubResp>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(default)]
pub struct DeleteRecordSubResp {
    /// 是否成功删除
    ///
    /// **示例值**: "true"
    #[serde(
        rename = "deleted",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub deleted: bool,
    /// 删除的记录 ID
    ///
    /// **示例值**: "recpCsf4ME"
    #[serde(
        rename = "record_id",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub record_id: String,
}

#[cfg(feature = "test-util")]
mod test_utils {
    use std::sync::Arc;

    use self::gen::bitable::BitableServiceMocker;
    use crate::core::mocker::*;

    use super::*;
    use crate::api::*;
    use crate::core::model::*;

    pub struct Mocker;

    pub trait MockFunc:
        Fn(
            BatchDeleteBitableRecordReq,
        ) -> Result<(BatchDeleteBitableRecordResp, CommonResponse), Error>
        + Send
        + Sync
        + 'static
    {
    }
    impl<
            T: Fn(
                    BatchDeleteBitableRecordReq,
                ) -> Result<(BatchDeleteBitableRecordResp, CommonResponse), Error>
                + Send
                + Sync
                + 'static,
        > MockFunc for T
    {
    }

    impl<'c, IStore: Store, IClient: HttpClient> BitableServiceMocker<'c, IStore, IClient> {
        pub fn mock_batch_delete_bitable_record<F: MockFunc>(
            &self,
            f: F,
        ) -> MockerBuilder<
            Mocker,
            BatchDeleteBitableRecordReq,
            BatchDeleteBitableRecordResp,
            Arc<dyn MockFunc>,
        > {
            MockerBuilder::new(self.cli.instance_id, Arc::new(f))
        }

        pub(super) fn get_mock_batch_delete_bitable_record(
            &self,
            req: &BatchDeleteBitableRecordReq,
        ) -> Option<Arc<dyn MockFunc>> {
            do_mock::<
                Mocker,
                BatchDeleteBitableRecordReq,
                BatchDeleteBitableRecordResp,
                Arc<dyn MockFunc>,
            >(self.cli.instance_id, req)
        }
    }
}

#[cfg(feature = "test-util")]
#[cfg(test)]
mod test {
    use crate::{
        api::gen::bitable::batch_delete_bitable_record::{
            BatchDeleteBitableRecordReq, BatchDeleteBitableRecordResp,
            BatchDeleteBitableRecordRespInner,
        },
        core::{model::CommonResponse, Lark},
    };

    #[tokio::test]
    async fn test_mock() {
        let lark = Lark::new("".to_owned(), "".into());
        let mocker = lark
            .bitable()
            .mock()
            .mock_batch_delete_bitable_record(|_| {
                Ok((
                    BatchDeleteBitableRecordResp::default(),
                    CommonResponse::default(),
                ))
            })
            .build();
        let res = lark
            .bitable()
            .batch_delete_bitable_record(BatchDeleteBitableRecordReq::default())
            .await;
        assert!(res.is_ok());
        mocker.clear();
        let res = lark
            .bitable()
            .batch_delete_bitable_record(BatchDeleteBitableRecordReq::default())
            .await;
        assert!(res.is_err());
    }

    const REQ: &str = r#"{
    "records": [
        "recwNXzPQv"
    ]
}"#;

    #[test]
    fn test_req() {
        if REQ == "{}" {
            return;
        }
        if let Err(e) = serde_json::from_str::<super::BatchDeleteBitableRecordReqBody>(REQ) {
            panic!("{}", e);
        }
    }

    const RESP: &str = r#"{
    "code": 0,
    "msg": "success",
    "data": {
        "records": [
            {
                "deleted": true,
                "record_id": "recpCsf4ME"
            }
        ]
    }
}"#;
    #[test]
    fn test_resp() {
        let res = serde_json::from_str::<BatchDeleteBitableRecordRespInner>(RESP);
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
