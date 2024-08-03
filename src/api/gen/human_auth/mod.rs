use crate::core::{Lark, LarkInner};

pub struct HumanAuthService<'client, Store, Client> {
    #[allow(dead_code)]
    pub(crate) cli: &'client LarkInner<Store, Client>,
}

#[cfg(feature = "test-util")]
impl<Store, Client> HumanAuthService<'_, Store, Client> {
    pub fn mock(&self) -> HumanAuthServiceMocker<Store, Client> {
        HumanAuthServiceMocker { cli: self.cli }
    }
}

#[cfg(feature = "test-util")]
pub struct HumanAuthServiceMocker<'client, Store, Client> {
    #[allow(dead_code)]
    pub(crate) cli: &'client LarkInner<Store, Client>,
}

impl<Store, Client> Lark<Store, Client> {
    pub fn human_auth(&self) -> HumanAuthService<Store, Client> {
        HumanAuthService { cli: &self.inner }
    }
}
pub mod create_identity;
pub mod crop_face_verify_image;
pub mod get_face_verify_auth_result;
pub mod upload_face_verify_image;
