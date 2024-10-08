//! doc url: <https://open.larkoffice.com/document/uAjLw4CM/ukTMukTMukTM/lingo-v1/classification/list>
// Code generated by gen_api. DO NOT EDIT.

use serde::{Deserialize, Serialize};

use crate::api::BaseResp;
use crate::api::HasBaseResp;
use crate::core::model::{ApiRequest, CommonResponse, MethodOption, ReqParam, StreamReqParam};
use crate::{
    core::{http_client::HttpClient, store::Store},
    error::Error,
};

use crate::api::gen::lingo::LingoService;

impl<'c, IStore: Store, IClient: HttpClient> LingoService<'c, IStore, IClient> {
    /// **api 版本: 2023-10-25T08:27:16+00:00**
    ///
    /// ## 获取词典分类
    ///
    /// 获取飞书词典当前分类。<br>
    ///
    /// 飞书词典目前为二级分类体系，每个词条可添加多个二级分类，但选择的二级分类必须从属于不同的一级分类。
    ///

    ///
    /// doc: <https://open.larkoffice.com/document/uAjLw4CM/ukTMukTMukTM/lingo-v1/classification/list>
    ///
    /// new doc: <https://open.larkoffice.com/document/uAjLw4CM/ukTMukTMukTM/lingo-v1/classification/list>
    ///
    /// api url: <https://open.feishu.cn/document_portal/v1/document/get_detail?fullPath=%2FuAjLw4CM%2FukTMukTMukTM%2Flingo-v1%2Fclassification%2Flist>
    pub async fn get_lingo_classification_list(
        &self,
        req: GetLingoClassificationListReq,
    ) -> Result<(GetLingoClassificationListResp, CommonResponse), Error> {
        self.get_lingo_classification_list_with_opt(req, Default::default())
            .await
    }

