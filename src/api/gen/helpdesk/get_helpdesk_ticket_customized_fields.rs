//! doc url: <https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/helpdesk-v1/ticket/customized_fields>
// Code generated by gen_api. DO NOT EDIT.

use serde::{Deserialize, Serialize};

use crate::api::BaseResp;
use crate::api::HasBaseResp;
use crate::core::model::{ApiRequest, CommonResponse, MethodOption, ReqParam, StreamReqParam};
use crate::{
    core::{http_client::HttpClient, store::Store},
    error::Error,
};

use crate::api::gen::helpdesk::HelpdeskService;

impl<'c, IStore: Store, IClient: HttpClient> HelpdeskService<'c, IStore, IClient> {
    /// **api 版本: 2023-08-15T07:34:21+00:00**
    ///
    /// ## 获取服务台自定义字段
    ///
    /// 该接口用于获取服务台自定义字段详情。
    ///

    ///
    /// doc: <https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/helpdesk-v1/ticket/customized_fields>
    ///
    /// new doc: <https://open.feishu.cn/document/server-docs/helpdesk-v1/ticket-management/ticket/customized_fields>
    ///
    /// api url: <https://open.feishu.cn/document_portal/v1/document/get_detail?fullPath=%2Fserver-docs%2Fhelpdesk-v1%2Fticket-management%2Fticket%2Fcustomized_fields>
    pub async fn get_helpdesk_ticket_customized_fields(
        &self,
        req: GetHelpdeskTicketCustomizedFieldsReq,
    ) -> Result<(GetHelpdeskTicketCustomizedFieldsResp, CommonResponse), Error> {
        self.get_helpdesk_ticket_customized_fields_with_opt(req, Default::default())
            .await
    }

    /// 参见 [get_helpdesk_ticket_customized_fields](#method.get_helpdesk_ticket_customized_fields) 函数
    pub async fn get_helpdesk_ticket_customized_fields_with_opt(
        &self,
        req: GetHelpdeskTicketCustomizedFieldsReq,
        method_option: MethodOption,
    ) -> Result<(GetHelpdeskTicketCustomizedFieldsResp, CommonResponse), Error> {
        #[cfg(feature = "test-util")]
        {
            if let Some(f) = self
                .mock()
                .get_mock_get_helpdesk_ticket_customized_fields(&req)
            {
                tracing::info!("[lark] Helpdesk#GetHelpdeskTicketCustomizedFields **mocking** api");
                return f(req);
            }
        }

        tracing::info!("[lark] Helpdesk#GetHelpdeskTicketCustomizedFields call api");

        let req = ApiRequest {
            scope: "Helpdesk",
            api: "GetHelpdeskTicketCustomizedFields",
            method: http::Method::GET,
            url: String::new()
                + self.cli.open_base_url.as_ref()
                + "/open-apis/helpdesk/v1/customized_fields",
            param_data: req.gen_param(),
            method_option,
            need_tenant_access_token: true,
            ..Default::default()
        };

        let (resp, common_resp): (GetHelpdeskTicketCustomizedFieldsRespInner, _) =
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
pub struct GetHelpdeskTicketCustomizedFieldsReq {
    /// visible only
    ///
    /// **示例值**: "true"
    #[api(
        kind = "query",
        name = "visible_only",
        v_type = "var",
        option = "false"
    )]
    pub visible_only: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone, lark_bot_sdk_macros::ApiBaseResp)]
