//! doc url: <https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/admin-v1/badge_image/create>
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

use crate::api::gen::admin::AdminService;

impl<'c, IStore: Store, IClient: HttpClient> AdminService<'c, IStore, IClient> {
    /// **api 版本: 2023-05-16T09:07:23+00:00**
    ///
    /// ## 上传勋章图片
    ///
    /// 通过该接口可以上传勋章详情图、挂饰图的文件，获取对应的文件key。
    ///

    ///
    /// doc: <https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/admin-v1/badge_image/create>
    ///
    /// new doc: <https://open.feishu.cn/document/server-docs/admin-v1/badge/badge/create>
    ///
    /// api url: <https://open.feishu.cn/document_portal/v1/document/get_detail?fullPath=%2Fserver-docs%2Fadmin-v1%2Fbadge%2Fbadge%2Fcreate>
    pub async fn upload_admin_badge_image<Data: StreamReqData>(
        &self,
        req: UploadAdminBadgeImageReq<Data>,
    ) -> Result<(UploadAdminBadgeImageResp, CommonResponse), Error> {
        self.upload_admin_badge_image_with_opt(req, Default::default())
            .await
    }

    /// 参见 [upload_admin_badge_image](#method.upload_admin_badge_image) 函数
    pub async fn upload_admin_badge_image_with_opt<Data: StreamReqData>(
        &self,
        req: UploadAdminBadgeImageReq<Data>,
        method_option: MethodOption,
    ) -> Result<(UploadAdminBadgeImageResp, CommonResponse), Error> {
        #[cfg(feature = "test-util")]
        {
            if let Some(f) = self.mock().get_mock_upload_admin_badge_image(&req) {
                tracing::info!("[lark] Admin#UploadAdminBadgeImage **mocking** api");
                return f(req);
            }
        }

        tracing::info!("[lark] Admin#UploadAdminBadgeImage call api");

        let req = ApiRequest::<()> {
            scope: "Admin",
            api: "UploadAdminBadgeImage",
            method: http::Method::POST,
            url: String::new()
                + self.cli.open_base_url.as_ref()
                + "/open-apis/admin/v1/badge_images",
            stream_param_data: req.gen_stream_param(),
            method_option,
            need_tenant_access_token: true,
            ..Default::default()
        };

        let (resp, common_resp): (UploadAdminBadgeImageRespInner, _) = self.cli.do_req(req).await?;
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
pub struct UploadAdminBadgeImageReq<Data: StreamReqData> {
    /// 勋章图片的文件，仅支持 PNG 格式，320 x 320 像素，大小不超过 1024 KB。
    ///
    /// **是否必填**: 是
    #[api(kind = "stream", name = "image_file", option = "false")]
    pub image_file: String,
    /// 图片的类型
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "1"
    ///
    /// **可选值**:
    ///
    /// `detail`: 勋章详情图
    ///
    /// `show`: 勋章挂饰图
    #[api(kind = "stream", name = "image_type", option = "false")]
    pub image_type: i64,
    #[api(kind = "stream", name = "#data#", option = "false")]
    pub data: Data,
}

#[derive(Debug, Serialize, Deserialize, Clone, lark_bot_sdk_macros::ApiBaseResp)]
struct UploadAdminBadgeImageRespInner {
    #[serde(flatten)]
    data: Option<UploadAdminBadgeImageResp>,
    #[serde(flatten)]
    base: BaseResp,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct UploadAdminBadgeImageResp {
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
    /// 图片的key
    ///
    /// **示例值**: "f02a98aa-1413-4af6-93ab-431ba9e5f2cg"
    #[serde(
        rename = "image_key",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub image_key: String,
}

#[cfg(feature = "test-util")]
mod test_utils {
    use std::sync::Arc;

    use self::gen::admin::AdminServiceMocker;
    use crate::core::mocker::*;

    use super::*;
    use crate::api::*;
    use crate::core::model::*;

    pub struct Mocker;

    pub trait MockFunc<D: StreamReqData>:
        Fn(
            UploadAdminBadgeImageReq<D>,
        ) -> Result<(UploadAdminBadgeImageResp, CommonResponse), Error>
        + Send
        + Sync
        + 'static
    {
    }
    impl<
            D: StreamReqData,
            T: Fn(
                    UploadAdminBadgeImageReq<D>,
                ) -> Result<(UploadAdminBadgeImageResp, CommonResponse), Error>
                + Send
                + Sync
                + 'static,
        > MockFunc<D> for T
    {
    }

    impl<'c, IStore: Store, IClient: HttpClient> AdminServiceMocker<'c, IStore, IClient> {
        pub fn mock_upload_admin_badge_image<T: StreamReqData, F: MockFunc<T>>(
            &self,
            f: F,
        ) -> MockerBuilder<
            Mocker,
            UploadAdminBadgeImageReq<T>,
            UploadAdminBadgeImageResp,
            Arc<dyn MockFunc<T>>,
        > {
            MockerBuilder::new(self.cli.instance_id, Arc::new(f))
        }

        pub(super) fn get_mock_upload_admin_badge_image<T: StreamReqData>(
            &self,
            req: &UploadAdminBadgeImageReq<T>,
        ) -> Option<Arc<dyn MockFunc<T>>> {
            do_mock::<
                Mocker,
                UploadAdminBadgeImageReq<T>,
                UploadAdminBadgeImageResp,
                Arc<dyn MockFunc<T>>,
            >(self.cli.instance_id, req)
        }
    }
}
