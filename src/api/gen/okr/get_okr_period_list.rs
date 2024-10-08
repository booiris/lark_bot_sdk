//! doc url: <https://open.larkoffice.com/document/uAjLw4CM/ukTMukTMukTM/reference/okr-v1/period/list>
// Code generated by gen_api. DO NOT EDIT.

use serde::{Deserialize, Serialize};

use crate::api::BaseResp;
use crate::api::HasBaseResp;
use crate::core::model::{ApiRequest, CommonResponse, MethodOption, ReqParam, StreamReqParam};
use crate::{
    core::{http_client::HttpClient, store::Store},
    error::Error,
};

use crate::api::gen::okr::OkrService;

impl<'c, IStore: Store, IClient: HttpClient> OkrService<'c, IStore, IClient> {
    /// **api 版本: 2023-07-03T10:42:34+00:00**
    ///
    /// ## 获取 OKR 周期列表
    ///
    /// 获取 OKR 周期列表。
    ///
    /// 使用<md-tag mode="inline" type="token-tenant">tenant_access_token</md-tag>需要额外申请权限<md-perm
    ///
    /// href="https://open.feishu.cn/document/ukTMukTMukTM/uQjN3QjL0YzN04CN2cDN">以应用身份访问OKR信息</md-perm>
    ///
    /// doc: <https://open.larkoffice.com/document/uAjLw4CM/ukTMukTMukTM/reference/okr-v1/period/list>
    ///
    /// new doc: <https://open.larkoffice.com/document/server-docs/okr-v1/period/list>
    ///
    /// api url: <https://open.feishu.cn/document_portal/v1/document/get_detail?fullPath=%2Fserver-docs%2Fokr-v1%2Fperiod%2Flist>
    pub async fn get_okr_period_list(
        &self,
        req: GetOkrPeriodListReq,
    ) -> Result<(GetOkrPeriodListResp, CommonResponse), Error> {
        self.get_okr_period_list_with_opt(req, Default::default())
            .await
    }

