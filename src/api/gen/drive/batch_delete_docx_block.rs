//! doc url: <https://open.larkoffice.com/document/ukTMukTMukTM/uUDN04SN0QjL1QDN/document-docx/docx-v1/document-block-children/batch_delete>
// Code generated by gen_api. DO NOT EDIT.

use serde::{Deserialize, Serialize};

use crate::api::BaseResp;
use crate::api::HasBaseResp;
use crate::core::model::{ApiRequest, CommonResponse, MethodOption, ReqParam, StreamReqParam};
use crate::{
    core::{http_client::HttpClient, store::Store},
    error::Error,
};

use crate::api::gen::drive::DriveService;

impl<'c, IStore: Store, IClient: HttpClient> DriveService<'c, IStore, IClient> {
    /// **api 版本: 2024-07-31T09:17:08+00:00**
    ///
    /// ## 删除块
    ///
    /// 指定需要操作的块，删除其指定范围的子块。如果操作成功，接口将返回应用删除操作后的文档版本号。
    ///
    /// 在调用此接口前，请仔细阅读[新版文档 OpenAPI 接口校验规则](https://bytedance.feishu.cn/docx/doxcnby5Y0yoACL3PdfZqrJEm6f#doxcngCsscGk0WacO258mYDgM6b)，了解相关规则及约束。
    ///
    /// **应用频率限制**：单个应用调用频率上限为每秒 3 次，超过该频率限制，接口将返回 HTTP 状态码 <font color="blue">400</font> 及错误码 <font color="blue">99991400</font>；
    ///
    /// **文档频率限制**：单篇文档并发编辑上限为每秒 3 次，超过该频率限制，接口将返回 HTTP 状态码 <font color="blue">429</font>，编辑操作包括：
    ///
    /// - 创建块
    ///
    /// - 删除块
    ///
    /// - 更新块
    ///
    /// - 批量更新块
    ///
    /// 当请求被限频，应用需要处理限频状态码，并使用指数退避算法或其它一些频控策略降低对 API 的调用速率。
    ///
    /// doc: <https://open.larkoffice.com/document/ukTMukTMukTM/uUDN04SN0QjL1QDN/document-docx/docx-v1/document-block-children/batch_delete>
    ///
    /// new doc: <https://open.larkoffice.com/document/server-docs/docs/docs/docx-v1/document-block/batch_delete>
    ///
    /// api url: <https://open.feishu.cn/document_portal/v1/document/get_detail?fullPath=%2Fserver-docs%2Fdocs%2Fdocs%2Fdocx-v1%2Fdocument-block%2Fbatch_delete>
    pub async fn batch_delete_docx_block(
        &self,
        req: BatchDeleteDocxBlockReq,
    ) -> Result<(BatchDeleteDocxBlockResp, CommonResponse), Error> {
        self.batch_delete_docx_block_with_opt(req, Default::default())
            .await
    }

    /// 参见 [batch_delete_docx_block](#method.batch_delete_docx_block) 函数
    pub async fn batch_delete_docx_block_with_opt(
        &self,
        req: BatchDeleteDocxBlockReq,
        method_option: MethodOption,
    ) -> Result<(BatchDeleteDocxBlockResp, CommonResponse), Error> {
        #[cfg(feature = "test-util")]
        {
            if let Some(f) = self.mock().get_mock_batch_delete_docx_block(&req) {
                tracing::info!("[lark] Drive#BatchDeleteDocxBlock **mocking** api");
                return f(req);
            }
        }

        tracing::info!("[lark] Drive#BatchDeleteDocxBlock call api");

        let req = ApiRequest {
            scope: "Drive",
            api: "BatchDeleteDocxBlock",
            method: http::Method::DELETE,
            url: String::new() + self.cli.open_base_url.as_ref() + "/open-apis/docx/v1/documents/:document_id/blocks/:block_id/children/batch_delete",
            param_data: req.gen_param(),
            method_option,
            need_tenant_access_token: true,
need_user_access_token: true,
            ..Default::default()
        };

        let (resp, common_resp): (BatchDeleteDocxBlockRespInner, _) = self.cli.do_req(req).await?;
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
pub struct BatchDeleteDocxBlockReq {
    /// 文档唯一标识。对应新版文档 Token，[点击了解如何获取云文档 Token](https://open.feishu.cn/document/ukTMukTMukTM/uczNzUjL3czM14yN3MTN#08bb5df6)。
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "doxcnePuYufKa49ISjhD8Ih0ikh"
    #[api(kind = "path", name = "document_id")]
    pub document_id: String,
    /// 父 Block 的唯一标识
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "doxcnO6UW6wAw2qIcYf4hZpFIth"
    #[api(kind = "path", name = "block_id")]
    pub block_id: String,
    /// 操作的文档版本，-1表示文档最新版本。若此时操作的版本为文档最新版本，则需要持有文档的阅读权限；若此时操作的版本为文档的历史版本，则需要持有文档的编辑权限。
    ///
    /// **示例值**: "-1"
    #[api(
        kind = "query",
        name = "document_revision_id",
        v_type = "var",
        option = "false"
    )]
    pub document_revision_id: i64,
    /// 操作的唯一标识，与接口返回值的 client_token 相对应，用于幂等的进行更新操作。此值为空表示将发起一次新的请求，此值非空表示幂等的进行更新操作。
    ///
    /// **示例值**: "fe599b60-450f-46ff-b2ef-9f6675625b97"
    #[api(
        kind = "query",
        name = "client_token",
        v_type = "var",
        option = "false"
    )]
    pub client_token: String,
    /// 删除的起始索引（操作区间左闭右开）
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "0"
    #[api(kind = "body", name = "start_index")]
    pub start_index: i64,
    /// 删除的末尾索引（操作区间左闭右开）
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "1"
    #[api(kind = "body", name = "end_index")]
    pub end_index: i64,
}

