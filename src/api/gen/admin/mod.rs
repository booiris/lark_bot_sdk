use crate::core::{Lark, LarkInner};

pub struct AdminService<'client, Store, Client> {
    #[allow(dead_code)]
    pub(crate) cli: &'client LarkInner<Store, Client>,
}

#[cfg(feature = "test-util")]
impl<Store, Client> AdminService<'_, Store, Client> {
    pub fn mock(&self) -> AdminServiceMocker<Store, Client> {
        AdminServiceMocker { cli: self.cli }
    }
}

#[cfg(feature = "test-util")]
pub struct AdminServiceMocker<'client, Store, Client> {
    #[allow(dead_code)]
    pub(crate) cli: &'client LarkInner<Store, Client>,
}

impl<Store, Client> Lark<Store, Client> {
    pub fn admin(&self) -> AdminService<Store, Client> {
        AdminService { cli: &self.inner }
    }
}
pub mod admin_reset_password;
pub mod create_admin_badge;
pub mod create_admin_badge_grant;
pub mod delete_admin_badge_grant;
pub mod get_admin_badge;
pub mod get_admin_badge_grant;
pub mod get_admin_badge_grant_list;
pub mod get_admin_badge_list;
pub mod get_admin_dept_stats;
pub mod get_admin_user_stats;
pub mod update_admin_badge;
pub mod update_admin_badge_grant;
pub mod upload_admin_badge_image;
