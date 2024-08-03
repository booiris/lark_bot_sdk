use crate::core::{Lark, LarkInner};

pub struct HireService<'client, Store, Client> {
    #[allow(dead_code)]
    pub(crate) cli: &'client LarkInner<Store, Client>,
}

#[cfg(feature = "test-util")]
impl<Store, Client> HireService<'_, Store, Client> {
    pub fn mock(&self) -> HireServiceMocker<Store, Client> {
        HireServiceMocker { cli: self.cli }
    }
}

#[cfg(feature = "test-util")]
pub struct HireServiceMocker<'client, Store, Client> {
    #[allow(dead_code)]
    pub(crate) cli: &'client LarkInner<Store, Client>,
}

impl<Store, Client> Lark<Store, Client> {
    pub fn hire(&self) -> HireService<Store, Client> {
        HireService { cli: &self.inner }
    }
}
pub mod add_hire_talent_to_folder;
pub mod batch_delete_hire_eco_account_custom_field;
pub mod batch_delete_hire_eco_background_check_custom_field;
pub mod batch_delete_hire_eco_background_check_package;
pub mod batch_delete_hire_eco_exam_paper;
pub mod batch_get_hire_talent;
pub mod batch_update_hire_eco_account_custom_field;
pub mod batch_update_hire_eco_background_check_custom_field;
pub mod batch_update_hire_eco_background_check_package;
pub mod batch_update_hire_eco_exam_paper;
pub mod cancel_hire_eco_background_check;
pub mod create_hire_application;
pub mod create_hire_eco_account_custom_field;
pub mod create_hire_eco_background_check_custom_field;
pub mod create_hire_eco_background_check_package;
pub mod create_hire_eco_exam_login_info;
pub mod create_hire_eco_exam_paper;
pub mod create_hire_external_application;
pub mod create_hire_external_background_check;
pub mod create_hire_external_interview;
pub mod create_hire_external_interview_assessment;
pub mod create_hire_job;
pub mod create_hire_note;
pub mod create_hire_offer;
pub mod create_hire_referral_account;
pub mod deactivate_hire_referral_account;
pub mod delete_hire_external_application;
pub mod get_hire_application;
pub mod get_hire_application_interview_list;
pub mod get_hire_application_list;
pub mod get_hire_attachment;
pub mod get_hire_attachment_preview;
pub mod get_hire_employee;
pub mod get_hire_employee_by_application;
pub mod get_hire_evaluation_list;
pub mod get_hire_interview_list;
pub mod get_hire_job;
pub mod get_hire_job_config;
pub mod get_hire_job_manager;
pub mod get_hire_job_process_list;
pub mod get_hire_note;
pub mod get_hire_note_list;
pub mod get_hire_offer;
pub mod get_hire_offer_by_application;
pub mod get_hire_offer_list;
pub mod get_hire_offer_schema;
pub mod get_hire_questionnaire_list;
pub mod get_hire_referral_by_application;
pub mod get_hire_referral_website_job_post;
pub mod get_hire_referral_website_job_post_list;
pub mod get_hire_resume_source;
pub mod get_hire_talent;
pub mod get_hire_talent_folder_list;
pub mod make_hire_transfer_onboard_by_application;
pub mod query_hire_talent_object;
pub mod reconcile_hire_referral_account;
pub mod terminate_hire_application;
pub mod update_hire_eco_background_check_progress;
pub mod update_hire_eco_background_check_result;
pub mod update_hire_ehr_import_task;
pub mod update_hire_employee;
pub mod update_hire_external_application;
pub mod update_hire_job;
pub mod update_hire_job_config;
pub mod update_hire_note;
pub mod update_hire_offer;
pub mod update_hire_offer_intern_status;
pub mod update_hire_offer_status;
pub mod update_hired_eco_exam_result;
pub mod withdraw_hire_referral_account;
