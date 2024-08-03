use crate::core::{Lark, LarkInner};

pub struct ApprovalService<'client, Store, Client> {
    #[allow(dead_code)]
    pub(crate) cli: &'client LarkInner<Store, Client>,
}

#[cfg(feature = "test-util")]
impl<Store, Client> ApprovalService<'_, Store, Client> {
    pub fn mock(&self) -> ApprovalServiceMocker<Store, Client> {
        ApprovalServiceMocker { cli: self.cli }
    }
}

#[cfg(feature = "test-util")]
pub struct ApprovalServiceMocker<'client, Store, Client> {
    #[allow(dead_code)]
    pub(crate) cli: &'client LarkInner<Store, Client>,
}

impl<Store, Client> Lark<Store, Client> {
    pub fn approval(&self) -> ApprovalService<Store, Client> {
        ApprovalService { cli: &self.inner }
    }
}
pub mod add_approval_instance_sign;
pub mod approve_approval_instance;
pub mod cancel_approval_instance;
pub mod check_approval_external_instance;
pub mod create_approval;
pub mod create_approval_carbon_copy;
pub mod create_approval_comment;
pub mod create_approval_external_approval;
pub mod create_approval_external_instance;
pub mod create_approval_instance;
pub mod delete_approval_comment;
pub mod get_approval;
pub mod get_approval_comment;
pub mod get_approval_external_approval;
pub mod get_approval_external_list;
pub mod get_approval_instance;
pub mod get_approval_instance_list;
pub mod get_approval_list;
pub mod get_approval_user_task_list;
pub mod preview_approval_instance;
pub mod reject_approval_instance;
pub mod remove_approval_comment;
pub mod resubmit_approval_instance_task;
pub mod rollback_approval_instance;
pub mod search_approval_carbon_copy;
pub mod search_approval_instance;
pub mod search_approval_task;
pub mod send_approval_message;
pub mod subscribe_approval_subscription;
pub mod transfer_approval_instance;
pub mod transform_approval_user_id;
pub mod unsubscribe_approval_subscription;
pub mod update_approval_message;
pub mod upload_approval_file;
