//! doc url: <https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/admin-v1/badge/get>
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
    /// **api 版本: 2023-05-16T09:07:24+00:00**
    ///
    /// ## 获取勋章详情
    ///
    /// 可以通过该接口查询勋章的详情。
    ///

    ///
    /// doc: <https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/admin-v1/badge/get>
    ///
    /// new doc: <https://open.feishu.cn/document/server-docs/admin-v1/badge/badge/get>
    ///
    /// api url: <https://open.feishu.cn/document_portal/v1/document/get_detail?fullPath=%2Fserver-docs%2Fadmin-v1%2Fbadge%2Fbadge%2Fget>
    pub async fn get_admin_badge(
        &self,
        req: GetAdminBadgeReq,
    ) -> Result<(GetAdminBadgeResp, CommonResponse), Error> {
        self.get_admin_badge_with_opt(req, Default::default()).await
    }

    /// 参见 [get_admin_badge](#method.get_admin_badge) 函数
    pub async fn get_admin_badge_with_opt(
        &self,
        req: GetAdminBadgeReq,
        method_option: MethodOption,
    ) -> Result<(GetAdminBadgeResp, CommonResponse), Error> {
        #[cfg(feature = "test-util")]
        {
            if let Some(f) = self.mock().get_mock_get_admin_badge(&req) {
                tracing::info!("[lark] Admin#GetAdminBadge **mocking** api");
                return f(req);
            }
        }

        tracing::info!("[lark] Admin#GetAdminBadge call api");

        let req = ApiRequest {
            scope: "Admin",
            api: "GetAdminBadge",
            method: http::Method::GET,
            url: String::new()
                + self.cli.open_base_url.as_ref()
                + "/open-apis/admin/v1/badges/:badge_id",
            param_data: req.gen_param(),
            method_option,
            need_tenant_access_token: true,
            ..Default::default()
        };

        let (resp, common_resp): (GetAdminBadgeRespInner, _) = self.cli.do_req(req).await?;
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
pub struct GetAdminBadgeReq {
    /// 勋章id
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
}

#[derive(Debug, Serialize, Deserialize, Clone, lark_bot_sdk_macros::ApiBaseResp)]
struct GetAdminBadgeRespInner {
    #[serde(flatten)]
    data: Option<GetAdminBadgeResp>,
    #[serde(flatten)]
    base: BaseResp,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct GetAdminBadgeResp {
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
    /// 勋章信息
    ///
    /// **示例值**: "{         "badge_id": "m_DjMzaK",         "name": "字节范勋章",         "explanation": "奖励给值得鼓励的同学",         "badge_image": {             "image_key": "a210ea02-e406-49ee-997c-9acc57c59eac",             "image_url": "https://s1-imfile.feishucdn.com/static-resource/v1/a210ea02-e406-49ee-997c-9acc57c59eac~?image_size=noop&cut_type=&quality=&format=image&sticker_format=.webp"         },         "show_image": {             "image_key": "c23dcb4f-ee63-49c8-9b7d-85a7fe8f13cj",             "image_url": "https://s1-imfile.feishucdn.com/static-resource/v1/c23dcb4f-ee63-49c8-9b7d-85a7fe8f13cj~?image_size=noop&cut_type=&quality=&format=image&sticker_format=.webp"         }     }"
    #[serde(
        rename = "badge",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub badge: BadgeSubResp,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(default)]
pub struct BadgeSubResp {
    /// 租户内勋章的唯一标识，该值由系统随机生成。
    ///
    /// **示例值**: "m_MzfKDM"
    ///
    /// **数据校验规则**：
    ///
    /// - **长度范围**: `1` 字符- `64` 字符
    #[serde(
        rename = "id",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub id: String,
    /// 租户内唯一的勋章名称，最多30个字符。
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "激励勋章"
    #[serde(
        rename = "name",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub name: String,
    /// 勋章的描述文案，最多100个字符。
    ///
    /// **示例值**: "这枚勋章为了激励员工颁发。"
    #[serde(
        rename = "explanation",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub explanation: String,
    /// 企业勋章的详情图Key。1.权限校验：非本租户上传的图片key，不能直接使用；2.时效校验：创建勋章，或者修改勋章图片key时，需使用1h内上传的图片key。
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "75a1949f-d9df-4b46-bc88-dacc51e88f3j"
    ///
    /// **数据校验规则**：
    ///
    /// - **最小长度**: `1` 字符
    #[serde(
        rename = "detail_image",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub detail_image: String,
    /// 企业勋章的头像挂饰图Key。1.权限校验：非本租户上传的图片key，不能直接使用；2.时效校验：创建勋章，或者修改勋章图片key时，需使用1h内上传的图片key。
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "03daa74a-159f-49e9-963e-b6c4d76103fj"
    ///
    /// **数据校验规则**：
    ///
    /// - **最小长度**: `1` 字符
    #[serde(
        rename = "show_image",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub show_image: String,
    /// 勋章的多语言名称，同name字段限制，最多30个字符。
    ///
    /// **示例值**: "{         "zh_cn": "激励勋章",         "en_us": "Incentive Medal",         "ja_jp": "奨励メダル"     }"
    #[serde(
        rename = "i18n_name",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub i18n_name: I18nSubResp,
    /// 勋章的多语言描述文案，同explanation字段限制，最多100个字符。
    ///
    /// **示例值**: "{         "zh_cn": "这枚勋章为了激励员工颁发。",         "en_us": "This medal is awarded to motivate employees.",         "ja_jp": "このメダルは、従業員のモチベーションを高めるために授与されます。"     }"
    #[serde(
        rename = "i18n_explanation",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub i18n_explanation: I18nSubResp,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(default)]
pub struct I18nSubResp {
    /// 中文文案
    ///
    /// **示例值**: "激励勋章"
    ///
    /// **数据校验规则**：
    ///
    /// - **最小长度**: `1` 字符
    #[serde(
        rename = "zh_cn",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub zh_cn: String,
    /// 英文文案
    ///
    /// **示例值**: "Incentive Medal"
    ///
    /// **数据校验规则**：
    ///
    /// - **最小长度**: `1` 字符
    #[serde(
        rename = "en_us",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub en_us: String,
    /// 日文文案
    ///
    /// **示例值**: "奨励メダル"
    ///
    /// **数据校验规则**：
    ///
    /// - **最小长度**: `1` 字符
    #[serde(
        rename = "ja_jp",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub ja_jp: String,
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
        Fn(GetAdminBadgeReq) -> Result<(GetAdminBadgeResp, CommonResponse), Error>
        + Send
        + Sync
        + 'static
    {
    }
    impl<
            T: Fn(GetAdminBadgeReq) -> Result<(GetAdminBadgeResp, CommonResponse), Error>
                + Send
                + Sync
                + 'static,
        > MockFunc for T
    {
    }

    impl<'c, IStore: Store, IClient: HttpClient> AdminServiceMocker<'c, IStore, IClient> {
        pub fn mock_get_admin_badge<F: MockFunc>(
            &self,
            f: F,
        ) -> MockerBuilder<Mocker, GetAdminBadgeReq, GetAdminBadgeResp, Arc<dyn MockFunc>> {
            MockerBuilder::new(self.cli.instance_id, Arc::new(f))
        }

        pub(super) fn get_mock_get_admin_badge(
            &self,
            req: &GetAdminBadgeReq,
        ) -> Option<Arc<dyn MockFunc>> {
            do_mock::<Mocker, GetAdminBadgeReq, GetAdminBadgeResp, Arc<dyn MockFunc>>(
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
        api::gen::admin::get_admin_badge::{
            GetAdminBadgeReq, GetAdminBadgeResp, GetAdminBadgeRespInner,
        },
        core::{model::CommonResponse, Lark},
    };

    #[tokio::test]
    async fn test_mock() {
        let lark = Lark::new("".to_owned(), "".into());
        let mocker = lark
            .admin()
            .mock()
            .mock_get_admin_badge(|_| Ok((GetAdminBadgeResp::default(), CommonResponse::default())))
            .build();
        let res = lark
            .admin()
            .get_admin_badge(GetAdminBadgeReq::default())
            .await;
        assert!(res.is_ok());
        mocker.clear();
        let res = lark
            .admin()
            .get_admin_badge(GetAdminBadgeReq::default())
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
        "badge": {
            "id": "m_MzfKDM",
            "name": "激励勋章",
            "explanation": "这枚勋章为了激励员工颁发。",
            "detail_image": "75a1949f-d9df-4b46-bc88-dacc51e88f3j",
            "show_image": "03daa74a-159f-49e9-963e-b6c4d76103fj",
            "i18n_name": {
                "zh_cn": "激励勋章",
                "en_us": "Incentive Medal",
                "ja_jp": "奨励メダル"
            },
            "i18n_explanation": {
                "zh_cn": "激励勋章",
                "en_us": "Incentive Medal",
                "ja_jp": "奨励メダル"
            }
        }
    }
}"#;
    #[test]
    fn test_resp() {
        let res = serde_json::from_str::<GetAdminBadgeRespInner>(RESP);
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
