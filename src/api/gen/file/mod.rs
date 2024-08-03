use crate::core::{Lark, LarkInner};

pub struct FileService<'client, Store, Client> {
    #[allow(dead_code)]
    pub(crate) cli: &'client LarkInner<Store, Client>,
}

#[cfg(feature = "test-util")]
impl<Store, Client> FileService<'_, Store, Client> {
    pub fn mock(&self) -> FileServiceMocker<Store, Client> {
        FileServiceMocker { cli: self.cli }
    }
}

#[cfg(feature = "test-util")]
pub struct FileServiceMocker<'client, Store, Client> {
    #[allow(dead_code)]
    pub(crate) cli: &'client LarkInner<Store, Client>,
}

impl<Store, Client> Lark<Store, Client> {
    pub fn file(&self) -> FileService<Store, Client> {
        FileService { cli: &self.inner }
    }
}
pub mod download_file;
pub mod download_image;
pub mod upload_file;
pub mod upload_image;
