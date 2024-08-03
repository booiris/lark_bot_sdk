use crate::core::{Lark, LarkInner};

pub struct EhrService<'client, Store, Client> {
    #[allow(dead_code)]
    pub(crate) cli: &'client LarkInner<Store, Client>,
}

#[cfg(feature = "test-util")]
impl<Store, Client> EhrService<'_, Store, Client> {
    pub fn mock(&self) -> EhrServiceMocker<Store, Client> {
        EhrServiceMocker { cli: self.cli }
    }
}

#[cfg(feature = "test-util")]
pub struct EhrServiceMocker<'client, Store, Client> {
    #[allow(dead_code)]
    pub(crate) cli: &'client LarkInner<Store, Client>,
}

impl<Store, Client> Lark<Store, Client> {
    pub fn ehr(&self) -> EhrService<Store, Client> {
        EhrService { cli: &self.inner }
    }
}
pub mod download_ehr_attachments;
pub mod get_ehr_employee_list;
