use crate::core::{Lark, LarkInner};

pub struct AcsService<'client, Store, Client> {
    #[allow(dead_code)]
    pub(crate) cli: &'client LarkInner<Store, Client>,
}

#[cfg(feature = "test-util")]
impl<Store, Client> AcsService<'_, Store, Client> {
    pub fn mock(&self) -> AcsServiceMocker<Store, Client> {
        AcsServiceMocker { cli: self.cli }
    }
}

#[cfg(feature = "test-util")]
pub struct AcsServiceMocker<'client, Store, Client> {
    #[allow(dead_code)]
    pub(crate) cli: &'client LarkInner<Store, Client>,
}

impl<Store, Client> Lark<Store, Client> {
    pub fn acs(&self) -> AcsService<Store, Client> {
        AcsService { cli: &self.inner }
    }
}
pub mod create_acs_rule_external;
pub mod create_acs_visitor;
pub mod delete_acs_rule_external;
pub mod delete_acs_visitor;
pub mod device_bind_acs_rule_external;
pub mod get_acs_access_record_list;
pub mod get_acs_access_record_photo;
pub mod get_acs_device_list;
pub mod get_acs_rule_external;
pub mod get_acs_user;
pub mod get_acs_user_face;
pub mod get_acs_user_list;
pub mod update_acs_user;
pub mod update_acs_user_face;
