use crate::core::{Lark, LarkInner};

pub struct MinaService<'client, Store, Client> {
    #[allow(dead_code)]
    pub(crate) cli: &'client LarkInner<Store, Client>,
}

#[cfg(feature = "test-util")]
impl<Store, Client> MinaService<'_, Store, Client> {
    pub fn mock(&self) -> MinaServiceMocker<Store, Client> {
        MinaServiceMocker { cli: self.cli }
    }
}

#[cfg(feature = "test-util")]
pub struct MinaServiceMocker<'client, Store, Client> {
    #[allow(dead_code)]
    pub(crate) cli: &'client LarkInner<Store, Client>,
}

impl<Store, Client> Lark<Store, Client> {
    pub fn mina(&self) -> MinaService<Store, Client> {
        MinaService { cli: &self.inner }
    }
}
pub mod mina_code_to_session;
