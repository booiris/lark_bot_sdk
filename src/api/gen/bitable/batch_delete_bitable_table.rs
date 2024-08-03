//! doc url: <https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/bitable-v1/app-table/batch_delete>
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
    /// **api 版本: 2023-08-03T07:18:14+00:00**
    ///
    /// ## 删除多个数据表
    ///
    /// 删除多个数据表。
    ///

    ///
    /// doc: <https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/bitable-v1/app-table/batch_delete>
    ///
    /// new doc: <https://open.feishu.cn/document/server-docs/docs/bitable-v1/app-table/batch_delete>
    ///
    /// api url: <https://open.feishu.cn/document_portal/v1/document/get_detail?fullPath=%2Fserver-docs%2Fdocs%2Fbitable-v1%2Fapp-table%2Fbatch_delete>
    pub async fn batch_delete_bitable_table(
        &self,
        req: BatchDeleteBitableTableReq,
    ) -> Result<(BatchDeleteBitableTableResp, CommonResponse), Error> {
        self.batch_delete_bitable_table_with_opt(req, Default::default())
            .await
    }

    /// 参见 [batch_delete_bitable_table](#method.batch_delete_bitable_table) 函数
    pub async fn batch_delete_bitable_table_with_opt(
        &self,
        req: BatchDeleteBitableTableReq,
        method_option: MethodOption,
    ) -> Result<(BatchDeleteBitableTableResp, CommonResponse), Error> {
        #[cfg(feature = "test-util")]
        {
            if let Some(f) = self.mock().get_mock_batch_delete_bitable_table(&req) {
                tracing::info!("[lark] Bitable#BatchDeleteBitableTable **mocking** api");
                return f(req);
            }
        }

        tracing::info!("[lark] Bitable#BatchDeleteBitableTable call api");

        let req = ApiRequest {
            scope: "Bitable",
            api: "BatchDeleteBitableTable",
            method: http::Method::POST,
            url: String::new()
                + self.cli.open_base_url.as_ref()
                + "/open-apis/bitable/v1/apps/:app_token/tables/batch_delete",
            param_data: req.gen_param(),
            method_option,
            need_tenant_access_token: true,
            need_user_access_token: true,
            ..Default::default()
        };

        let (resp, common_resp): (BatchDeleteBitableTableRespInner, _) =
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
pub struct BatchDeleteBitableTableReq {
    /// 多维表格的唯一标识符 [app_token 参数说明](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/bitable/notification#8121eebe)
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "appbcbWCzen6D8dezhoCH2RpMAh"
    ///
    /// **数据校验规则**：
    ///
    /// - **最小长度**: `1` 字符
    #[api(kind = "path", name = "app_token")]
    pub app_token: String,

    /// 待删除的数据表的id [table_id 参数说明]，当前一次操作最多支持50个数据表(https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/bitable/notification#735fe883)
    ///
    /// **示例值**: "["tblsRc9GRRXKqhvW"]"
    #[api(kind = "body", name = "table_ids")]
    pub table_ids: Vec<Option<String>>,
}

#[derive(Debug, Serialize, Deserialize, Clone, lark_bot_sdk_macros::ApiBaseResp)]
struct BatchDeleteBitableTableRespInner {
    #[serde(flatten)]
    data: Option<BatchDeleteBitableTableResp>,
    #[serde(flatten)]
    base: BaseResp,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct BatchDeleteBitableTableResp {
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

    use self::gen::bitable::BitableServiceMocker;
    use crate::core::mocker::*;

    use super::*;
    use crate::api::*;
    use crate::core::model::*;

    pub struct Mocker;

    pub trait MockFunc:
        Fn(
            BatchDeleteBitableTableReq,
        ) -> Result<(BatchDeleteBitableTableResp, CommonResponse), Error>
        + Send
        + Sync
        + 'static
    {
    }
    impl<
            T: Fn(
                    BatchDeleteBitableTableReq,
                ) -> Result<(BatchDeleteBitableTableResp, CommonResponse), Error>
                + Send
                + Sync
                + 'static,
        > MockFunc for T
    {
    }

    impl<'c, IStore: Store, IClient: HttpClient> BitableServiceMocker<'c, IStore, IClient> {
        pub fn mock_batch_delete_bitable_table<F: MockFunc>(
            &self,
            f: F,
        ) -> MockerBuilder<
            Mocker,
            BatchDeleteBitableTableReq,
            BatchDeleteBitableTableResp,
            Arc<dyn MockFunc>,
        > {
            MockerBuilder::new(self.cli.instance_id, Arc::new(f))
        }

        pub(super) fn get_mock_batch_delete_bitable_table(
            &self,
            req: &BatchDeleteBitableTableReq,
        ) -> Option<Arc<dyn MockFunc>> {
            do_mock::<
                Mocker,
                BatchDeleteBitableTableReq,
                BatchDeleteBitableTableResp,
                Arc<dyn MockFunc>,
            >(self.cli.instance_id, req)
        }
    }
}

#[cfg(feature = "test-util")]
#[cfg(test)]
mod test {
    use crate::{
        api::gen::bitable::batch_delete_bitable_table::{
            BatchDeleteBitableTableReq, BatchDeleteBitableTableResp,
            BatchDeleteBitableTableRespInner,
        },
        core::{model::CommonResponse, Lark},
    };

    #[tokio::test]
    async fn test_mock() {
        let lark = Lark::new("".to_owned(), "".into());
        let mocker = lark
            .bitable()
            .mock()
            .mock_batch_delete_bitable_table(|_| {
                Ok((
                    BatchDeleteBitableTableResp::default(),
                    CommonResponse::default(),
                ))
            })
            .build();
        let res = lark
            .bitable()
            .batch_delete_bitable_table(BatchDeleteBitableTableReq::default())
            .await;
        assert!(res.is_ok());
        mocker.clear();
        let res = lark
            .bitable()
            .batch_delete_bitable_table(BatchDeleteBitableTableReq::default())
            .await;
        assert!(res.is_err());
    }

    const REQ: &str = r#"{
    "table_ids": [
        "tbl1TkhyTWDkSoZ3"
    ]
}"#;

    #[test]
    fn test_req() {
        if REQ == "{}" {
            return;
        }
        if let Err(e) = serde_json::from_str::<super::BatchDeleteBitableTableReqBody>(REQ) {
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
        let res = serde_json::from_str::<BatchDeleteBitableTableRespInner>(RESP);
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