    /// 参见 [get_lingo_classification_list](#method.get_lingo_classification_list) 函数
    pub async fn get_lingo_classification_list_with_opt(
        &self,
        req: GetLingoClassificationListReq,
        method_option: MethodOption,
    ) -> Result<(GetLingoClassificationListResp, CommonResponse), Error> {
        #[cfg(feature = "test-util")]
        {
            if let Some(f) = self.mock().get_mock_get_lingo_classification_list(&req) {
                tracing::info!("[lark] Lingo#GetLingoClassificationList **mocking** api");
                return f(req);
            }
        }

        tracing::info!("[lark] Lingo#GetLingoClassificationList call api");

        let req = ApiRequest {
            scope: "Lingo",
            api: "GetLingoClassificationList",
            method: http::Method::GET,
            url: String::new()
                + self.cli.open_base_url.as_ref()
                + "/open-apis/lingo/v1/classifications",
            param_data: req.gen_param(),
            method_option,
            need_tenant_access_token: true,
            need_user_access_token: true,
            ..Default::default()
        };

        let (resp, common_resp): (GetLingoClassificationListRespInner, _) =
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
pub struct GetLingoClassificationListReq {
    /// 分页大小
    ///
    /// **示例值**: "20"
    #[api(kind = "query", name = "page_size", v_type = "var", option = "false")]
    pub page_size: i64,
    /// 分页标记，第一次请求不填，表示从头开始遍历；分页查询结果还有更多项时会同时返回新的 page_token，下次遍历可采用该 page_token 获取查询结果
    ///
    /// **示例值**: "408ecac018b2e3518db37275e812aad7bb8ad3e755fc886f322ac6c430ba"
    #[api(kind = "query", name = "page_token", v_type = "var", option = "false")]
    pub page_token: String,
    /// 词库ID（不传默认范围为全员词库）
    ///
    /// 如以应用身份获取非全员词库中的分类，需要在“词库设置”页面添加应用；若以用户身份获取非全员词库中的分类，该用户需要拥有对应词库的可见权限。
    ///
    /// **示例值**: "7202510112396640276"
    #[api(kind = "query", name = "repo_id", v_type = "var", option = "false")]
    pub repo_id: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, lark_bot_sdk_macros::ApiBaseResp)]
struct GetLingoClassificationListRespInner {
    #[serde(flatten)]
    data: Option<GetLingoClassificationListResp>,
    #[serde(flatten)]
    base: BaseResp,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct GetLingoClassificationListResp {
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
    /// 分类
    #[serde(
        rename = "items",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub items: Vec<ClassificationSubResp>,
    /// 分页标记，当 has_more 为 true 时，会同时返回新的 page_token，否则不返回 page_token
    ///
    /// **示例值**: "408ecac018b2e3518db37275e812****bb8ad3e755fc886f322ac6c430ba"
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
pub struct ClassificationSubResp {
    /// 二级分类 ID
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "7049606926****37761"
    #[serde(
        rename = "id",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub id: String,
    /// 二级分类名称
    ///
    /// **示例值**: "行业术语"
    #[serde(
        rename = "name",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub name: String,
    /// 对应一级分类 ID
    ///
    /// **示例值**: "704960692***837777"
    #[serde(
        rename = "father_id",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub father_id: String,
    /// 国际化分类名
    ///
    /// **数据校验规则**：
    ///
    /// - **长度范围**: `1` 字符- `3` 字符
    #[serde(
        rename = "i18n_names",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub i18n_names: Vec<I18nClsNameSubResp>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(default)]
pub struct I18nClsNameSubResp {
    /// 语言类型
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "1"
    ///
    /// **可选值**:
    ///
    /// `ZH_CN`: 中文
    ///
    /// `EN_US`: 英文
    ///
    /// `JA_JP`: 日文
    #[serde(
        rename = "language",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub language: i64,
    /// 分类名
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "词典分类"
    ///
    /// **数据校验规则**：
    ///
    /// - **长度范围**: `1` 字符- `20` 字符
    #[serde(
        rename = "name",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub name: String,
}

#[cfg(feature = "test-util")]
mod test_utils {
    use std::sync::Arc;

    use self::gen::lingo::LingoServiceMocker;
    use crate::core::mocker::*;

    use super::*;
    use crate::api::*;
    use crate::core::model::*;

    pub struct Mocker;

    pub trait MockFunc:
        Fn(
            GetLingoClassificationListReq,
        ) -> Result<(GetLingoClassificationListResp, CommonResponse), Error>
        + Send
        + Sync
        + 'static
    {
    }
    impl<
            T: Fn(
                    GetLingoClassificationListReq,
                )
                    -> Result<(GetLingoClassificationListResp, CommonResponse), Error>
                + Send
                + Sync
                + 'static,
        > MockFunc for T
    {
    }

    impl<'c, IStore: Store, IClient: HttpClient> LingoServiceMocker<'c, IStore, IClient> {
        pub fn mock_get_lingo_classification_list<F: MockFunc>(
            &self,
            f: F,
        ) -> MockerBuilder<
            Mocker,
            GetLingoClassificationListReq,
            GetLingoClassificationListResp,
            Arc<dyn MockFunc>,
        > {
            MockerBuilder::new(self.cli.instance_id, Arc::new(f))
        }

        pub(super) fn get_mock_get_lingo_classification_list(
            &self,
            req: &GetLingoClassificationListReq,
        ) -> Option<Arc<dyn MockFunc>> {
            do_mock::<
                Mocker,
                GetLingoClassificationListReq,
                GetLingoClassificationListResp,
                Arc<dyn MockFunc>,
            >(self.cli.instance_id, req)
        }
    }
}

#[cfg(feature = "test-util")]
#[cfg(test)]
mod test {
    use crate::{
        api::gen::lingo::get_lingo_classification_list::{
            GetLingoClassificationListReq, GetLingoClassificationListResp,
            GetLingoClassificationListRespInner,
        },
        core::{model::CommonResponse, Lark},
    };

    #[tokio::test]
    async fn test_mock() {
        let lark = Lark::new("".to_owned(), "".into());
        let mocker = lark
            .lingo()
            .mock()
            .mock_get_lingo_classification_list(|_| {
                Ok((
                    GetLingoClassificationListResp::default(),
                    CommonResponse::default(),
                ))
            })
            .build();
        let res = lark
            .lingo()
            .get_lingo_classification_list(GetLingoClassificationListReq::default())
            .await;
        assert!(res.is_ok());
        mocker.clear();
        let res = lark
            .lingo()
            .get_lingo_classification_list(GetLingoClassificationListReq::default())
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
        "items": [
            {
                "id": "7049606926****37761",
                "name": "行业术语",
                "father_id": "704960692***837777",
                "i18n_names": [
                    {
                        "language": 1,
                        "name": "词典分类"
                    }
                ]
            }
        ],
        "page_token": "408ecac018b2e3518db37275e812****bb8ad3e755fc886f322ac6c430ba",
        "has_more": false
    }
}"#;
    #[test]
    fn test_resp() {
        let res = serde_json::from_str::<GetLingoClassificationListRespInner>(RESP);
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
