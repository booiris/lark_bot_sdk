//! doc url: <https://open.larkoffice.com/document/uAjLw4CM/ukTMukTMukTM/reference/drive-v1/file-view_record/list>
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
    /// **api 版本: 2024-05-29T08:43:43+00:00**
    ///
    /// ## 获取文件访问记录
    ///
    /// 获取文档、电子表格、多维表格等文件的历史访问记录，包括访问者的 ID、姓名、头像和最近访问时间。
    ///

    ///
    /// doc: <https://open.larkoffice.com/document/uAjLw4CM/ukTMukTMukTM/reference/drive-v1/file-view_record/list>
    ///
    /// new doc: <https://open.larkoffice.com/document/server-docs/docs/drive-v1/file-view_record/list>
    ///
    /// api url: <https://open.feishu.cn/document_portal/v1/document/get_detail?fullPath=%2Fserver-docs%2Fdocs%2Fdrive-v1%2Ffile-view_record%2Flist>
    pub async fn get_drive_file_view_record_list(
        &self,
        req: GetDriveFileViewRecordListReq,
    ) -> Result<(GetDriveFileViewRecordListResp, CommonResponse), Error> {
        self.get_drive_file_view_record_list_with_opt(req, Default::default())
            .await
    }

    /// 参见 [get_drive_file_view_record_list](#method.get_drive_file_view_record_list) 函数
    pub async fn get_drive_file_view_record_list_with_opt(
        &self,
        req: GetDriveFileViewRecordListReq,
        method_option: MethodOption,
    ) -> Result<(GetDriveFileViewRecordListResp, CommonResponse), Error> {
        #[cfg(feature = "test-util")]
        {
            if let Some(f) = self.mock().get_mock_get_drive_file_view_record_list(&req) {
                tracing::info!("[lark] Drive#GetDriveFileViewRecordList **mocking** api");
                return f(req);
            }
        }

        tracing::info!("[lark] Drive#GetDriveFileViewRecordList call api");

        let req = ApiRequest {
            scope: "Drive",
            api: "GetDriveFileViewRecordList",
            method: http::Method::GET,
            url: String::new()
                + self.cli.open_base_url.as_ref()
                + "/open-apis/drive/v1/files/:file_token/view_records",
            param_data: req.gen_param(),
            method_option,
            need_tenant_access_token: true,
            need_user_access_token: true,
            ..Default::default()
        };

        let (resp, common_resp): (GetDriveFileViewRecordListRespInner, _) =
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
pub struct GetDriveFileViewRecordListReq {
    /// 文件 token。获取方式参考[文件概述](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/drive-v1/file/file-overview)。
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "XIHSdYSI7oMEU1xrsnxc8fabcef"
    #[api(kind = "path", name = "file_token")]
    pub file_token: String,
    /// 分页大小
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "10"
    #[api(kind = "query", name = "page_size", v_type = "var", option = "false")]
    pub page_size: i64,
    /// 分页标记，第一次请求不填，表示从头开始遍历；分页查询结果还有更多项时会同时返回新的 page_token，下次遍历可采用该 page_token 获取查询结果
    ///
    /// **示例值**: "1674037112--7189934631754563585"
    #[api(kind = "query", name = "page_token", v_type = "var", option = "false")]
    pub page_token: String,
    /// 文件类型
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "docx"
    ///
    /// **可选值**:
    ///
    /// `doc`: 旧版文档
    ///
    /// `docx`: 新版文档
    ///
    /// `sheet`: 电子表格
    ///
    /// `bitable`: 多维表格
    ///
    /// `mindnote`: 思维笔记
    ///
    /// `wiki`: 知识库文档
    ///
    /// `file`: 文件
    #[api(kind = "query", name = "file_type", v_type = "var", option = "false")]
    pub file_type: String,
    /// 返回的访问者 ID 的类型。
    ///
    /// **当值为`user_id`时，字段权限要求**：
    ///
    /// <md-perm name="contact:user.employee_id:readonly" desc="获取用户 user ID" support_app_types="custom" tags="">获取用户 user ID</md-perm>
    ///
    /// **示例值**: "open_id"
    ///
    /// **可选值**:
    ///
    /// `user_id`: 标识一个用户在某个租户内的身份。同一个用户在租户 A 和租户 B 内的 User ID 是不同的。在同一个租户内，一个用户的 User ID 在所有应用（包括商店应用）中都保持一致。User ID 主要用于在不同的应用间打通用户数据。[了解更多：如何获取 User ID？](https://open.feishu.cn/document/uAjLw4CM/ugTN1YjL4UTN24CO1UjN/trouble-shooting/how-to-obtain-user-id)
    ///
    /// `union_id`: 标识一个用户在某个应用开发商下的身份。同一用户在同一开发商下的应用中的 Union ID 是相同的，在不同开发商下的应用中的 Union ID 是不同的。通过 Union ID，应用开发商可以把同个用户在多个应用中的身份关联起来。[了解更多：如何获取 Union ID？](https://open.feishu.cn/document/uAjLw4CM/ugTN1YjL4UTN24CO1UjN/trouble-shooting/how-to-obtain-union-id)
    ///
    /// `open_id`: 标识一个用户在某个应用中的身份。同一个用户在不同应用中的 Open ID 不同。[了解更多：如何获取 Open ID](https://open.feishu.cn/document/uAjLw4CM/ugTN1YjL4UTN24CO1UjN/trouble-shooting/how-to-obtain-openid)
    #[api(
        kind = "query",
        name = "viewer_id_type",
        v_type = "var",
        option = "false"
    )]
    pub viewer_id_type: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, lark_bot_sdk_macros::ApiBaseResp)]
