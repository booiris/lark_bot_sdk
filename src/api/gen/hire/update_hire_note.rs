//! doc url: <https://open.feishu.cn/document/ukTMukTMukTM/uMzM1YjLzMTN24yMzUjN/hire-v1/note/patch>
// Code generated by gen_api. DO NOT EDIT.

use serde::{Deserialize, Serialize};

use crate::api::BaseResp;
use crate::api::HasBaseResp;
use crate::core::model::{ApiRequest, CommonResponse, MethodOption, ReqParam, StreamReqParam};
use crate::{
    core::{http_client::HttpClient, store::Store},
    error::Error,
};

use crate::api::gen::hire::HireService;

impl<'c, IStore: Store, IClient: HttpClient> HireService<'c, IStore, IClient> {
    /// **api 版本: 2024-07-10T14:33:08+00:00**
    ///
    /// ## 更新备注
    ///
    /// 根据备注 ID 更新备注信息。
    ///

    ///
    /// doc: <https://open.feishu.cn/document/ukTMukTMukTM/uMzM1YjLzMTN24yMzUjN/hire-v1/note/patch>
    ///
    /// new doc: <https://open.feishu.cn/document/server-docs/hire-v1/candidate-management/note/patch>
    ///
    /// api url: <https://open.feishu.cn/document_portal/v1/document/get_detail?fullPath=%2Fserver-docs%2Fhire-v1%2Fcandidate-management%2Fnote%2Fpatch>
    pub async fn update_hire_note(
        &self,
        req: UpdateHireNoteReq,
    ) -> Result<(UpdateHireNoteResp, CommonResponse), Error> {
        self.update_hire_note_with_opt(req, Default::default())
            .await
    }

    /// 参见 [update_hire_note](#method.update_hire_note) 函数
    pub async fn update_hire_note_with_opt(
        &self,
        req: UpdateHireNoteReq,
        method_option: MethodOption,
    ) -> Result<(UpdateHireNoteResp, CommonResponse), Error> {
        #[cfg(feature = "test-util")]
        {
            if let Some(f) = self.mock().get_mock_update_hire_note(&req) {
                tracing::info!("[lark] Hire#UpdateHireNote **mocking** api");
                return f(req);
            }
        }

        tracing::info!("[lark] Hire#UpdateHireNote call api");

        let req = ApiRequest {
            scope: "Hire",
            api: "UpdateHireNote",
            method: http::Method::PATCH,
            url: String::new()
                + self.cli.open_base_url.as_ref()
                + "/open-apis/hire/v1/notes/:note_id",
            param_data: req.gen_param(),
            method_option,
            need_tenant_access_token: true,
            ..Default::default()
        };

        let (resp, common_resp): (UpdateHireNoteRespInner, _) = self.cli.do_req(req).await?;
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
pub struct UpdateHireNoteReq {
    /// 备注 ID，可通过[获取备注列表](https://open.feishu.cn/document/ukTMukTMukTM/uMzM1YjLzMTN24yMzUjN/hire-v1/note/list)获取
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "6960663240925956401"
    #[api(kind = "path", name = "note_id")]
    pub note_id: String,
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
    /// `people_admin_id`: 以people_admin_id来识别用户
    #[api(
        kind = "query",
        name = "user_id_type",
        v_type = "var",
        option = "false"
    )]
    pub user_id_type: String,
    /// 备注内容
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "这是一个备注"
    #[api(kind = "body", name = "content")]
    pub content: String,
    /// 更新人 ID，请传入与`user_id_type`相匹配的ID
    ///
    /// **示例值**: "ou_f476cb099ac9227c9bae09ce46112579"
    #[api(kind = "body", name = "operator_id")]
    pub operator_id: Option<String>,
    /// 是否通知被@的用户
    ///
    /// **示例值**: "false"
    #[api(kind = "body", name = "notify_mentioned_user")]
    pub notify_mentioned_user: Option<bool>,
    /// 被@用户列表
    #[api(kind = "body", name = "mention_entity_list")]
    pub mention_entity_list: Vec<Option<MentionEntitySubReq>>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(default)]
pub struct MentionEntitySubReq {
    /// 被@用户在 content 中的偏移量
    ///
    /// - 取值范围：0 ~ content.length
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "3"
    #[serde(
        rename = "offset",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub offset: i64,
    /// 被@用户的 ID，请传入与 `user_id_type` 类型相匹配的 ID
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "on_94a1ee5551019f18cd73d9f111898cf2"
    #[serde(
        rename = "user_id",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub user_id: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, lark_bot_sdk_macros::ApiBaseResp)]
