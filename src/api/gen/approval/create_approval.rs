//! doc url: <https://open.larkoffice.com/document/uAjLw4CM/ukTMukTMukTM/reference/approval-v4/approval/create>
// Code generated by gen_api. DO NOT EDIT.

use serde::{Deserialize, Serialize};

use crate::api::BaseResp;
use crate::api::HasBaseResp;
use crate::core::model::{ApiRequest, CommonResponse, MethodOption, ReqParam, StreamReqParam};
use crate::{
    core::{http_client::HttpClient, store::Store},
    error::Error,
};

use crate::api::gen::approval::ApprovalService;

impl<'c, IStore: Store, IClient: HttpClient> ApprovalService<'c, IStore, IClient> {
    /// **api 版本: 2024-07-03T02:49:01+00:00**
    ///
    /// ## 创建审批定义
    ///
    /// 用于通过接口创建简单的审批定义，可以灵活指定定义的基础信息、表单和流程等。创建成功后，不支持从审批管理后台删除该定义。不推荐企业自建应用使用，如有需要尽量联系管理员在审批管理后台创建定义。
    ///
    /// 接口谨慎调用，创建后的审批定义无法停用/删除
    ///
    /// doc: <https://open.larkoffice.com/document/uAjLw4CM/ukTMukTMukTM/reference/approval-v4/approval/create>
    ///
    /// new doc: <https://open.larkoffice.com/document/server-docs/approval-v4/approval/create>
    ///
    /// api url: <https://open.feishu.cn/document_portal/v1/document/get_detail?fullPath=%2Fserver-docs%2Fapproval-v4%2Fapproval%2Fcreate>
    pub async fn create_approval(
        &self,
        req: CreateApprovalReq,
    ) -> Result<(CreateApprovalResp, CommonResponse), Error> {
        self.create_approval_with_opt(req, Default::default()).await
    }

    /// 参见 [create_approval](#method.create_approval) 函数
    pub async fn create_approval_with_opt(
        &self,
        req: CreateApprovalReq,
        method_option: MethodOption,
    ) -> Result<(CreateApprovalResp, CommonResponse), Error> {
        #[cfg(feature = "test-util")]
        {
            if let Some(f) = self.mock().get_mock_create_approval(&req) {
                tracing::info!("[lark] Approval#CreateApproval **mocking** api");
                return f(req);
            }
        }

        tracing::info!("[lark] Approval#CreateApproval call api");

        let req = ApiRequest {
            scope: "Approval",
            api: "CreateApproval",
            method: http::Method::POST,
            url: String::new()
                + self.cli.open_base_url.as_ref()
                + "/open-apis/approval/v4/approvals",
            param_data: req.gen_param(),
            method_option,
            need_tenant_access_token: true,
            ..Default::default()
        };

        let (resp, common_resp): (CreateApprovalRespInner, _) = self.cli.do_req(req).await?;
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
pub struct CreateApprovalReq {
    /// 此次调用中使用的部门ID的类型
    ///
    /// **示例值**: "open_department_id"
    ///
    /// **可选值**:
    ///
    /// `DepartmentId`: 以自定义department_id来标识部门
    ///
    /// `OpenDepartmentId`: 以open_department_id来标识部门
    #[api(
        kind = "query",
        name = "department_id_type",
        v_type = "var",
        option = "false"
    )]
    pub department_id_type: String,
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
    /// 审批名称的国际化文案 Key，以 @i18n@ 开头，长度不得少于 9 个字符
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "@i18n@approval_name"
    #[api(kind = "body", name = "approval_name")]
    pub approval_name: String,
    /// 审批定义 Code。不传值表示新建；传入指定 Code 表示覆盖原定义内容进行全量更新
    ///
    /// **示例值**: "7C468A54-8745-2245-9675-08B7C63E7A85"
    #[api(kind = "body", name = "approval_code")]
    pub approval_code: Option<String>,
    /// 审批描述的国际化文案 Key，以 @i18n@ 开头，长度不得少于 9 个字符
    ///
    /// **示例值**: "@i18n@description"
    #[api(kind = "body", name = "description")]
    pub description: Option<String>,
    /// viewers 字段指定了哪些人能从审批应用的前台发起该审批（列表最大支持长度200）。
    ///
    /// 1. 当 viewer_type 为 USER，需要填写viewer_user_id；
    ///
    /// 2. 当 viewer_type 为DEPARTMENT，需要填写viewer_department_id；
    ///
    /// 3. 当 viewer_type 为TENANT或NONE时，viewer_user_id和viewer_department_id无需填写
    ///
    /// **是否必填**: 是
    #[api(kind = "body", name = "viewers")]
    pub viewers: Vec<Option<ApprovalCreateViewersSubReq>>,
    /// 审批定义表单
    ///
    /// **是否必填**: 是
    #[api(kind = "body", name = "form")]
    pub form: ApprovalFormSubReq,
    /// 审批定义节点，需要将开始节点作为 list 第一个元素，结束节点作为最后一个元素
    ///
    /// **是否必填**: 是
    #[api(kind = "body", name = "node_list")]
    pub node_list: Vec<Option<ApprovalNodeSubReq>>,
    /// 审批定义其他设置
    #[api(kind = "body", name = "settings")]
    pub settings: Option<ApprovalSettingSubReq>,
    /// 审批定义配置项，用于配置对应审批定义是否可以由用户在审批后台进行修改
    #[api(kind = "body", name = "config")]
    pub config: Option<ApprovalConfigSubReq>,
    /// 审批图标枚举，默认为 0。详情参考[资源介绍](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/approval-v4/approval/overview-of-approval-resources)。
    ///
    /// **示例值**: "0"
    #[api(kind = "body", name = "icon")]
    pub icon: Option<i64>,
    /// 国际化文案
    ///
    /// **是否必填**: 是
    #[api(kind = "body", name = "i18n_resources")]
    pub i18n_resources: Vec<Option<I18nResourceSubReq>>,
    /// 根据user_id_type填写流程管理员的用户id（列表最大支持长度200）
    ///
    /// **示例值**: "["1c5ea995"]"
    #[api(kind = "body", name = "process_manager_ids")]
    pub process_manager_ids: Vec<Option<String>>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(default)]
