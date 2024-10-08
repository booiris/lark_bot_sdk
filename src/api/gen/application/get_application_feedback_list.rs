//! doc url: <https://open.larkoffice.com/document/uAjLw4CM/ukTMukTMukTM/application-v6/application-feedback/list>
// Code generated by gen_api. DO NOT EDIT.

use serde::{Deserialize, Serialize};

use crate::api::BaseResp;
use crate::api::HasBaseResp;
use crate::core::model::{ApiRequest, CommonResponse, MethodOption, ReqParam, StreamReqParam};
use crate::{
    core::{http_client::HttpClient, store::Store},
    error::Error,
};

use crate::api::gen::application::ApplicationService;

impl<'c, IStore: Store, IClient: HttpClient> ApplicationService<'c, IStore, IClient> {
    /// **api 版本: 2024-07-23T07:32:42+00:00**
    ///
    /// ## 获取应用反馈列表
    ///
    /// 查询应用的反馈数据
    ///

    ///
    /// doc: <https://open.larkoffice.com/document/uAjLw4CM/ukTMukTMukTM/application-v6/application-feedback/list>
    ///
    /// new doc: <https://open.larkoffice.com/document/server-docs/application-v6/application-feedback/list>
    ///
    /// api url: <https://open.feishu.cn/document_portal/v1/document/get_detail?fullPath=%2Fserver-docs%2Fapplication-v6%2Fapplication-feedback%2Flist>
    pub async fn get_application_feedback_list(
        &self,
        req: GetApplicationFeedbackListReq,
    ) -> Result<(GetApplicationFeedbackListResp, CommonResponse), Error> {
        self.get_application_feedback_list_with_opt(req, Default::default())
            .await
    }

    /// 参见 [get_application_feedback_list](#method.get_application_feedback_list) 函数
    pub async fn get_application_feedback_list_with_opt(
        &self,
        req: GetApplicationFeedbackListReq,
        method_option: MethodOption,
    ) -> Result<(GetApplicationFeedbackListResp, CommonResponse), Error> {
        #[cfg(feature = "test-util")]
        {
            if let Some(f) = self.mock().get_mock_get_application_feedback_list(&req) {
                tracing::info!("[lark] Application#GetApplicationFeedbackList **mocking** api");
                return f(req);
            }
        }

        tracing::info!("[lark] Application#GetApplicationFeedbackList call api");

        let req = ApiRequest {
            scope: "Application",
            api: "GetApplicationFeedbackList",
            method: http::Method::GET,
            url: String::new()
                + self.cli.open_base_url.as_ref()
                + "/open-apis/application/v6/applications/:app_id/feedbacks",
            param_data: req.gen_param(),
            method_option,
            need_tenant_access_token: true,
            ..Default::default()
        };

        let (resp, common_resp): (GetApplicationFeedbackListRespInner, _) =
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
pub struct GetApplicationFeedbackListReq {
    /// 目标应用 ID（本租户创建的所有应用）
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "cli_9f115af860f7901b"
    #[api(kind = "path", name = "app_id")]
    pub app_id: String,
    /// 查询的起始日期，格式为yyyy-mm-dd。不填则默认为当前日期减去180天。
    ///
    /// **示例值**: "2022-01-30"
    #[api(kind = "query", name = "from_date", v_type = "var", option = "false")]
    pub from_date: String,
    /// 查询的结束日期，格式为yyyy-mm-dd。不填默认为当前日期。
    ///
    /// 只能查询 180 天内的数据。
    ///
    /// **示例值**: "2022-01-30"
    #[api(kind = "query", name = "to_date", v_type = "var", option = "false")]
    pub to_date: String,
    /// 反馈类型，不填写则表示查询所有反馈类型。
    ///
    /// **示例值**: "1"
    ///
    /// **可选值**:
    ///
    /// `Fault`: 故障反馈
    ///
    /// `Advice`: 产品建议
    #[api(
        kind = "query",
        name = "feedback_type",
        v_type = "var",
        option = "false"
    )]
    pub feedback_type: i64,
    /// 反馈处理状态，不填写则表示查询所有处理类型。
    ///
    /// **示例值**: "0"
    ///
    /// **可选值**:
    ///
    /// `Unmarked`: 反馈未处理
    ///
    /// `Marked`: 反馈已处理
    ///
    /// `Processing`: 反馈处理中
    ///
    /// `Closed`: 反馈已关闭
    #[api(kind = "query", name = "status", v_type = "var", option = "false")]
    pub status: i64,
    /// 用户 ID 类型
    ///
    /// **示例值**: "open_id"
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
    /// 分页标记，第一次请求不填，表示从头开始遍历；分页查询结果还有更多项时会同时返回新的 page_token，下次遍历可采用该 page_token 获取查询结果
    ///
    /// **示例值**: ""7064688334618378259""
    #[api(kind = "query", name = "page_token", v_type = "var", option = "false")]
    pub page_token: String,
    /// 本次拉取反馈列表最大个数
    ///
    /// **示例值**: "100"
    #[api(kind = "query", name = "page_size", v_type = "var", option = "false")]
    pub page_size: i64,
}

