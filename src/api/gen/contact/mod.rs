use crate::core::{Lark, LarkInner};

pub struct ContactService<'client, Store, Client> {
    #[allow(dead_code)]
    pub(crate) cli: &'client LarkInner<Store, Client>,
}

#[cfg(feature = "test-util")]
impl<Store, Client> ContactService<'_, Store, Client> {
    pub fn mock(&self) -> ContactServiceMocker<Store, Client> {
        ContactServiceMocker { cli: self.cli }
    }
}

#[cfg(feature = "test-util")]
pub struct ContactServiceMocker<'client, Store, Client> {
    #[allow(dead_code)]
    pub(crate) cli: &'client LarkInner<Store, Client>,
}

impl<Store, Client> Lark<Store, Client> {
    pub fn contact(&self) -> ContactService<Store, Client> {
        ContactService { cli: &self.inner }
    }
}
pub mod add_contact_group_member;
pub mod batch_add_contact_group_member;
pub mod batch_create_contact_functional_role_member;
pub mod batch_delete_contact_functional_role_member;
pub mod batch_delete_contact_group_member;
pub mod batch_get_department;
pub mod batch_get_user;
pub mod batch_get_user_by_id;
pub mod batch_get_user_by_id_old;
pub mod bind_contact_unit_department;
pub mod create_contact_functional_role;
pub mod create_contact_group;
pub mod create_contact_job_family;
pub mod create_contact_job_level;
pub mod create_contact_unit;
pub mod create_department;
pub mod create_employee_type_enum;
pub mod create_user;
pub mod delete_contact_functional_role;
pub mod delete_contact_group;
pub mod delete_contact_group_member;
pub mod delete_contact_job_family;
pub mod delete_contact_job_level;
pub mod delete_contact_unit;
pub mod delete_department;
pub mod delete_employee_type_enum;
pub mod delete_user;
pub mod get_contact_custom_attr_list;
pub mod get_contact_functional_role_member;
pub mod get_contact_functional_role_member_scope;
pub mod get_contact_group;
pub mod get_contact_group_list;
pub mod get_contact_group_member;
pub mod get_contact_job_family;
pub mod get_contact_job_family_list;
pub mod get_contact_job_level;
pub mod get_contact_job_level_list;
pub mod get_contact_job_title;
pub mod get_contact_job_title_list;
pub mod get_contact_member_group_list;
pub mod get_contact_scope_list;
pub mod get_contact_unit;
pub mod get_contact_unit_department_list;
pub mod get_contact_unit_list;
pub mod get_contact_work_city;
pub mod get_contact_work_city_list;
pub mod get_department;
pub mod get_department_list;
pub mod get_department_list_old;
pub mod get_employee_type_enum_list;
pub mod get_parent_department;
pub mod get_user;
pub mod get_user_list;
pub mod get_user_list_old;
pub mod resurrect_user;
pub mod search_department;
pub mod search_user_old;
pub mod unbind_contact_unit_department;
pub mod unbind_department_chat;
pub mod update_contact_functional_role;
pub mod update_contact_functional_role_member_scope;
pub mod update_contact_group;
pub mod update_contact_job_family;
pub mod update_contact_job_level;
pub mod update_contact_unit;
pub mod update_department;
pub mod update_department_id;
pub mod update_department_patch;
pub mod update_employee_type_enum_patch;
pub mod update_user;
pub mod update_user_id;
pub mod update_user_patch;
