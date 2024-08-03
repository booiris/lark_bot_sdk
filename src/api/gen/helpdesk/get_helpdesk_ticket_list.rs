//! doc url: <https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/helpdesk-v1/ticket/list>
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
    /// ## 查询全部工单详情
    ///
    /// 该接口用于获取全部工单详情。仅支持自建应用。
    ///

    ///
    /// doc: <https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/helpdesk-v1/ticket/list>
    ///
    /// new doc: <https://open.feishu.cn/document/server-docs/helpdesk-v1/ticket-management/ticket/list>
    ///
    /// api url: <https://open.feishu.cn/document_portal/v1/document/get_detail?fullPath=%2Fserver-docs%2Fhelpdesk-v1%2Fticket-management%2Fticket%2Flist>
    pub async fn get_helpdesk_ticket_list(
        &self,
        req: GetHelpdeskTicketListReq,
    ) -> Result<(GetHelpdeskTicketListResp, CommonResponse), Error> {
        self.get_helpdesk_ticket_list_with_opt(req, Default::default())
            .await
    }

    /// 参见 [get_helpdesk_ticket_list](#method.get_helpdesk_ticket_list) 函数
    pub async fn get_helpdesk_ticket_list_with_opt(
        &self,
        req: GetHelpdeskTicketListReq,
        method_option: MethodOption,
    ) -> Result<(GetHelpdeskTicketListResp, CommonResponse), Error> {
        #[cfg(feature = "test-util")]
        {
            if let Some(f) = self.mock().get_mock_get_helpdesk_ticket_list(&req) {
                tracing::info!("[lark] Helpdesk#GetHelpdeskTicketList **mocking** api");
                return f(req);
            }
        }

        tracing::info!("[lark] Helpdesk#GetHelpdeskTicketList call api");

        let req = ApiRequest {
            scope: "Helpdesk",
            api: "GetHelpdeskTicketList",
            method: http::Method::GET,
            url: String::new() + self.cli.open_base_url.as_ref() + "/open-apis/helpdesk/v1/tickets",
            param_data: req.gen_param(),
            method_option,
            need_tenant_access_token: true,
            ..Default::default()
        };

        let (resp, common_resp): (GetHelpdeskTicketListRespInner, _) = self.cli.do_req(req).await?;
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
pub struct GetHelpdeskTicketListReq {
    /// 搜索条件：工单ID
    ///
    /// **示例值**: "123456"
    #[api(kind = "query", name = "ticket_id", v_type = "var", option = "false")]
    pub ticket_id: String,
    /// 搜索条件: 客服id
    ///
    /// **示例值**: "ou_b5de90429xxx"
    #[api(kind = "query", name = "agent_id", v_type = "var", option = "false")]
    pub agent_id: String,
    /// 搜索条件: 关单客服id
    ///
    /// **示例值**: "ou_b5de90429xxx"
    #[api(
        kind = "query",
        name = "closed_by_id",
        v_type = "var",
        option = "false"
    )]
    pub closed_by_id: String,
    /// 搜索条件: 工单类型 1:bot 2:人工
    ///
    /// **示例值**: "1"
    #[api(kind = "query", name = "type", v_type = "var", option = "false")]
    pub query_type: i64,
    /// 搜索条件: 工单渠道
    ///
    /// **示例值**: "0"
    #[api(kind = "query", name = "channel", v_type = "var", option = "false")]
    pub channel: i64,
    /// 搜索条件: 工单是否解决 1:没解决 2:已解决
    ///
    /// **示例值**: "1"
    #[api(kind = "query", name = "solved", v_type = "var", option = "false")]
    pub solved: i64,
    /// 搜索条件: 工单评分
    ///
    /// **示例值**: "1"
    #[api(kind = "query", name = "score", v_type = "var", option = "false")]
    pub score: i64,
    /// 搜索条件: 工单状态列表
    ///
    /// **示例值**: "1"
    #[api(
        kind = "query",
        name = "status_list",
        v_type = "list",
        option = "false"
    )]
    pub status_list: Vec<i64>,
    /// 搜索条件: 用户名称
    ///
    /// **示例值**: "abc"
    #[api(kind = "query", name = "guest_name", v_type = "var", option = "false")]
    pub guest_name: String,
    /// 搜索条件: 用户id
    ///
    /// **示例值**: "ou_b5de90429xxx"
    #[api(kind = "query", name = "guest_id", v_type = "var", option = "false")]
    pub guest_id: String,
    /// 搜索条件: 用户标签列表
    ///
    /// **示例值**: "备注"
    #[api(kind = "query", name = "tags", v_type = "list", option = "false")]
    pub tags: Vec<String>,
    /// 页数, 从1开始, 默认为1
    ///
    /// **示例值**: "1"
    #[api(kind = "query", name = "page", v_type = "var", option = "false")]
    pub page: i64,
    /// 当前页大小，最大为200， 默认为20。分页查询最多累计返回一万条数据，超过一万条请更改查询条件，推荐通过时间查询。
    ///
    /// **示例值**: "20"
    #[api(kind = "query", name = "page_size", v_type = "var", option = "false")]
    pub page_size: i64,
    /// 搜索条件: 工单创建起始时间 ms (也需要填上create_time_end)，相当于>=create_time_start
    ///
    /// **示例值**: "1616920429000"
    #[api(
        kind = "query",
        name = "create_time_start",
        v_type = "var",
        option = "false"
    )]
    pub create_time_start: i64,
    /// 搜索条件: 工单创建结束时间 ms (也需要填上create_time_start)，相当于<=create_time_end
    ///
    /// **示例值**: "1616920429000"
    #[api(
        kind = "query",
        name = "create_time_end",
        v_type = "var",
        option = "false"
    )]
    pub create_time_end: i64,
    /// 搜索条件: 工单修改起始时间 ms (也需要填上update_time_end)
    ///
    /// **示例值**: "1616920429000"
    #[api(
        kind = "query",
        name = "update_time_start",
        v_type = "var",
        option = "false"
    )]
    pub update_time_start: i64,
    /// 搜索条件: 工单修改结束时间 ms(也需要填上update_time_start)
    ///
    /// **示例值**: "1616920429000"
    #[api(
        kind = "query",
        name = "update_time_end",
        v_type = "var",
        option = "false"
    )]
    pub update_time_end: i64,
}

