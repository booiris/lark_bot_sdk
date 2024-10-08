//! doc url: <https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/drive-v1/media/upload_part>
// Code generated by gen_api. DO NOT EDIT.

use serde::{Deserialize, Serialize};

use crate::api::BaseResp;
use crate::api::HasBaseResp;
use crate::core::model::{
    ApiRequest, CommonResponse, MethodOption, ReqParam, StreamReqData, StreamReqParam,
};
use crate::{
    core::{http_client::HttpClient, store::Store},
    error::Error,
};

use crate::api::gen::drive::DriveService;

impl<'c, IStore: Store, IClient: HttpClient> DriveService<'c, IStore, IClient> {
    /// **api 版本: 2024-06-06T12:20:56+00:00**
    ///
    /// ## 分片上传素材-上传分片
    ///
    /// 根据 [预上传](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/drive-v1/media/upload_prepare)接口返回的上传事务 ID 和分片策略上传对应的素材分片。上传完成后，你可调用 [分片上传素材（完成上传）](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/drive-v1/media/upload_finish)触发完成上传。
    ///

    ///
    /// doc: <https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/drive-v1/media/upload_part>
    ///
    /// new doc: <https://open.feishu.cn/document/server-docs/docs/drive-v1/media/multipart-upload-media/upload_part>
    ///
    /// api url: <https://open.feishu.cn/document_portal/v1/document/get_detail?fullPath=%2Fserver-docs%2Fdocs%2Fdrive-v1%2Fmedia%2Fmultipart-upload-media%2Fupload_part>
    pub async fn part_upload_drive_media<Data: StreamReqData>(
        &self,
        req: PartUploadDriveMediaReq<Data>,
    ) -> Result<(PartUploadDriveMediaResp, CommonResponse), Error> {
        self.part_upload_drive_media_with_opt(req, Default::default())
            .await
    }

    /// 参见 [part_upload_drive_media](#method.part_upload_drive_media) 函数
    pub async fn part_upload_drive_media_with_opt<Data: StreamReqData>(
        &self,
        req: PartUploadDriveMediaReq<Data>,
        method_option: MethodOption,
    ) -> Result<(PartUploadDriveMediaResp, CommonResponse), Error> {
        #[cfg(feature = "test-util")]
        {
            if let Some(f) = self.mock().get_mock_part_upload_drive_media(&req) {
                tracing::info!("[lark] Drive#PartUploadDriveMedia **mocking** api");
                return f(req);
            }
        }

        tracing::info!("[lark] Drive#PartUploadDriveMedia call api");

        let req = ApiRequest::<()> {
            scope: "Drive",
            api: "PartUploadDriveMedia",
            method: http::Method::POST,
            url: String::new()
                + self.cli.open_base_url.as_ref()
                + "/open-apis/drive/v1/medias/upload_part",
            stream_param_data: req.gen_stream_param(),
            method_option,
            need_tenant_access_token: true,
            need_user_access_token: true,
            ..Default::default()
        };

        let (resp, common_resp): (PartUploadDriveMediaRespInner, _) = self.cli.do_req(req).await?;
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

#[derive(Debug, lark_bot_sdk_macros::ApiReqParams)]
pub struct PartUploadDriveMediaReq<Data: StreamReqData> {
    /// 分片上传事务的 ID。通过调用[分片上传素材（预上传）](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/drive-v1/media/upload_prepare)接口获取。
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "7111211691345512356"
    #[api(kind = "stream", name = "upload_id", option = "false")]
    pub upload_id: String,
    /// 块号，从 0 开始计数。
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "0"
    #[api(kind = "stream", name = "seq", option = "false")]
    pub seq: i64,
    /// 块的大小，单位为字节。
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "4194304"
    #[api(kind = "stream", name = "size", option = "false")]
    pub size: i64,
    /// 素材文件的 Adler-32 校验和
    ///
    /// **示例值**: "3248270248"
    #[api(kind = "stream", name = "checksum", option = "true")]
    pub checksum: Option<String>,
    #[api(kind = "stream", name = "#data#", option = "false")]
    pub data: Data,
}

#[derive(Debug, Serialize, Deserialize, Clone, lark_bot_sdk_macros::ApiBaseResp)]
struct PartUploadDriveMediaRespInner {
    #[serde(flatten)]
    data: Option<PartUploadDriveMediaResp>,
    #[serde(flatten)]
    base: BaseResp,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct PartUploadDriveMediaResp {
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

    pub trait MockFunc<D: StreamReqData>:
        Fn(PartUploadDriveMediaReq<D>) -> Result<(PartUploadDriveMediaResp, CommonResponse), Error>
        + Send
        + Sync
        + 'static
    {
    }
    impl<
            D: StreamReqData,
            T: Fn(
                    PartUploadDriveMediaReq<D>,
                ) -> Result<(PartUploadDriveMediaResp, CommonResponse), Error>
                + Send
                + Sync
                + 'static,
        > MockFunc<D> for T
    {
    }

    impl<'c, IStore: Store, IClient: HttpClient> DriveServiceMocker<'c, IStore, IClient> {
        pub fn mock_part_upload_drive_media<T: StreamReqData, F: MockFunc<T>>(
            &self,
            f: F,
        ) -> MockerBuilder<
            Mocker,
            PartUploadDriveMediaReq<T>,
            PartUploadDriveMediaResp,
            Arc<dyn MockFunc<T>>,
        > {
            MockerBuilder::new(self.cli.instance_id, Arc::new(f))
        }

        pub(super) fn get_mock_part_upload_drive_media<T: StreamReqData>(
            &self,
            req: &PartUploadDriveMediaReq<T>,
        ) -> Option<Arc<dyn MockFunc<T>>> {
            do_mock::<
                Mocker,
                PartUploadDriveMediaReq<T>,
                PartUploadDriveMediaResp,
                Arc<dyn MockFunc<T>>,
            >(self.cli.instance_id, req)
        }
    }
}
