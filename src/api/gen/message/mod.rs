use crate::core::{Lark, LarkInner};

pub struct MessageService<'client, Store, Client> {
    #[allow(dead_code)]
    pub(crate) cli: &'client LarkInner<Store, Client>,
}

#[cfg(feature = "test-util")]
impl<Store, Client> MessageService<'_, Store, Client> {
    pub fn mock(&self) -> MessageServiceMocker<Store, Client> {
        MessageServiceMocker { cli: self.cli }
    }
}

#[cfg(feature = "test-util")]
pub struct MessageServiceMocker<'client, Store, Client> {
    #[allow(dead_code)]
    pub(crate) cli: &'client LarkInner<Store, Client>,
}

impl<Store, Client> Lark<Store, Client> {
    pub fn message(&self) -> MessageService<Store, Client> {
        MessageService { cli: &self.inner }
    }
}
pub mod batch_delete_message;
pub mod batch_send_old_raw_message;
pub mod create_message_pin;
pub mod create_message_reaction;
pub mod delete_ephemeral_message;
pub mod delete_message;
pub mod delete_message_pin;
pub mod delete_message_reaction;
pub mod forward_message;
pub mod forward_thread_message;
pub mod get_batch_sent_message_progress;
pub mod get_batch_sent_message_read_user;
pub mod get_message;
pub mod get_message_file;
pub mod get_message_list;
pub mod get_message_pin_list;
pub mod get_message_reaction_list;
pub mod get_message_read_user_list;
pub mod get_message_special_focus_list;
pub mod get_message_special_focus_unread;
pub mod merge_forward_message;
pub mod reply_raw_message;
pub mod send_ephemeral_message;
pub mod send_raw_message;
pub mod send_raw_message_old;
pub mod send_urgent_app_message;
pub mod send_urgent_phone_message;
pub mod send_urgent_sms_message;
pub mod update_message;
pub mod update_message_delay;
pub mod update_message_edit;