struct GetDriveFileViewRecordListRespInner {
    #[serde(flatten)]
    data: Option<GetDriveFileViewRecordListResp>,
    #[serde(flatten)]
    base: BaseResp,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct GetDriveFileViewRecordListResp {
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
    /// 访问记录列表
    #[serde(
        rename = "items",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub items: Vec<FileViewRecordSubResp>,
    /// 分页标记，当 has_more 为 true 时，会同时返回新的 page_token，否则不返回 page_token
    ///
    /// **示例值**: "1674037112--7189934631754563585"
    #[serde(
        rename = "page_token",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub page_token: String,
    /// 是否还有更多项
    ///
    /// **示例值**: "true"
    #[serde(
        rename = "has_more",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub has_more: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(default)]
pub struct FileViewRecordSubResp {
    /// 访问者 ID
    ///
    /// **示例值**: "ou_cc19b2bfb93f8a44db4b4d6eababcef"
    #[serde(
        rename = "viewer_id",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub viewer_id: String,
    /// 访问者姓名
    ///
    /// **示例值**: "zhangsan"
    #[serde(
        rename = "name",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub name: String,
    /// 访问者头像的 URL
    ///
    /// **示例值**: "https://foo.icon.com/xxxx"
    #[serde(
        rename = "avatar_url",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub avatar_url: String,
    /// 最近访问时间。Unix 时间戳，单位为秒。
    ///
    /// **示例值**: "1679284285"
    #[serde(
        rename = "last_view_time",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub last_view_time: String,
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
        Fn(
            GetDriveFileViewRecordListReq,
        ) -> Result<(GetDriveFileViewRecordListResp, CommonResponse), Error>
        + Send
        + Sync
        + 'static
    {
    }
    impl<
            T: Fn(
                    GetDriveFileViewRecordListReq,
                )
                    -> Result<(GetDriveFileViewRecordListResp, CommonResponse), Error>
                + Send
                + Sync
                + 'static,
        > MockFunc for T
    {
    }

    impl<'c, IStore: Store, IClient: HttpClient> DriveServiceMocker<'c, IStore, IClient> {
        pub fn mock_get_drive_file_view_record_list<F: MockFunc>(
            &self,
            f: F,
        ) -> MockerBuilder<
            Mocker,
            GetDriveFileViewRecordListReq,
            GetDriveFileViewRecordListResp,
            Arc<dyn MockFunc>,
        > {
            MockerBuilder::new(self.cli.instance_id, Arc::new(f))
        }

        pub(super) fn get_mock_get_drive_file_view_record_list(
            &self,
            req: &GetDriveFileViewRecordListReq,
        ) -> Option<Arc<dyn MockFunc>> {
            do_mock::<
                Mocker,
                GetDriveFileViewRecordListReq,
                GetDriveFileViewRecordListResp,
                Arc<dyn MockFunc>,
            >(self.cli.instance_id, req)
        }
    }
}

#[cfg(feature = "test-util")]
#[cfg(test)]
mod test {
    use crate::{
        api::gen::drive::get_drive_file_view_record_list::{
            GetDriveFileViewRecordListReq, GetDriveFileViewRecordListResp,
            GetDriveFileViewRecordListRespInner,
        },
        core::{model::CommonResponse, Lark},
    };

    #[tokio::test]
    async fn test_mock() {
        let lark = Lark::new("".to_owned(), "".into());
        let mocker = lark
            .drive()
            .mock()
            .mock_get_drive_file_view_record_list(|_| {
                Ok((
                    GetDriveFileViewRecordListResp::default(),
                    CommonResponse::default(),
                ))
            })
            .build();
        let res = lark
            .drive()
            .get_drive_file_view_record_list(GetDriveFileViewRecordListReq::default())
            .await;
        assert!(res.is_ok());
        mocker.clear();
        let res = lark
            .drive()
            .get_drive_file_view_record_list(GetDriveFileViewRecordListReq::default())
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
        "items": [
            {
                "viewer_id": "ou_cc19b2bfb93f8a44db4b4d6eababcef",
                "name": "zhangsan",
                "avatar_url": "https://foo.icon.com/xxxx",
                "last_view_time": "1679284285"
            }
        ],
        "page_token": "1674037112--7189934631754563585",
        "has_more": true
    }
}"#;
    #[test]
    fn test_resp() {
        let res = serde_json::from_str::<GetDriveFileViewRecordListRespInner>(RESP);
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
