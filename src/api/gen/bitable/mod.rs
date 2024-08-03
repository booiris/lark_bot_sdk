use crate::core::{Lark, LarkInner};

pub struct BitableService<'client, Store, Client> {
    #[allow(dead_code)]
    pub(crate) cli: &'client LarkInner<Store, Client>,
}

#[cfg(feature = "test-util")]
impl<Store, Client> BitableService<'_, Store, Client> {
    pub fn mock(&self) -> BitableServiceMocker<Store, Client> {
        BitableServiceMocker { cli: self.cli }
    }
}

#[cfg(feature = "test-util")]
pub struct BitableServiceMocker<'client, Store, Client> {
    #[allow(dead_code)]
    pub(crate) cli: &'client LarkInner<Store, Client>,
}

impl<Store, Client> Lark<Store, Client> {
    pub fn bitable(&self) -> BitableService<Store, Client> {
        BitableService { cli: &self.inner }
    }
}
pub mod batch_create_bitable_app_role_member;
pub mod batch_create_bitable_record;
pub mod batch_create_bitable_table;
pub mod batch_delete_bitable_app_role_member;
pub mod batch_delete_bitable_record;
pub mod batch_delete_bitable_table;
pub mod batch_update_bitable_record;
pub mod copy_bitable_app;
pub mod copy_bitable_dashboard;
pub mod create_bitable_app;
pub mod create_bitable_app_role;
pub mod create_bitable_app_role_member;
pub mod create_bitable_field;
pub mod create_bitable_record;
pub mod create_bitable_table;
pub mod create_bitable_view;
pub mod delete_bitable_app_role;
pub mod delete_bitable_app_role_member;
pub mod delete_bitable_field;
pub mod delete_bitable_record;
pub mod delete_bitable_table;
pub mod delete_bitable_view;
pub mod get_bitable_app_role_list;
pub mod get_bitable_app_role_member_list;
pub mod get_bitable_dashboard_list;
pub mod get_bitable_field_list;
pub mod get_bitable_meta;
pub mod get_bitable_record;
pub mod get_bitable_record_list;
pub mod get_bitable_table_form;
pub mod get_bitable_table_form_field_list;
pub mod get_bitable_table_list;
pub mod get_bitable_view;
pub mod get_bitable_view_list;
pub mod search_bitable_record;
pub mod update_bitable_app_role;
pub mod update_bitable_field;
pub mod update_bitable_meta;
pub mod update_bitable_record;
pub mod update_bitable_table;
pub mod update_bitable_table_form;
pub mod update_bitable_table_form_field;
pub mod update_bitable_view;
