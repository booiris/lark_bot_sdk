use crate::core::{Lark, LarkInner};

pub struct PersonalSettingsService<'client, Store, Client> {
    #[allow(dead_code)]
    pub(crate) cli: &'client LarkInner<Store, Client>,
}

#[cfg(feature = "test-util")]
impl<Store, Client> PersonalSettingsService<'_, Store, Client> {
    pub fn mock(&self) -> PersonalSettingsServiceMocker<Store, Client> {
        PersonalSettingsServiceMocker { cli: self.cli }
    }
}

#[cfg(feature = "test-util")]
pub struct PersonalSettingsServiceMocker<'client, Store, Client> {
    #[allow(dead_code)]
    pub(crate) cli: &'client LarkInner<Store, Client>,
}

impl<Store, Client> Lark<Store, Client> {
    pub fn personal_settings(&self) -> PersonalSettingsService<Store, Client> {
        PersonalSettingsService { cli: &self.inner }
    }
}
pub mod batch_close_personal_settings_system_status;
pub mod batch_open_personal_settings_system_status;
pub mod create_personal_settings_system_status;
pub mod delete_personal_settings_system_status;
pub mod get_personal_settings_system_status_list;
pub mod update_personal_settings_system_status;