pub struct ApprovalFormSubReq {
    /// 审批定义表单，json 数组，见下方form_content字段说明
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "[{\"id\":\"user_name\", \"type\": \"input\", \"required\":true, \"name\":\"@i18n@widget1\"}]"
    #[serde(
        rename = "form_content",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub form_content: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(default)]
pub struct ApprovalSettingSubReq {
    /// 审批实例通过后允许撤回的时间，以秒为单位，默认 31 天，0 为不可撤回
    ///
    /// **示例值**: "0"
    #[serde(
        rename = "revert_interval",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub revert_interval: Option<i64>,
    /// 是否支持审批通过第一个节点后撤回，默认为1，0为不支持
    ///
    /// **示例值**: "0"
    #[serde(
        rename = "revert_option",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub revert_option: Option<i64>,
    /// 拒绝设置
    ///
    /// **示例值**: "0"
    ///
    /// **可选值**:
    ///
    /// `RejectDefault`: 默认设置，流程被终止
    ///
    /// `RejectSubmit`: 退回至发起人，发起人可编辑流程后重新提交
    #[serde(
        rename = "reject_option",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub reject_option: Option<i64>,
    /// 快捷审批配置项，开启后可在卡片上直接审批。默认值1为启用， 0为禁用
    ///
    /// **示例值**: "1"
    ///
    /// **可选值**:
    ///
    /// `Close`: 禁用
    ///
    /// `Open`: 启用
    #[serde(
        rename = "quick_approval_option",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub quick_approval_option: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(default)]
pub struct ApprovalConfigSubReq {
    /// 允许用户修改可见范围
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "false"
    #[serde(
        rename = "can_update_viewer",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub can_update_viewer: bool,
    /// 允许用户更新表单
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "false"
    #[serde(
        rename = "can_update_form",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub can_update_form: bool,
    /// 允许用户更新流程定义
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "false"
    #[serde(
        rename = "can_update_process",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub can_update_process: bool,
    /// 允许用户更新撤回设置
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "false"
    #[serde(
        rename = "can_update_revert",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub can_update_revert: bool,
    /// 帮助文档链接
    ///
    /// **示例值**: "https://xxx.xxx.xxx"
    #[serde(
        rename = "help_url",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub help_url: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(default)]
