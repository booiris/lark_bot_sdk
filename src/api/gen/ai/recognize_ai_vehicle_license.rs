//! doc url: <https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/ai/document_ai-v1/vehicle_license/recognize>
// Code generated by gen_api. DO NOT EDIT.

use serde::{Deserialize, Serialize};

use crate::api::BaseResp;
use crate::api::HasBaseResp;
use crate::core::model::{
    ApiRequest, CommonResponse, MethodOption, ReqParam, StreamReqData, StreamReqParam,
};
use crate::{
    core::{http_client::HttpClient, store::Store},
    error::Error,
};

use crate::api::gen::ai::AiService;

impl<'c, IStore: Store, IClient: HttpClient> AiService<'c, IStore, IClient> {
    /// **api 版本: 2023-10-31T16:06:20+00:00**
    ///
    /// ## 识别文件中的行驶证
    ///
    /// 行驶证识别接口，支持JPG/JPEG/PNG/BMP四种文件类型的一次性的识别。
    ///
    /// 单租户限流：10QPS，同租户下的应用没有限流，共享本租户的 10QPS 限流
    ///
    /// doc: <https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/ai/document_ai-v1/vehicle_license/recognize>
    ///
    /// new doc: <https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/ai/document_ai-v1/vehicle_license/recognize>
    ///
    /// api url: <https://open.feishu.cn/document_portal/v1/document/get_detail?fullPath=%2FuAjLw4CM%2FukTMukTMukTM%2Freference%2Fai%2Fdocument_ai-v1%2Fvehicle_license%2Frecognize>
    pub async fn recognize_ai_vehicle_license<Data: StreamReqData>(
        &self,
        req: RecognizeAiVehicleLicenseReq<Data>,
    ) -> Result<(RecognizeAiVehicleLicenseResp, CommonResponse), Error> {
        self.recognize_ai_vehicle_license_with_opt(req, Default::default())
            .await
    }

    /// 参见 [recognize_ai_vehicle_license](#method.recognize_ai_vehicle_license) 函数
    pub async fn recognize_ai_vehicle_license_with_opt<Data: StreamReqData>(
        &self,
        req: RecognizeAiVehicleLicenseReq<Data>,
        method_option: MethodOption,
    ) -> Result<(RecognizeAiVehicleLicenseResp, CommonResponse), Error> {
        #[cfg(feature = "test-util")]
        {
            if let Some(f) = self.mock().get_mock_recognize_ai_vehicle_license(&req) {
                tracing::info!("[lark] Ai#RecognizeAiVehicleLicense **mocking** api");
                return f(req);
            }
        }

        tracing::info!("[lark] Ai#RecognizeAiVehicleLicense call api");

        let req = ApiRequest::<()> {
            scope: "Ai",
            api: "RecognizeAiVehicleLicense",
            method: http::Method::POST,
            url: String::new()
                + self.cli.open_base_url.as_ref()
                + "/open-apis/document_ai/v1/vehicle_license/recognize",
            stream_param_data: req.gen_stream_param(),
            method_option,
            need_tenant_access_token: true,
            ..Default::default()
        };

        let (resp, common_resp): (RecognizeAiVehicleLicenseRespInner, _) =
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

#[derive(Debug, lark_bot_sdk_macros::ApiReqParams)]
pub struct RecognizeAiVehicleLicenseReq<Data: StreamReqData> {
    #[api(kind = "stream", name = "#data#", option = "false")]
    pub data: Data,
}

#[derive(Debug, Serialize, Deserialize, Clone, lark_bot_sdk_macros::ApiBaseResp)]
struct RecognizeAiVehicleLicenseRespInner {
    #[serde(flatten)]
    data: Option<RecognizeAiVehicleLicenseResp>,
    #[serde(flatten)]
    base: BaseResp,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct RecognizeAiVehicleLicenseResp {
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
    /// 行驶证信息
    #[serde(
        rename = "vehicle_license",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub vehicle_license: VehicleLicenseSubResp,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(default)]
pub struct VehicleLicenseSubResp {
    /// 识别出的实体类型
    #[serde(
        rename = "entities",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub entities: Vec<VehicleEntitySubResp>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(default)]
pub struct VehicleEntitySubResp {
    /// 识别的字段种类
    ///
    /// **示例值**: "vehicle_type"
    ///
    /// **可选值**:
    ///
    /// `PlateNumber`: 号牌号码
    ///
    /// `VehicleType`: 车辆类型
    ///
    /// `Owner`: 所有人
    ///
    /// `Address`: 住址
    ///
    /// `UseCharacter`: 使用性质
    ///
    /// `Model`: 品牌型号
    ///
    /// `Vin`: 车辆识别代号
    ///
    /// `EngineNumber`: 发动机号码
    ///
    /// `RegisterDate`: 注册日期
    ///
    /// `IssueDate`: 发证日期
    ///
    /// `LicenseIssuingAuthority`: 发证机关
    ///
    /// `DocumentID`: 档案编号
    ///
    /// `ApprovedPassengersCapacity`: 核定载人数
    ///
    /// `TotalMass`: 总质量
    ///
    /// `CurbWeight`: 整备质量
    ///
    /// `RatifiedLoadCapacity`: 核定载质量
    ///
    /// `Gabarite`: 外廓尺寸
    ///
    /// `TractionMass`: 准牵引总质量
    ///
    /// `Remarks`: 备注
    ///
    /// `InspectionRecord`: 检验记录
    #[serde(
        rename = "type",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub body_type: String,
    /// 识别出字段的文本信息
    ///
    /// **示例值**: "小型普通客车"
    #[serde(
        rename = "value",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub value: String,
}

#[cfg(feature = "test-util")]
mod test_utils {
    use std::sync::Arc;

    use self::gen::ai::AiServiceMocker;
    use crate::core::mocker::*;

    use super::*;
    use crate::api::*;
    use crate::core::model::*;

    pub struct Mocker;

    pub trait MockFunc<D: StreamReqData>:
        Fn(
            RecognizeAiVehicleLicenseReq<D>,
        ) -> Result<(RecognizeAiVehicleLicenseResp, CommonResponse), Error>
        + Send
        + Sync
        + 'static
    {
    }
    impl<
            D: StreamReqData,
            T: Fn(
                    RecognizeAiVehicleLicenseReq<D>,
                )
                    -> Result<(RecognizeAiVehicleLicenseResp, CommonResponse), Error>
                + Send
                + Sync
                + 'static,
        > MockFunc<D> for T
    {
    }

    impl<'c, IStore: Store, IClient: HttpClient> AiServiceMocker<'c, IStore, IClient> {
        pub fn mock_recognize_ai_vehicle_license<T: StreamReqData, F: MockFunc<T>>(
            &self,
            f: F,
        ) -> MockerBuilder<
            Mocker,
            RecognizeAiVehicleLicenseReq<T>,
            RecognizeAiVehicleLicenseResp,
            Arc<dyn MockFunc<T>>,
        > {
            MockerBuilder::new(self.cli.instance_id, Arc::new(f))
        }

        pub(super) fn get_mock_recognize_ai_vehicle_license<T: StreamReqData>(
            &self,
            req: &RecognizeAiVehicleLicenseReq<T>,
        ) -> Option<Arc<dyn MockFunc<T>>> {
            do_mock::<
                Mocker,
                RecognizeAiVehicleLicenseReq<T>,
                RecognizeAiVehicleLicenseResp,
                Arc<dyn MockFunc<T>>,
            >(self.cli.instance_id, req)
        }
    }
}
