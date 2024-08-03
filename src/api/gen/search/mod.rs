use crate::core::{Lark, LarkInner};

pub struct SearchService<'client, Store, Client> {
    #[allow(dead_code)]
    pub(crate) cli: &'client LarkInner<Store, Client>,
}

#[cfg(feature = "test-util")]
impl<Store, Client> SearchService<'_, Store, Client> {
    pub fn mock(&self) -> SearchServiceMocker<Store, Client> {
        SearchServiceMocker { cli: self.cli }
    }
}

#[cfg(feature = "test-util")]
pub struct SearchServiceMocker<'client, Store, Client> {
    #[allow(dead_code)]
    pub(crate) cli: &'client LarkInner<Store, Client>,
}

impl<Store, Client> Lark<Store, Client> {
    pub fn search(&self) -> SearchService<Store, Client> {
        SearchService { cli: &self.inner }
    }
}
pub mod batch_create_search_data_source_item;
pub mod create_search_data_source;
pub mod create_search_data_source_item;
pub mod create_search_schema;
pub mod delete_search_data_source;
pub mod delete_search_data_source_item;
pub mod delete_search_schema;
pub mod get_search_data_source;
pub mod get_search_data_source_item;
pub mod get_search_data_source_list;
pub mod get_search_schema;
pub mod search_app;
pub mod search_message;
pub mod update_search_data_source;
pub mod update_search_schema;
