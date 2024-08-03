use crate::core::{Lark, LarkInner};

pub struct HelpdeskService<'client, Store, Client> {
    #[allow(dead_code)]
    pub(crate) cli: &'client LarkInner<Store, Client>,
}

#[cfg(feature = "test-util")]
impl<Store, Client> HelpdeskService<'_, Store, Client> {
    pub fn mock(&self) -> HelpdeskServiceMocker<Store, Client> {
        HelpdeskServiceMocker { cli: self.cli }
    }
}

#[cfg(feature = "test-util")]
pub struct HelpdeskServiceMocker<'client, Store, Client> {
    #[allow(dead_code)]
    pub(crate) cli: &'client LarkInner<Store, Client>,
}

impl<Store, Client> Lark<Store, Client> {
    pub fn helpdesk(&self) -> HelpdeskService<Store, Client> {
        HelpdeskService { cli: &self.inner }
    }
}
pub mod answer_helpdesk_ticket_user_query;
pub mod cancel_approve_helpdesk_notification;
pub mod cancel_send_helpdesk_notification;
pub mod create_helpdesk_agent_schedule;
pub mod create_helpdesk_agent_skill;
pub mod create_helpdesk_category;
pub mod create_helpdesk_faq;
pub mod create_helpdesk_notification;
pub mod create_helpdesk_ticket_customized_field;
pub mod delete_helpdesk_agent_schedule;
pub mod delete_helpdesk_agent_skill;
pub mod delete_helpdesk_category;
pub mod delete_helpdesk_faq;
pub mod delete_helpdesk_ticket_customized_field;
pub mod download_helpdesk_ticket_image;
pub mod execute_send_helpdesk_notification;
pub mod get_helpdesk_agent_email;
pub mod get_helpdesk_agent_schedule;
pub mod get_helpdesk_agent_schedule_list;
pub mod get_helpdesk_agent_skill;
pub mod get_helpdesk_agent_skill_list;
pub mod get_helpdesk_agent_skill_rule_list;
pub mod get_helpdesk_category;
pub mod get_helpdesk_category_list;
pub mod get_helpdesk_faq;
pub mod get_helpdesk_faq_image;
pub mod get_helpdesk_faq_list;
pub mod get_helpdesk_notification;
pub mod get_helpdesk_ticket;
pub mod get_helpdesk_ticket_customized_field;
pub mod get_helpdesk_ticket_customized_field_list;
pub mod get_helpdesk_ticket_customized_fields;
pub mod get_helpdesk_ticket_list;
pub mod get_helpdesk_ticket_message_list;
pub mod preview_helpdesk_notification;
pub mod search_helpdesk_faq;
pub mod send_helpdesk_message;
pub mod send_helpdesk_ticket_message;
pub mod start_helpdesk_service;
pub mod submit_approve_helpdesk_notification;
pub mod subscribe_helpdesk_event;
pub mod unsubscribe_helpdesk_event;
pub mod update_helpdesk_agent;
pub mod update_helpdesk_agent_schedule;
pub mod update_helpdesk_agent_skill;
pub mod update_helpdesk_category;
pub mod update_helpdesk_faq;
pub mod update_helpdesk_notification;
pub mod update_helpdesk_ticket;
pub mod update_helpdesk_ticket_customized_field;