pub struct ApprovalCreateViewersSubReq {
    /// 可见人类型
    ///
    /// **示例值**: "USER"
    ///
    /// **可选值**:
    ///
    /// `Tenant`: 租户内可见
    ///
    /// `Department`: 指定部门
    ///
    /// `User`: 指定用户
    ///
    /// `None`: 任何人都不可见
    #[serde(
        rename = "viewer_type",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub viewer_type: Option<String>,
    /// 当 viewer_type 是 USER，根据user_id_type填写用户id
    ///
    /// **示例值**: "19a294c2"
    #[serde(
        rename = "viewer_user_id",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub viewer_user_id: Option<String>,
    /// 当 viewer_type 为DEPARTMENT，根据department_id_type填写部门id
    ///
    /// **示例值**: "od-ac9d697abfa990b715dcc33d58a62a9d"
    #[serde(
        rename = "viewer_department_id",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub viewer_department_id: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(default)]
pub struct ApprovalNodeSubReq {
    /// 节点 ID，开始节点的 ID 为 START，结束节点的 ID 为 END，开始和结束节点不需要指定 name、node_type 以及 approver
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "START"
    #[serde(
        rename = "id",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub id: String,
    /// 节点名称的国际化文案 Key，以 @i18n@ 开头，长度不得少于 9 个字符
    ///
    /// **示例值**: "@i18n@node_name"
    #[serde(
        rename = "name",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub name: Option<String>,
    /// 审批类型枚举,当 node_type 为依次审批时，审批人必须为『发起人自选』
    ///
    /// **示例值**: "AND"
    ///
    /// **可选值**:
    ///
    /// `And`: 会签
    ///
    /// `Or`: 或签
    ///
    /// `Sequental`: 依次审批
    #[serde(
        rename = "node_type",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub node_type: Option<String>,
    /// 审批人列表
    #[serde(
        rename = "approver",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub approver: Vec<Option<ApprovalApproverCcerSubReq>>,
    /// 抄送人列表
    #[serde(
        rename = "ccer",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub ccer: Vec<Option<ApprovalApproverCcerSubReq>>,
    /// 表单项的控件权限
    #[serde(
        rename = "privilege_field",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub privilege_field: Option<FieldGroupSubReq>,
    /// 自选审批人是否允许多选
    ///
    /// **示例值**: "false"
    #[serde(
        rename = "approver_chosen_multi",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub approver_chosen_multi: Option<bool>,
    /// 自选审批人选择范围
    #[serde(
        rename = "approver_chosen_range",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub approver_chosen_range: Vec<Option<ApproverRangeSubReq>>,
    /// 审批人为提交人时的操作
    ///
    /// **示例值**: "STARTER"
    ///
    /// **可选值**:
    ///
    /// `STARTER`: 发起人本人审批
    ///
    /// `AUTO_PASS`: 自动通过
    ///
    /// `SUPERVISOR`: 直属上级审批
    ///
    /// `DEPARTMENT_MANAGER`: 直属部门负责人审批
    #[serde(
        rename = "starter_assignee",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub starter_assignee: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(default)]
pub struct I18nResourceSubReq {
    /// 语言可选值有： zh-CN：中文 en-US：英文 ja-JP：日文
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "zh-CN"
    ///
    /// **可选值**:
    ///
    /// `Zhcn`: 中文
    ///
    /// `Enus`: 英文
    ///
    /// `Jajp`: 日文
    #[serde(
        rename = "locale",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub locale: String,
    /// 文案 key, value, i18n key 以 @i18n@ 开头； 该字段主要用于做国际化，允许用户同时传多个语言的文案，审批中心会根据用户当前的语音环境使用对应的文案，如果没有传用户当前的语音环境文案，则会使用默认的语言文案。
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "{ "@i18n@1": "权限申请", "@i18n@2": "OA审批", "@i18n@3": "Permission" }"
    #[serde(
        rename = "texts",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub texts: Vec<Option<I18nResourceTextSubReq>>,
    /// 是否默认语言，默认语言需要包含所有key，非默认语言如果key不存在会使用默认语言代替
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "true"
    #[serde(
        rename = "is_default",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub is_default: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(default)]
pub struct FieldGroupSubReq {
    /// 可写权限的表单项的 id列表
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "9293493"
    #[serde(
        rename = "writable",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub writable: Vec<Option<String>>,
    /// 可读权限的表单项的 id列表
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "9293493"
    #[serde(
        rename = "readable",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub readable: Vec<Option<String>>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(default)]
