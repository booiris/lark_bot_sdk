use crate::core::{Lark, LarkInner};

pub struct AiService<'client, Store, Client> {
    #[allow(dead_code)]
    pub(crate) cli: &'client LarkInner<Store, Client>,
}

#[cfg(feature = "test-util")]
impl<Store, Client> AiService<'_, Store, Client> {
    pub fn mock(&self) -> AiServiceMocker<Store, Client> {
        AiServiceMocker { cli: self.cli }
    }
}

#[cfg(feature = "test-util")]
pub struct AiServiceMocker<'client, Store, Client> {
    #[allow(dead_code)]
    pub(crate) cli: &'client LarkInner<Store, Client>,
}

impl<Store, Client> Lark<Store, Client> {
    pub fn ai(&self) -> AiService<Store, Client> {
        AiService { cli: &self.inner }
    }
}
pub mod detect_face_attributes;
pub mod detect_text_language;
pub mod extract_ai_contract_field;
pub mod parse_ai_resume;
pub mod recognize_ai_bank_card;
pub mod recognize_ai_business_card;
pub mod recognize_ai_business_license;
pub mod recognize_ai_chinese_passport;
pub mod recognize_ai_driving_license;
pub mod recognize_ai_food_manage_license;
pub mod recognize_ai_food_produce_license;
pub mod recognize_ai_health_certificate;
pub mod recognize_ai_hkm_mainland_travel_permit;
pub mod recognize_ai_taxi_invoice;
pub mod recognize_ai_train_invoice;
pub mod recognize_ai_tw_mainland_travel_permit;
pub mod recognize_ai_vat_invoice;
pub mod recognize_ai_vehicle_invoice;
pub mod recognize_ai_vehicle_license;
pub mod recognize_aiid_card;
pub mod recognize_basic_image;
pub mod recognize_speech_file;
pub mod recognize_speech_stream;
pub mod translate_text;
