//! doc url: <https://open.feishu.cn/document/ukTMukTMukTM/uUDN04SN0QjL1QDN/wiki-v2/space-node/copy>
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
    /// **api 版本: 2024-07-31T11:17:57+00:00**
    ///
    /// ## 创建知识空间节点副本
    ///
    /// 此接口用于在知识空间创建节点副本到指定位置。
    ///

    ///
    /// doc: <https://open.feishu.cn/document/ukTMukTMukTM/uUDN04SN0QjL1QDN/wiki-v2/space-node/copy>
    ///
    /// new doc: <https://open.feishu.cn/document/server-docs/docs/wiki-v2/space-node/copy>
    ///
    /// api url: <https://open.feishu.cn/document_portal/v1/document/get_detail?fullPath=%2Fserver-docs%2Fdocs%2Fwiki-v2%2Fspace-node%2Fcopy>
    pub async fn copy_wiki_node(
        &self,
        req: CopyWikiNodeReq,
    ) -> Result<(CopyWikiNodeResp, CommonResponse), Error> {
        self.copy_wiki_node_with_opt(req, Default::default()).await
    }

    /// 参见 [copy_wiki_node](#method.copy_wiki_node) 函数
    pub async fn copy_wiki_node_with_opt(
        &self,
        req: CopyWikiNodeReq,
        method_option: MethodOption,
    ) -> Result<(CopyWikiNodeResp, CommonResponse), Error> {
        #[cfg(feature = "test-util")]
        {
            if let Some(f) = self.mock().get_mock_copy_wiki_node(&req) {
                tracing::info!("[lark] Drive#CopyWikiNode **mocking** api");
                return f(req);
            }
        }

        tracing::info!("[lark] Drive#CopyWikiNode call api");

        let req = ApiRequest {
            scope: "Drive",
            api: "CopyWikiNode",
            method: http::Method::POST,
            url: String::new()
                + self.cli.open_base_url.as_ref()
                + "/open-apis/wiki/v2/spaces/:space_id/nodes/:node_token/copy",
            param_data: req.gen_param(),
            method_option,
            need_tenant_access_token: true,
            need_user_access_token: true,
            ..Default::default()
        };

        let (resp, common_resp): (CopyWikiNodeRespInner, _) = self.cli.do_req(req).await?;
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
pub struct CopyWikiNodeReq {
    /// 知识空间id
    ///
    /// **示例值**: "6946843325487912356"
    #[api(kind = "path", name = "space_id")]
    pub space_id: String,
    /// 节点token
    ///
    /// **示例值**: "wikcnKQ1k3p******8Vabce"
    #[api(kind = "path", name = "node_token")]
    pub node_token: String,

    /// 目标父节点 Token。
    ///
    /// - 目标知识空间 ID 与目标父节点 Token 不可同时为空。
    ///
    /// **示例值**: "wikcnKQ1k3p******8Vabce"
    ///
    /// **数据校验规则**：
    ///
    /// - **长度范围**: `0` 字符- `999` 字符
    #[api(kind = "body", name = "target_parent_token")]
    pub target_parent_token: Option<String>,
    /// 目标知识空间 ID。
    ///
    /// - 目标知识空间 ID 与目标父节点 Token 不可同时为空。
    ///
    /// **示例值**: "6946843325487912356"
    #[api(kind = "body", name = "target_space_id")]
    pub target_space_id: Option<String>,
    /// 复制后的新标题。如果填空，则新标题为空。如果不填，则使用原节点标题。
    ///
    /// **示例值**: "新标题。"
    #[api(kind = "body", name = "title")]
    pub title: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, lark_bot_sdk_macros::ApiBaseResp)]
