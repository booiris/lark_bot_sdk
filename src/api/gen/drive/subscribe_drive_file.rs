//! doc url: <https://open.larkoffice.com/document/uAjLw4CM/ukTMukTMukTM/reference/drive-v1/file/subscribe>
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
    /// **api 版本: 2024-08-01T12:50:01+00:00**
    ///
    /// ## 订阅云文档事件
    ///
    /// 该接口用于订阅云文档的各类通知事件。了解事件订阅的配置流程和使用场景，参考[事件概述](https://open.feishu.cn/document/ukTMukTMukTM/uUTNz4SN1MjL1UzM)。了解云文档支持的事件类型，参考[事件列表](https://open.feishu.cn/document/ukTMukTMukTM/uYDNxYjL2QTM24iN0EjN/event-list)。
    ///

    ///
    /// doc: <https://open.larkoffice.com/document/uAjLw4CM/ukTMukTMukTM/reference/drive-v1/file/subscribe>
    ///
    /// new doc: <https://open.larkoffice.com/document/server-docs/docs/drive-v1/event/subscribe>
    ///
    /// api url: <https://open.feishu.cn/document_portal/v1/document/get_detail?fullPath=%2Fserver-docs%2Fdocs%2Fdrive-v1%2Fevent%2Fsubscribe>
    pub async fn subscribe_drive_file(
        &self,
        req: SubscribeDriveFileReq,
    ) -> Result<(SubscribeDriveFileResp, CommonResponse), Error> {
        self.subscribe_drive_file_with_opt(req, Default::default())
            .await
    }

    /// 参见 [subscribe_drive_file](#method.subscribe_drive_file) 函数
    pub async fn subscribe_drive_file_with_opt(
        &self,
        req: SubscribeDriveFileReq,
        method_option: MethodOption,
    ) -> Result<(SubscribeDriveFileResp, CommonResponse), Error> {
        #[cfg(feature = "test-util")]
        {
            if let Some(f) = self.mock().get_mock_subscribe_drive_file(&req) {
                tracing::info!("[lark] Drive#SubscribeDriveFile **mocking** api");
                return f(req);
            }
        }

        tracing::info!("[lark] Drive#SubscribeDriveFile call api");

        let req = ApiRequest {
            scope: "Drive",
            api: "SubscribeDriveFile",
            method: http::Method::POST,
            url: String::new()
                + self.cli.open_base_url.as_ref()
                + "/open-apis/drive/v1/files/:file_token/subscribe",
            param_data: req.gen_param(),
            method_option,
            need_tenant_access_token: true,
            need_user_access_token: true,
            ..Default::default()
        };

        let (resp, common_resp): (SubscribeDriveFileRespInner, _) = self.cli.do_req(req).await?;
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
pub struct SubscribeDriveFileReq {
    /// 云文档的 token。了解如何获取各类云文档的 token，参考[云空间常见问题](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/drive-v1/faq)。
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "doccnfYZzTlvXqZIGTdAHKabcef"
    #[api(kind = "path", name = "file_token")]
    pub file_token: String,
    /// 云文档类型
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "docx"
    ///
    /// **可选值**:
    ///
    /// `doc`: 旧版文档。已不推荐使用
    ///
    /// `docx`: 新版文档
    ///
    /// `sheet`: 电子表格
    ///
    /// `bitable`: 多维表格
    ///
    /// `folder`: 文件夹
    #[api(kind = "query", name = "file_type", v_type = "var", option = "false")]
    pub file_type: String,
    /// 事件类型，`file_type` 为 `folder `（文件夹）时必填 `file.created_in_folder_v1`
    ///
    /// **示例值**: "file.created_in_folder_v1"
    #[api(kind = "query", name = "event_type", v_type = "var", option = "false")]
    pub event_type: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, lark_bot_sdk_macros::ApiBaseResp)]
struct SubscribeDriveFileRespInner {
    #[serde(flatten)]
    data: Option<SubscribeDriveFileResp>,
    #[serde(flatten)]
    base: BaseResp,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct SubscribeDriveFileResp {
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

    use self::gen::drive::DriveServiceMocker;
    use crate::core::mocker::*;

    use super::*;
    use crate::api::*;
    use crate::core::model::*;

    pub struct Mocker;

    pub trait MockFunc:
        Fn(SubscribeDriveFileReq) -> Result<(SubscribeDriveFileResp, CommonResponse), Error>
        + Send
        + Sync
        + 'static
    {
    }
    impl<
            T: Fn(SubscribeDriveFileReq) -> Result<(SubscribeDriveFileResp, CommonResponse), Error>
                + Send
                + Sync
                + 'static,
        > MockFunc for T
    {
    }

    impl<'c, IStore: Store, IClient: HttpClient> DriveServiceMocker<'c, IStore, IClient> {
        pub fn mock_subscribe_drive_file<F: MockFunc>(
            &self,
            f: F,
        ) -> MockerBuilder<Mocker, SubscribeDriveFileReq, SubscribeDriveFileResp, Arc<dyn MockFunc>>
        {
            MockerBuilder::new(self.cli.instance_id, Arc::new(f))
        }

        pub(super) fn get_mock_subscribe_drive_file(
            &self,
            req: &SubscribeDriveFileReq,
        ) -> Option<Arc<dyn MockFunc>> {
            do_mock::<Mocker, SubscribeDriveFileReq, SubscribeDriveFileResp, Arc<dyn MockFunc>>(
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
        api::gen::drive::subscribe_drive_file::{
            SubscribeDriveFileReq, SubscribeDriveFileResp, SubscribeDriveFileRespInner,
        },
        core::{model::CommonResponse, Lark},
    };

    #[tokio::test]
    async fn test_mock() {
        let lark = Lark::new("".to_owned(), "".into());
        let mocker = lark
            .drive()
            .mock()
            .mock_subscribe_drive_file(|_| {
                Ok((SubscribeDriveFileResp::default(), CommonResponse::default()))
            })
            .build();
        let res = lark
            .drive()
            .subscribe_drive_file(SubscribeDriveFileReq::default())
            .await;
        assert!(res.is_ok());
        mocker.clear();
        let res = lark
            .drive()
            .subscribe_drive_file(SubscribeDriveFileReq::default())
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
    "data": {}
}"#;
    #[test]
    fn test_resp() {
        let res = serde_json::from_str::<SubscribeDriveFileRespInner>(RESP);
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
