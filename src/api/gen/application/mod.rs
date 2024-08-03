use crate::core::{Lark, LarkInner};

pub struct ApplicationService<'client, Store, Client> {
    #[allow(dead_code)]
    pub(crate) cli: &'client LarkInner<Store, Client>,
}

#[cfg(feature = "test-util")]
impl<Store, Client> ApplicationService<'_, Store, Client> {
    pub fn mock(&self) -> ApplicationServiceMocker<Store, Client> {
        ApplicationServiceMocker { cli: self.cli }
    }
}

#[cfg(feature = "test-util")]
pub struct ApplicationServiceMocker<'client, Store, Client> {
    #[allow(dead_code)]
    pub(crate) cli: &'client LarkInner<Store, Client>,
}

impl<Store, Client> Lark<Store, Client> {
    pub fn application(&self) -> ApplicationService<Store, Client> {
        ApplicationService { cli: &self.inner }
    }
}
pub mod check_application_visible_white_black_list;
pub mod check_user_is_in_application_paid_scope;
pub mod get_application;
pub mod get_application_app_admin_user_list;
pub mod get_application_app_list;
pub mod get_application_app_visibility;
pub mod get_application_contacts_range_configuration;
pub mod get_application_favourite;
pub mod get_application_feedback_list;
pub mod get_application_order;
pub mod get_application_order_list;
pub mod get_application_recommend;
pub mod get_application_recommend_rule_list;
pub mod get_application_under_audit_list;
pub mod get_application_usage_department_overview;
pub mod get_application_usage_overview;
pub mod get_application_usage_trend;
pub mod get_application_user_admin_scope;
pub mod get_application_user_visible_app;
pub mod get_application_version;
pub mod get_application_version_contacts_range_suggest;
pub mod get_application_version_list;
pub mod is_application_user_admin;
pub mod search_application_custom_workplace_access_data;
pub mod search_application_workplace_access_data;
pub mod search_application_workplace_block_access_data;
pub mod set_application_app_badge;
pub mod update_application;
pub mod update_application_app_management;
pub mod update_application_app_visibility;
pub mod update_application_app_visibility_v6;
pub mod update_application_contacts_range_configuration;
pub mod update_application_feedback;
pub mod update_application_version;
