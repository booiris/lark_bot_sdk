//! doc url: <https://open.larkoffice.com/document/uAjLw4CM/ukTMukTMukTM/reference/approval-v4/instance/get>
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
    /// **api 版本: 2023-11-02T02:50:04+00:00**
    ///
    /// ## 获取单个审批实例详情
    ///
    /// 通过审批实例 Instance Code  获取审批实例详情。Instance Code 由 [批量获取审批实例](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/approval-v4/instance/list) 接口获取。
    ///

    ///
    /// doc: <https://open.larkoffice.com/document/uAjLw4CM/ukTMukTMukTM/reference/approval-v4/instance/get>
    ///
    /// new doc: <https://open.larkoffice.com/document/server-docs/approval-v4/instance/get>
    ///
    /// api url: <https://open.feishu.cn/document_portal/v1/document/get_detail?fullPath=%2Fserver-docs%2Fapproval-v4%2Finstance%2Fget>
    pub async fn get_approval_instance(
        &self,
        req: GetApprovalInstanceReq,
    ) -> Result<(GetApprovalInstanceResp, CommonResponse), Error> {
        self.get_approval_instance_with_opt(req, Default::default())
            .await
    }

    /// 参见 [get_approval_instance](#method.get_approval_instance) 函数
    pub async fn get_approval_instance_with_opt(
        &self,
        req: GetApprovalInstanceReq,
        method_option: MethodOption,
    ) -> Result<(GetApprovalInstanceResp, CommonResponse), Error> {
        #[cfg(feature = "test-util")]
        {
            if let Some(f) = self.mock().get_mock_get_approval_instance(&req) {
                tracing::info!("[lark] Approval#GetApprovalInstance **mocking** api");
                return f(req);
            }
        }

        tracing::info!("[lark] Approval#GetApprovalInstance call api");

        let req = ApiRequest {
            scope: "Approval",
            api: "GetApprovalInstance",
            method: http::Method::GET,
            url: String::new()
                + self.cli.open_base_url.as_ref()
                + "/open-apis/approval/v4/instances/:instance_id",
            param_data: req.gen_param(),
            method_option,
            need_tenant_access_token: true,
            ..Default::default()
        };

        let (resp, common_resp): (GetApprovalInstanceRespInner, _) = self.cli.do_req(req).await?;
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
pub struct GetApprovalInstanceReq {
    /// 审批实例 Code，若在创建的时候传了 uuid，也可以通过传 uuid 获取。
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "81D31358-93AF-92D6-7425-01A5D67C4E71"
    #[api(kind = "path", name = "instance_id")]
    pub instance_id: String,
    /// 语言。默认值为[创建审批定义](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/approval-v4/approval/create)时在 i18n_resources 字段中配置的语言。
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
    #[api(kind = "query", name = "locale", v_type = "var", option = "false")]
    pub locale: String,
    /// 发起审批用户 id，仅自建应用可返回。
    ///
    /// **示例值**: "f7cb567e"
    #[api(kind = "query", name = "user_id", v_type = "var", option = "false")]
    pub user_id: String,
    /// 用户 ID 类型
    ///
    /// **示例值**: "user_id"
    ///
    /// **可选值**:
    ///
    /// `OpenId`: 标识一个用户在某个应用中的身份。同一个用户在不同应用中的 Open ID 不同。[了解更多：如何获取 Open ID](https://open.feishu.cn/document/uAjLw4CM/ugTN1YjL4UTN24CO1UjN/trouble-shooting/how-to-obtain-openid)
    ///
    /// `UnionId`: 标识一个用户在某个应用开发商下的身份。同一用户在同一开发商下的应用中的 Union ID 是相同的，在不同开发商下的应用中的 Union ID 是不同的。通过 Union ID，应用开发商可以把同个用户在多个应用中的身份关联起来。[了解更多：如何获取 Union ID？](https://open.feishu.cn/document/uAjLw4CM/ugTN1YjL4UTN24CO1UjN/trouble-shooting/how-to-obtain-union-id)
    ///
    /// `UserId`: 标识一个用户在某个租户内的身份。同一个用户在租户 A 和租户 B 内的 User ID 是不同的。在同一个租户内，一个用户的 User ID 在所有应用（包括商店应用）中都保持一致。User ID 主要用于在不同的应用间打通用户数据。[了解更多：如何获取 User ID？](https://open.feishu.cn/document/uAjLw4CM/ugTN1YjL4UTN24CO1UjN/trouble-shooting/how-to-obtain-user-id)
    #[api(
        kind = "query",
        name = "user_id_type",
        v_type = "var",
        option = "false"
    )]
    pub user_id_type: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, lark_bot_sdk_macros::ApiBaseResp)]
