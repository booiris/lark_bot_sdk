use crate::core::{Lark, LarkInner};

pub struct MailService<'client, Store, Client> {
    #[allow(dead_code)]
    pub(crate) cli: &'client LarkInner<Store, Client>,
}

#[cfg(feature = "test-util")]
impl<Store, Client> MailService<'_, Store, Client> {
    pub fn mock(&self) -> MailServiceMocker<Store, Client> {
        MailServiceMocker { cli: self.cli }
    }
}

#[cfg(feature = "test-util")]
pub struct MailServiceMocker<'client, Store, Client> {
    #[allow(dead_code)]
    pub(crate) cli: &'client LarkInner<Store, Client>,
}

impl<Store, Client> Lark<Store, Client> {
    pub fn mail(&self) -> MailService<Store, Client> {
        MailService { cli: &self.inner }
    }
}
pub mod batch_create_mail_group_manager;
pub mod batch_create_mail_group_member;
pub mod batch_create_mail_group_permission_member;
pub mod batch_create_public_mailbox_member;
pub mod batch_delete_mail_group_manager;
pub mod batch_delete_mail_group_member;
pub mod batch_delete_mail_group_permission_member;
pub mod batch_delete_public_mailbox_member;
pub mod clear_public_mailbox_member;
pub mod create_mail_group;
pub mod create_mail_group_alias;
pub mod create_mail_group_member;
pub mod create_mail_group_permission_member;
pub mod create_mail_public_mailbox_alias;
pub mod create_mail_user_mailbox_alias;
pub mod create_public_mailbox;
pub mod create_public_mailbox_member;
pub mod delete_mail_group;
pub mod delete_mail_group_alias;
pub mod delete_mail_group_member;
pub mod delete_mail_group_permission_member;
pub mod delete_mail_public_mailbox_alias;
pub mod delete_mail_user_mailbox;
pub mod delete_mail_user_mailbox_alias;
pub mod delete_public_mailbox;
pub mod delete_public_mailbox_member;
pub mod get_mail_group;
pub mod get_mail_group_alias_list;
pub mod get_mail_group_list;
pub mod get_mail_group_manager_list;
pub mod get_mail_group_member;
pub mod get_mail_group_member_list;
pub mod get_mail_group_permission_member;
pub mod get_mail_group_permission_member_list;
pub mod get_mail_public_mailbox_alias_list;
pub mod get_mail_user;
pub mod get_mail_user_mailbox_alias_list;
pub mod get_public_mailbox;
pub mod get_public_mailbox_list;
pub mod get_public_mailbox_member;
pub mod get_public_mailbox_member_list;
pub mod update_mail_group;
pub mod update_mail_group_patch;
pub mod update_public_mailbox;
pub mod update_public_mailbox_patch;
