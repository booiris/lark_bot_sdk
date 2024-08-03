//! doc url: <https://open.larkoffice.com/document/uAjLw4CM/ukTMukTMukTM/reference/vc-v1/room_level/del>
// Code generated by gen_api. DO NOT EDIT.

use serde::{Deserialize, Serialize};

use crate::api::BaseResp;
use crate::api::HasBaseResp;
use crate::core::model::{ApiRequest, CommonResponse, MethodOption, ReqParam, StreamReqParam};
use crate::{
    core::{http_client::HttpClient, store::Store},
    error::Error,
};

use crate::api::gen::vc::VcService;

impl<'c, IStore: Store, IClient: HttpClient> VcService<'c, IStore, IClient> {
    /// **api 版本: 2024-07-23T07:32:57+00:00**
    ///
    /// ## 删除会议室层级
    ///
    /// 该接口可以用来删除某个会议室层级。
    ///

    ///
    /// doc: <https://open.larkoffice.com/document/uAjLw4CM/ukTMukTMukTM/reference/vc-v1/room_level/del>
    ///
    /// new doc: <https://open.larkoffice.com/document/server-docs/vc-v1/room_level/del>
    ///
    /// api url: <https://open.feishu.cn/document_portal/v1/document/get_detail?fullPath=%2Fserver-docs%2Fvc-v1%2Froom_level%2Fdel>
    pub async fn delete_vc_room_level(
        &self,
        req: DeleteVcRoomLevelReq,
    ) -> Result<(DeleteVcRoomLevelResp, CommonResponse), Error> {
        self.delete_vc_room_level_with_opt(req, Default::default())
            .await
    }

    /// 参见 [delete_vc_room_level](#method.delete_vc_room_level) 函数
    pub async fn delete_vc_room_level_with_opt(
        &self,
        req: DeleteVcRoomLevelReq,
        method_option: MethodOption,
    ) -> Result<(DeleteVcRoomLevelResp, CommonResponse), Error> {
        #[cfg(feature = "test-util")]
        {
            if let Some(f) = self.mock().get_mock_delete_vc_room_level(&req) {
                tracing::info!("[lark] Vc#DeleteVcRoomLevel **mocking** api");
                return f(req);
            }
        }

        tracing::info!("[lark] Vc#DeleteVcRoomLevel call api");

        let req = ApiRequest {
            scope: "Vc",
            api: "DeleteVcRoomLevel",
            method: http::Method::POST,
            url: String::new()
                + self.cli.open_base_url.as_ref()
                + "/open-apis/vc/v1/room_levels/del",
            param_data: req.gen_param(),
            method_option,
            need_tenant_access_token: true,
            ..Default::default()
        };

        let (resp, common_resp): (DeleteVcRoomLevelRespInner, _) = self.cli.do_req(req).await?;
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
pub struct DeleteVcRoomLevelReq {
    /// 层级ID
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "omb_4ad1a2c7a2fbc5fc9570f38456931293"
    ///
    /// **数据校验规则**：
    ///
    /// - **长度范围**: `1` 字符- `100` 字符
    #[api(kind = "body", name = "room_level_id")]
    pub room_level_id: String,
    /// 是否删除所有子层级
    ///
    /// **示例值**: "false"
    #[api(kind = "body", name = "delete_child")]
    pub delete_child: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize, Clone, lark_bot_sdk_macros::ApiBaseResp)]
struct DeleteVcRoomLevelRespInner {
    #[serde(flatten)]
    data: Option<DeleteVcRoomLevelResp>,
    #[serde(flatten)]
    base: BaseResp,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct DeleteVcRoomLevelResp {
    /// \-
    #[serde(
        rename = "data",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub data: (),
}

#[cfg(feature = "test-util")]
mod test_utils {
    use std::sync::Arc;

    use self::gen::vc::VcServiceMocker;
    use crate::core::mocker::*;

    use super::*;
    use crate::api::*;
    use crate::core::model::*;

    pub struct Mocker;

    pub trait MockFunc:
        Fn(DeleteVcRoomLevelReq) -> Result<(DeleteVcRoomLevelResp, CommonResponse), Error>
        + Send
        + Sync
        + 'static
    {
    }
    impl<
            T: Fn(DeleteVcRoomLevelReq) -> Result<(DeleteVcRoomLevelResp, CommonResponse), Error>
                + Send
                + Sync
                + 'static,
        > MockFunc for T
    {
    }

    impl<'c, IStore: Store, IClient: HttpClient> VcServiceMocker<'c, IStore, IClient> {
        pub fn mock_delete_vc_room_level<F: MockFunc>(
            &self,
            f: F,
        ) -> MockerBuilder<Mocker, DeleteVcRoomLevelReq, DeleteVcRoomLevelResp, Arc<dyn MockFunc>>
        {
            MockerBuilder::new(self.cli.instance_id, Arc::new(f))
        }

        pub(super) fn get_mock_delete_vc_room_level(
            &self,
            req: &DeleteVcRoomLevelReq,
        ) -> Option<Arc<dyn MockFunc>> {
            do_mock::<Mocker, DeleteVcRoomLevelReq, DeleteVcRoomLevelResp, Arc<dyn MockFunc>>(
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
        api::gen::vc::delete_vc_room_level::{
            DeleteVcRoomLevelReq, DeleteVcRoomLevelResp, DeleteVcRoomLevelRespInner,
        },
        core::{model::CommonResponse, Lark},
    };

    #[tokio::test]
    async fn test_mock() {
        let lark = Lark::new("".to_owned(), "".into());
        let mocker = lark
            .vc()
            .mock()
            .mock_delete_vc_room_level(|_| {
                Ok((DeleteVcRoomLevelResp::default(), CommonResponse::default()))
            })
            .build();
        let res = lark
            .vc()
            .delete_vc_room_level(DeleteVcRoomLevelReq::default())
            .await;
        assert!(res.is_ok());
        mocker.clear();
        let res = lark
            .vc()
            .delete_vc_room_level(DeleteVcRoomLevelReq::default())
            .await;
        assert!(res.is_err());
    }

    const REQ: &str = r#"{
    "room_level_id": "omb_4ad1a2c7a2fbc5fc9570f38456931293",
    "delete_child": false
}"#;

    #[test]
    fn test_req() {
        if REQ == "{}" {
            return;
        }
        if let Err(e) = serde_json::from_str::<super::DeleteVcRoomLevelReqBody>(REQ) {
            panic!("{}", e);
        }
    }

    const RESP: &str = r#"{
    "code": 0,
    "msg": "success",
    "data": {}
}"#;
    #[test]
    fn test_resp() {
        let res = serde_json::from_str::<DeleteVcRoomLevelRespInner>(RESP);
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