pub struct ApprovalApproverCcerSubReq {
    /// 审批/抄送人类型，
    ///
    /// 1. 当 type 为 Supervisor、SupervisorTopDown、DepartmentManager 、DepartmentManagerTopDown 这 4 种时，需要在 level 中填写对应的级数，例如：由下往上三级主管审批，level = 3；
    ///
    /// 2. 当 type 为 Personal 时，需要填写对应的user_id ，用于指定用户；
    ///
    /// 3. 当 type 为 Free 发起人自选时，不需要指定 user_id 和level；
    ///
    /// 4. ccer不支持 Free 发起人自选
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "Supervisor"
    ///
    /// **可选值**:
    ///
    /// `Supervisor`: 主管审批（由下往上）
    ///
    /// `SupervisorTopDown`: 主管审批（从上往下）
    ///
    /// `DepartmentManager`: 部门负责人审批（由下往上）
    ///
    /// `DepartmentManagerTopDown`: 部门负责人审批（从上往下）
    ///
    /// `Personal`: 指定成员
    ///
    /// `Free`: 发起人自选
    #[serde(
        rename = "type",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub body_type: String,
    /// 用户id，根据user_id_type填写
    ///
    /// **示例值**: "f7cb567e"
    #[serde(
        rename = "user_id",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub user_id: Option<String>,
    /// 审批级数，当 type 为 Supervisor、SupervisorTopDown、DepartmentManager 、DepartmentManagerTopDown 这 4 种时，需要在 level 中填写对应的级数，例如：由下往上三级主管审批，level = 3
    ///
    /// **示例值**: "3"
    #[serde(
        rename = "level",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub level: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(default)]
pub struct ApproverRangeSubReq {
    /// 审批人类型
    ///
    /// **示例值**: "ALL"
    ///
    /// **可选值**:
    ///
    /// `ALL`: 全租户
    ///
    /// `PERSONAL`: 指定审批人
    ///
    /// `ROLE`: 指定角色
    #[serde(
        rename = "type",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub body_type: Option<String>,
    /// 审批人id
    ///
    /// **示例值**: "f7cb567e"
    #[serde(
        rename = "id_list",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub id_list: Vec<Option<String>>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(default)]
pub struct I18nResourceTextSubReq {
    /// 文案key
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "@i18n@1"
    #[serde(
        rename = "key",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub key: String,
    /// 文案
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "people"
    #[serde(
        rename = "value",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub value: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, lark_bot_sdk_macros::ApiBaseResp)]
struct CreateApprovalRespInner {
    #[serde(flatten)]
    data: Option<CreateApprovalResp>,
    #[serde(flatten)]
    base: BaseResp,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct CreateApprovalResp {
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
    /// 审批定义 Code
    ///
    /// **示例值**: "81D31358-93AF-92D6-7425-01A5D67C4E71"
    #[serde(
        rename = "approval_code",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub approval_code: String,
    /// 审批定义 id
    ///
    /// **示例值**: "7090754740375519252"
    #[serde(
        rename = "approval_id",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub approval_id: String,
}

#[cfg(feature = "test-util")]
mod test_utils {
    use std::sync::Arc;

    use self::gen::approval::ApprovalServiceMocker;
    use crate::core::mocker::*;

    use super::*;
    use crate::api::*;
    use crate::core::model::*;

    pub struct Mocker;

    pub trait MockFunc:
        Fn(CreateApprovalReq) -> Result<(CreateApprovalResp, CommonResponse), Error>
        + Send
        + Sync
        + 'static
    {
    }
    impl<
            T: Fn(CreateApprovalReq) -> Result<(CreateApprovalResp, CommonResponse), Error>
                + Send
                + Sync
                + 'static,
        > MockFunc for T
    {
    }

    impl<'c, IStore: Store, IClient: HttpClient> ApprovalServiceMocker<'c, IStore, IClient> {
        pub fn mock_create_approval<F: MockFunc>(
            &self,
            f: F,
        ) -> MockerBuilder<Mocker, CreateApprovalReq, CreateApprovalResp, Arc<dyn MockFunc>>
        {
            MockerBuilder::new(self.cli.instance_id, Arc::new(f))
        }

