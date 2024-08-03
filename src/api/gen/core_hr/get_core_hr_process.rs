//! doc url: <https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/corehr-v2/process/get>
// Code generated by gen_api. DO NOT EDIT.

use serde::{Deserialize, Serialize};

use crate::api::BaseResp;
use crate::api::HasBaseResp;
use crate::core::model::{ApiRequest, CommonResponse, MethodOption, ReqParam, StreamReqParam};
use crate::{
    core::{http_client::HttpClient, store::Store},
    error::Error,
};

use crate::api::gen::core_hr::CoreHrService;

impl<'c, IStore: Store, IClient: HttpClient> CoreHrService<'c, IStore, IClient> {
    /// **api 版本: 2024-08-01T02:51:42+00:00**
    ///
    /// ## 获取单个流程详情
    ///
    /// 根据流程实例 id（process_id）获取单个流程详情。比如流程状态、流程发起人、流程发起时间、流程摘要、流程里的所有待办、已办、抄送任务等。
    ///

    ///
    /// doc: <https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/corehr-v2/process/get>
    ///
    /// new doc: <https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/corehr-v2/process/get>
    ///
    /// api url: <https://open.feishu.cn/document_portal/v1/document/get_detail?fullPath=%2FuAjLw4CM%2FukTMukTMukTM%2Fcorehr-v2%2Fprocess%2Fget>
    pub async fn get_core_hr_process(
        &self,
        req: GetCoreHrProcessReq,
    ) -> Result<(GetCoreHrProcessResp, CommonResponse), Error> {
        self.get_core_hr_process_with_opt(req, Default::default())
            .await
    }

    /// 参见 [get_core_hr_process](#method.get_core_hr_process) 函数
    pub async fn get_core_hr_process_with_opt(
        &self,
        req: GetCoreHrProcessReq,
        method_option: MethodOption,
    ) -> Result<(GetCoreHrProcessResp, CommonResponse), Error> {
        #[cfg(feature = "test-util")]
        {
            if let Some(f) = self.mock().get_mock_get_core_hr_process(&req) {
                tracing::info!("[lark] CoreHr#GetCoreHrProcess **mocking** api");
                return f(req);
            }
        }

        tracing::info!("[lark] CoreHr#GetCoreHrProcess call api");

        let req = ApiRequest {
            scope: "CoreHr",
            api: "GetCoreHrProcess",
            method: http::Method::GET,
            url: String::new()
                + self.cli.open_base_url.as_ref()
                + "/open-apis/corehr/v2/processes/:process_id",
            param_data: req.gen_param(),
            method_option,
            need_tenant_access_token: true,
            ..Default::default()
        };

        let (resp, common_resp): (GetCoreHrProcessRespInner, _) = self.cli.do_req(req).await?;
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
pub struct GetCoreHrProcessReq {
    /// 流程实例ID。可通过[查询流程实例列表](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/corehr-v2/process/list)接口获取
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "7278949005675988535"
    #[api(kind = "path", name = "process_id")]
    pub process_id: String,
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
    ///
    /// `people_corehr_id`: 以飞书人事的 ID 来识别用户
    #[api(
        kind = "query",
        name = "user_id_type",
        v_type = "var",
        option = "false"
    )]
    pub user_id_type: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, lark_bot_sdk_macros::ApiBaseResp)]
