//! doc url: <https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/drive-v1/file/copy>
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
    /// **api 版本: 2024-07-23T07:32:25+00:00**
    ///
    /// ## 复制文件
    ///
    /// 该接口用于将用户云空间中的文件复制至其它文件夹下。不支持复制文件夹。
    ///

    ///
    /// doc: <https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/drive-v1/file/copy>
    ///
    /// new doc: <https://open.feishu.cn/document/server-docs/docs/drive-v1/file/copy>
    ///
    /// api url: <https://open.feishu.cn/document_portal/v1/document/get_detail?fullPath=%2Fserver-docs%2Fdocs%2Fdrive-v1%2Ffile%2Fcopy>
    pub async fn copy_drive_file(
        &self,
        req: CopyDriveFileReq,
    ) -> Result<(CopyDriveFileResp, CommonResponse), Error> {
        self.copy_drive_file_with_opt(req, Default::default()).await
    }

    /// 参见 [copy_drive_file](#method.copy_drive_file) 函数
    pub async fn copy_drive_file_with_opt(
        &self,
        req: CopyDriveFileReq,
        method_option: MethodOption,
    ) -> Result<(CopyDriveFileResp, CommonResponse), Error> {
        #[cfg(feature = "test-util")]
        {
            if let Some(f) = self.mock().get_mock_copy_drive_file(&req) {
                tracing::info!("[lark] Drive#CopyDriveFile **mocking** api");
                return f(req);
            }
        }

        tracing::info!("[lark] Drive#CopyDriveFile call api");

        let req = ApiRequest {
            scope: "Drive",
            api: "CopyDriveFile",
            method: http::Method::POST,
            url: String::new()
                + self.cli.open_base_url.as_ref()
                + "/open-apis/drive/v1/files/:file_token/copy",
            param_data: req.gen_param(),
            method_option,
            need_tenant_access_token: true,
            need_user_access_token: true,
            ..Default::default()
        };

        let (resp, common_resp): (CopyDriveFileRespInner, _) = self.cli.do_req(req).await?;
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
pub struct CopyDriveFileReq {
    /// 被复制的源文件的 token。了解如何获取文件 token，参考[文件概述](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/drive-v1/file/file-overview)。
    ///
    /// **示例值**: "doccngpahSdXrFPIBD4XdIabcef"
    #[api(kind = "path", name = "file_token")]
    pub file_token: String,
    /// 用户 ID 类型
    ///
    /// **示例值**: "open_id"
    ///
    /// **可选值**:
    ///
    /// `open_id`: 标识一个用户在某个应用中的身份。同一个用户在不同应用中的 Open ID 不同。[了解更多：如何获取 Open ID](https://open.feishu.cn/document/uAjLw4CM/ugTN1YjL4UTN24CO1UjN/trouble-shooting/how-to-obtain-openid)
    ///
    /// `union_id`: 标识一个用户在某个应用开发商下的身份。同一用户在同一开发商下的应用中的 Union ID 是相同的，在不同开发商下的应用中的 Union ID 是不同的。通过 Union ID，应用开发商可以把同个用户在多个应用中的身份关联起来。[了解更多：如何获取 Union ID？](https://open.feishu.cn/document/uAjLw4CM/ugTN1YjL4UTN24CO1UjN/trouble-shooting/how-to-obtain-union-id)
    ///
    /// `user_id`: 标识一个用户在某个租户内的身份。同一个用户在租户 A 和租户 B 内的 User ID 是不同的。在同一个租户内，一个用户的 User ID 在所有应用（包括商店应用）中都保持一致。User ID 主要用于在不同的应用间打通用户数据。[了解更多：如何获取 User ID？](https://open.feishu.cn/document/uAjLw4CM/ugTN1YjL4UTN24CO1UjN/trouble-shooting/how-to-obtain-user-id)
    #[api(
        kind = "query",
        name = "user_id_type",
        v_type = "var",
        option = "false"
    )]
    pub user_id_type: String,
    /// 复制的新文件的名称
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "Demo copy"
    #[api(kind = "body", name = "name")]
    pub name: String,
    /// 被复制的源文件的类型。该参数为必填，请忽略左侧必填列的“否”。若该参数值为空或与实际文件类型不匹配，接口将返回失败。
    ///
    /// **示例值**: "doc"
    ///
    /// **可选值**:
    ///
    /// `File`: 文件类型
    ///
    /// `Doc`: 文档类型
    ///
    /// `Sheet`: 电子表格类型
    ///
    /// `Bitable`: 多维表格类型
    ///
    /// `Docx`: 新版文档类型
    ///
    /// `Mindnote`: 思维笔记类型
    ///
    /// `Slides`: 幻灯片类型
    #[api(kind = "body", name = "type")]
    pub body_type: Option<String>,
    /// 目标文件夹的 token。若传入根文件夹 token，表示复制的新文件将被创建在云空间根目录。了解如何获取文件夹 token，参考[文件夹概述](https://open.feishu.cn/document/ukTMukTMukTM/ugTNzUjL4UzM14CO1MTN/folder-overview)。
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "fldbcO1UuPz8VwnpPx5a92abcef"
    #[api(kind = "body", name = "folder_token")]
    pub folder_token: String,
    /// 自定义请求附加参数，用于实现特殊的复制语义
    #[api(kind = "body", name = "extra")]
    pub extra: Vec<Option<PropertySubReq>>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(default)]
