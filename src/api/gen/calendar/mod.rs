use crate::core::{Lark, LarkInner};

pub struct CalendarService<'client, Store, Client> {
    #[allow(dead_code)]
    pub(crate) cli: &'client LarkInner<Store, Client>,
}

#[cfg(feature = "test-util")]
impl<Store, Client> CalendarService<'_, Store, Client> {
    pub fn mock(&self) -> CalendarServiceMocker<Store, Client> {
        CalendarServiceMocker { cli: self.cli }
    }
}

#[cfg(feature = "test-util")]
pub struct CalendarServiceMocker<'client, Store, Client> {
    #[allow(dead_code)]
    pub(crate) cli: &'client LarkInner<Store, Client>,
}

impl<Store, Client> Lark<Store, Client> {
    pub fn calendar(&self) -> CalendarService<Store, Client> {
        CalendarService { cli: &self.inner }
    }
}
pub mod batch_get_calendar_meeting_room_freebusy;
pub mod batch_get_calendar_meeting_room_summary;
pub mod create_calendar;
pub mod create_calendar_acl;
pub mod create_calendar_event;
pub mod create_calendar_event_attendee;
pub mod create_calendar_event_meeting_chat;
pub mod create_calendar_exchange_binding;
pub mod create_calendar_timeoff_event;
pub mod delete_calendar;
pub mod delete_calendar_acl;
pub mod delete_calendar_event;
pub mod delete_calendar_event_attendee;
pub mod delete_calendar_event_meeting_chat;
pub mod delete_calendar_exchange_binding;
pub mod delete_calendar_timeoff_event;
pub mod generate_caldav_conf;
pub mod get_calendar;
pub mod get_calendar_acl_list;
pub mod get_calendar_event;
pub mod get_calendar_event_attendee_chat_member_list;
pub mod get_calendar_event_attendee_list;
pub mod get_calendar_event_instance_list;
pub mod get_calendar_event_instance_view_list;
pub mod get_calendar_event_list;
pub mod get_calendar_exchange_binding;
pub mod get_calendar_free_busy_list;
pub mod get_calendar_list;
pub mod get_primary_calendar;
pub mod reply_calendar_event;
pub mod reply_calendar_meeting_room_instance;
pub mod search_calendar;
pub mod search_calendar_event;
pub mod subscribe_calendar;
pub mod subscribe_calendar_acl;
pub mod subscribe_calendar_change_event;
pub mod subscribe_calendar_event;
pub mod unsubscribe_calendar;
pub mod unsubscribe_calendar_acl;
pub mod unsubscribe_calendar_change_event;
pub mod unsubscribe_calendar_event;
pub mod update_calendar;
pub mod update_calendar_event;
