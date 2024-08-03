use crate::core::{Lark, LarkInner};

pub struct AuthService<'client, Store, Client> {
    pub(super) cli: &'client LarkInner<Store, Client>,
}

#[cfg(feature = "test-util")]
impl<Store, Client> AuthService<'_, Store, Client> {
    pub fn mock(&self) -> AuthServiceMocker<Store, Client> {
        AuthServiceMocker { cli: self.cli }
    }
}

#[cfg(feature = "test-util")]
pub struct AuthServiceMocker<'client, Store, Client> {
    pub(super) cli: &'client LarkInner<Store, Client>,
}

impl<Store, Client> Lark<Store, Client> {
    pub fn auth(&self) -> AuthService<Store, Client> {
        AuthService { cli: &self.inner }
    }
}

impl<Store, Client> LarkInner<Store, Client> {
    pub(crate) fn auth(&self) -> AuthService<Store, Client> {
        AuthService { cli: self }
    }
}
