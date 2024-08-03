use crate::core::{Lark, LarkInner};

pub struct TenantService<'client, Store, Client> {
    #[allow(dead_code)]
    pub(crate) cli: &'client LarkInner<Store, Client>,
}

#[cfg(feature = "test-util")]
impl<Store, Client> TenantService<'_, Store, Client> {
    pub fn mock(&self) -> TenantServiceMocker<Store, Client> {
        TenantServiceMocker { cli: self.cli }
    }
}

#[cfg(feature = "test-util")]
pub struct TenantServiceMocker<'client, Store, Client> {
    #[allow(dead_code)]
    pub(crate) cli: &'client LarkInner<Store, Client>,
}

impl<Store, Client> Lark<Store, Client> {
    pub fn tenant(&self) -> TenantService<Store, Client> {
        TenantService { cli: &self.inner }
    }
}
pub mod get_tenant;
pub mod get_tenant_product_assign_info;
