use crate::core::{Lark, LarkInner};

pub struct JssdkService<'client, Store, Client> {
    #[allow(dead_code)]
    pub(crate) cli: &'client LarkInner<Store, Client>,
}

#[cfg(feature = "test-util")]
impl<Store, Client> JssdkService<'_, Store, Client> {
    pub fn mock(&self) -> JssdkServiceMocker<Store, Client> {
        JssdkServiceMocker { cli: self.cli }
    }
}

#[cfg(feature = "test-util")]
pub struct JssdkServiceMocker<'client, Store, Client> {
    #[allow(dead_code)]
    pub(crate) cli: &'client LarkInner<Store, Client>,
}

impl<Store, Client> Lark<Store, Client> {
    pub fn jssdk(&self) -> JssdkService<Store, Client> {
        JssdkService { cli: &self.inner }
    }
}
pub mod get_jssdk_ticket;
