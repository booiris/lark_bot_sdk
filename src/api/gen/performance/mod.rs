use crate::core::{Lark, LarkInner};

pub struct PerformanceService<'client, Store, Client> {
    #[allow(dead_code)]
    pub(crate) cli: &'client LarkInner<Store, Client>,
}

#[cfg(feature = "test-util")]
impl<Store, Client> PerformanceService<'_, Store, Client> {
    pub fn mock(&self) -> PerformanceServiceMocker<Store, Client> {
        PerformanceServiceMocker { cli: self.cli }
    }
}

#[cfg(feature = "test-util")]
pub struct PerformanceServiceMocker<'client, Store, Client> {
    #[allow(dead_code)]
    pub(crate) cli: &'client LarkInner<Store, Client>,
}

impl<Store, Client> Lark<Store, Client> {
    pub fn performance(&self) -> PerformanceService<Store, Client> {
        PerformanceService { cli: &self.inner }
    }
}
pub mod get_performance_semester_list;
pub mod get_performance_stage_task_by_page;
pub mod get_performance_stage_task_by_user;