#[derive(Debug, Serialize, Deserialize, Clone, lark_bot_sdk_macros::ApiBaseResp)]
struct BatchDeleteDocxBlockRespInner {
    #[serde(flatten)]
    data: Option<BatchDeleteDocxBlockResp>,
    #[serde(flatten)]
    base: BaseResp,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct BatchDeleteDocxBlockResp {
    #[serde(
        rename = "data",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub data: DataSubResp,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(default)]
pub struct DataSubResp {
    /// 当前删除操作成功后文档的版本号
    ///
    /// **示例值**: "1"
    #[serde(
        rename = "document_revision_id",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub document_revision_id: i64,
    /// 操作的唯一标识，更新请求中使用此值表示幂等的进行此次更新
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "fe599b60-450f-46ff-b2ef-9f6675625b97"
    #[serde(
        rename = "client_token",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub client_token: String,
}

#[cfg(feature = "test-util")]
mod test_utils {
    use std::sync::Arc;

    use self::gen::drive::DriveServiceMocker;
    use crate::core::mocker::*;

    use super::*;
    use crate::api::*;
    use crate::core::model::*;

    pub struct Mocker;

    pub trait MockFunc:
        Fn(BatchDeleteDocxBlockReq) -> Result<(BatchDeleteDocxBlockResp, CommonResponse), Error>
        + Send
        + Sync
        + 'static
    {
    }
    impl<
            T: Fn(
                    BatchDeleteDocxBlockReq,
                ) -> Result<(BatchDeleteDocxBlockResp, CommonResponse), Error>
                + Send
                + Sync
                + 'static,
        > MockFunc for T
    {
    }

    impl<'c, IStore: Store, IClient: HttpClient> DriveServiceMocker<'c, IStore, IClient> {
        pub fn mock_batch_delete_docx_block<F: MockFunc>(
            &self,
            f: F,
        ) -> MockerBuilder<
            Mocker,
            BatchDeleteDocxBlockReq,
            BatchDeleteDocxBlockResp,
            Arc<dyn MockFunc>,
        > {
            MockerBuilder::new(self.cli.instance_id, Arc::new(f))
        }

        pub(super) fn get_mock_batch_delete_docx_block(
            &self,
            req: &BatchDeleteDocxBlockReq,
        ) -> Option<Arc<dyn MockFunc>> {
            do_mock::<Mocker, BatchDeleteDocxBlockReq, BatchDeleteDocxBlockResp, Arc<dyn MockFunc>>(
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
        api::gen::drive::batch_delete_docx_block::{
            BatchDeleteDocxBlockReq, BatchDeleteDocxBlockResp, BatchDeleteDocxBlockRespInner,
        },
        core::{model::CommonResponse, Lark},
    };

    #[tokio::test]
    async fn test_mock() {
        let lark = Lark::new("".to_owned(), "".into());
        let mocker = lark
            .drive()
            .mock()
            .mock_batch_delete_docx_block(|_| {
                Ok((
                    BatchDeleteDocxBlockResp::default(),
                    CommonResponse::default(),
                ))
            })
            .build();
        let res = lark
            .drive()
            .batch_delete_docx_block(BatchDeleteDocxBlockReq::default())
            .await;
        assert!(res.is_ok());
        mocker.clear();
        let res = lark
            .drive()
            .batch_delete_docx_block(BatchDeleteDocxBlockReq::default())
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

    const RESP: &str = "{}";
    #[test]
    fn test_resp() {
        let res = serde_json::from_str::<BatchDeleteDocxBlockRespInner>(RESP);
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
