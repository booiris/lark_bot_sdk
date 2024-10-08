//! doc url: <https://open.larkoffice.com/document/ukTMukTMukTM/uUDN04SN0QjL1QDN/sheets-v3/spreadsheet-sheet-float_image/patch>
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
    /// **api 版本: 2024-07-31T09:17:20+00:00**
    ///
    /// ## 更新浮动图片
    ///
    /// 更新已有的浮动图片位置和宽高，包括 range、width、height、offset_x 和 offset_y，不包括 float_image_id 和 float_image_token。
    ///
    /// 浮动图片更新参考：[浮动图片指南](https://open.feishu.cn/document/ukTMukTMukTM/uUDN04SN0QjL1QDN/sheets-v3/spreadsheet-sheet-float_image/float-image-user-guide)
    ///
    /// doc: <https://open.larkoffice.com/document/ukTMukTMukTM/uUDN04SN0QjL1QDN/sheets-v3/spreadsheet-sheet-float_image/patch>
    ///
    /// new doc: <https://open.larkoffice.com/document/server-docs/docs/sheets-v3/spreadsheet-sheet-float_image/patch>
    ///
    /// api url: <https://open.feishu.cn/document_portal/v1/document/get_detail?fullPath=%2Fserver-docs%2Fdocs%2Fsheets-v3%2Fspreadsheet-sheet-float_image%2Fpatch>
    pub async fn update_sheet_float_image(
        &self,
        req: UpdateSheetFloatImageReq,
    ) -> Result<(UpdateSheetFloatImageResp, CommonResponse), Error> {
        self.update_sheet_float_image_with_opt(req, Default::default())
            .await
    }

    /// 参见 [update_sheet_float_image](#method.update_sheet_float_image) 函数
    pub async fn update_sheet_float_image_with_opt(
        &self,
        req: UpdateSheetFloatImageReq,
        method_option: MethodOption,
    ) -> Result<(UpdateSheetFloatImageResp, CommonResponse), Error> {
        #[cfg(feature = "test-util")]
        {
            if let Some(f) = self.mock().get_mock_update_sheet_float_image(&req) {
                tracing::info!("[lark] Drive#UpdateSheetFloatImage **mocking** api");
                return f(req);
            }
        }

        tracing::info!("[lark] Drive#UpdateSheetFloatImage call api");

        let req = ApiRequest {
            scope: "Drive",
            api: "UpdateSheetFloatImage",
            method: http::Method::PATCH,
            url: String::new() + self.cli.open_base_url.as_ref() + "/open-apis/sheets/v3/spreadsheets/:spreadsheet_token/sheets/:sheet_id/float_images/:float_image_id",
            param_data: req.gen_param(),
            method_option,
            need_tenant_access_token: true,
need_user_access_token: true,
            ..Default::default()
        };

        let (resp, common_resp): (UpdateSheetFloatImageRespInner, _) = self.cli.do_req(req).await?;
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
pub struct UpdateSheetFloatImageReq {
    /// 表格 token
    ///
    /// **示例值**: "shtcnmBA*****yGehy8"
    #[api(kind = "path", name = "spreadsheet_token")]
    pub spreadsheet_token: String,
    /// 子表 id
    ///
    /// **示例值**: "0b**12"
    #[api(kind = "path", name = "sheet_id")]
    pub sheet_id: String,
    /// 浮动图片 id
    ///
    /// **示例值**: "ye06SS14ph"
    #[api(kind = "path", name = "float_image_id")]
    pub float_image_id: String,

    /// 【更新时不用传，创建需要】浮动图片 token，需要先上传图片到表格获得此 token 之后再进行浮动图片的相关操作
    ///
    /// **示例值**: "boxbcbQsaSqIXsxxxxx1HCPJFbh"
    #[api(kind = "body", name = "float_image_token")]
    pub float_image_token: Option<String>,
    /// 浮动图片的左上角单元格定位，只支持一个单元格
    ///
    /// **示例值**: "0b**12!A1:A1"
    #[api(kind = "body", name = "range")]
    pub range: Option<String>,
    /// 浮动图片的宽度，大于等于 20px
    ///
    /// **示例值**: "100"
    #[api(kind = "body", name = "width")]
    pub width: Option<f64>,
    /// 浮动图片的高度，大于等于 20px
    ///
    /// **示例值**: "100"
    #[api(kind = "body", name = "height")]
    pub height: Option<f64>,
    /// 浮动图片左上角所在位置相对于所在单元格左上角的横向偏移，大于等于0且小于所在单元格的宽度
    ///
    /// **示例值**: "0"
    #[api(kind = "body", name = "offset_x")]
    pub offset_x: Option<f64>,
    /// 浮动图片左上角所在位置相对于所在单元格左上角的纵向偏移，大于等于0且小于所在单元格的高度
    ///
    /// **示例值**: "0"
    #[api(kind = "body", name = "offset_y")]
    pub offset_y: Option<f64>,
}

#[derive(Debug, Serialize, Deserialize, Clone, lark_bot_sdk_macros::ApiBaseResp)]
struct UpdateSheetFloatImageRespInner {
    #[serde(flatten)]
    data: Option<UpdateSheetFloatImageResp>,
    #[serde(flatten)]
    base: BaseResp,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct UpdateSheetFloatImageResp {
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
    /// 浮动图片信息
    #[serde(
        rename = "float_image",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub float_image: FloatImageSubResp,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(default)]
pub struct FloatImageSubResp {
    /// 浮动图片 id
    ///
    /// **示例值**: "ye06SS14ph"
    #[serde(
        rename = "float_image_id",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub float_image_id: String,
    /// 【更新时不用传，创建需要】浮动图片 token，需要先上传图片到表格获得此 token 之后再进行浮动图片的相关操作
    ///
    /// **示例值**: "boxbcbQsaSqIXsxxxxx1HCPJFbh"
    #[serde(
        rename = "float_image_token",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub float_image_token: String,
    /// 浮动图片的左上角单元格定位，只支持一个单元格
    ///
    /// **示例值**: "0b**12!A1:A1"
    #[serde(
        rename = "range",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub range: String,
    /// 浮动图片的宽度，大于等于 20px
    ///
    /// **示例值**: "100"
    #[serde(
        rename = "width",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub width: f64,
    /// 浮动图片的高度，大于等于 20px
    ///
    /// **示例值**: "100"
    #[serde(
        rename = "height",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub height: f64,
    /// 浮动图片左上角所在位置相对于所在单元格左上角的横向偏移，大于等于0且小于所在单元格的宽度
    ///
    /// **示例值**: "0"
    #[serde(
        rename = "offset_x",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub offset_x: f64,
    /// 浮动图片左上角所在位置相对于所在单元格左上角的纵向偏移，大于等于0且小于所在单元格的高度
    ///
    /// **示例值**: "0"
    #[serde(
        rename = "offset_y",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub offset_y: f64,
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
        Fn(UpdateSheetFloatImageReq) -> Result<(UpdateSheetFloatImageResp, CommonResponse), Error>
        + Send
        + Sync
        + 'static
    {
    }
    impl<
            T: Fn(
                    UpdateSheetFloatImageReq,
                ) -> Result<(UpdateSheetFloatImageResp, CommonResponse), Error>
                + Send
                + Sync
                + 'static,
        > MockFunc for T
    {
    }

    impl<'c, IStore: Store, IClient: HttpClient> DriveServiceMocker<'c, IStore, IClient> {
        pub fn mock_update_sheet_float_image<F: MockFunc>(
            &self,
            f: F,
        ) -> MockerBuilder<
            Mocker,
            UpdateSheetFloatImageReq,
            UpdateSheetFloatImageResp,
            Arc<dyn MockFunc>,
        > {
            MockerBuilder::new(self.cli.instance_id, Arc::new(f))
        }

        pub(super) fn get_mock_update_sheet_float_image(
            &self,
            req: &UpdateSheetFloatImageReq,
        ) -> Option<Arc<dyn MockFunc>> {
            do_mock::<Mocker, UpdateSheetFloatImageReq, UpdateSheetFloatImageResp, Arc<dyn MockFunc>>(
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
        api::gen::drive::update_sheet_float_image::{
            UpdateSheetFloatImageReq, UpdateSheetFloatImageResp, UpdateSheetFloatImageRespInner,
        },
        core::{model::CommonResponse, Lark},
    };

    #[tokio::test]
    async fn test_mock() {
        let lark = Lark::new("".to_owned(), "".into());
        let mocker = lark
            .drive()
            .mock()
            .mock_update_sheet_float_image(|_| {
                Ok((
                    UpdateSheetFloatImageResp::default(),
                    CommonResponse::default(),
                ))
            })
            .build();
        let res = lark
            .drive()
            .update_sheet_float_image(UpdateSheetFloatImageReq::default())
            .await;
        assert!(res.is_ok());
        mocker.clear();
        let res = lark
            .drive()
            .update_sheet_float_image(UpdateSheetFloatImageReq::default())
            .await;
        assert!(res.is_err());
    }

    const REQ: &str = r#"{
    "float_image_token": "boxbcbQsaSqIXsxxxxx1HCPJFbh",
    "range": "0b**12!A1:A1",
    "width": 100,
    "height": 100,
    "offset_x": 0,
    "offset_y": 0
}"#;

    #[test]
    fn test_req() {
        if REQ == "{}" {
            return;
        }
        if let Err(e) = serde_json::from_str::<super::UpdateSheetFloatImageReqBody>(REQ) {
            panic!("{}", e);
        }
    }

    const RESP: &str = r#"{
    "code": 0,
    "msg": "success",
    "data": {
        "float_image": {
            "float_image_id": "ye06SS14ph",
            "float_image_token": "boxbcbQsaSqIXsxxxxx1HCPJFbh",
            "range": "0b**12!A1:A1",
            "width": 100,
            "height": 100,
            "offset_x": 0,
            "offset_y": 0
        }
    }
}"#;
    #[test]
    fn test_resp() {
        let res = serde_json::from_str::<UpdateSheetFloatImageRespInner>(RESP);
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
