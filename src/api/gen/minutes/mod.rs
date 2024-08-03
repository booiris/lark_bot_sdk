use crate::core::{Lark, LarkInner};

pub struct MinutesService<'client, Store, Client> {
    #[allow(dead_code)]
    pub(crate) cli: &'client LarkInner<Store, Client>,
}

#[cfg(feature = "test-util")]
impl<Store, Client> MinutesService<'_, Store, Client> {
    pub fn mock(&self) -> MinutesServiceMocker<Store, Client> {
        MinutesServiceMocker { cli: self.cli }
    }
}

#[cfg(feature = "test-util")]
pub struct MinutesServiceMocker<'client, Store, Client> {
    #[allow(dead_code)]
    pub(crate) cli: &'client LarkInner<Store, Client>,
}

impl<Store, Client> Lark<Store, Client> {
    pub fn minutes(&self) -> MinutesService<Store, Client> {
        MinutesService { cli: &self.inner }
    }
}
pub mod get_minutes_minute;
pub mod get_minutes_statistics;