        pub(super) fn get_mock_create_approval(
            &self,
            req: &CreateApprovalReq,
        ) -> Option<Arc<dyn MockFunc>> {
            do_mock::<Mocker, CreateApprovalReq, CreateApprovalResp, Arc<dyn MockFunc>>(
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
        api::gen::approval::create_approval::{
            CreateApprovalReq, CreateApprovalResp, CreateApprovalRespInner,
        },
        core::{model::CommonResponse, Lark},
    };

    #[tokio::test]
    async fn test_mock() {
        let lark = Lark::new("".to_owned(), "".into());
        let mocker = lark
            .approval()
            .mock()
            .mock_create_approval(|_| {
                Ok((CreateApprovalResp::default(), CommonResponse::default()))
            })
            .build();
        let res = lark
            .approval()
            .create_approval(CreateApprovalReq::default())
            .await;
        assert!(res.is_ok());
        mocker.clear();
        let res = lark
            .approval()
            .create_approval(CreateApprovalReq::default())
            .await;
        assert!(res.is_err());
    }

    const REQ: &str = r#"{
    "approval_name": "@i18n@approval_name",
    "approval_code": "813718CE-F38D-45CA-A5C1-ACF4F564B526",
    "viewers":[
        {
            "viewer_type":"TENANT",
            "viewer_user_id":""
        }
    ],
    "form": {
        "form_content": "[{\"id\":\"111\",\"name\":\"@i18n@event_name\",\"required\":true,\"type\":\"input\"},{\"id\":\"222\",\"name\":\"@i18n@time_interval\",\"required\":true,\"type\":\"dateInterval\",\"value\":{\"format\":\"YYYY-MM-DD hh:mm\",\"intervalAllowModify\":false}},{\"id\":\"333\",\"name\":\"@i18n@event_type\",\"type\":\"radioV2\",\"value\":[{\"key\":\"1\",\"text\":\"@i18n@recurrence_event\"},{\"key\":\"2\",\"text\":\"@i18n@single_event\"}]},{\"id\":\"444\",\"name\":\"@i18n@attende_count\",\"required\":true,\"type\":\"number\"},{\"id\":\"555\",\"name\":\"@i18n@apply_reason\",\"required\":true,\"type\":\"textarea\"}]"
        },

    "node_list": [{
          "id": "START",
          "privilege_field":{ 
				 "writable": ["111","222"],
				 "readable": ["111","222"]
		  }
        },{
          "id": "7106864726566",
          "privilege_field":{ 
				 "writable": ["111","222"],
				 "readable": ["111","222"]
		  },
          "name": "@i18n@node_name",
          "node_type": "AND",
          "approver": [
            {
              "type": "Personal",
              "user_id": "59a92c4a"
            }
          ],
          "ccer": [
            {
              "type": "Supervisor",
              "level": "2"
            }
          ]
        },{
          "id": "END"
        }],
    "settings" : {
          "revert_interval":0
        },
    "config" : {
          "can_update_viewer": false,
          "can_update_form": true,
          "can_update_process": true,
          "can_update_revert": true,
          "help_url":"https://xxx.xxx.xxx"
        },
    "icon": 1,
    "i18n_resources" : [{
          "locale": "zh-CN",
          "texts" : [
              {"key":"@i18n@approval_name","value":"审批名称"},
              {"key":"@i18n@event_name","value":"日程名称"},
              {"key":"@i18n@node_name","value":"审批"},
              {"key":"@i18n@time_interval","value":"日程名称"},
              {"key":"@i18n@event_type","value":"日程类型"},
              {"key":"@i18n@recurrence_event","value":"重复性日程"},
              {"key":"@i18n@single_event","value":"单次日程"},
              {"key":"@i18n@attende_count","value":"参与人数量"},
              {"key":"@i18n@apply_reason","value":"申请原因"}
            ],
          "is_default": true
        }],
    "process_manager_ids": ["1c5ea995"]
}"#;

    #[test]
    fn test_req() {
        if REQ == "{}" {
            return;
        }
        if let Err(e) = serde_json::from_str::<super::CreateApprovalReqBody>(REQ) {
            panic!("{}", e);
        }
    }

    const RESP: &str = r#"{
    "code": 0,
    "msg": "success",
    "data": {
        "approval_code": "81D31358-93AF-92D6-7425-01A5D67C4E71",
        "approval_id": "7090754740375519252"
    }
}"#;
    #[test]
    fn test_resp() {
        let res = serde_json::from_str::<CreateApprovalRespInner>(RESP);
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
