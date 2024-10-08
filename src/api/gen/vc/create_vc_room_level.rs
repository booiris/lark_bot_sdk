//! doc url: <https://open.larkoffice.com/document/uAjLw4CM/ukTMukTMukTM/reference/vc-v1/room_level/create>
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
    /// ## 创建会议室层级
    ///
    /// 该接口用于创建会议室层级。
    ///

    ///
    /// doc: <https://open.larkoffice.com/document/uAjLw4CM/ukTMukTMukTM/reference/vc-v1/room_level/create>
    ///
    /// new doc: <https://open.larkoffice.com/document/server-docs/vc-v1/room_level/create>
    ///
    /// api url: <https://open.feishu.cn/document_portal/v1/document/get_detail?fullPath=%2Fserver-docs%2Fvc-v1%2Froom_level%2Fcreate>
    pub async fn create_vc_room_level(
        &self,
        req: CreateVcRoomLevelReq,
    ) -> Result<(CreateVcRoomLevelResp, CommonResponse), Error> {
        self.create_vc_room_level_with_opt(req, Default::default())
            .await
    }

    /// 参见 [create_vc_room_level](#method.create_vc_room_level) 函数
    pub async fn create_vc_room_level_with_opt(
        &self,
        req: CreateVcRoomLevelReq,
        method_option: MethodOption,
    ) -> Result<(CreateVcRoomLevelResp, CommonResponse), Error> {
        #[cfg(feature = "test-util")]
        {
            if let Some(f) = self.mock().get_mock_create_vc_room_level(&req) {
                tracing::info!("[lark] Vc#CreateVcRoomLevel **mocking** api");
                return f(req);
            }
        }

        tracing::info!("[lark] Vc#CreateVcRoomLevel call api");

        let req = ApiRequest {
            scope: "Vc",
            api: "CreateVcRoomLevel",
            method: http::Method::POST,
            url: String::new() + self.cli.open_base_url.as_ref() + "/open-apis/vc/v1/room_levels",
            param_data: req.gen_param(),
            method_option,
            need_tenant_access_token: true,
            ..Default::default()
        };

        let (resp, common_resp): (CreateVcRoomLevelRespInner, _) = self.cli.do_req(req).await?;
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
pub struct CreateVcRoomLevelReq {
    /// 层级名称
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "测试层级"
    #[api(kind = "body", name = "name")]
    pub name: String,
    /// 父层级ID。
    ///
    /// **说明**：如需在租户层级（即根层级）下创建会议室层级，可以先调用[查询会议室详情](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/vc-v1/room/get)接口，将路径参数 `room_level_id` 传入 `0` 进行查询，返回结果中的 `room_level_id` 值即为根层级 ID。
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "omb_4ad1a2c7a2fbc5fc9570f38456931293"
    #[api(kind = "body", name = "parent_id")]
    pub parent_id: String,
    /// 自定义层级ID
    ///
    /// **示例值**: "10000"
    #[api(kind = "body", name = "custom_group_id")]
    pub custom_group_id: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, lark_bot_sdk_macros::ApiBaseResp)]
struct CreateVcRoomLevelRespInner {
    #[serde(flatten)]
    data: Option<CreateVcRoomLevelResp>,
    #[serde(flatten)]
    base: BaseResp,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct CreateVcRoomLevelResp {
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
    /// 层级详情
    #[serde(
        rename = "room_level",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub room_level: RoomLevelSubResp,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(default)]
pub struct RoomLevelSubResp {
    /// 层级ID
    ///
    /// **示例值**: "层级ID"
    #[serde(
        rename = "room_level_id",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub room_level_id: String,
    /// 层级名称
    ///
    /// **示例值**: "测试层级"
    #[serde(
        rename = "name",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub name: String,
    /// 父层级ID
    ///
    /// **示例值**: "omb_4ad1a2c7a2fbc5fc9570f38456931293"
    #[serde(
        rename = "parent_id",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub parent_id: String,
    /// 层级路径
    ///
    /// **示例值**: "[omb_8d020b12fe49e82847c2af3c193d5754, omb_8d020b12fe49e82847c2af3c193d5754]"
    #[serde(
        rename = "path",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub path: Vec<String>,
    /// 是否有子层级
    ///
    /// **示例值**: "false"
    #[serde(
        rename = "has_child",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub has_child: bool,
    /// 自定义层级ID
    ///
    /// **示例值**: "10000"
    #[serde(
        rename = "custom_group_id",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub custom_group_id: String,
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
        Fn(CreateVcRoomLevelReq) -> Result<(CreateVcRoomLevelResp, CommonResponse), Error>
        + Send
        + Sync
        + 'static
    {
    }
    impl<
            T: Fn(CreateVcRoomLevelReq) -> Result<(CreateVcRoomLevelResp, CommonResponse), Error>
                + Send
                + Sync
                + 'static,
        > MockFunc for T
    {
    }

    impl<'c, IStore: Store, IClient: HttpClient> VcServiceMocker<'c, IStore, IClient> {
        pub fn mock_create_vc_room_level<F: MockFunc>(
            &self,
            f: F,
        ) -> MockerBuilder<Mocker, CreateVcRoomLevelReq, CreateVcRoomLevelResp, Arc<dyn MockFunc>>
        {
            MockerBuilder::new(self.cli.instance_id, Arc::new(f))
        }

        pub(super) fn get_mock_create_vc_room_level(
            &self,
            req: &CreateVcRoomLevelReq,
        ) -> Option<Arc<dyn MockFunc>> {
            do_mock::<Mocker, CreateVcRoomLevelReq, CreateVcRoomLevelResp, Arc<dyn MockFunc>>(
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
        api::gen::vc::create_vc_room_level::{
            CreateVcRoomLevelReq, CreateVcRoomLevelResp, CreateVcRoomLevelRespInner,
        },
        core::{model::CommonResponse, Lark},
    };

    #[tokio::test]
    async fn test_mock() {
        let lark = Lark::new("".to_owned(), "".into());
        let mocker = lark
            .vc()
            .mock()
            .mock_create_vc_room_level(|_| {
                Ok((CreateVcRoomLevelResp::default(), CommonResponse::default()))
            })
            .build();
        let res = lark
            .vc()
            .create_vc_room_level(CreateVcRoomLevelReq::default())
            .await;
        assert!(res.is_ok());
        mocker.clear();
        let res = lark
            .vc()
            .create_vc_room_level(CreateVcRoomLevelReq::default())
            .await;
        assert!(res.is_err());
    }

    const REQ: &str = r#"{
    "name": "测试层级",
    "parent_id": "omb_4ad1a2c7a2fbc5fc9570f38456931293",
    "custom_group_id": "10000"
}"#;

    #[test]
    fn test_req() {
        if REQ == "{}" {
            return;
        }
        if let Err(e) = serde_json::from_str::<super::CreateVcRoomLevelReqBody>(REQ) {
            panic!("{}", e);
        }
    }

    const RESP: &str = r#"{
    "code": 0,
    "msg": "success",
    "data": {
        "room_level": {
            "room_level_id": "层级ID",
            "name": "测试层级",
            "parent_id": "omb_4ad1a2c7a2fbc5fc9570f38456931293",
            "path": [
                "omb_4ad1a2c7a2fbc5fc9570f38456931293"
            ],
            "has_child": false,
            "custom_group_id": "10000"
        }
    }
}"#;
    #[test]
    fn test_resp() {
        let res = serde_json::from_str::<CreateVcRoomLevelRespInner>(RESP);
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
