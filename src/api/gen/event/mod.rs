use crate::core::{Lark, LarkInner};

pub struct EventService<'client, Store, Client> {
    #[allow(dead_code)]
    pub(crate) cli: &'client LarkInner<Store, Client>,
}

#[cfg(feature = "test-util")]
impl<Store, Client> EventService<'_, Store, Client> {
    pub fn mock(&self) -> EventServiceMocker<Store, Client> {
        EventServiceMocker { cli: self.cli }
    }
}

#[cfg(feature = "test-util")]
pub struct EventServiceMocker<'client, Store, Client> {
    #[allow(dead_code)]
    pub(crate) cli: &'client LarkInner<Store, Client>,
}

impl<Store, Client> Lark<Store, Client> {
    pub fn event(&self) -> EventService<Store, Client> {
        EventService { cli: &self.inner }
    }
}
pub mod get_event_outbound_ip_list;
