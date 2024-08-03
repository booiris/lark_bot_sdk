//! doc url: <https://open.feishu.cn/document/ukTMukTMukTM/uUDN04SN0QjL1QDN/wiki-v2/task/get>
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
    /// ## 获取任务结果
    ///
    /// 该方法用于获取wiki异步任务的结果
    ///
    /// 知识库权限要求，当前 access token 所代表的用户或应用（机器人）：
    ///
    /// - 为任务创建者
    ///
    /// doc: <https://open.feishu.cn/document/ukTMukTMukTM/uUDN04SN0QjL1QDN/wiki-v2/task/get>
    ///
    /// new doc: <https://open.feishu.cn/document/server-docs/docs/wiki-v2/task/get>
    ///
    /// api url: <https://open.feishu.cn/document_portal/v1/document/get_detail?fullPath=%2Fserver-docs%2Fdocs%2Fwiki-v2%2Ftask%2Fget>
    pub async fn get_wiki_task(
        &self,
        req: GetWikiTaskReq,
    ) -> Result<(GetWikiTaskResp, CommonResponse), Error> {
        self.get_wiki_task_with_opt(req, Default::default()).await
    }

    /// 参见 [get_wiki_task](#method.get_wiki_task) 函数
    pub async fn get_wiki_task_with_opt(
        &self,
        req: GetWikiTaskReq,
        method_option: MethodOption,
    ) -> Result<(GetWikiTaskResp, CommonResponse), Error> {
        #[cfg(feature = "test-util")]
        {
            if let Some(f) = self.mock().get_mock_get_wiki_task(&req) {
                tracing::info!("[lark] Drive#GetWikiTask **mocking** api");
                return f(req);
            }
        }

        tracing::info!("[lark] Drive#GetWikiTask call api");

        let req = ApiRequest {
            scope: "Drive",
            api: "GetWikiTask",
            method: http::Method::GET,
            url: String::new()
                + self.cli.open_base_url.as_ref()
                + "/open-apis/wiki/v2/tasks/:task_id",
            param_data: req.gen_param(),
            method_option,
            need_tenant_access_token: true,
            need_user_access_token: true,
            ..Default::default()
        };

        let (resp, common_resp): (GetWikiTaskRespInner, _) = self.cli.do_req(req).await?;
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
pub struct GetWikiTaskReq {
    /// 任务id
    ///
    /// **示例值**: "7037044037068177428-075c9481e6a0007c1df689dfbe5b55a08b6b06f7"
    #[api(kind = "path", name = "task_id")]
    pub task_id: String,
    /// 任务类型
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "move"
    ///
    /// **可选值**:
    ///
    /// `Move`: [移动云空间文档至知识空间](https://open.feishu.cn/document/ukTMukTMukTM/uUDN04SN0QjL1QDN/wiki-v2/space-node/move_docs_to_wiki)任务
    #[api(kind = "query", name = "task_type", v_type = "var", option = "false")]
    pub task_type: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, lark_bot_sdk_macros::ApiBaseResp)]
struct GetWikiTaskRespInner {
    #[serde(flatten)]
    data: Option<GetWikiTaskResp>,
    #[serde(flatten)]
    base: BaseResp,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct GetWikiTaskResp {
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
    /// 任务结果
    ///
    /// **是否必填**: 是
    #[serde(
        rename = "task",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub task: TaskResultSubResp,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(default)]
pub struct TaskResultSubResp {
    /// 任务id
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "7037044037068177428-075c9481e6a0007c1df689dfbe5b55a08b6b06f7"
    #[serde(
        rename = "task_id",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub task_id: String,
    /// [移动云空间文档至知识空间](https://open.feishu.cn/document/ukTMukTMukTM/uUDN04SN0QjL1QDN/wiki-v2/space-node/move_docs_to_wiki)任务结果
    #[serde(
        rename = "move_result",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub move_result: Vec<MoveResultSubResp>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(default)]
pub struct MoveResultSubResp {
    /// 移动完成的节点信息
    ///
    /// **是否必填**: 是
    #[serde(
        rename = "node",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub node: NodeSubResp,
    /// 节点移动状态码
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "0"
    #[serde(
        rename = "status",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub status: i64,
    /// 节点移动状态信息
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "success"
    #[serde(
        rename = "status_msg",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub status_msg: String,
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
        Fn(GetWikiTaskReq) -> Result<(GetWikiTaskResp, CommonResponse), Error>
        + Send
        + Sync
        + 'static
    {
    }
    impl<
            T: Fn(GetWikiTaskReq) -> Result<(GetWikiTaskResp, CommonResponse), Error>
                + Send
                + Sync
                + 'static,
        > MockFunc for T
    {
    }

    impl<'c, IStore: Store, IClient: HttpClient> DriveServiceMocker<'c, IStore, IClient> {
        pub fn mock_get_wiki_task<F: MockFunc>(
            &self,
            f: F,
        ) -> MockerBuilder<Mocker, GetWikiTaskReq, GetWikiTaskResp, Arc<dyn MockFunc>> {
            MockerBuilder::new(self.cli.instance_id, Arc::new(f))
        }

        pub(super) fn get_mock_get_wiki_task(
            &self,
            req: &GetWikiTaskReq,
        ) -> Option<Arc<dyn MockFunc>> {
            do_mock::<Mocker, GetWikiTaskReq, GetWikiTaskResp, Arc<dyn MockFunc>>(
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
        api::gen::drive::get_wiki_task::{GetWikiTaskReq, GetWikiTaskResp, GetWikiTaskRespInner},
        core::{model::CommonResponse, Lark},
    };

    #[tokio::test]
    async fn test_mock() {
        let lark = Lark::new("".to_owned(), "".into());
        let mocker = lark
            .drive()
            .mock()
            .mock_get_wiki_task(|_| Ok((GetWikiTaskResp::default(), CommonResponse::default())))
            .build();
        let res = lark.drive().get_wiki_task(GetWikiTaskReq::default()).await;
        assert!(res.is_ok());
        mocker.clear();
        let res = lark.drive().get_wiki_task(GetWikiTaskReq::default()).await;
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
        "task": {
            "task_id": "7037044037068177428-075c9481e6a0007c1df689dfbe5b55a08b6b06f7",
            "move_result": [
                {
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
                    },
                    "status": 0,
                    "status_msg": "success"
                }
            ]
        }
    }
}"#;
    #[test]
    fn test_resp() {
        let res = serde_json::from_str::<GetWikiTaskRespInner>(RESP);
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
