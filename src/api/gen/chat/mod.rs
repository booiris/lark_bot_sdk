use crate::core::{Lark, LarkInner};

pub struct ChatService<'client, Store, Client> {
    #[allow(dead_code)]
    pub(crate) cli: &'client LarkInner<Store, Client>,
}

#[cfg(feature = "test-util")]
impl<Store, Client> ChatService<'_, Store, Client> {
    pub fn mock(&self) -> ChatServiceMocker<Store, Client> {
        ChatServiceMocker { cli: self.cli }
    }
}

#[cfg(feature = "test-util")]
pub struct ChatServiceMocker<'client, Store, Client> {
    #[allow(dead_code)]
    pub(crate) cli: &'client LarkInner<Store, Client>,
}

impl<Store, Client> Lark<Store, Client> {
    pub fn chat(&self) -> ChatService<Store, Client> {
        ChatService { cli: &self.inner }
    }
}
pub mod add_chat_member;
pub mod create_chat;
pub mod create_chat_manager;
pub mod create_chat_menu_tree;
pub mod create_chat_tab;
pub mod delete_chat;
pub mod delete_chat_manager;
pub mod delete_chat_member;
pub mod delete_chat_menu_tree;
pub mod delete_chat_tab;
pub mod delete_chat_top_notice;
pub mod gen_chat_share_link;
pub mod get_chat;
pub mod get_chat_announcement;
pub mod get_chat_list_of_self;
pub mod get_chat_member_list;
pub mod get_chat_menu_tree;
pub mod get_chat_moderation;
pub mod get_chat_old;
pub mod get_chat_tab_list;
pub mod is_in_chat;
pub mod join_chat;
pub mod search_chat;
pub mod sort_chat_menu_tree;
pub mod sort_chat_tab;
pub mod update_chat;
pub mod update_chat_announcement;
pub mod update_chat_menu_tree;
pub mod update_chat_moderation;
pub mod update_chat_tab;
pub mod update_chat_top_notice;