pub struct PropertySubReq {
    /// 自定义属性键对象
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "target_type"
    #[serde(
        rename = "key",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub key: String,
    /// 自定义属性值对象
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "docx"
    #[serde(
        rename = "value",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub value: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, lark_bot_sdk_macros::ApiBaseResp)]
struct CopyDriveFileRespInner {
    #[serde(flatten)]
    data: Option<CopyDriveFileResp>,
    #[serde(flatten)]
    base: BaseResp,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct CopyDriveFileResp {
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
    /// 复制的新文件信息
    #[serde(
        rename = "file",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub file: FileSubResp,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(default)]
pub struct FileSubResp {
    /// 复制的新文件 token
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "doxcnUkUOWtOelpFcha2Zabcef"
    #[serde(
        rename = "token",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub token: String,
    /// 新文件的名称
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "Demo copy"
    ///
    /// **数据校验规则**：
    ///
    /// - **最大长度**: `250` 字符
    #[serde(
        rename = "name",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub name: String,
    /// 新文件的类型
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "docx"
    #[serde(
        rename = "type",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub body_type: String,
    /// 新文件的父文件夹 token
    ///
    /// **示例值**: "fldbcO1UuPz8VwnpPx5a92abcef"
    #[serde(
        rename = "parent_token",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub parent_token: String,
    /// 文件在浏览器中的 URL 链接
    ///
    /// **示例值**: "https://feishu.cn/docx/doxcnUkUOWtOelpFcha2Zabcef"
    #[serde(
        rename = "url",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub url: String,
    /// 快捷方式文件信息（该参数不会返回）
    #[serde(
        rename = "shortcut_info",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub shortcut_info: ShortcutInfoSubResp,
    /// 文件创建时间（该参数不会返回）
    ///
    /// **示例值**: "1686125119"
    #[serde(
        rename = "created_time",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub created_time: String,
    /// 文件最近修改时间（该参数不会返回）
    ///
    /// **示例值**: "1686125119"
    #[serde(
        rename = "modified_time",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub modified_time: String,
    /// 文件所有者（该参数不会返回）
    ///
    /// **示例值**: "ou_b13d41c02edc52ce66aaae67bf1abcef"
    #[serde(
        rename = "owner_id",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub owner_id: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(default)]
pub struct ShortcutInfoSubResp {
    /// 快捷方式指向的源文件类型
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "docx"
    #[serde(
        rename = "target_type",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub target_type: String,
    /// 快捷方式指向的源文件 token
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "doxcnUkUOWtOelpFcha2Zabcef"
    #[serde(
        rename = "target_token",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub target_token: String,
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
        Fn(CopyDriveFileReq) -> Result<(CopyDriveFileResp, CommonResponse), Error>
        + Send
        + Sync
        + 'static
    {
    }
    impl<
            T: Fn(CopyDriveFileReq) -> Result<(CopyDriveFileResp, CommonResponse), Error>
                + Send
                + Sync
                + 'static,
        > MockFunc for T
    {
    }

    impl<'c, IStore: Store, IClient: HttpClient> DriveServiceMocker<'c, IStore, IClient> {
        pub fn mock_copy_drive_file<F: MockFunc>(
            &self,
            f: F,
        ) -> MockerBuilder<Mocker, CopyDriveFileReq, CopyDriveFileResp, Arc<dyn MockFunc>> {
            MockerBuilder::new(self.cli.instance_id, Arc::new(f))
        }

        pub(super) fn get_mock_copy_drive_file(
            &self,
            req: &CopyDriveFileReq,
        ) -> Option<Arc<dyn MockFunc>> {
            do_mock::<Mocker, CopyDriveFileReq, CopyDriveFileResp, Arc<dyn MockFunc>>(
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
        api::gen::drive::copy_drive_file::{
            CopyDriveFileReq, CopyDriveFileResp, CopyDriveFileRespInner,
        },
        core::{model::CommonResponse, Lark},
    };

    #[tokio::test]
    async fn test_mock() {
        let lark = Lark::new("".to_owned(), "".into());
        let mocker = lark
            .drive()
            .mock()
            .mock_copy_drive_file(|_| Ok((CopyDriveFileResp::default(), CommonResponse::default())))
            .build();
        let res = lark
            .drive()
            .copy_drive_file(CopyDriveFileReq::default())
            .await;
        assert!(res.is_ok());
        mocker.clear();
        let res = lark
            .drive()
            .copy_drive_file(CopyDriveFileReq::default())
            .await;
        assert!(res.is_err());
    }

    const REQ: &str = r#"{
    "name": "Demo copy",
    "type": "file",
    "folder_token": "fldbcO1UuPz8VwnpPx5a92abcef"
}"#;

    #[test]
    fn test_req() {
        if REQ == "{}" {
            return;
        }
        if let Err(e) = serde_json::from_str::<super::CopyDriveFileReqBody>(REQ) {
            panic!("{}", e);
        }
    }

    const RESP: &str = r#"{
    "code": 0,
    "data": {
        "file": {
            "name": "Demo copy",
            "parent_token": "fldcnBh8LrnX42dr1pBYclabcef",
            "token": "doxcnUkUOWtOelpFcha2Z9abcef",
            "type": "docx",
            "url": "https://feishu.cn/docx/doxcnUkUOWtOelpFcha2Zabcef"
        }
    },
    "msg": "success"
}"#;
    #[test]
    fn test_resp() {
        let res = serde_json::from_str::<CopyDriveFileRespInner>(RESP);
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
