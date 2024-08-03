use crate::core::{Lark, LarkInner};

pub struct OkrService<'client, Store, Client> {
    #[allow(dead_code)]
    pub(crate) cli: &'client LarkInner<Store, Client>,
}

#[cfg(feature = "test-util")]
impl<Store, Client> OkrService<'_, Store, Client> {
    pub fn mock(&self) -> OkrServiceMocker<Store, Client> {
        OkrServiceMocker { cli: self.cli }
    }
}

#[cfg(feature = "test-util")]
pub struct OkrServiceMocker<'client, Store, Client> {
    #[allow(dead_code)]
    pub(crate) cli: &'client LarkInner<Store, Client>,
}

impl<Store, Client> Lark<Store, Client> {
    pub fn okr(&self) -> OkrService<Store, Client> {
        OkrService { cli: &self.inner }
    }
}
pub mod batch_get_okr;
pub mod batch_update_okr_metric_source_table_item;
pub mod create_okr_period;
pub mod create_okr_progress_record;
pub mod delete_okr_progress_record;
pub mod get_okr_metric_source_list;
pub mod get_okr_metric_source_table_item;
pub mod get_okr_metric_source_table_item_list;
pub mod get_okr_metric_source_table_list;
pub mod get_okr_period_list;
pub mod get_okr_period_rule_list;
pub mod get_okr_progress_record;
pub mod get_user_okr_list;
pub mod update_okr_metric_source_table_item;
pub mod update_okr_period;
pub mod update_okr_progress_record;
pub mod upload_okr_image;
