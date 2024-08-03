//! doc url: <https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/admin-v1/badge-grant/list>
// Code generated by gen_api. DO NOT EDIT.

use serde::{Deserialize, Serialize};

use crate::api::BaseResp;
use crate::api::HasBaseResp;
use crate::core::model::{ApiRequest, CommonResponse, MethodOption, ReqParam, StreamReqParam};
use crate::{
    core::{http_client::HttpClient, store::Store},
    error::Error,
};

use crate::api::gen::admin::AdminService;

impl<'c, IStore: Store, IClient: HttpClient> AdminService<'c, IStore, IClient> {
    /// **api 版本: 2023-05-16T09:07:25+00:00**
    ///
    /// ## 获取授予名单列表
    ///
    /// 通过该接口可以获取特定勋章下的授予名单列表，授予名单的排列顺序按照创建时间倒序排列。
    ///

    ///
    /// doc: <https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/admin-v1/badge-grant/list>
    ///
    /// new doc: <https://open.feishu.cn/document/server-docs/admin-v1/badge/badge-grant/list>
    ///
    /// api url: <https://open.feishu.cn/document_portal/v1/document/get_detail?fullPath=%2Fserver-docs%2Fadmin-v1%2Fbadge%2Fbadge-grant%2Flist>
    pub async fn get_admin_badge_grant_list(
        &self,
        req: GetAdminBadgeGrantListReq,
    ) -> Result<(GetAdminBadgeGrantListResp, CommonResponse), Error> {
        self.get_admin_badge_grant_list_with_opt(req, Default::default())
            .await
    }

