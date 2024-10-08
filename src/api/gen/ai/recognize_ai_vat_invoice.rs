//! doc url: <https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/ai/document_ai-v1/vat_invoice/recognize>
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
    /// **api 版本: 2024-03-04T02:55:41+00:00**
    ///
    /// ## 识别文件中的增值税发票
    ///
    /// 增值税发票识别接口，支持JPG/JPEG/PNG/PDF/BMP/OFD六种文件类型的一次性的识别。
    ///
    /// 单租户限流：10QPS，同租户下的应用没有限流，共享本租户的 10QPS 限流
    ///
    /// doc: <https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/ai/document_ai-v1/vat_invoice/recognize>
    ///
    /// new doc: <https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/ai/document_ai-v1/vat_invoice/recognize>
    ///
    /// api url: <https://open.feishu.cn/document_portal/v1/document/get_detail?fullPath=%2FuAjLw4CM%2FukTMukTMukTM%2Freference%2Fai%2Fdocument_ai-v1%2Fvat_invoice%2Frecognize>
    pub async fn recognize_ai_vat_invoice<Data: StreamReqData>(
        &self,
        req: RecognizeAiVatInvoiceReq<Data>,
    ) -> Result<(RecognizeAiVatInvoiceResp, CommonResponse), Error> {
        self.recognize_ai_vat_invoice_with_opt(req, Default::default())
            .await
    }

    /// 参见 [recognize_ai_vat_invoice](#method.recognize_ai_vat_invoice) 函数
    pub async fn recognize_ai_vat_invoice_with_opt<Data: StreamReqData>(
        &self,
        req: RecognizeAiVatInvoiceReq<Data>,
        method_option: MethodOption,
    ) -> Result<(RecognizeAiVatInvoiceResp, CommonResponse), Error> {
        #[cfg(feature = "test-util")]
        {
            if let Some(f) = self.mock().get_mock_recognize_ai_vat_invoice(&req) {
                tracing::info!("[lark] Ai#RecognizeAiVatInvoice **mocking** api");
                return f(req);
            }
        }

        tracing::info!("[lark] Ai#RecognizeAiVatInvoice call api");

        let req = ApiRequest::<()> {
            scope: "Ai",
            api: "RecognizeAiVatInvoice",
            method: http::Method::POST,
            url: String::new()
                + self.cli.open_base_url.as_ref()
                + "/open-apis/document_ai/v1/vat_invoice/recognize",
            stream_param_data: req.gen_stream_param(),
            method_option,
            need_tenant_access_token: true,
            ..Default::default()
        };

        let (resp, common_resp): (RecognizeAiVatInvoiceRespInner, _) = self.cli.do_req(req).await?;
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
pub struct RecognizeAiVatInvoiceReq<Data: StreamReqData> {
    #[api(kind = "stream", name = "#data#", option = "false")]
    pub data: Data,
}

#[derive(Debug, Serialize, Deserialize, Clone, lark_bot_sdk_macros::ApiBaseResp)]
struct RecognizeAiVatInvoiceRespInner {
    #[serde(flatten)]
    data: Option<RecognizeAiVatInvoiceResp>,
    #[serde(flatten)]
    base: BaseResp,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct RecognizeAiVatInvoiceResp {
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
    /// 增值税发票信息
    #[serde(
        rename = "vat_invoices",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub vat_invoices: Vec<VatInvoiceSubResp>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(default)]
pub struct VatInvoiceSubResp {
    /// 识别出的实体列表
    #[serde(
        rename = "entities",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub entities: Vec<VatEntitySubResp>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(default)]
pub struct VatEntitySubResp {
    /// 识别的实体类型
    ///
    /// **示例值**: "buyer_name"
    ///
    /// **可选值**:
    ///
    /// `InvoiceName`: 发票抬头
    ///
    /// `InvoiceCode`: 发票代码
    ///
    /// `InvoideNo`: 发票号码
    ///
    /// `InvoiceDate`: 开票日期
    ///
    /// `TotalPrice`: 合计金额（不含税）
    ///
    /// `TotalTax`: 合计税额
    ///
    /// `BigTotalPriceAndTax`: 合计总额（大写）
    ///
    /// `CheckCode`: 校验码
    ///
    /// `TotalPriceAndTax`: 合计总额
    ///
    /// `BuyerName`: 购买方名称
    ///
    /// `BuyerTaxPayerNo`: 购买方纳税人识别号
    ///
    /// `BuyerAddressPhone`: 购买方地址&电话所有人
    ///
    /// `BuyerAccount`: 购买方开户行&账号
    ///
    /// `SellerName`: 销售方名称
    ///
    /// `SellerTaxPayerNo`: 销售方纳税人识别号
    ///
    /// `SellerAddressPhone`: 销售方地址&电话
    ///
    /// `SellerAccount`: 销售方开户行&账号
    ///
    /// `Payee`: 收款人
    #[serde(
        rename = "type",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub body_type: String,
    /// 识别出字段的文本信息
    ///
    /// **示例值**: "发呆公司"
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
            RecognizeAiVatInvoiceReq<D>,
        ) -> Result<(RecognizeAiVatInvoiceResp, CommonResponse), Error>
        + Send
        + Sync
        + 'static
    {
    }
    impl<
            D: StreamReqData,
            T: Fn(
                    RecognizeAiVatInvoiceReq<D>,
                ) -> Result<(RecognizeAiVatInvoiceResp, CommonResponse), Error>
                + Send
                + Sync
                + 'static,
        > MockFunc<D> for T
    {
    }

    impl<'c, IStore: Store, IClient: HttpClient> AiServiceMocker<'c, IStore, IClient> {
        pub fn mock_recognize_ai_vat_invoice<T: StreamReqData, F: MockFunc<T>>(
            &self,
            f: F,
        ) -> MockerBuilder<
            Mocker,
            RecognizeAiVatInvoiceReq<T>,
            RecognizeAiVatInvoiceResp,
            Arc<dyn MockFunc<T>>,
        > {
            MockerBuilder::new(self.cli.instance_id, Arc::new(f))
        }

        pub(super) fn get_mock_recognize_ai_vat_invoice<T: StreamReqData>(
            &self,
            req: &RecognizeAiVatInvoiceReq<T>,
        ) -> Option<Arc<dyn MockFunc<T>>> {
            do_mock::<
                Mocker,
                RecognizeAiVatInvoiceReq<T>,
                RecognizeAiVatInvoiceResp,
                Arc<dyn MockFunc<T>>,
            >(self.cli.instance_id, req)
        }
    }
}
