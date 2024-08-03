use crate::core::{Lark, LarkInner};

pub struct BaikeService<'client, Store, Client> {
    #[allow(dead_code)]
    pub(crate) cli: &'client LarkInner<Store, Client>,
}

#[cfg(feature = "test-util")]
impl<Store, Client> BaikeService<'_, Store, Client> {
    pub fn mock(&self) -> BaikeServiceMocker<Store, Client> {
        BaikeServiceMocker { cli: self.cli }
    }
}

#[cfg(feature = "test-util")]
pub struct BaikeServiceMocker<'client, Store, Client> {
    #[allow(dead_code)]
    pub(crate) cli: &'client LarkInner<Store, Client>,
}

impl<Store, Client> Lark<Store, Client> {
    pub fn baike(&self) -> BaikeService<Store, Client> {
        BaikeService { cli: &self.inner }
    }
}
pub mod create_baike_draft;
pub mod create_baike_entity;
pub mod create_baike_update;
pub mod download_baike_image;
pub mod extract_baike_entity;
pub mod get_baike_classification_list;
pub mod get_baike_entity;
pub mod get_baike_entity_list;
pub mod highlight_baike_entity;
pub mod match_baike_entity;
pub mod search_baike_entity;
pub mod update_baike_entity;
pub mod upload_baike_image;
