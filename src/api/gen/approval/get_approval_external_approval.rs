//! doc url: <https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/approval-v4/external_approval/get>
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
    /// **api 版本: 2023-07-13T06:47:08+00:00**
    ///
    /// ## 查看指定三方审批定义
    ///
    /// 根据 Approval Code 获取之前同步的某个三方审批定义的详情数据
    ///

    ///
    /// doc: <https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/approval-v4/external_approval/get>
    ///
    /// new doc: <https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/approval-v4/external_approval/get>
    ///
    /// api url: <https://open.feishu.cn/document_portal/v1/document/get_detail?fullPath=%2FuAjLw4CM%2FukTMukTMukTM%2Freference%2Fapproval-v4%2Fexternal_approval%2Fget>
    pub async fn get_approval_external_approval(
        &self,
        req: GetApprovalExternalApprovalReq,
    ) -> Result<(GetApprovalExternalApprovalResp, CommonResponse), Error> {
        self.get_approval_external_approval_with_opt(req, Default::default())
            .await
    }

    /// 参见 [get_approval_external_approval](#method.get_approval_external_approval) 函数
    pub async fn get_approval_external_approval_with_opt(
        &self,
        req: GetApprovalExternalApprovalReq,
        method_option: MethodOption,
    ) -> Result<(GetApprovalExternalApprovalResp, CommonResponse), Error> {
        #[cfg(feature = "test-util")]
        {
            if let Some(f) = self.mock().get_mock_get_approval_external_approval(&req) {
                tracing::info!("[lark] Approval#GetApprovalExternalApproval **mocking** api");
                return f(req);
            }
        }

        tracing::info!("[lark] Approval#GetApprovalExternalApproval call api");

        let req = ApiRequest {
            scope: "Approval",
            api: "GetApprovalExternalApproval",
            method: http::Method::GET,
            url: String::new()
                + self.cli.open_base_url.as_ref()
                + "/open-apis/approval/v4/external_approvals/:approval_code",
            param_data: req.gen_param(),
            method_option,
            need_tenant_access_token: true,
            ..Default::default()
        };

        let (resp, common_resp): (GetApprovalExternalApprovalRespInner, _) =
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
pub struct GetApprovalExternalApprovalReq {
    /// 调用[创建三方审批定义](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/approval-v4/external_approval/create)时返回的审批定义code
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "7C468A54-8745-2245-9675-08B7C63E7A85"
    #[api(kind = "path", name = "approval_code")]
    pub approval_code: String,
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
}

#[derive(Debug, Serialize, Deserialize, Clone, lark_bot_sdk_macros::ApiBaseResp)]
struct GetApprovalExternalApprovalRespInner {
    #[serde(flatten)]
    data: Option<GetApprovalExternalApprovalResp>,
    #[serde(flatten)]
    base: BaseResp,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct GetApprovalExternalApprovalResp {
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
    /// 审批定义名称
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "@i18n@1"
    #[serde(
        rename = "approval_name",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub approval_name: String,
    /// 创建三方审批定义时传入的定义code
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "permission_test"
    #[serde(
        rename = "approval_code",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub approval_code: String,
    /// 审批定义所属分组
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "work_group"
    #[serde(
        rename = "group_code",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub group_code: String,
    /// 分组名称
    ///
    /// **示例值**: "@i18n@2"
    #[serde(
        rename = "group_name",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub group_name: String,
    /// 审批定义的说明
    ///
    /// **示例值**: "@i18n@2"
    #[serde(
        rename = "description",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub description: String,
    /// 三方审批定义相关
    #[serde(
        rename = "external",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub external: ApprovalCreateExternalSubResp,
    /// 可见人列表
    #[serde(
        rename = "viewers",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub viewers: Vec<ApprovalCreateViewersSubResp>,
    /// 国际化文案
    #[serde(
        rename = "i18n_resources",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub i18n_resources: Vec<I18nResourceSubResp>,
    /// 流程管理员
    #[serde(
        rename = "managers",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub managers: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(default)]
pub struct ApprovalCreateExternalSubResp {
    /// 列表中用于提示审批来自哪里，i18n key， 注意不需要“来自”前缀，审批中心会拼上前缀
    ///
    /// **示例值**: "@i18n@3"
    #[serde(
        rename = "biz_name",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub biz_name: String,
    /// 审批定义业务类别
    ///
    /// **示例值**: "permission"
    #[serde(
        rename = "biz_type",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub biz_type: String,
    /// 移动端发起链接，如果设置了该链接，则会在移动端审批发起页展示该审批，用户点击后会跳转到该链接进行发起； 如果不填，则在mobile端不显示该审批
    ///
    /// **示例值**: "https://applink.feishu.cn/client/mini_program/open?appId=cli_9c90fc38e07a9101&path=pages/approval-form/index?id=9999"
    #[serde(
        rename = "create_link_mobile",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub create_link_mobile: String,
    /// PC端发起链接，如果设置了该链接，则会在PC端审批发起页展示该审批，用户点击后会跳转到该链接进行发起； 如果不填，则在PC端不显示该审批
    ///
    /// **示例值**: "https://applink.feishu.cn/client/mini_program/open?mode=appCenter&appId=cli_9c90fc38e07a9101&path=pc/pages/create-form/index?id=9999"
    #[serde(
        rename = "create_link_pc",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub create_link_pc: String,
    /// 审批实例、审批任务、审批抄送是否要在PC端展示，如果为 true，则PC端列表会展示该定义下的实例信息；否则，不展示
    ///
    /// **示例值**: "true"
    #[serde(
        rename = "support_pc",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub support_pc: bool,
    /// 审批实例、审批任务、审批抄送是否要在移动端展示，如果为 true，则移动端列表会展示该定义下的实例信息；否则，不展示
    ///
    /// **示例值**: "true"
    #[serde(
        rename = "support_mobile",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub support_mobile: bool,
    /// 是否支持批量已读
    ///
    /// **示例值**: "true"
    #[serde(
        rename = "support_batch_read",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub support_batch_read: bool,
    /// 是否支持标注可读
    ///
    /// **示例值**: "true"
    #[serde(
        rename = "enable_mark_readed",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub enable_mark_readed: bool,
    /// 是否支持快速操作
    ///
    /// **示例值**: "true"
    #[serde(
        rename = "enable_quick_operate",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub enable_quick_operate: bool,
    /// 三方系统的操作回调 url，【待审批】列表的任务审批人点同意或拒绝操作后，审批中心调用该地址通知三方系统，回调地址相关信息可参考：[三方快捷审批回调
    ///
    /// ](https://open.feishu.cn/document/ukTMukTMukTM/ukjNyYjL5YjM24SO2IjN/quick-approval-callback)
    ///
    /// **示例值**: "http://www.feishu.cn/approval/openapi/instanceOperate"
    #[serde(
        rename = "action_callback_url",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub action_callback_url: String,
    /// 回调时带的 token， 用于业务系统验证请求来自审批,具体参考: [回调token部分](https://open.feishu.cn/document/ukTMukTMukTM/uUTNz4SN1MjL1UzM)
    ///
    /// **示例值**: "sdjkljkx9lsadf110"
    #[serde(
        rename = "action_callback_token",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub action_callback_token: String,
    /// 请求参数加密密钥，如果配置了该参数，则会对请求参数进行加密，业务需要对请求进行解密，加解密算法参考:[加密部分](https://open.feishu.cn/document/ukTMukTMukTM/uADM4QjLwADO04CMwgDN)
    ///
    /// **示例值**: "gfdqedvsadfgfsd"
    #[serde(
        rename = "action_callback_key",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub action_callback_key: String,
    /// 是否支持批量审批
    ///
    /// **示例值**: "true"
    #[serde(
        rename = "allow_batch_operate",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub allow_batch_operate: bool,
    /// 审批流程数据是否不纳入效率统计
    ///
    /// **示例值**: "true"
    #[serde(
        rename = "exclude_efficiency_statistics",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub exclude_efficiency_statistics: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(default)]
pub struct ApprovalCreateViewersSubResp {
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
    pub viewer_type: String,
    /// 当 viewer_type 是 USER，根据user_id_type返回用户id
    ///
    /// **示例值**: "19a294c2"
    #[serde(
        rename = "viewer_user_id",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub viewer_user_id: String,
    /// 当 view_type 为DEPARTMENT，根据department_id_type返回部门id
    ///
    /// **示例值**: "od-ac9d697abfa990b715dcc33d58a62a9d"
    #[serde(
        rename = "viewer_department_id",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub viewer_department_id: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(default)]
pub struct I18nResourceSubResp {
    /// 语言
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
    /// 文案 key, value, i18n key 以 @i18n@ 开头； 该字段主要用于做国际化，允许用户同时传多个语言的文案，审批中心会根据用户当前的语音环境使用对应的文案，如果没有传用户当前的语音环境文案，则会使用默认的语言文案
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "{ "@i18n@1": "权限申请", "@i18n@2": "OA审批", "@i18n@3": "Permission" }"
    #[serde(
        rename = "texts",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub texts: Vec<I18nResourceTextSubResp>,
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
pub struct I18nResourceTextSubResp {
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
    /// 文案value
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "审批定义"
    #[serde(
        rename = "value",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub value: String,
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
        Fn(
            GetApprovalExternalApprovalReq,
        ) -> Result<(GetApprovalExternalApprovalResp, CommonResponse), Error>
        + Send
        + Sync
        + 'static
    {
    }
    impl<
            T: Fn(
                    GetApprovalExternalApprovalReq,
                )
                    -> Result<(GetApprovalExternalApprovalResp, CommonResponse), Error>
                + Send
                + Sync
                + 'static,
        > MockFunc for T
    {
    }

    impl<'c, IStore: Store, IClient: HttpClient> ApprovalServiceMocker<'c, IStore, IClient> {
        pub fn mock_get_approval_external_approval<F: MockFunc>(
            &self,
            f: F,
        ) -> MockerBuilder<
            Mocker,
            GetApprovalExternalApprovalReq,
            GetApprovalExternalApprovalResp,
            Arc<dyn MockFunc>,
        > {
            MockerBuilder::new(self.cli.instance_id, Arc::new(f))
        }

        pub(super) fn get_mock_get_approval_external_approval(
            &self,
            req: &GetApprovalExternalApprovalReq,
        ) -> Option<Arc<dyn MockFunc>> {
            do_mock::<
                Mocker,
                GetApprovalExternalApprovalReq,
                GetApprovalExternalApprovalResp,
                Arc<dyn MockFunc>,
            >(self.cli.instance_id, req)
        }
    }
}

#[cfg(feature = "test-util")]
#[cfg(test)]
mod test {
    use crate::{
        api::gen::approval::get_approval_external_approval::{
            GetApprovalExternalApprovalReq, GetApprovalExternalApprovalResp,
            GetApprovalExternalApprovalRespInner,
        },
        core::{model::CommonResponse, Lark},
    };

    #[tokio::test]
    async fn test_mock() {
        let lark = Lark::new("".to_owned(), "".into());
        let mocker = lark
            .approval()
            .mock()
            .mock_get_approval_external_approval(|_| {
                Ok((
                    GetApprovalExternalApprovalResp::default(),
                    CommonResponse::default(),
                ))
            })
            .build();
        let res = lark
            .approval()
            .get_approval_external_approval(GetApprovalExternalApprovalReq::default())
            .await;
        assert!(res.is_ok());
        mocker.clear();
        let res = lark
            .approval()
            .get_approval_external_approval(GetApprovalExternalApprovalReq::default())
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
        "approval_name": "@i18n@1",
        "approval_code": "permission_test",
        "group_code": "work_group",
        "group_name": "@i18n@2",
        "description": "@i18n@2",
        "external": {
            "biz_name": "@i18n@3",
            "biz_type": "permission",
            "create_link_mobile": "https://applink.feishu.cn/client/mini_program/open?appId=cli_9c90fc38e07a9101&path=pages/approval-form/index?id=9999",
            "create_link_pc": "https://applink.feishu.cn/client/mini_program/open?mode=appCenter&appId=cli_9c90fc38e07a9101&path=pc/pages/create-form/index?id=9999",
            "support_pc": true,
            "support_mobile": true,
            "support_batch_read": true,
            "enable_mark_readed": true,
            "enable_quick_operate": true,
            "action_callback_url": "http://www.feishu.cn/approval/openapi/instanceOperate",
            "action_callback_token": "sdjkljkx9lsadf110",
            "action_callback_key": "gfdqedvsadfgfsd",
            "allow_batch_operate": true,
            "exclude_efficiency_statistics": true
        },
        "viewers": [
            {
                "viewer_type": "USER",
                "viewer_user_id": "19a294c2",
                "viewer_department_id": "od-ac9d697abfa990b715dcc33d58a62a9d"
            }
        ],
        "i18n_resources": [
            {
                "locale": "zh-CN",
                "texts": [
                    {
                        "key": "@i18n@1",
                        "value": "审批定义"
                    }
                ],
                "is_default": true
            }
        ],
        "managers": [
            "19a294c2"
        ]
    }
}"#;
    #[test]
    fn test_resp() {
        let res = serde_json::from_str::<GetApprovalExternalApprovalRespInner>(RESP);
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