#[derive(Debug, Serialize, Deserialize, Clone, lark_bot_sdk_macros::ApiBaseResp)]
struct GetApplicationFeedbackListRespInner {
    #[serde(flatten)]
    data: Option<GetApplicationFeedbackListResp>,
    #[serde(flatten)]
    base: BaseResp,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct GetApplicationFeedbackListResp {
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
    /// 应用的反馈列表
    ///
    /// **数据校验规则**：
    ///
    /// - **长度范围**: `0` 字符- `1000` 字符
    #[serde(
        rename = "feedback_list",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub feedback_list: Vec<ApplicationFeedbackSubResp>,
    /// 是否还有更多项
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "true"
    #[serde(
        rename = "has_more",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub has_more: bool,
    /// 分页标记，当 has_more 为 true 时，会同时返回新的 page_token，否则不返回 page_token
    ///
    /// **示例值**: "7064688334618378259"
    #[serde(
        rename = "page_token",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub page_token: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(default)]
pub struct ApplicationFeedbackSubResp {
    /// 应用反馈 ID，应用反馈记录唯一标识
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "7057888018203574291"
    #[serde(
        rename = "feedback_id",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub feedback_id: String,
    /// 被反馈应用ID
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "cli_9b445f5258795107"
    #[serde(
        rename = "app_id",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub app_id: String,
    /// 反馈提交时间，格式为yyyy-mm-dd hh:mm:ss
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "2022-01-30 11:30:12"
    #[serde(
        rename = "feedback_time",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub feedback_time: String,
    /// 反馈用户的租户名， 查询 isv 应用时返回
    ///
    /// **示例值**: "字节跳动"
    #[serde(
        rename = "tenant_name",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub tenant_name: String,
    /// 反馈类型
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "1"
    ///
    /// **可选值**:
    ///
    /// `Fault`: 故障反馈
    ///
    /// `Advice`: 产品建议
    #[serde(
        rename = "feedback_type",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub feedback_type: i64,
    /// 反馈处理状态
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "0"
    ///
    /// **可选值**:
    ///
    /// `Unmarked`: 反馈未处理
    ///
    /// `Marked`: 反馈已处理
    ///
    /// `Processing`: 反馈处理中
    ///
    /// `Closed`: 反馈已关闭
    #[serde(
        rename = "status",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub status: i64,
    /// 故障类型列表：1: 黑屏 2: 白屏 3: 无法打开小程序  4: 卡顿 5: 小程序闪退 6: 页面加载慢 7: 死机 8: 其他异常
    ///
    /// **示例值**: "[1,2,3]"
    ///
    /// **数据校验规则**：
    ///
    /// - **长度范围**: `0` 字符- `30` 字符
    #[serde(
        rename = "fault_type",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub fault_type: Vec<i64>,
    /// 故障时间，格式为yyyy-mm-dd hh:mm:ss
    ///
    /// **示例值**: "2022-01-30 11:30:12"
    #[serde(
        rename = "fault_time",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub fault_time: String,
    /// 反馈来源：1： 小程序 2：网页应用 3：机器人 4：webSDK
    ///
    /// **示例值**: "1"
    ///
    /// **可选值**:
    ///
    /// `MP`: 小程序
    ///
    /// `WebPage`: 网页应用
    ///
    /// `Bot`: 机器人
    ///
    /// `WebSDK`: WebSDK
    #[serde(
        rename = "source",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub source: i64,
    /// 用户联系方式，只有用户填写联系方式后返回
    ///
    /// **示例值**: "wang@bytedance.com"
    #[serde(
        rename = "contact",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub contact: String,
    /// 反馈处理时间，格式为yyyy-mm-dd hh:mm:ss
    ///
    /// **示例值**: "2022-01-30 11:30:12"
    #[serde(
        rename = "update_time",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub update_time: String,
    /// 反馈问题描述
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "反馈描述"
    #[serde(
        rename = "description",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub description: String,
    /// 反馈用户id，租户内用户的唯一标识 ，ID值与查询参数中的user_id_type对应
    ///
    /// **示例值**: "ou_9565b69967831233761cc2f11b4c089f"
    #[serde(
        rename = "user_id",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub user_id: String,
    /// 操作者id，租户内用户的唯一标识， ID值与查询参数中的user_id_type 对应
    ///
    /// **示例值**: "ou_9565b69967831233761cc2f11b4c089f"
    #[serde(
        rename = "operator_id",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub operator_id: String,
    /// 反馈图片url列表，url 过期时间三天
    ///
    /// **数据校验规则**：
    ///
    /// - **最大长度**: `10` 字符
    #[serde(
        rename = "images",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub images: Vec<String>,
    /// 反馈页面路径
    ///
    /// - 如触发反馈的应用类型为小程序，则上报小程序当前页面的path信息
    ///
    /// - 如触发反馈的应用类型为网页或网页应用，则上报当前网页的url信息
    ///
    /// - 如为其他应用类型，则字段返回值为空
    ///
    /// **示例值**: "index/page"
    #[serde(
        rename = "feedback_path",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub feedback_path: String,
}

#[cfg(feature = "test-util")]
mod test_utils {
    use std::sync::Arc;

