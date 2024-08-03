use crate::core::{Lark, LarkInner};

pub struct VcService<'client, Store, Client> {
    #[allow(dead_code)]
    pub(crate) cli: &'client LarkInner<Store, Client>,
}

#[cfg(feature = "test-util")]
impl<Store, Client> VcService<'_, Store, Client> {
    pub fn mock(&self) -> VcServiceMocker<Store, Client> {
        VcServiceMocker { cli: self.cli }
    }
}

#[cfg(feature = "test-util")]
pub struct VcServiceMocker<'client, Store, Client> {
    #[allow(dead_code)]
    pub(crate) cli: &'client LarkInner<Store, Client>,
}

impl<Store, Client> Lark<Store, Client> {
    pub fn vc(&self) -> VcService<Store, Client> {
        VcService { cli: &self.inner }
    }
}
pub mod apply_vc_reserve;
pub mod batch_get_vc_room;
pub mod batch_get_vc_room_level;
pub mod create_vc_room;
pub mod create_vc_room_level;
pub mod delete_vc_reserve;
pub mod delete_vc_room;
pub mod delete_vc_room_level;
pub mod download_vc_export_file;
pub mod end_vc_meeting;
pub mod export_vc_meeting_list;
pub mod export_vc_participant_list;
pub mod export_vc_participant_quality_list;
pub mod export_vc_resource_reservation_list;
pub mod get_vc_alert_list;
pub mod get_vc_daily_report;
pub mod get_vc_export_task;
pub mod get_vc_meeting;
pub mod get_vc_meeting_list;
pub mod get_vc_meeting_recording;
pub mod get_vc_participant_list;
pub mod get_vc_participant_quality_list;
pub mod get_vc_reserve;
pub mod get_vc_reserve_active_meeting;
pub mod get_vc_reserve_config;
pub mod get_vc_reserve_config_admin;
pub mod get_vc_reserve_config_disable_inform;
pub mod get_vc_reserve_config_form;
pub mod get_vc_resource_reservation_list;
pub mod get_vc_room;
pub mod get_vc_room_level;
pub mod get_vc_room_level_list;
pub mod get_vc_room_list;
pub mod get_vc_scope_config;
pub mod get_vc_top_user_report;
pub mod invite_vc_meeting;
pub mod kickout_vc_meeting;
pub mod list_vc_meeting_by_no;
pub mod search_vc_room;
pub mod search_vc_room_level;
pub mod set_vc_host_meeting;
pub mod set_vc_permission_meeting_recording;
pub mod set_vc_scope_config;
pub mod start_vc_meeting_recording;
pub mod stop_vc_meeting_recording;
pub mod update_vc_reserve;
pub mod update_vc_reserve_config;
pub mod update_vc_reserve_config_admin;
pub mod update_vc_reserve_config_disable_inform;
pub mod update_vc_reserve_config_form;
pub mod update_vc_room;
pub mod update_vc_room_level;