struct GetApprovalInstanceRespInner {
    #[serde(flatten)]
    data: Option<GetApprovalInstanceResp>,
    #[serde(flatten)]
    base: BaseResp,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct GetApprovalInstanceResp {
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
    /// 审批名称
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "Payment"
    #[serde(
        rename = "approval_name",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub approval_name: String,
    /// 审批创建时间
    ///
    /// **示例值**: "1564590532967"
    #[serde(
        rename = "start_time",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub start_time: String,
    /// 审批完成时间，未完成为 0
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "1564590532967"
    #[serde(
        rename = "end_time",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub end_time: String,
    /// 发起审批用户
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "f3ta757q"
    #[serde(
        rename = "user_id",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub user_id: String,
    /// 发起审批用户 open id
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "ou_3cda9c969f737aaa05e6915dce306cb9"
    #[serde(
        rename = "open_id",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub open_id: String,
    /// 审批单编号
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "202102060002"
    #[serde(
        rename = "serial_number",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub serial_number: String,
    /// 发起审批用户所在部门
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "od-8ec33ffec336c3a39a278bc25e931676"
    #[serde(
        rename = "department_id",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub department_id: String,
    /// 审批实例状态
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "PENDING"
    ///
    /// **可选值**:
    ///
    /// `Pending`: 审批中
    ///
    /// `Approved`: 通过
    ///
    /// `Rejected`: 拒绝
    ///
    /// `Canceled`: 撤回
    ///
    /// `Deleted`: 删除
    #[serde(
        rename = "status",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub status: String,
    /// 用户的唯一标识 id
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "1234567"
    #[serde(
        rename = "uuid",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub uuid: String,
    /// json 字符串，控件值详情见下方
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "[{\"id\": \"widget1\",\"custom_id\": \"user_info\",\"name\": \"Item application\",\"type\": \"textarea\"}]"
    #[serde(
        rename = "form",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub form: String,
    /// 审批任务列表
    ///
    /// **是否必填**: 是
    #[serde(
        rename = "task_list",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub task_list: Vec<InstanceTaskSubResp>,
    /// 评论列表
    ///
    /// **是否必填**: 是
    #[serde(
        rename = "comment_list",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub comment_list: Vec<InstanceCommentSubResp>,
    /// 审批动态
    ///
    /// **是否必填**: 是
    #[serde(
        rename = "timeline",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub timeline: Vec<InstanceTimelineSubResp>,
    /// 修改的原实例 code,仅在查询修改实例时显示该字段
    ///
    /// **示例值**: "81D31358-93AF-92D6-7425-01A5D67C4E71"
    #[serde(
        rename = "modified_instance_code",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub modified_instance_code: String,
    /// 撤销的原实例 code,仅在查询撤销实例时显示该字段
    ///
    /// **示例值**: "81D31358-93AF-92D6-7425-01A5D67C4E71"
    #[serde(
        rename = "reverted_instance_code",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub reverted_instance_code: String,
    /// 审批定义 Code
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "7C468A54-8745-2245-9675-08B7C63E7A85"
    #[serde(
        rename = "approval_code",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub approval_code: String,
    /// 单据是否被撤销
    ///
    /// **示例值**: "false"
    #[serde(
        rename = "reverted",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub reverted: bool,
    /// 审批实例 Code
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "81D31358-93AF-92D6-7425-01A5D67C4E71"
    #[serde(
        rename = "instance_code",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub instance_code: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(default)]
pub struct InstanceTaskSubResp {
    /// task id
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "1234"
    #[serde(
        rename = "id",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub id: String,
    /// 审批人的用户 id，自动通过、自动拒绝 时为空
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "f7cb567e"
    #[serde(
        rename = "user_id",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub user_id: String,
    /// 审批人 open id
    ///
    /// **示例值**: "ou_123457"
    #[serde(
        rename = "open_id",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub open_id: String,
    /// 任务状态
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "PENDING"
    ///
    /// **可选值**:
    ///
    /// `Pending`: 审批中
    ///
    /// `Approved`: 通过
    ///
    /// `Rejected`: 拒绝
    ///
    /// `Transferred`: 已转交
    ///
    /// `Done`: 完成
    #[serde(
        rename = "status",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub status: String,
    /// task 所属节点 id
    ///
    /// **示例值**: "46e6d96cfa756980907209209ec03b64"
    #[serde(
        rename = "node_id",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub node_id: String,
    /// task 所属节点名称
    ///
    /// **示例值**: "开始"
    #[serde(
        rename = "node_name",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub node_name: String,
    /// task 所属节点自定义 id, 如果没设置自定义 id, 则不返回该字段
    ///
    /// **示例值**: "manager"
    #[serde(
        rename = "custom_node_id",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub custom_node_id: String,
    /// 审批方式
    ///
    /// **示例值**: "AND"
    ///
    /// **可选值**:
    ///
    /// `And`: 会签
    ///
    /// `Or`: 或签
    ///
    /// `AutoPass`: 自动通过
    ///
    /// `AutoReject`: 自动拒绝
    ///
    /// `Sequential`: 按顺序
    #[serde(
        rename = "type",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub body_type: String,
    /// task 开始时间
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "1564590532967"
    #[serde(
        rename = "start_time",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub start_time: String,
    /// task 完成时间, 未完成为 0
    ///
    /// **示例值**: "0"
    #[serde(
        rename = "end_time",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub end_time: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(default)]
pub struct InstanceCommentSubResp {
    /// 评论 id
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "1234"
    #[serde(
        rename = "id",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub id: String,
    /// 发表评论用户
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "f7cb567e"
    #[serde(
        rename = "user_id",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub user_id: String,
    /// 发表评论用户 open id
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "ou_123456"
    #[serde(
        rename = "open_id",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub open_id: String,
    /// 评论内容
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "ok"
    #[serde(
        rename = "comment",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub comment: String,
    /// 1564590532967
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "评论时间"
    #[serde(
        rename = "create_time",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub create_time: String,
    /// 评论附件
    #[serde(
        rename = "files",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub files: Vec<FileSubResp>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(default)]
pub struct InstanceTimelineSubResp {
    /// 动态类型，不同类型 ext 内的 user_id_list 含义不一样
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "PASS"
    ///
    /// **可选值**:
    ///
    /// `Start`: 审批开始
    ///
    /// `Pass`: 通过
    ///
    /// `Reject`: 拒绝
    ///
    /// `AutoPass`: 自动通过
    ///
    /// `AutoReject`: 自动拒绝
    ///
    /// `RemoveRepeat`: 去重
    ///
    /// `Transfer`: 转交
    ///
    /// `AddApproverBefore`: 前加签
    ///
    /// `AddApprover`: 并加签
    ///
    /// `AddApproverAfter`: 后加签
    ///
    /// `DeleteApprover`: 减签
    ///
    /// `RollbackSelected`: 指定回退
    ///
    /// `Rollback`: 全部回退
    ///
    /// `Cancel`: 撤回
    ///
    /// `Delete`: 删除
    ///
    /// `Cc`: 抄送
    #[serde(
        rename = "type",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub body_type: String,
    /// 发生时间
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "1564590532967"
    #[serde(
        rename = "create_time",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub create_time: String,
    /// 动态产生用户
    ///
    /// **示例值**: "f7cb567e"
    #[serde(
        rename = "user_id",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub user_id: String,
    /// 动态产生用户 open id
    ///
    /// **示例值**: "ou_123456"
    #[serde(
        rename = "open_id",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub open_id: String,
    /// 被抄送人列表
    #[serde(
        rename = "user_id_list",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub user_id_list: Vec<String>,
    /// 被抄送人列表
    #[serde(
        rename = "open_id_list",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub open_id_list: Vec<String>,
    /// 产生动态关联的task_id
    ///
    /// **示例值**: "1234"
    #[serde(
        rename = "task_id",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub task_id: String,
    /// 理由
    ///
    /// **示例值**: "ok"
    #[serde(
        rename = "comment",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub comment: String,
    /// 抄送人列表
    #[serde(
        rename = "cc_user_list",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub cc_user_list: Vec<InstanceCcUserSubResp>,
    /// 动态其他信息，json格式，目前包括 user_id_list, user_id，open_id_list，open_id
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "{\"user_id\":\"62d4a44c\",\"open_id\":\"ou_123456\"}"
    #[serde(
        rename = "ext",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub ext: String,
    /// 产生 task 的节点 key
    ///
    /// **示例值**: "APPROVAL_240330_4058663"
    #[serde(
        rename = "node_key",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub node_key: String,
    /// 审批附件
    #[serde(
        rename = "files",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub files: Vec<FileSubResp>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(default)]
pub struct FileSubResp {
    /// 附件路径
    ///
    /// **示例值**: "https://p3-approval-sign.byteimg.com/lark-approval-attachment/image/20220714/1/332f3596-0845-4746-a4bc-818d54ad435b.png~tplv-ottatrvjsm-image.image?x-expires=1659033558&x-signature=6edF3k%2BaHeAuvfcBRGOkbckoUl4%3D#.png"
    #[serde(
        rename = "url",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub url: String,
    /// 附件大小
    ///
    /// **示例值**: "186823"
    #[serde(
        rename = "file_size",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub file_size: i64,
    /// 附件标题
    ///
    /// **示例值**: "e018906140ed9388234bd03b0.png"
    #[serde(
        rename = "title",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub title: String,
    /// 附件类别
    ///
    /// **示例值**: "image"
    #[serde(
        rename = "type",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub body_type: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(default)]
pub struct InstanceCcUserSubResp {
    /// 抄送人 user id
    ///
    /// **示例值**: "eea5gefe"
    #[serde(
        rename = "user_id",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub user_id: String,
    /// 审批实例内抄送唯一标识
    ///
    /// **示例值**: "123445"
    #[serde(
        rename = "cc_id",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub cc_id: String,
    /// 抄送人 open id
    ///
    /// **示例值**: "ou_12345"
    #[serde(
        rename = "open_id",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub open_id: String,
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
        Fn(GetApprovalInstanceReq) -> Result<(GetApprovalInstanceResp, CommonResponse), Error>
        + Send
        + Sync
        + 'static
    {
    }
    impl<
            T: Fn(
                    GetApprovalInstanceReq,
                ) -> Result<(GetApprovalInstanceResp, CommonResponse), Error>
                + Send
                + Sync
                + 'static,
        > MockFunc for T
    {
    }

