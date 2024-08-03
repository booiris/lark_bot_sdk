use crate::core::{Lark, LarkInner};

pub struct VerificationService<'client, Store, Client> {
    #[allow(dead_code)]
    pub(crate) cli: &'client LarkInner<Store, Client>,
}

#[cfg(feature = "test-util")]
impl<Store, Client> VerificationService<'_, Store, Client> {
    pub fn mock(&self) -> VerificationServiceMocker<Store, Client> {
        VerificationServiceMocker { cli: self.cli }
    }
}

#[cfg(feature = "test-util")]
pub struct VerificationServiceMocker<'client, Store, Client> {
    #[allow(dead_code)]
    pub(crate) cli: &'client LarkInner<Store, Client>,
}

impl<Store, Client> Lark<Store, Client> {
    pub fn verification(&self) -> VerificationService<Store, Client> {
        VerificationService { cli: &self.inner }
    }
}
pub mod get_verification;