struct GetHelpdeskTicketCustomizedFieldsRespInner {
    #[serde(flatten)]
    data: Option<GetHelpdeskTicketCustomizedFieldsResp>,
    #[serde(flatten)]
    base: BaseResp,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct GetHelpdeskTicketCustomizedFieldsResp {
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
    /// 用户自定义字段
    #[serde(
        rename = "user_customized_fields",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub user_customized_fields: Vec<UserCustomizedFieldSubResp>,
    /// 自定义工单字段
    #[serde(
        rename = "ticket_customized_fields",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub ticket_customized_fields: Vec<TicketCustomizedFieldSubResp>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(default)]
pub struct UserCustomizedFieldSubResp {
    /// 字段ID
    ///
    /// **示例值**: "6746384425543548981"
    #[serde(
        rename = "user_customized_field_id",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub user_customized_field_id: String,
    /// 旧字段ID，向后兼容用
    ///
    /// **示例值**: "6746384425543548981"
    #[serde(
        rename = "id",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub id: String,
    /// 服务台ID
    ///
    /// **示例值**: "1542164574896126"
    #[serde(
        rename = "helpdesk_id",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub helpdesk_id: String,
    /// 字段键
    ///
    /// **示例值**: "company_id3"
    #[serde(
        rename = "key_name",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub key_name: String,
    /// 字段展示名称
    ///
    /// **示例值**: "Company ID"
    #[serde(
        rename = "display_name",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub display_name: String,
    /// 字段在列表中的展示位置
    ///
    /// **示例值**: "1"
    #[serde(
        rename = "position",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub position: String,
    /// 字段类型
    ///
    /// **示例值**: "string"
    #[serde(
        rename = "field_type",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub field_type: String,
    /// 字段描述信息
    ///
    /// **示例值**: "租户ID"
    #[serde(
        rename = "description",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub description: String,
    /// 字段是否可见
    ///
    /// **示例值**: "false"
    #[serde(
        rename = "visible",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub visible: bool,
    /// 字段是否可编辑
    ///
    /// **示例值**: "false"
    #[serde(
        rename = "editable",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub editable: bool,
    /// 字段是否必填
    ///
    /// **示例值**: "false"
    #[serde(
        rename = "required",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub required: bool,
    /// 字段创建时间
    ///
    /// **示例值**: "1574040677000"
    #[serde(
        rename = "created_at",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub created_at: String,
    /// 字段修改时间
    ///
    /// **示例值**: "1574040677000"
    #[serde(
        rename = "updated_at",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub updated_at: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(default)]
pub struct TicketCustomizedFieldSubResp {
    /// 工单自定义字段ID
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "6834320707288072194"
    #[serde(
        rename = "ticket_customized_field_id",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub ticket_customized_field_id: String,
    /// 服务台ID
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "1542164574896126"
    #[serde(
        rename = "helpdesk_id",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub helpdesk_id: String,
    /// 键名
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "test dropdown"
    #[serde(
        rename = "key_name",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub key_name: String,
    /// 名称
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "test dropdown"
    #[serde(
        rename = "display_name",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub display_name: String,
    /// 字段在列表后台管理列表中的位置
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "3"
    #[serde(
        rename = "position",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub position: String,
    /// 类型
    ///
    /// string - 单行文本
    ///
    /// multiline - 多行文本
    ///
    /// dropdown - 下拉列表
    ///
    /// dropdown_nested - 级联下拉
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "dropdown"
    #[serde(
        rename = "field_type",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub field_type: String,
    /// 描述
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "下拉示例"
    #[serde(
        rename = "description",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub description: String,
    /// 是否可见
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "true"
    #[serde(
        rename = "visible",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub visible: bool,
    /// 是否可以修改
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "true"
    #[serde(
        rename = "editable",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub editable: bool,
    /// 是否必填
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "false"
    #[serde(
        rename = "required",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub required: bool,
    /// 创建时间
    ///
    /// **示例值**: "1591239289000"
    #[serde(
        rename = "created_at",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub created_at: String,
    /// 更新时间
    ///
    /// **示例值**: "1591239289000"
    #[serde(
        rename = "updated_at",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub updated_at: String,
    /// 创建用户
    #[serde(
        rename = "created_by",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub created_by: TicketUserSubResp,
    /// 更新用户
    #[serde(
        rename = "updated_by",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub updated_by: TicketUserSubResp,
    /// 是否支持多选，仅在字段类型是dropdown的时候有效
    ///
    /// **示例值**: "true"
    #[serde(
        rename = "dropdown_allow_multiple",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub dropdown_allow_multiple: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(default)]
pub struct TicketUserSubResp {
    /// 用户ID
    ///
    /// **示例值**: "ou_37019b7c830210acd88fdce886e25c71"
    #[serde(
        rename = "id",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub id: String,
    /// 用户头像url
    ///
    /// **示例值**: "https://xxxx"
    #[serde(
        rename = "avatar_url",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub avatar_url: String,
    /// 用户名
    ///
    /// **示例值**: "abc"
    #[serde(
        rename = "name",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub name: String,
    /// 用户邮箱
    ///
    /// **示例值**: "xxxx@abc.com"
    #[serde(
        rename = "email",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub email: String,
    /// 所在部门名称
    ///
    /// **示例值**: "用户部门名称(有权限才展示)"
    #[serde(
        rename = "department",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub department: String,
    /// 城市
    ///
    /// **示例值**: "城市"
    #[serde(
        rename = "city",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub city: String,
    /// 国家代号(CountryCode)，参考：http://www.mamicode.com/info-detail-2186501.html
    ///
    /// **示例值**: "国家"
    #[serde(
        rename = "country",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub country: String,
}

#[cfg(feature = "test-util")]
mod test_utils {
    use std::sync::Arc;

    use self::gen::helpdesk::HelpdeskServiceMocker;
    use crate::core::mocker::*;