#[derive(Debug, Serialize, Deserialize, Clone, lark_bot_sdk_macros::ApiBaseResp)]
struct GetHelpdeskTicketListRespInner {
    #[serde(flatten)]
    data: Option<GetHelpdeskTicketListResp>,
    #[serde(flatten)]
    base: BaseResp,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct GetHelpdeskTicketListResp {
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
    /// 工单总数 (单次请求最大为10000条)
    ///
    /// **示例值**: "100"
    #[serde(
        rename = "total",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub total: i64,
    /// 工单
    #[serde(
        rename = "tickets",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub tickets: Vec<TicketSubResp>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(default)]
pub struct TicketSubResp {
    /// 工单ID
    ///
    /// [可以从工单列表里面取](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/helpdesk-v1/ticket/list)
    ///
    /// [也可以订阅工单创建事件获取](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/helpdesk-v1/ticket/events/created)
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "6626871355780366331"
    #[serde(
        rename = "ticket_id",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub ticket_id: String,
    /// 服务台ID
    ///
    /// **示例值**: "6626871355780366330"
    #[serde(
        rename = "helpdesk_id",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub helpdesk_id: String,
    /// 工单创建用户
    #[serde(
        rename = "guest",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub guest: TicketUserSubResp,
    /// 备注
    #[serde(
        rename = "comments",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub comments: CommentsSubResp,
    /// 工单阶段：1. 机器人 2. 人工
    ///
    /// **示例值**: "1"
    #[serde(
        rename = "ticket_type",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub ticket_type: i64,
    /// 工单状态，1：已创建 2: 处理中 3: 排队中 4：待定 5：待用户响应 50: 被机器人关闭 51: 被客服关闭 52: 用户自己关闭
    ///
    /// **示例值**: "50"
    #[serde(
        rename = "status",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub status: i64,
    /// 工单评分，1：不满意，2:一般，3:满意
    ///
    /// **示例值**: "1"
    #[serde(
        rename = "score",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub score: i64,
    /// 工单创建时间
    ///
    /// **示例值**: "1616920429000"
    #[serde(
        rename = "created_at",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub created_at: i64,
    /// 工单更新时间，没有值时为-1
    ///
    /// **示例值**: "1616920429000"
    #[serde(
        rename = "updated_at",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub updated_at: i64,
    /// 工单结束时间
    ///
    /// **示例值**: "1616920429000"
    #[serde(
        rename = "closed_at",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub closed_at: i64,
    /// 不满意原因
    #[serde(
        rename = "dissatisfaction_reason",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub dissatisfaction_reason: I18nSubResp,
    /// 工单客服
    #[serde(
        rename = "agents",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub agents: Vec<TicketUserSubResp>,
    /// 工单渠道，描述：
    ///
    /// 9：Open API 2：二维码 14：分享 13：搜索 其他数字：其他渠道
    ///
    /// **示例值**: "0"
    #[serde(
        rename = "channel",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub channel: i64,
    /// 工单是否解决 1:没解决 2:已解决
    ///
    /// **示例值**: "1"
    #[serde(
        rename = "solve",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub solve: i64,
    /// 关单用户ID
    #[serde(
        rename = "closed_by",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub closed_by: TicketUserSubResp,
    /// 工单协作者
    #[serde(
        rename = "collaborators",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub collaborators: Vec<TicketUserSubResp>,
    /// 自定义字段列表，没有值时不设置
    ///
    /// 下拉菜单的value对应工单字段里面的children.display_name
    ///
    /// [获取全部工单自定义字段](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/helpdesk-v1/ticket_customized_field/list-ticket-customized-fields)
    #[serde(
        rename = "customized_fields",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub customized_fields: Vec<CustomizedFieldDisplayItemSubResp>,
    /// 客服服务时长，客服最后一次回复时间距离客服进入时间间隔，单位分钟
    ///
    /// **示例值**: "42624.95"
    #[serde(
        rename = "agent_service_duration",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub agent_service_duration: f64,
    /// 客服首次回复时间距离客服进入时间的间隔(秒)
    ///
    /// **示例值**: "123869"
    #[serde(
        rename = "agent_first_response_duration",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub agent_first_response_duration: i64,
    /// 机器人服务时间：客服进入时间距离工单创建时间的间隔，单位秒
    ///
    /// **示例值**: "1"
    #[serde(
        rename = "bot_service_duration",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub bot_service_duration: i64,
    /// 客服解决时长，关单时间距离客服进入时间的间隔，单位秒
    ///
    /// **示例值**: "66"
    #[serde(
        rename = "agent_resolution_time",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub agent_resolution_time: i64,
    /// 工单实际处理时间：从客服进入到关单，单位秒
    ///
    /// **示例值**: "68"
    #[serde(
        rename = "actual_processing_time",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub actual_processing_time: i64,
    /// 客服进入时间，单位毫秒
    ///
    /// **示例值**: "1636444596000"
    #[serde(
        rename = "agent_entry_time",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub agent_entry_time: i64,
    /// 客服首次回复时间，单位毫秒
    ///
    /// **示例值**: "1636444696000"
    #[serde(
        rename = "agent_first_response_time",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub agent_first_response_time: i64,
    /// 客服最后回复时间，单位毫秒
    ///
    /// **示例值**: "1636444796000"
    #[serde(
        rename = "agent_last_response_time",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub agent_last_response_time: i64,
    /// 主责客服
    #[serde(
        rename = "agent_owner",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub agent_owner: TicketUserSubResp,
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

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(default)]
pub struct CommentsSubResp {
    /// 备注
    ///
    /// **示例值**: "备注内容"
    #[serde(
        rename = "content",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub content: String,
    /// 备注时间，单位毫秒
    ///
    /// **示例值**: "1690970837624"
    #[serde(
        rename = "created_at",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub created_at: i64,
    /// 备注ID
    ///
    /// **示例值**: "12"
    #[serde(
        rename = "id",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub id: i64,
    /// 备注人头像
    ///
    /// **示例值**: "备注人头像"
    #[serde(
        rename = "user_avatar_url",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub user_avatar_url: String,
    /// 备注人姓名
    ///
    /// **示例值**: "备注人姓名"
    #[serde(
        rename = "user_name",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub user_name: String,
    /// 备注人ID
    ///
    /// **示例值**: "7262656095919128578"
    #[serde(
        rename = "user_id",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub user_id: i64,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(default)]
pub struct I18nSubResp {
    /// 中文描述
    ///
    /// **示例值**: "答案看不懂"
    #[serde(
        rename = "zh_cn",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub zh_cn: String,
    /// 英文描述
    ///
    /// **示例值**: "I don't understand"
    #[serde(
        rename = "en_us",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub en_us: String,
    /// 日文描述
    ///
    /// **示例值**: "回答が複雑すぎる"
    #[serde(
        rename = "ja_jp",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub ja_jp: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(default)]
pub struct CustomizedFieldDisplayItemSubResp {
    /// 自定义字段ID
    ///
    /// **示例值**: "123"
    #[serde(
        rename = "id",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub id: String,
    /// 自定义字段值
    ///
    /// **示例值**: "value"
    #[serde(
        rename = "value",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub value: String,
    /// 键名
    ///
    /// **示例值**: "key"
    #[serde(
        rename = "key_name",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub key_name: String,
    /// 展示名称
    ///
    /// **示例值**: "display name"
    #[serde(
        rename = "display_name",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub display_name: String,
    /// 展示位置
    ///
    /// **示例值**: "1"
    #[serde(
        rename = "position",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub position: i64,
    /// 是否必填
    ///
    /// **示例值**: "true"
    #[serde(
        rename = "required",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub required: bool,
    /// 是否可修改
    ///
    /// **示例值**: "true"
    #[serde(
        rename = "editable",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub editable: bool,
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
        Fn(GetHelpdeskTicketListReq) -> Result<(GetHelpdeskTicketListResp, CommonResponse), Error>
        + Send
        + Sync
        + 'static
    {
    }
    impl<
            T: Fn(
                    GetHelpdeskTicketListReq,
                ) -> Result<(GetHelpdeskTicketListResp, CommonResponse), Error>
                + Send
                + Sync
                + 'static,
        > MockFunc for T
    {
    }