    impl<'c, IStore: Store, IClient: HttpClient> ApprovalServiceMocker<'c, IStore, IClient> {
        pub fn mock_get_approval_instance<F: MockFunc>(
            &self,
            f: F,
        ) -> MockerBuilder<Mocker, GetApprovalInstanceReq, GetApprovalInstanceResp, Arc<dyn MockFunc>>
        {
            MockerBuilder::new(self.cli.instance_id, Arc::new(f))
        }

        pub(super) fn get_mock_get_approval_instance(
            &self,
            req: &GetApprovalInstanceReq,
        ) -> Option<Arc<dyn MockFunc>> {
            do_mock::<Mocker, GetApprovalInstanceReq, GetApprovalInstanceResp, Arc<dyn MockFunc>>(
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
        api::gen::approval::get_approval_instance::{
            GetApprovalInstanceReq, GetApprovalInstanceResp, GetApprovalInstanceRespInner,
        },
        core::{model::CommonResponse, Lark},
    };

    #[tokio::test]
    async fn test_mock() {
        let lark = Lark::new("".to_owned(), "".into());
        let mocker = lark
            .approval()
            .mock()
            .mock_get_approval_instance(|_| {
                Ok((
                    GetApprovalInstanceResp::default(),
                    CommonResponse::default(),
                ))
            })
            .build();
        let res = lark
            .approval()
            .get_approval_instance(GetApprovalInstanceReq::default())
            .await;
        assert!(res.is_ok());
        mocker.clear();
        let res = lark
            .approval()
            .get_approval_instance(GetApprovalInstanceReq::default())
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
        "approval_name": "Payment",
        "start_time": "1564590532967",
        "end_time": "1564590532967",
        "user_id": "f3ta757q",
        "open_id": "ou_3cda9c969f737aaa05e6915dce306cb9",
        "serial_number": "202102060002",
        "department_id": "od-8ec33ffec336c3a39a278bc25e931676",
        "status": "PENDING",
        "uuid": "1234567",
        "form": "[{\"id\": \"widget1\",\"custom_id\": \"user_info\",\"name\": \"Item application\",\"type\": \"textarea\"}]",
        "task_list": [
            {
                "id": "1234",
                "user_id": "f7cb567e",
                "open_id": "ou_123457",
                "status": "PENDING",
                "node_id": "46e6d96cfa756980907209209ec03b64",
                "node_name": "开始",
                "custom_node_id": "manager",
                "type": "AND",
                "start_time": "1564590532967",
                "end_time": "0"
            }
        ],
        "comment_list": [
            {
                "id": "1234",
                "user_id": "f7cb567e",
                "open_id": "ou_123456",
                "comment": "ok",
                "create_time": "评论时间",
                "files": [
                    {
                        "url": "https://p3-approval-sign.byteimg.com/lark-approval-attachment/image/20220714/1/332f3596-0845-4746-a4bc-818d54ad435b.png~tplv-ottatrvjsm-image.image?x-expires=1659033558&x-signature=6edF3k%2BaHeAuvfcBRGOkbckoUl4%3D#.png",
                        "file_size": 186823,
                        "title": "e018906140ed9388234bd03b0.png",
                        "type": "image"
                    }
                ]
            }
        ],
        "timeline": [
            {
                "type": "PASS",
                "create_time": "1564590532967",
                "user_id": "f7cb567e",
                "open_id": "ou_123456",
                "user_id_list": [
                    "eea5gefe"
                ],
                "open_id_list": [
                    "ou_123456"
                ],
                "task_id": "1234",
                "comment": "ok",
                "cc_user_list": [
                    {
                        "user_id": "eea5gefe",
                        "cc_id": "123445",
                        "open_id": "ou_12345"
                    }
                ],
                "ext": "{\"user_id\":\"62d4a44c\",\"open_id\":\"ou_123456\"}",
                "node_key": "APPROVAL_240330_4058663",
                "files": [
                    {
                        "url": "https://p3-approval-sign.byteimg.com/lark-approval-attachment/image/20220714/1/332f3596-0845-4746-a4bc-818d54ad435b.png~tplv-ottatrvjsm-image.image?x-expires=1659033558&x-signature=6edF3k%2BaHeAuvfcBRGOkbckoUl4%3D#.png",
                        "file_size": 186823,
                        "title": "e018906140ed9388234bd03b0.png",
                        "type": "image"
                    }
                ]
            }
        ],
        "modified_instance_code": "81D31358-93AF-92D6-7425-01A5D67C4E71",
        "reverted_instance_code": "81D31358-93AF-92D6-7425-01A5D67C4E71",
        "approval_code": "7C468A54-8745-2245-9675-08B7C63E7A85",
        "reverted": false,
        "instance_code": "81D31358-93AF-92D6-7425-01A5D67C4E71"
    }
}"#;
    #[test]
    fn test_resp() {
        let res = serde_json::from_str::<GetApprovalInstanceRespInner>(RESP);
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