struct UpdateHireNoteRespInner {
    #[serde(flatten)]
    data: Option<UpdateHireNoteResp>,
    #[serde(flatten)]
    base: BaseResp,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct UpdateHireNoteResp {
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
    /// 备注数据
    #[serde(
        rename = "note",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub note: NoteSubResp,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(default)]
pub struct NoteSubResp {
    /// 备注ID
    ///
    /// **示例值**: "6949805467799537964"
    #[serde(
        rename = "id",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub id: String,
    /// 人才ID
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "6916472453069883661"
    #[serde(
        rename = "talent_id",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub talent_id: String,
    /// 投递ID
    ///
    /// **示例值**: "6891565253964859661"
    #[serde(
        rename = "application_id",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub application_id: String,
    /// 是否私密
    ///
    /// **示例值**: "false"
    #[serde(
        rename = "is_private",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub is_private: bool,
    /// 创建时间，毫秒时间戳
    ///
    /// **示例值**: "1618209327096"
    #[serde(
        rename = "create_time",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub create_time: i64,
    /// 更新时间，毫秒时间戳
    ///
    /// **示例值**: "1618209327096"
    #[serde(
        rename = "modify_time",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub modify_time: i64,
    /// 创建人ID，与入参 `user_id_type` 类型一致
    ///
    /// **示例值**: "ou_f476cb099ac9227c9bae09ce46112579"
    #[serde(
        rename = "creator_id",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub creator_id: String,
    /// 备注内容
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "这是一个备注"
    #[serde(
        rename = "content",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub content: String,
}

#[cfg(feature = "test-util")]
mod test_utils {
    use std::sync::Arc;

    use self::gen::hire::HireServiceMocker;
    use crate::core::mocker::*;

    use super::*;
    use crate::api::*;
    use crate::core::model::*;

    pub struct Mocker;

    pub trait MockFunc:
        Fn(UpdateHireNoteReq) -> Result<(UpdateHireNoteResp, CommonResponse), Error>
        + Send
        + Sync
        + 'static
    {
    }
    impl<
            T: Fn(UpdateHireNoteReq) -> Result<(UpdateHireNoteResp, CommonResponse), Error>
                + Send
                + Sync
                + 'static,
        > MockFunc for T
    {
    }

    impl<'c, IStore: Store, IClient: HttpClient> HireServiceMocker<'c, IStore, IClient> {
        pub fn mock_update_hire_note<F: MockFunc>(
            &self,
            f: F,
        ) -> MockerBuilder<Mocker, UpdateHireNoteReq, UpdateHireNoteResp, Arc<dyn MockFunc>>
        {
            MockerBuilder::new(self.cli.instance_id, Arc::new(f))
        }

        pub(super) fn get_mock_update_hire_note(
            &self,
            req: &UpdateHireNoteReq,
        ) -> Option<Arc<dyn MockFunc>> {
            do_mock::<Mocker, UpdateHireNoteReq, UpdateHireNoteResp, Arc<dyn MockFunc>>(
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
        api::gen::hire::update_hire_note::{
            UpdateHireNoteReq, UpdateHireNoteResp, UpdateHireNoteRespInner,
        },
        core::{model::CommonResponse, Lark},
    };

    #[tokio::test]
    async fn test_mock() {
        let lark = Lark::new("".to_owned(), "".into());
        let mocker = lark
            .hire()
            .mock()
            .mock_update_hire_note(|_| {
                Ok((UpdateHireNoteResp::default(), CommonResponse::default()))
            })
            .build();
        let res = lark
            .hire()
            .update_hire_note(UpdateHireNoteReq::default())
            .await;
        assert!(res.is_ok());
        mocker.clear();
        let res = lark
            .hire()
            .update_hire_note(UpdateHireNoteReq::default())
            .await;
        assert!(res.is_err());
    }

    const REQ: &str = r#"{
    "content": "这是一个备注",
    "operator_id": "ou_f476cb099ac9227c9bae09ce46112579",
    "notify_mentioned_user": false,
    "mention_entity_list": [
        {
            "offset": 3,
            "user_id": "on_94a1ee5551019f18cd73d9f111898cf2"
        }
    ]
}"#;

    #[test]
    fn test_req() {
        if REQ == "{}" {
            return;
        }
        if let Err(e) = serde_json::from_str::<super::UpdateHireNoteReqBody>(REQ) {
            panic!("{}", e);
        }
    }

    const RESP: &str = r#"{
    "code": 0,
    "msg": "success",
    "data": {
        "note": {
            "id": "6949805467799537964",
            "talent_id": "6916472453069883661",
            "application_id": "6891565253964859661",
            "is_private": false,
            "create_time": 1618209327096,
            "modify_time": 1618209327096,
            "creator_id": "ou_f476cb099ac9227c9bae09ce46112579",
            "content": "这是一个备注"
        }
    }
}"#;
    #[test]
    fn test_resp() {
        let res = serde_json::from_str::<UpdateHireNoteRespInner>(RESP);
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
