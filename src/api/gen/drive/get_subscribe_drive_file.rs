//! doc url: <https://open.larkoffice.com/document/uAjLw4CM/ukTMukTMukTM/reference/drive-v1/file/get_subscribe>
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
    /// **api 版本: 2024-07-19T05:46:32+00:00**
    ///
    /// ## 查询云文档事件订阅状态
    ///
    /// 该接口用于查询云文档事件的订阅状态。了解事件订阅的配置流程和使用场景，参考[事件概述](https://open.feishu.cn/document/ukTMukTMukTM/uUTNz4SN1MjL1UzM)。了解云文档支持的事件类型，参考[事件列表](https://open.feishu.cn/document/ukTMukTMukTM/uYDNxYjL2QTM24iN0EjN/event-list)。
    ///

    ///
    /// doc: <https://open.larkoffice.com/document/uAjLw4CM/ukTMukTMukTM/reference/drive-v1/file/get_subscribe>
    ///
    /// new doc: <https://open.larkoffice.com/document/uAjLw4CM/ukTMukTMukTM/reference/drive-v1/file/get_subscribe>
    ///
    /// api url: <https://open.feishu.cn/document_portal/v1/document/get_detail?fullPath=%2FuAjLw4CM%2FukTMukTMukTM%2Freference%2Fdrive-v1%2Ffile%2Fget_subscribe>
    pub async fn get_subscribe_drive_file(
        &self,
        req: GetSubscribeDriveFileReq,
    ) -> Result<(GetSubscribeDriveFileResp, CommonResponse), Error> {
        self.get_subscribe_drive_file_with_opt(req, Default::default())
            .await
    }

    /// 参见 [get_subscribe_drive_file](#method.get_subscribe_drive_file) 函数
    pub async fn get_subscribe_drive_file_with_opt(
        &self,
        req: GetSubscribeDriveFileReq,
        method_option: MethodOption,
    ) -> Result<(GetSubscribeDriveFileResp, CommonResponse), Error> {
        #[cfg(feature = "test-util")]
        {
            if let Some(f) = self.mock().get_mock_get_subscribe_drive_file(&req) {
                tracing::info!("[lark] Drive#GetSubscribeDriveFile **mocking** api");
                return f(req);
            }
        }

        tracing::info!("[lark] Drive#GetSubscribeDriveFile call api");

        let req = ApiRequest {
            scope: "Drive",
            api: "GetSubscribeDriveFile",
            method: http::Method::GET,
            url: String::new()
                + self.cli.open_base_url.as_ref()
                + "/open-apis/drive/v1/files/:file_token/get_subscribe",
            param_data: req.gen_param(),
            method_option,
            need_tenant_access_token: true,
            need_user_access_token: true,
            ..Default::default()
        };

        let (resp, common_resp): (GetSubscribeDriveFileRespInner, _) = self.cli.do_req(req).await?;
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
pub struct GetSubscribeDriveFileReq {
    /// 云文档的 token。了解如何获取各类云文档的 token，参考[云空间常见问题](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/drive-v1/faq)。
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "doccnfYZzTlvXqZIGTdAHKabcef"
    #[api(kind = "path", name = "file_token")]
    pub file_token: String,
    /// 文档类型
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
    /// `file`: 文件
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
struct GetSubscribeDriveFileRespInner {
    #[serde(flatten)]
    data: Option<GetSubscribeDriveFileResp>,
    #[serde(flatten)]
    base: BaseResp,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct GetSubscribeDriveFileResp {
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
    /// 是否有订阅，取值 true 表示已订阅；false 表示未订阅
    ///
    /// **示例值**: "\-"
    #[serde(
        rename = "is_subseribe",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub is_subseribe: bool,
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
        Fn(GetSubscribeDriveFileReq) -> Result<(GetSubscribeDriveFileResp, CommonResponse), Error>
        + Send
        + Sync
        + 'static
    {
    }
    impl<
            T: Fn(
                    GetSubscribeDriveFileReq,
                ) -> Result<(GetSubscribeDriveFileResp, CommonResponse), Error>
                + Send
                + Sync
                + 'static,
        > MockFunc for T
    {
    }

    impl<'c, IStore: Store, IClient: HttpClient> DriveServiceMocker<'c, IStore, IClient> {
        pub fn mock_get_subscribe_drive_file<F: MockFunc>(
            &self,
            f: F,
        ) -> MockerBuilder<
            Mocker,
            GetSubscribeDriveFileReq,
            GetSubscribeDriveFileResp,
            Arc<dyn MockFunc>,
        > {
            MockerBuilder::new(self.cli.instance_id, Arc::new(f))
        }

        pub(super) fn get_mock_get_subscribe_drive_file(
            &self,
            req: &GetSubscribeDriveFileReq,
        ) -> Option<Arc<dyn MockFunc>> {
            do_mock::<Mocker, GetSubscribeDriveFileReq, GetSubscribeDriveFileResp, Arc<dyn MockFunc>>(
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
        api::gen::drive::get_subscribe_drive_file::{
            GetSubscribeDriveFileReq, GetSubscribeDriveFileResp, GetSubscribeDriveFileRespInner,
        },
        core::{model::CommonResponse, Lark},
    };

    #[tokio::test]
    async fn test_mock() {
        let lark = Lark::new("".to_owned(), "".into());
        let mocker = lark
            .drive()
            .mock()
            .mock_get_subscribe_drive_file(|_| {
                Ok((
                    GetSubscribeDriveFileResp::default(),
                    CommonResponse::default(),
                ))
            })
            .build();
        let res = lark
            .drive()
            .get_subscribe_drive_file(GetSubscribeDriveFileReq::default())
            .await;
        assert!(res.is_ok());
        mocker.clear();
        let res = lark
            .drive()
            .get_subscribe_drive_file(GetSubscribeDriveFileReq::default())
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
        "is_subscribe": true
    }
}"#;
    #[test]
    fn test_resp() {
        let res = serde_json::from_str::<GetSubscribeDriveFileRespInner>(RESP);
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
