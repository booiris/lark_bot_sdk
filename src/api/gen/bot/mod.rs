use crate::core::{Lark, LarkInner};

pub struct BotService<'client, Store, Client> {
    #[allow(dead_code)]
    pub(crate) cli: &'client LarkInner<Store, Client>,
}

#[cfg(feature = "test-util")]
impl<Store, Client> BotService<'_, Store, Client> {
    pub fn mock(&self) -> BotServiceMocker<Store, Client> {
        BotServiceMocker { cli: self.cli }
    }
}

#[cfg(feature = "test-util")]
pub struct BotServiceMocker<'client, Store, Client> {
    #[allow(dead_code)]
    pub(crate) cli: &'client LarkInner<Store, Client>,
}

impl<Store, Client> Lark<Store, Client> {
    pub fn bot(&self) -> BotService<Store, Client> {
        BotService { cli: &self.inner }
    }
}
pub mod add_bot_to_chat;
pub mod get_bot_info;
