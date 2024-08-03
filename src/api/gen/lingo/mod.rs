use crate::core::{Lark, LarkInner};

pub struct LingoService<'client, Store, Client> {
    #[allow(dead_code)]
    pub(crate) cli: &'client LarkInner<Store, Client>,
}

#[cfg(feature = "test-util")]
impl<Store, Client> LingoService<'_, Store, Client> {
    pub fn mock(&self) -> LingoServiceMocker<Store, Client> {
        LingoServiceMocker { cli: self.cli }
    }
}

#[cfg(feature = "test-util")]
pub struct LingoServiceMocker<'client, Store, Client> {
    #[allow(dead_code)]
    pub(crate) cli: &'client LarkInner<Store, Client>,
}

impl<Store, Client> Lark<Store, Client> {
    pub fn lingo(&self) -> LingoService<Store, Client> {
        LingoService { cli: &self.inner }
    }
}
pub mod create_lingo_draft;
pub mod create_lingo_entity;
pub mod delete_lingo_entity;
pub mod download_lingo_file;
pub mod extract_lingo_entity;
pub mod get_lingo_classification_list;
pub mod get_lingo_entity;
pub mod get_lingo_entity_list;
pub mod get_lingo_repo_list;
pub mod highlight_lingo_entity;
pub mod match_lingo_entity;
pub mod search_lingo_entity;
pub mod update_lingo_draft;
pub mod update_lingo_entity;
pub mod upload_lingo_file;
