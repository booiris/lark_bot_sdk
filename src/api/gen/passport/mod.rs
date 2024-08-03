use crate::core::{Lark, LarkInner};

pub struct PassportService<'client, Store, Client> {
    #[allow(dead_code)]
    pub(crate) cli: &'client LarkInner<Store, Client>,
}

#[cfg(feature = "test-util")]
impl<Store, Client> PassportService<'_, Store, Client> {
    pub fn mock(&self) -> PassportServiceMocker<Store, Client> {
        PassportServiceMocker { cli: self.cli }
    }
}

#[cfg(feature = "test-util")]
pub struct PassportServiceMocker<'client, Store, Client> {
    #[allow(dead_code)]
    pub(crate) cli: &'client LarkInner<Store, Client>,
}

impl<Store, Client> Lark<Store, Client> {
    pub fn passport(&self) -> PassportService<Store, Client> {
        PassportService { cli: &self.inner }
    }
}
pub mod get_passport_session;