struct GetCoreHrProcessRespInner {
    #[serde(flatten)]
    data: Option<GetCoreHrProcessResp>,
    #[serde(flatten)]
    base: BaseResp,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct GetCoreHrProcessResp {
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
    /// 流程实例ID
    ///
    /// **示例值**: "7278949005675988535"
    #[serde(
        rename = "process_id",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub process_id: String,
    /// 流程状态
    ///
    /// **示例值**: "1"
    ///
    /// **可选值**:
    ///
    /// `Running`: 进行中
    ///
    /// `Reject`: 拒绝
    ///
    /// `Withdraw`: 撤回
    ///
    /// `Revoke`: 撤销
    ///
    /// `Complete`: 已完成
    #[serde(
        rename = "status",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub status: i64,
    /// 业务类型ID
    ///
    /// **示例值**: "leave"
    #[serde(
        rename = "flow_template_id",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub flow_template_id: String,
    /// 业务类型名称
    #[serde(
        rename = "flow_template_name",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub flow_template_name: DataengineI18nSubResp,
    /// 流程定义ID
    ///
    /// **示例值**: "people_6961286846093788680_7081951411982077732"
    #[serde(
        rename = "flow_definition_id",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub flow_definition_id: String,
    /// 流程定义名称
    #[serde(
        rename = "flow_definition_name",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub flow_definition_name: DataengineI18nSubResp,
    /// 流程发起人ID
    ///
    /// **示例值**: "7124991993901827628"
    #[serde(
        rename = "initiator_id",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub initiator_id: String,
    /// 流程发起人姓名
    #[serde(
        rename = "initiator_name",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub initiator_name: DataengineI18nSubResp,
    /// 流程发起时间，Unix毫秒时间戳
    ///
    /// **示例值**: "1694769814036"
    #[serde(
        rename = "create_time",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub create_time: String,
    /// 流程结束时间，Unix毫秒时间戳
    ///
    /// **示例值**: "1694769814036"
    #[serde(
        rename = "complete_time",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub complete_time: String,
    /// 发起单据地址
    #[serde(
        rename = "start_links",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub start_links: ProcessLinkSubResp,
    /// 流程摘要，会随着流程流转发生变化
    #[serde(
        rename = "abstracts",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub abstracts: Vec<ProcessAbstractItemSubResp>,
    /// 待办列表
    #[serde(
        rename = "todos",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub todos: Vec<ProcessTodoItemSubResp>,
    /// 抄送列表
    #[serde(
        rename = "cc_list",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub cc_list: Vec<ProcessCcItemSubResp>,
    /// 已办列表
    #[serde(
        rename = "done_list",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub done_list: Vec<ProcessDoneItemSubResp>,
    /// 普通流程或撤销流程等
    ///
    /// **示例值**: "1"
    ///
    /// **可选值**:
    ///
    /// `Common`: 普通流程
    ///
    /// `CheXiao`: 撤销流程
    #[serde(
        rename = "properties",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub properties: i64,
    /// 系统待办列表
    #[serde(
        rename = "system_todos",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub system_todos: Vec<ProcessSystemTodoItemSubResp>,
    /// 系统已办列表
    #[serde(
        rename = "system_done_list",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub system_done_list: Vec<ProcessSystemDoneItemSubResp>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(default)]
pub struct DataengineI18nSubResp {
    /// 中文值
    ///
    /// **示例值**: "中文"
    #[serde(
        rename = "zh_cn",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub zh_cn: String,
    /// 英文值
    ///
    /// **示例值**: "English"
    #[serde(
        rename = "en_us",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub en_us: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(default)]
pub struct ProcessLinkSubResp {
    /// web端单据详情页地址
    ///
    /// **示例值**: "http://xxxx.com/xxx/xxx?a=b"
    #[serde(
        rename = "web_link",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub web_link: String,
    /// 飞书pc端单据详情页地址
    ///
    /// **示例值**: "https://applink.feishu.cn/client/mini_program/open?appId=xxx"
    #[serde(
        rename = "pc_link",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub pc_link: String,
    /// 飞书移动端单据详情页地址
    ///
    /// **示例值**: "https://applink.feishu.cn/client/mini_program/open?appId=xxx"
    #[serde(
        rename = "mobile_link",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub mobile_link: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(default)]
pub struct ProcessAbstractItemSubResp {
    /// 摘要标题
    #[serde(
        rename = "name",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub name: DataengineI18nSubResp,
    /// 摘要值
    #[serde(
        rename = "value",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub value: DataengineI18nSubResp,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(default)]
pub struct ProcessTodoItemSubResp {
    /// 单据ID
    ///
    /// **示例值**: "7278949005675988535"
    #[serde(
        rename = "approver_id",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub approver_id: String,
    /// 单据类型
    ///
    /// **示例值**: "1"
    ///
    /// **可选值**:
    ///
    /// `Approver`: 审批单
    ///
    /// `Form`: 表单
    #[serde(
        rename = "type",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub body_type: i64,
    /// 单据地址
    #[serde(
        rename = "links",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub links: ProcessLinkSubResp,
    /// 操作人ID
    ///
    /// **示例值**: "7124991993901827628"
    #[serde(
        rename = "operator_id",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub operator_id: String,
    /// 操作人姓名
    #[serde(
        rename = "operator_name",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub operator_name: DataengineI18nSubResp,
    /// 节点名称
    #[serde(
        rename = "node_name",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub node_name: DataengineI18nSubResp,
    /// 创建时间，Unix毫秒时间戳
    ///
    /// **示例值**: "1694769814036"
    #[serde(
        rename = "create_time",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub create_time: String,
    /// 节点定义ID（注：在回退场景，同一个节点会对应多个节点实例）
    ///
    /// **示例值**: "approval_d25b5eddfef"
    #[serde(
        rename = "node_definition_id",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub node_definition_id: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(default)]
pub struct ProcessCcItemSubResp {
    /// 单据ID
    ///
    /// **示例值**: "7278949005675988535"
    #[serde(
        rename = "approver_id",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub approver_id: String,
    /// 单据地址
    #[serde(
        rename = "links",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub links: ProcessLinkSubResp,
    /// 抄送人ID
    ///
    /// **示例值**: "7124991993901827628"
    #[serde(
        rename = "operator_id",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub operator_id: String,
    /// 抄送人姓名
    #[serde(
        rename = "operator_name",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub operator_name: DataengineI18nSubResp,
    /// 节点名称
    #[serde(
        rename = "node_name",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub node_name: DataengineI18nSubResp,
    /// 抄送时间，Unix毫秒时间戳
    ///
    /// **示例值**: "1694769814036"
    #[serde(
        rename = "create_time",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub create_time: String,
    /// 节点定义ID（注：在回退场景，同一个节点会对应多个节点实例）
    ///
    /// **示例值**: "approval_d25b5eddfef"
    #[serde(
        rename = "node_definition_id",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub node_definition_id: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(default)]
pub struct ProcessDoneItemSubResp {
    /// 单据ID
    ///
    /// **示例值**: "7278949005675988535"
    #[serde(
        rename = "approver_id",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub approver_id: String,
    /// 单据类型
    ///
    /// **示例值**: "1"
    ///
    /// **可选值**:
    ///
    /// `Approver`: 审批单
    ///
    /// `Form`: 表单
    #[serde(
        rename = "type",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub body_type: i64,
    /// 单据状态
    ///
    /// **示例值**: "3"
    ///
    /// **可选值**:
    ///
    /// `Approved`: 已完成
    ///
    /// `Reject`: 拒绝
    ///
    /// `Cancel`: 取消
    #[serde(
        rename = "status",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub status: i64,
    /// 单据地址
    #[serde(
        rename = "links",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub links: ProcessLinkSubResp,
    /// 操作人ID
    ///
    /// **示例值**: "7124991993901827628"
    #[serde(
        rename = "operator_id",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub operator_id: String,
    /// 操作人姓名
    #[serde(
        rename = "operator_name",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub operator_name: DataengineI18nSubResp,
    /// 节点名称
    #[serde(
        rename = "node_name",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub node_name: DataengineI18nSubResp,
    /// 创建时间，Unix毫秒时间戳
    ///
    /// **示例值**: "1694769814036"
    #[serde(
        rename = "create_time",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub create_time: String,
    /// 完成时间，Unix毫秒时间戳
    ///
    /// **示例值**: "1694769814036"
    #[serde(
        rename = "complete_time",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub complete_time: String,
    /// 节点定义ID（注：在回退场景，同一个节点会对应多个节点实例）
    ///
    /// **示例值**: "approval_d25b5eddfef"
    #[serde(
        rename = "node_definition_id",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub node_definition_id: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(default)]
pub struct ProcessSystemTodoItemSubResp {
    /// 单据ID
    ///
    /// **示例值**: "7278949005675988535"
    #[serde(
        rename = "approver_id",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub approver_id: String,
    /// 单据类型
    ///
    /// **示例值**: "1"
    ///
    /// **可选值**:
    ///
    /// `Approver`: 审批单
    ///
    /// `Form`: 表单
    #[serde(
        rename = "type",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub body_type: i64,
    /// 单据地址
    #[serde(
        rename = "links",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub links: ProcessLinkSubResp,
    /// 操作人姓名
    #[serde(
        rename = "operator_name",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub operator_name: DataengineI18nSubResp,
    /// 节点名称
    #[serde(
        rename = "node_name",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub node_name: DataengineI18nSubResp,
    /// 创建时间，Unix毫秒时间戳
    ///
    /// **示例值**: "1694769814036"
    #[serde(
        rename = "create_time",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub create_time: String,
    /// 节点定义ID（注：在回退场景，同一个节点会对应多个节点实例）
    ///
    /// **示例值**: "approval_d25b5eddfef"
    #[serde(
        rename = "node_definition_id",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub node_definition_id: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(default)]
pub struct ProcessSystemDoneItemSubResp {
    /// 单据ID
    ///
    /// **示例值**: "7278949005675988535"
    #[serde(
        rename = "approver_id",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub approver_id: String,
    /// 单据类型
    ///
    /// **示例值**: "1"
    ///
    /// **可选值**:
    ///
    /// `Approver`: 审批单
    ///
    /// `Form`: 表单
    #[serde(
        rename = "type",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub body_type: i64,
    /// 单据状态
    ///
    /// **示例值**: "3"
    ///
    /// **可选值**:
    ///
    /// `Approved`: 已完成
    ///
    /// `Reject`: 拒绝
    ///
    /// `Cancel`: 取消
    #[serde(
        rename = "status",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub status: i64,
    /// 单据地址
    #[serde(
        rename = "links",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub links: ProcessLinkSubResp,
    /// 操作人姓名
    ///
    /// **示例值**: "7124991993901827628"
    #[serde(
        rename = "operator_name",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub operator_name: DataengineI18nSubResp,
    /// 节点名称
    #[serde(
        rename = "node_name",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub node_name: DataengineI18nSubResp,
    /// 创建时间，Unix毫秒时间戳
    ///
    /// **示例值**: "1694769814036"
    #[serde(
        rename = "create_time",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub create_time: String,
    /// 完成时间，Unix毫秒时间戳
    ///
    /// **示例值**: "1694769814036"
    #[serde(
        rename = "complete_time",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub complete_time: String,
    /// 节点定义ID（注：在回退场景，同一个节点会对应多个节点实例）
    ///
    /// **示例值**: "approval_d25b5eddfef"
    #[serde(
        rename = "node_definition_id",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub node_definition_id: String,
}

#[cfg(feature = "test-util")]
mod test_utils {
    use std::sync::Arc;