    use self::gen::application::ApplicationServiceMocker;
    use crate::core::mocker::*;

    use super::*;
    use crate::api::*;
    use crate::core::model::*;

    pub struct Mocker;

    pub trait MockFunc:
        Fn(
            GetApplicationFeedbackListReq,
        ) -> Result<(GetApplicationFeedbackListResp, CommonResponse), Error>
        + Send
        + Sync
        + 'static
    {
    }
    impl<
            T: Fn(
                    GetApplicationFeedbackListReq,
                )
                    -> Result<(GetApplicationFeedbackListResp, CommonResponse), Error>
                + Send
                + Sync
                + 'static,
        > MockFunc for T
    {
    }

    impl<'c, IStore: Store, IClient: HttpClient> ApplicationServiceMocker<'c, IStore, IClient> {
        pub fn mock_get_application_feedback_list<F: MockFunc>(
            &self,
            f: F,
        ) -> MockerBuilder<
            Mocker,
            GetApplicationFeedbackListReq,
            GetApplicationFeedbackListResp,
            Arc<dyn MockFunc>,
        > {
            MockerBuilder::new(self.cli.instance_id, Arc::new(f))
        }

        pub(super) fn get_mock_get_application_feedback_list(
            &self,
            req: &GetApplicationFeedbackListReq,
        ) -> Option<Arc<dyn MockFunc>> {
            do_mock::<
                Mocker,
                GetApplicationFeedbackListReq,
                GetApplicationFeedbackListResp,
                Arc<dyn MockFunc>,
            >(self.cli.instance_id, req)
        }
    }
}

#[cfg(feature = "test-util")]
#[cfg(test)]
mod test {
    use crate::{
        api::gen::application::get_application_feedback_list::{
            GetApplicationFeedbackListReq, GetApplicationFeedbackListResp,
            GetApplicationFeedbackListRespInner,
        },
        core::{model::CommonResponse, Lark},
    };

    #[tokio::test]
    async fn test_mock() {
        let lark = Lark::new("".to_owned(), "".into());
        let mocker = lark
            .application()
            .mock()
            .mock_get_application_feedback_list(|_| {
                Ok((
                    GetApplicationFeedbackListResp::default(),
                    CommonResponse::default(),
                ))
            })
            .build();
        let res = lark
            .application()
            .get_application_feedback_list(GetApplicationFeedbackListReq::default())
            .await;
        assert!(res.is_ok());
        mocker.clear();
        let res = lark
            .application()
            .get_application_feedback_list(GetApplicationFeedbackListReq::default())
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
        "feedback_list": [
            {
                "feedback_id": "7057888018203574291",
                "app_id": "cli_9b445f5258795107",
                "feedback_time": "2022-01-30 11:30:12",
                "tenant_name": "字节跳动",
                "feedback_type": 1,
                "status": 0,
                "fault_type": [
                    1,
                    2,
                    3
                ],
                "fault_time": "2022-01-30 11:30:12",
                "source": 1,
                "contact": "wang@bytedance.com",
                "update_time": "2022-01-30 11:30:12",
                "description": "反馈描述",
                "user_id": "ou_9565b69967831233761cc2f11b4c089f",
                "operator_id": "ou_9565b69967831233761cc2f11b4c089f",
                "images": [
                    "https://p6-lark-openplatform-image-sign.byteimg.com/*"
                ],
                "feedback_path": "index/page"
            }
        ],
        "has_more": true,
        "page_token": "7064688334618378259"
    }
}"#;
    #[test]
    fn test_resp() {
        let res = serde_json::from_str::<GetApplicationFeedbackListRespInner>(RESP);
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