struct CopyWikiNodeRespInner {
    #[serde(flatten)]
    data: Option<CopyWikiNodeResp>,
    #[serde(flatten)]
    base: BaseResp,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct CopyWikiNodeResp {
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
    /// 创建副本后的新节点
    ///
    /// **是否必填**: 是
    #[serde(
        rename = "node",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub node: NodeSubResp,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(default)]
pub struct NodeSubResp {
    /// 知识空间id
    ///
    /// [获取方式](https://open.feishu.cn/document/ukTMukTMukTM/uUDN04SN0QjL1QDN/wiki-overview)
    ///
    /// **示例值**: "6946843325487912356"
    #[serde(
        rename = "space_id",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub space_id: String,
    /// 节点token
    ///
    /// **示例值**: "wikcnKQ1k3p******8Vabcef"
    #[serde(
        rename = "node_token",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub node_token: String,
    /// 对应文档类型的token，可根据 obj_type 判断属于哪种文档类型。
    ///
    /// **示例值**: "doccnzAaOD******Wabcdef"
    #[serde(
        rename = "obj_token",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub obj_token: String,
    /// 文档类型，对于快捷方式，该字段是对应的实体的obj_type。
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "doc"
    ///
    /// **可选值**:
    ///
    /// `ObjTypeDoc`: 旧版文档
    ///
    /// `ObjTypeSheet`: 表格
    ///
    /// `ObjTypeMindNote`: 思维导图
    ///
    /// `ObjTypeBitable`: 多维表格
    ///
    /// `ObjTypeFile`: 文件
    ///
    /// `ObjTypeDocx`: 新版文档
    ///
    /// `ObjTypeSlides`: 幻灯片
    #[serde(
        rename = "obj_type",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub obj_type: String,
    /// 父节点 token。若当前节点为一级节点，父节点 token 为空。
    ///
    /// **示例值**: "wikcnKQ1k3p******8Vabcef"
    #[serde(
        rename = "parent_node_token",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub parent_node_token: String,
    /// 节点类型
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "origin"
    ///
    /// **可选值**:
    ///
    /// `NodeTypeEntity`: 实体
    ///
    /// `NodeTypeShortCut`: 快捷方式
    #[serde(
        rename = "node_type",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub node_type: String,
    /// 快捷方式对应的实体node_token，当节点为快捷方式时，该值不为空。
    ///
    /// **示例值**: "wikcnKQ1k3p******8Vabcef"
    #[serde(
        rename = "origin_node_token",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub origin_node_token: String,
    /// 快捷方式对应的实体所在的space id
    ///
    /// **示例值**: "6946843325487912356"
    #[serde(
        rename = "origin_space_id",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub origin_space_id: String,
    /// 是否有子节点
    ///
    /// **示例值**: "false"
    #[serde(
        rename = "has_child",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub has_child: bool,
    /// 文档标题
    ///
    /// **示例值**: "标题"
    #[serde(
        rename = "title",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub title: String,
    /// 文档创建时间
    ///
    /// **示例值**: "1642402428"
    #[serde(
        rename = "obj_create_time",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub obj_create_time: String,
    /// 文档最近编辑时间
    ///
    /// **示例值**: "1642402428"
    #[serde(
        rename = "obj_edit_time",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub obj_edit_time: String,
    /// 节点创建时间
    ///
    /// **示例值**: "1642402428"
    #[serde(
        rename = "node_create_time",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub node_create_time: String,
    /// 节点创建者
    ///
    /// **示例值**: "ou_xxxxx"
    #[serde(
        rename = "creator",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub creator: String,
    /// 节点所有者
    ///
    /// **示例值**: "ou_xxxxx"
    #[serde(
        rename = "owner",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub owner: String,
    /// 节点创建者
    ///
    /// **示例值**: "ou_xxxxx"
    #[serde(
        rename = "node_creator",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub node_creator: String,
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
        Fn(CopyWikiNodeReq) -> Result<(CopyWikiNodeResp, CommonResponse), Error>
        + Send
        + Sync
        + 'static
    {
    }
    impl<
            T: Fn(CopyWikiNodeReq) -> Result<(CopyWikiNodeResp, CommonResponse), Error>
                + Send
                + Sync
                + 'static,
        > MockFunc for T
    {
    }

    impl<'c, IStore: Store, IClient: HttpClient> DriveServiceMocker<'c, IStore, IClient> {
        pub fn mock_copy_wiki_node<F: MockFunc>(
            &self,
            f: F,
        ) -> MockerBuilder<Mocker, CopyWikiNodeReq, CopyWikiNodeResp, Arc<dyn MockFunc>> {
            MockerBuilder::new(self.cli.instance_id, Arc::new(f))
        }

        pub(super) fn get_mock_copy_wiki_node(
            &self,
            req: &CopyWikiNodeReq,
        ) -> Option<Arc<dyn MockFunc>> {
            do_mock::<Mocker, CopyWikiNodeReq, CopyWikiNodeResp, Arc<dyn MockFunc>>(
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
        api::gen::drive::copy_wiki_node::{
            CopyWikiNodeReq, CopyWikiNodeResp, CopyWikiNodeRespInner,
        },
        core::{model::CommonResponse, Lark},
    };

    #[tokio::test]
    async fn test_mock() {
        let lark = Lark::new("".to_owned(), "".into());
        let mocker = lark
            .drive()
            .mock()
            .mock_copy_wiki_node(|_| Ok((CopyWikiNodeResp::default(), CommonResponse::default())))
            .build();
        let res = lark
            .drive()
            .copy_wiki_node(CopyWikiNodeReq::default())
            .await;
        assert!(res.is_ok());
        mocker.clear();
        let res = lark
            .drive()
            .copy_wiki_node(CopyWikiNodeReq::default())
            .await;
        assert!(res.is_err());
    }

    const REQ: &str = r#"{
    "target_parent_token": "wikcnKQ1k3p******8Vabce",
    "target_space_id": "6946843325487912356",
    "title": "新标题。"
}"#;

    #[test]
    fn test_req() {
        if REQ == "{}" {
            return;
        }
        if let Err(e) = serde_json::from_str::<super::CopyWikiNodeReqBody>(REQ) {
            panic!("{}", e);
        }
    }

    const RESP: &str = r#"{
    "code": 0,
    "msg": "success",
    "data": {
        "node": {
            "space_id": "6946843325487912356",
            "node_token": "wikcnKQ1k3p******8Vabcef",
            "obj_token": "doccnzAaOD******Wabcdef",
            "obj_type": "doc",
            "parent_node_token": "wikcnKQ1k3p******8Vabcef",
            "node_type": "origin",
            "origin_node_token": "wikcnKQ1k3p******8Vabcef",
            "origin_space_id": "6946843325487912356",
            "has_child": false,
            "title": "标题",
            "obj_create_time": "1642402428",
            "obj_edit_time": "1642402428",
            "node_create_time": "1642402428",
            "creator": "ou_xxxxx",
            "owner": "ou_xxxxx",
            "node_creator": "ou_xxxxx"
        }
    }
}"#;
    #[test]
    fn test_resp() {
        let res = serde_json::from_str::<CopyWikiNodeRespInner>(RESP);
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