    use super::*;
    use crate::api::*;
    use crate::core::model::*;

    pub struct Mocker;

    pub trait MockFunc:
        Fn(
            GetHelpdeskTicketCustomizedFieldsReq,
        ) -> Result<(GetHelpdeskTicketCustomizedFieldsResp, CommonResponse), Error>
        + Send
        + Sync
        + 'static
    {
    }
    impl<
            T: Fn(
                    GetHelpdeskTicketCustomizedFieldsReq,
                )
                    -> Result<(GetHelpdeskTicketCustomizedFieldsResp, CommonResponse), Error>
                + Send
                + Sync
                + 'static,
        > MockFunc for T
    {
    }

    impl<'c, IStore: Store, IClient: HttpClient> HelpdeskServiceMocker<'c, IStore, IClient> {
        pub fn mock_get_helpdesk_ticket_customized_fields<F: MockFunc>(
            &self,
            f: F,
        ) -> MockerBuilder<
            Mocker,
            GetHelpdeskTicketCustomizedFieldsReq,
            GetHelpdeskTicketCustomizedFieldsResp,
            Arc<dyn MockFunc>,
        > {
            MockerBuilder::new(self.cli.instance_id, Arc::new(f))
        }

        pub(super) fn get_mock_get_helpdesk_ticket_customized_fields(
            &self,
            req: &GetHelpdeskTicketCustomizedFieldsReq,
        ) -> Option<Arc<dyn MockFunc>> {
            do_mock::<
                Mocker,
                GetHelpdeskTicketCustomizedFieldsReq,
                GetHelpdeskTicketCustomizedFieldsResp,
                Arc<dyn MockFunc>,
            >(self.cli.instance_id, req)
        }
    }
}

#[cfg(feature = "test-util")]
#[cfg(test)]
mod test {
    use crate::{
        api::gen::helpdesk::get_helpdesk_ticket_customized_fields::{
            GetHelpdeskTicketCustomizedFieldsReq, GetHelpdeskTicketCustomizedFieldsResp,
            GetHelpdeskTicketCustomizedFieldsRespInner,
        },
        core::{model::CommonResponse, Lark},
    };

    #[tokio::test]
    async fn test_mock() {
        let lark = Lark::new("".to_owned(), "".into());
        let mocker = lark
            .helpdesk()
            .mock()
            .mock_get_helpdesk_ticket_customized_fields(|_| {
                Ok((
                    GetHelpdeskTicketCustomizedFieldsResp::default(),
                    CommonResponse::default(),
                ))
            })
            .build();
        let res = lark
            .helpdesk()
            .get_helpdesk_ticket_customized_fields(GetHelpdeskTicketCustomizedFieldsReq::default())
            .await;
        assert!(res.is_ok());
        mocker.clear();
        let res = lark
            .helpdesk()
            .get_helpdesk_ticket_customized_fields(GetHelpdeskTicketCustomizedFieldsReq::default())
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
        "user_customized_fields": [
            {
                "user_customized_field_id": "6746384425543548981",
                "id": "6746384425543548981",
                "helpdesk_id": "1542164574896126",
                "key_name": "company_id3",
                "display_name": "Company ID",
                "position": "1",
                "field_type": "string",
                "description": "租户ID",
                "visible": false,
                "editable": false,
                "required": false,
                "created_at": "1574040677000",
                "updated_at": "1574040677000"
            }
        ],
        "ticket_customized_fields": [
            {
                "ticket_customized_field_id": "6834320707288072194",
                "helpdesk_id": "1542164574896126",
                "key_name": "test dropdown",
                "display_name": "test dropdown",
                "position": "3",
                "field_type": "dropdown",
                "description": "下拉示例",
                "visible": true,
                "editable": true,
                "required": false,
                "created_at": "1591239289000",
                "updated_at": "1591239289000",
                "created_by": {
                    "id": "ou_37019b7c830210acd88fdce886e25c71",
                    "avatar_url": "https://xxxx",
                    "name": "abc",
                    "email": "xxxx@abc.com",
                    "department": "用户部门名称(有权限才展示)",
                    "city": "城市",
                    "country": "国家"
                },
                "updated_by": {
                    "id": "ou_37019b7c830210acd88fdce886e25c71",
                    "avatar_url": "https://xxxx",
                    "name": "abc",
                    "email": "xxxx@abc.com",
                    "department": "用户部门名称(有权限才展示)",
                    "city": "城市",
                    "country": "国家"
                },
                "dropdown_allow_multiple": true
            }
        ]
    }
}"#;
    #[test]
    fn test_resp() {
        let res = serde_json::from_str::<GetHelpdeskTicketCustomizedFieldsRespInner>(RESP);
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
