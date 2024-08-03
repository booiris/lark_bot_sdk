use crate::core::{Lark, LarkInner};

pub struct AttendanceService<'client, Store, Client> {
    #[allow(dead_code)]
    pub(crate) cli: &'client LarkInner<Store, Client>,
}

#[cfg(feature = "test-util")]
impl<Store, Client> AttendanceService<'_, Store, Client> {
    pub fn mock(&self) -> AttendanceServiceMocker<Store, Client> {
        AttendanceServiceMocker { cli: self.cli }
    }
}

#[cfg(feature = "test-util")]
pub struct AttendanceServiceMocker<'client, Store, Client> {
    #[allow(dead_code)]
    pub(crate) cli: &'client LarkInner<Store, Client>,
}

impl<Store, Client> Lark<Store, Client> {
    pub fn attendance(&self) -> AttendanceService<Store, Client> {
        AttendanceService { cli: &self.inner }
    }
}
pub mod batch_create_attendance_user_daily_shift;
pub mod batch_create_attendance_user_flow;
pub mod batch_get_attendance_user_flow;
pub mod create_attendance_group;
pub mod create_attendance_shift;
pub mod create_attendance_user_approval;
pub mod create_attendance_user_task_remedy;
pub mod delete_attendance_group;
pub mod delete_attendance_shift;
pub mod download_attendance_file;
pub mod get_attendance_group;
pub mod get_attendance_group_list;
pub mod get_attendance_leave_employ_expire_record;
pub mod get_attendance_shift;
pub mod get_attendance_shift_detail;
pub mod get_attendance_shift_list;
pub mod get_attendance_user_approval;
pub mod get_attendance_user_daily_shift;
pub mod get_attendance_user_flow;
pub mod get_attendance_user_setting_list;
pub mod get_attendance_user_stats_data;
pub mod get_attendance_user_stats_field;
pub mod get_attendance_user_stats_view;
pub mod get_attendance_user_task;
pub mod get_attendance_user_task_remedy;
pub mod get_attendance_user_task_remedy_allowed_remedy_list;
pub mod search_attendance_group;
pub mod update_attendance_leave_accrual_record;
pub mod update_attendance_remedy_approval;
pub mod update_attendance_user_setting;
pub mod update_attendance_user_stats_view;
pub mod upload_attendance_file;