    /// 参见 [get_okr_period_list](#method.get_okr_period_list) 函数
    pub async fn get_okr_period_list_with_opt(
        &self,
        req: GetOkrPeriodListReq,
        method_option: MethodOption,
    ) -> Result<(GetOkrPeriodListResp, CommonResponse), Error> {
        #[cfg(feature = "test-util")]
        {
            if let Some(f) = self.mock().get_mock_get_okr_period_list(&req) {
                tracing::info!("[lark] Okr#GetOkrPeriodList **mocking** api");
                return f(req);
            }
        }

        tracing::info!("[lark] Okr#GetOkrPeriodList call api");

        let req = ApiRequest {
            scope: "Okr",
            api: "GetOkrPeriodList",
            method: http::Method::GET,
            url: String::new() + self.cli.open_base_url.as_ref() + "/open-apis/okr/v1/periods",
            param_data: req.gen_param(),
            method_option,
            need_tenant_access_token: true,
            ..Default::default()
        };

        let (resp, common_resp): (GetOkrPeriodListRespInner, _) = self.cli.do_req(req).await?;
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
pub struct GetOkrPeriodListReq {
    /// 分页标记，第一次请求不填，表示从头开始遍历；分页查询结果还有更多项时会同时返回新的 page_token，下次遍历可采用该 page_token 获取查询结果
    ///
    /// **示例值**: "xaasdasdax"
    #[api(kind = "query", name = "page_token", v_type = "var", option = "false")]
    pub page_token: String,
    /// 分页大小，默认10
    ///
    /// **示例值**: "10"
    #[api(kind = "query", name = "page_size", v_type = "var", option = "false")]
    pub page_size: i64,
}

#[derive(Debug, Serialize, Deserialize, Clone, lark_bot_sdk_macros::ApiBaseResp)]
struct GetOkrPeriodListRespInner {
    #[serde(flatten)]
    data: Option<GetOkrPeriodListResp>,
    #[serde(flatten)]
    base: BaseResp,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct GetOkrPeriodListResp {
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
    /// 分页标记，当 has_more 为 true 时，会同时返回新的 page_token，否则不返回 page_token
    ///
    /// **示例值**: "asdasdasd"
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
    /// 数据项
    #[serde(
        rename = "items",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub items: Vec<PeriodSubResp>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(default)]
pub struct PeriodSubResp {
    /// id
    ///
    /// **示例值**: "635782378412311"
    #[serde(
        rename = "id",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub id: String,
    /// 中文名称
    ///
    /// **示例值**: "中文周期"
    #[serde(
        rename = "zh_name",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub zh_name: String,
    /// 英文名称
    ///
    /// **示例值**: "english period"
    #[serde(
        rename = "en_name",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub en_name: String,
    /// 启用状态
    ///
    /// **示例值**: "0"
    ///
    /// **可选值**:
    ///
    /// `default`: 正常状态
    ///
    /// `normal`: 暂不处理
    ///
    /// `invalid`: 标记失效
    ///
    /// `hidden`: 隐藏周期
    #[serde(
        rename = "status",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub status: i64,
    /// 周期开始时间
    ///
    /// **示例值**: "1686740948123"
    #[serde(
        rename = "period_start_time",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub period_start_time: String,
    /// 周期结束时间
    ///
    /// **示例值**: "1686740948123"
    #[serde(
        rename = "period_end_time",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub period_end_time: String,
}

#[cfg(feature = "test-util")]
mod test_utils {
    use std::sync::Arc;

    use self::gen::okr::OkrServiceMocker;
    use crate::core::mocker::*;

    use super::*;
    use crate::api::*;
    use crate::core::model::*;

    pub struct Mocker;

    pub trait MockFunc:
        Fn(GetOkrPeriodListReq) -> Result<(GetOkrPeriodListResp, CommonResponse), Error>
        + Send
        + Sync
        + 'static
    {
    }
    impl<
            T: Fn(GetOkrPeriodListReq) -> Result<(GetOkrPeriodListResp, CommonResponse), Error>
                + Send
                + Sync
                + 'static,
        > MockFunc for T
    {
    }

    impl<'c, IStore: Store, IClient: HttpClient> OkrServiceMocker<'c, IStore, IClient> {
        pub fn mock_get_okr_period_list<F: MockFunc>(
            &self,
            f: F,
        ) -> MockerBuilder<Mocker, GetOkrPeriodListReq, GetOkrPeriodListResp, Arc<dyn MockFunc>>
        {
            MockerBuilder::new(self.cli.instance_id, Arc::new(f))
        }

        pub(super) fn get_mock_get_okr_period_list(
            &self,
            req: &GetOkrPeriodListReq,
        ) -> Option<Arc<dyn MockFunc>> {
            do_mock::<Mocker, GetOkrPeriodListReq, GetOkrPeriodListResp, Arc<dyn MockFunc>>(
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
        api::gen::okr::get_okr_period_list::{
            GetOkrPeriodListReq, GetOkrPeriodListResp, GetOkrPeriodListRespInner,
        },
        core::{model::CommonResponse, Lark},
    };

    #[tokio::test]
    async fn test_mock() {
        let lark = Lark::new("".to_owned(), "".into());
        let mocker = lark
            .okr()
            .mock()
            .mock_get_okr_period_list(|_| {
                Ok((GetOkrPeriodListResp::default(), CommonResponse::default()))
            })
            .build();
        let res = lark
            .okr()
            .get_okr_period_list(GetOkrPeriodListReq::default())
            .await;
        assert!(res.is_ok());
        mocker.clear();
        let res = lark
            .okr()
            .get_okr_period_list(GetOkrPeriodListReq::default())
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
    "data": {
        "has_more": false,
        "items": [
            {
                "en_name": "Jan - Mar 2022",
                "id": "7071200999834255380",
                "period_end_time": "1577721600000",
                "period_start_time": "1546272000000",
                "status": 0,
                "zh_name": "2022 年 1 月 - 3 月"
            }
        ]
    },
    "msg": "success"
}"#;
    #[test]
    fn test_resp() {
        let res = serde_json::from_str::<GetOkrPeriodListRespInner>(RESP);
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