    impl<'c, IStore: Store, IClient: HttpClient> HelpdeskServiceMocker<'c, IStore, IClient> {
        pub fn mock_get_helpdesk_ticket_list<F: MockFunc>(
            &self,
            f: F,
        ) -> MockerBuilder<
            Mocker,
            GetHelpdeskTicketListReq,
            GetHelpdeskTicketListResp,
            Arc<dyn MockFunc>,
        > {
            MockerBuilder::new(self.cli.instance_id, Arc::new(f))
        }

        pub(super) fn get_mock_get_helpdesk_ticket_list(
            &self,
            req: &GetHelpdeskTicketListReq,
        ) -> Option<Arc<dyn MockFunc>> {
            do_mock::<Mocker, GetHelpdeskTicketListReq, GetHelpdeskTicketListResp, Arc<dyn MockFunc>>(
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
        api::gen::helpdesk::get_helpdesk_ticket_list::{
            GetHelpdeskTicketListReq, GetHelpdeskTicketListResp, GetHelpdeskTicketListRespInner,
        },
        core::{model::CommonResponse, Lark},
    };

    #[tokio::test]
    async fn test_mock() {
        let lark = Lark::new("".to_owned(), "".into());
        let mocker = lark
            .helpdesk()
            .mock()
            .mock_get_helpdesk_ticket_list(|_| {
                Ok((
                    GetHelpdeskTicketListResp::default(),
                    CommonResponse::default(),
                ))
            })
            .build();
        let res = lark
            .helpdesk()
            .get_helpdesk_ticket_list(GetHelpdeskTicketListReq::default())
            .await;
        assert!(res.is_ok());
        mocker.clear();
        let res = lark
            .helpdesk()
            .get_helpdesk_ticket_list(GetHelpdeskTicketListReq::default())
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
        "total": 100,
        "tickets": [
            {
                "ticket_id": "6626871355780366331",
                "helpdesk_id": "6626871355780366330",
                "guest": {
                    "id": "ou_37019b7c830210acd88fdce886e25c71",
                    "avatar_url": "https://xxxx",
                    "name": "abc",
                    "email": "xxxx@abc.com",
                    "department": "用户部门名称(有权限才展示)",
                    "city": "城市",
                    "country": "国家"
                },
                "comments": {
                    "content": "备注内容",
                    "created_at": 1690970837624,
                    "id": 12,
                    "user_avatar_url": "备注人头像",
                    "user_name": "备注人姓名",
                    "user_id": 7262656095919128578
                },
                "ticket_type": 1,
                "status": 50,
                "score": 1,
                "created_at": 1616920429000,
                "updated_at": 1616920429000,
                "closed_at": 1616920429000,
                "dissatisfaction_reason": {
                    "zh_cn": "答案看不懂",
                    "en_us": "I don't understand",
                    "ja_jp": "回答が複雑すぎる"
                },
                "agents": [
                    {
                        "id": "ou_37019b7c830210acd88fdce886e25c71",
                        "avatar_url": "https://xxxx",
                        "name": "abc",
                        "email": "xxxx@abc.com",
                        "department": "用户部门名称(有权限才展示)",
                        "city": "城市",
                        "country": "国家"
                    }
                ],
                "channel": 0,
                "solve": 1,
                "closed_by": {
                    "id": "ou_37019b7c830210acd88fdce886e25c71",
                    "avatar_url": "https://xxxx",
                    "name": "abc",
                    "email": "xxxx@abc.com",
                    "department": "用户部门名称(有权限才展示)",
                    "city": "城市",
                    "country": "国家"
                },
                "collaborators": [
                    {
                        "id": "ou_37019b7c830210acd88fdce886e25c71",
                        "avatar_url": "https://xxxx",
                        "name": "abc",
                        "email": "xxxx@abc.com",
                        "department": "用户部门名称(有权限才展示)",
                        "city": "城市",
                        "country": "国家"
                    }
                ],
                "customized_fields": [
                    {
                        "id": "123",
                        "value": "value",
                        "key_name": "key",
                        "display_name": "display name",
                        "position": 1,
                        "required": true,
                        "editable": true
                    }
                ],
                "agent_service_duration": 42624.95,
                "agent_first_response_duration": 123869,
                "bot_service_duration": 1,
                "agent_resolution_time": 66,
                "actual_processing_time": 68,
                "agent_entry_time": 1636444596000,
                "agent_first_response_time": 1636444696000,
                "agent_last_response_time": 1636444796000,
                "agent_owner": {
                    "id": "ou_37019b7c830210acd88fdce886e25c71",
                    "avatar_url": "https://xxxx",
                    "name": "abc",
                    "email": "xxxx@abc.com",
                    "department": "用户部门名称(有权限才展示)",
                    "city": "城市",
                    "country": "国家"
                }
            }
        ]
    }
}"#;
    #[test]
    fn test_resp() {
        let res = serde_json::from_str::<GetHelpdeskTicketListRespInner>(RESP);
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