    /// 参见 [get_admin_badge_grant_list](#method.get_admin_badge_grant_list) 函数
    pub async fn get_admin_badge_grant_list_with_opt(
        &self,
        req: GetAdminBadgeGrantListReq,
        method_option: MethodOption,
    ) -> Result<(GetAdminBadgeGrantListResp, CommonResponse), Error> {
        #[cfg(feature = "test-util")]
        {
            if let Some(f) = self.mock().get_mock_get_admin_badge_grant_list(&req) {
                tracing::info!("[lark] Admin#GetAdminBadgeGrantList **mocking** api");
                return f(req);
            }
        }

        tracing::info!("[lark] Admin#GetAdminBadgeGrantList call api");

        let req = ApiRequest {
            scope: "Admin",
            api: "GetAdminBadgeGrantList",
            method: http::Method::GET,
            url: String::new()
                + self.cli.open_base_url.as_ref()
                + "/open-apis/admin/v1/badges/:badge_id/grants",
            param_data: req.gen_param(),
            method_option,
            need_tenant_access_token: true,
            ..Default::default()
        };

        let (resp, common_resp): (GetAdminBadgeGrantListRespInner, _) =
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
pub struct GetAdminBadgeGrantListReq {
    /// 企业勋章的唯一ID
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "m_DjMzaK"
    ///
    /// **数据校验规则**：
    ///
    /// - **长度范围**: `1` 字符- `64` 字符
    #[api(kind = "path", name = "badge_id")]
    pub badge_id: String,
    /// 分页大小
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "10"
    #[api(kind = "query", name = "page_size", v_type = "var", option = "false")]
    pub page_size: i64,
    /// 分页标记，第一次请求不填，表示从头开始遍历；分页查询结果还有更多项时会同时返回新的 page_token，下次遍历可采用该 page_token 获取查询结果
    ///
    /// **示例值**: "om5fn1"
    ///
    /// **数据校验规则**：
    ///
    /// - **长度范围**: `1` 字符- `64` 字符
    #[api(kind = "query", name = "page_token", v_type = "var", option = "false")]
    pub page_token: String,
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
    /// 此次调用中使用的部门ID的类型。
    ///
    /// **示例值**: "open_department_id"
    ///
    /// **可选值**:
    ///
    /// `department_id`: 以自定义department_id来标识部门
    ///
    /// `open_department_id`: 以open_department_id来标识部门
    #[api(
        kind = "query",
        name = "department_id_type",
        v_type = "var",
        option = "false"
    )]
    pub department_id_type: String,
    /// 授予名单名称，精确匹配。
    ///
    /// **示例值**: "激励勋章的授予名单"
    ///
    /// **数据校验规则**：
    ///
    /// - **最小长度**: `1` 字符
    #[api(kind = "query", name = "name", v_type = "var", option = "false")]
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, lark_bot_sdk_macros::ApiBaseResp)]
struct GetAdminBadgeGrantListRespInner {
    #[serde(flatten)]
    data: Option<GetAdminBadgeGrantListResp>,
    #[serde(flatten)]
    base: BaseResp,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct GetAdminBadgeGrantListResp {
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
    /// 授予名单列表
    ///
    /// **示例值**: "[{     "grant_id": "g_uS4yux",     "badge_id": "m_DjMzaK",     "name": "授权给全员用户的周年授予名单",     "grant_type": 1,     "timezone": "Asia/Shanghai",     "is_grant_all": true,     "rule_detail": {         "anniversary": 6,         "effective_period": 0     } }]"
    ///
    /// **数据校验规则**：
    ///
    /// - **长度范围**: `0` 字符- `50` 字符
    #[serde(
        rename = "grants",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub grants: Vec<GrantSubResp>,
    /// 分页标记，当 has_more 为 true 时，会同时返回新的 page_token，否则不返回 page_token
    ///
    /// **示例值**: "om5fn1"
    ///
    /// **数据校验规则**：
    ///
    /// - **长度范围**: `1` 字符- `64` 字符
    #[serde(
        rename = "page_token",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub page_token: String,
    /// 是否还有更多项
    ///
    /// **示例值**: "false"
    #[serde(
        rename = "has_more",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub has_more: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(default)]
pub struct GrantSubResp {
    /// 租户内授予名单的唯一标识，该值由系统随机生成。
    ///
    /// **示例值**: "g_49Z7CQ"
    ///
    /// **数据校验规则**：
    ///
    /// - **长度范围**: `1` 字符- `64` 字符
    #[serde(
        rename = "id",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub id: String,
    /// 企业勋章的唯一ID
    ///
    /// **示例值**: "m_qTR2HM"
    ///
    /// **数据校验规则**：
    ///
    /// - **长度范围**: `1` 字符- `64` 字符
    #[serde(
        rename = "badge_id",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub badge_id: String,
    /// 勋章下唯一的授予事项，最多100个字符。
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "激励勋章的授予名单"
    #[serde(
        rename = "name",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub name: String,
    /// 授予名单类型
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "0"
    ///
    /// **可选值**:
    ///
    /// `manual`: 手动选择有效期
    ///
    /// `join_time`: 匹配系统入职时间
    #[serde(
        rename = "grant_type",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub grant_type: i64,
    /// 授予名单的生效时间对应的时区，用于检查RuleDetail的时间戳的取值是否规范，取值范围为TZ database name
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "Asia/Shanghai"
    ///
    /// **数据校验规则**：
    ///
    /// - **最小长度**: `1` 字符
    #[serde(
        rename = "time_zone",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub time_zone: String,
    /// 规则详情
    ///
    /// **是否必填**: 是
    #[serde(
        rename = "rule_detail",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub rule_detail: RuleDetailSubResp,
    /// 是否授予给全员。1.为false时，需要关联1~500个用户群体。2.为true时，不可关联用户、用户组、部门。
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "false"
    #[serde(
        rename = "is_grant_all",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub is_grant_all: bool,
    /// 授予的用户ID列表，授予名单列表接口返回结果中不返回该字段，只在详情接口返回
    ///
    /// **示例值**: "[u273y71]"
    #[serde(
        rename = "user_ids",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub user_ids: Vec<String>,
    /// 授予的部门ID列表，授予名单列表接口返回结果中不返回该字段，只在详情接口返回
    #[serde(
        rename = "department_ids",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub department_ids: Vec<String>,
    /// 授予的用户组ID列表，授予名单列表接口返回结果中不返回该字段，只在详情接口返回
    ///
    /// **示例值**: "[g122817]"
    #[serde(
        rename = "group_ids",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub group_ids: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(default)]
pub struct RuleDetailSubResp {
    /// 开始生效的时间戳。1.手动设置有效期类型勋章，配置有效期限需要配置该字段；2.时间戳必须是所在时区当天的零点时间戳，如时区为Asia/Shanghai时区时的1649606400
    ///
    /// **示例值**: "1649606400"
    #[serde(
        rename = "effective_time",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub effective_time: String,
    /// 结束生效的时间戳。1.手动设置有效期类型勋章，配置有效期限需要配置该字段；2.最大值：不得超过effective_time+100 年；3.非永久有效：时间戳必须是所在时区当天的23:59:59时间戳，如时区为Asia/Shanghai时区时的1649692799；4.永久有效：传值为0即可
    ///
    /// **示例值**: "1649692799"
    #[serde(
        rename = "expiration_time",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub expiration_time: String,
    /// 入职周年日。根据入职时间发放类型勋章，需要配置该字段。
    ///
    /// **示例值**: "1"
    #[serde(
        rename = "anniversary",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub anniversary: i64,
    /// 有效期限。根据入职时间发放类型勋章，需要配置该字段。
    ///
    /// **示例值**: "1"
    ///
    /// **可选值**:
    ///
    /// `one_year`: 有效期为一年
    ///
    /// `permanent`: 永久有效
    #[serde(
        rename = "effective_period",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub effective_period: i64,
}

#[cfg(feature = "test-util")]
mod test_utils {
    use std::sync::Arc;