    use self::gen::core_hr::CoreHrServiceMocker;
    use crate::core::mocker::*;

    use super::*;
    use crate::api::*;
    use crate::core::model::*;

    pub struct Mocker;

    pub trait MockFunc:
        Fn(GetCoreHrProcessReq) -> Result<(GetCoreHrProcessResp, CommonResponse), Error>
        + Send
        + Sync
        + 'static
    {
    }
    impl<
            T: Fn(GetCoreHrProcessReq) -> Result<(GetCoreHrProcessResp, CommonResponse), Error>
                + Send
                + Sync
                + 'static,
        > MockFunc for T
    {
    }

    impl<'c, IStore: Store, IClient: HttpClient> CoreHrServiceMocker<'c, IStore, IClient> {
        pub fn mock_get_core_hr_process<F: MockFunc>(
            &self,
            f: F,
        ) -> MockerBuilder<Mocker, GetCoreHrProcessReq, GetCoreHrProcessResp, Arc<dyn MockFunc>>
        {
            MockerBuilder::new(self.cli.instance_id, Arc::new(f))
        }

        pub(super) fn get_mock_get_core_hr_process(
            &self,
            req: &GetCoreHrProcessReq,
        ) -> Option<Arc<dyn MockFunc>> {
            do_mock::<Mocker, GetCoreHrProcessReq, GetCoreHrProcessResp, Arc<dyn MockFunc>>(
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
        api::gen::core_hr::get_core_hr_process::{
            GetCoreHrProcessReq, GetCoreHrProcessResp, GetCoreHrProcessRespInner,
        },
        core::{model::CommonResponse, Lark},
    };

    #[tokio::test]
    async fn test_mock() {
        let lark = Lark::new("".to_owned(), "".into());
        let mocker = lark
            .core_hr()
            .mock()
            .mock_get_core_hr_process(|_| {
                Ok((GetCoreHrProcessResp::default(), CommonResponse::default()))
            })
            .build();
        let res = lark
            .core_hr()
            .get_core_hr_process(GetCoreHrProcessReq::default())
            .await;
        assert!(res.is_ok());
        mocker.clear();
        let res = lark
            .core_hr()
            .get_core_hr_process(GetCoreHrProcessReq::default())
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
        "process_id": "7278949005675988535",
        "status": 1,
        "flow_template_id": "leave",
        "flow_template_name": {
            "zh_cn": "中文",
            "en_us": "English"
        },
        "flow_definition_id": "people_6961286846093788680_7081951411982077732",
        "flow_definition_name": {
            "zh_cn": "中文",
            "en_us": "English"
        },
        "initiator_id": "7124991993901827628",
        "initiator_name": {
            "zh_cn": "中文",
            "en_us": "English"
        },
        "create_time": "1694769814036",
        "complete_time": "1694769814036",
        "start_links": {
            "web_link": "http://xxxx.com/xxx/xxx?a=b",
            "pc_link": "https://applink.feishu.cn/client/mini_program/open?appId=xxx",
            "mobile_link": "https://applink.feishu.cn/client/mini_program/open?appId=xxx"
        },
        "abstracts": [
            {
                "name": {
                    "zh_cn": "中文",
                    "en_us": "English"
                },
                "value": {
                    "zh_cn": "中文",
                    "en_us": "English"
                }
            }
        ],
        "todos": [
            {
                "approver_id": "7278949005675988535",
                "type": 1,
                "links": {
                    "web_link": "http://xxxx.com/xxx/xxx?a=b",
                    "pc_link": "https://applink.feishu.cn/client/mini_program/open?appId=xxx",
                    "mobile_link": "https://applink.feishu.cn/client/mini_program/open?appId=xxx"
                },
                "operator_id": "7124991993901827628",
                "operator_name": {
                    "zh_cn": "中文",
                    "en_us": "English"
                },
                "node_name": {
                    "zh_cn": "中文",
                    "en_us": "English"
                },
                "create_time": "1694769814036",
                "node_definition_id": "approval_d25b5eddfef"
            }
        ],
        "cc_list": [
            {
                "approver_id": "7278949005675988535",
                "links": {
                    "web_link": "http://xxxx.com/xxx/xxx?a=b",
                    "pc_link": "https://applink.feishu.cn/client/mini_program/open?appId=xxx",
                    "mobile_link": "https://applink.feishu.cn/client/mini_program/open?appId=xxx"
                },
                "operator_id": "7124991993901827628",
                "operator_name": {
                    "zh_cn": "中文",
                    "en_us": "English"
                },
                "node_name": {
                    "zh_cn": "中文",
                    "en_us": "English"
                },
                "create_time": "1694769814036",
                "node_definition_id": "approval_d25b5eddfef"
            }
        ],
        "done_list": [
            {
                "approver_id": "7278949005675988535",
                "type": 1,
                "status": 3,
                "links": {
                    "web_link": "http://xxxx.com/xxx/xxx?a=b",
                    "pc_link": "https://applink.feishu.cn/client/mini_program/open?appId=xxx",
                    "mobile_link": "https://applink.feishu.cn/client/mini_program/open?appId=xxx"
                },
                "operator_id": "7124991993901827628",
                "operator_name": {
                    "zh_cn": "中文",
                    "en_us": "English"
                },
                "node_name": {
                    "zh_cn": "中文",
                    "en_us": "English"
                },
                "create_time": "1694769814036",
                "complete_time": "1694769814036",
                "node_definition_id": "approval_d25b5eddfef"
            }
        ],
        "properties": 1,
        "system_todos": [
            {
                "approver_id": "7278949005675988535",
                "type": 1,
                "links": {
                    "web_link": "http://xxxx.com/xxx/xxx?a=b",
                    "pc_link": "https://applink.feishu.cn/client/mini_program/open?appId=xxx",
                    "mobile_link": "https://applink.feishu.cn/client/mini_program/open?appId=xxx"
                },
                "operator_name": {
                    "zh_cn": "中文",
                    "en_us": "English"
                },
                "node_name": {
                    "zh_cn": "中文",
                    "en_us": "English"
                },
                "create_time": "1694769814036",
                "node_definition_id": "approval_d25b5eddfef"
            }
        ],
        "system_done_list": [
            {
                "approver_id": "7278949005675988535",
                "type": 1,
                "status": 3,
                "links": {
                    "web_link": "http://xxxx.com/xxx/xxx?a=b",
                    "pc_link": "https://applink.feishu.cn/client/mini_program/open?appId=xxx",
                    "mobile_link": "https://applink.feishu.cn/client/mini_program/open?appId=xxx"
                },
                "operator_name": {
                    "zh_cn": "中文",
                    "en_us": "English"
                },
                "node_name": {
                    "zh_cn": "中文",
                    "en_us": "English"
                },
                "create_time": "1694769814036",
                "complete_time": "1694769814036",
                "node_definition_id": "approval_d25b5eddfef"
            }
        ]
    }
}"#;
    #[test]
    fn test_resp() {
        let res = serde_json::from_str::<GetCoreHrProcessRespInner>(RESP);
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