    use self::gen::admin::AdminServiceMocker;
    use crate::core::mocker::*;

    use super::*;
    use crate::api::*;
    use crate::core::model::*;

    pub struct Mocker;

    pub trait MockFunc:
        Fn(GetAdminBadgeGrantListReq) -> Result<(GetAdminBadgeGrantListResp, CommonResponse), Error>
        + Send
        + Sync
        + 'static
    {
    }
    impl<
            T: Fn(
                    GetAdminBadgeGrantListReq,
                ) -> Result<(GetAdminBadgeGrantListResp, CommonResponse), Error>
                + Send
                + Sync
                + 'static,
        > MockFunc for T
    {
    }

    impl<'c, IStore: Store, IClient: HttpClient> AdminServiceMocker<'c, IStore, IClient> {
        pub fn mock_get_admin_badge_grant_list<F: MockFunc>(
            &self,
            f: F,
        ) -> MockerBuilder<
            Mocker,
            GetAdminBadgeGrantListReq,
            GetAdminBadgeGrantListResp,
            Arc<dyn MockFunc>,
        > {
            MockerBuilder::new(self.cli.instance_id, Arc::new(f))
        }

        pub(super) fn get_mock_get_admin_badge_grant_list(
            &self,
            req: &GetAdminBadgeGrantListReq,
        ) -> Option<Arc<dyn MockFunc>> {
            do_mock::<
                Mocker,
                GetAdminBadgeGrantListReq,
                GetAdminBadgeGrantListResp,
                Arc<dyn MockFunc>,
            >(self.cli.instance_id, req)
        }
    }
}

#[cfg(feature = "test-util")]
#[cfg(test)]
mod test {
    use crate::{
        api::gen::admin::get_admin_badge_grant_list::{
            GetAdminBadgeGrantListReq, GetAdminBadgeGrantListResp, GetAdminBadgeGrantListRespInner,
        },
        core::{model::CommonResponse, Lark},
    };

    #[tokio::test]
    async fn test_mock() {
        let lark = Lark::new("".to_owned(), "".into());
        let mocker = lark
            .admin()
            .mock()
            .mock_get_admin_badge_grant_list(|_| {
                Ok((
                    GetAdminBadgeGrantListResp::default(),
                    CommonResponse::default(),
                ))
            })
            .build();
        let res = lark
            .admin()
            .get_admin_badge_grant_list(GetAdminBadgeGrantListReq::default())
            .await;
        assert!(res.is_ok());
        mocker.clear();
        let res = lark
            .admin()
            .get_admin_badge_grant_list(GetAdminBadgeGrantListReq::default())
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
        "grants": [
            {
                "id": "g_49Z7CQ",
                "badge_id": "m_qTR2HM",
                "name": "激励勋章的授予名单",
                "grant_type": 0,
                "time_zone": "Asia/Shanghai",
                "rule_detail": {
                    "effective_time": "1649606400",
                    "expiration_time": "1649692799",
                    "anniversary": 1,
                    "effective_period": 1
                },
                "is_grant_all": false,
                "user_ids": [
                    "u273y71"
                ],
                "department_ids": [
                    "h121921"
                ],
                "group_ids": [
                    "g122817"
                ]
            }
        ],
        "page_token": "om5fn1",
        "has_more": false
    }
}"#;
    #[test]
    fn test_resp() {
        let res = serde_json::from_str::<GetAdminBadgeGrantListRespInner>(RESP);
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
