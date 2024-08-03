use crate::core::{Lark, LarkInner};

pub struct CoreHrService<'client, Store, Client> {
    #[allow(dead_code)]
    pub(crate) cli: &'client LarkInner<Store, Client>,
}

#[cfg(feature = "test-util")]
impl<Store, Client> CoreHrService<'_, Store, Client> {
    pub fn mock(&self) -> CoreHrServiceMocker<Store, Client> {
        CoreHrServiceMocker { cli: self.cli }
    }
}

#[cfg(feature = "test-util")]
pub struct CoreHrServiceMocker<'client, Store, Client> {
    #[allow(dead_code)]
    pub(crate) cli: &'client LarkInner<Store, Client>,
}

impl<Store, Client> Lark<Store, Client> {
    pub fn core_hr(&self) -> CoreHrService<Store, Client> {
        CoreHrService { cli: &self.inner }
    }
}
pub mod active_core_hr_cost_center;
pub mod batch_get_core_hr_company;
pub mod batch_get_core_hr_department;
pub mod batch_get_core_hr_employee;
pub mod batch_get_core_hr_job_data;
pub mod batch_get_core_hr_job_family;
pub mod batch_get_core_hr_job_level;
pub mod batch_get_core_hr_location;
pub mod batch_get_core_hrbp_by_employee;
pub mod create_core_hr_company;
pub mod create_core_hr_contract;
pub mod create_core_hr_cost_center;
pub mod create_core_hr_cost_center_version;
pub mod create_core_hr_department;
pub mod create_core_hr_employee_type;
pub mod create_core_hr_employment;
pub mod create_core_hr_job;
pub mod create_core_hr_job_change;
pub mod create_core_hr_job_data;
pub mod create_core_hr_job_family;
pub mod create_core_hr_job_level;
pub mod create_core_hr_leave_granting_record;
pub mod create_core_hr_location;
pub mod create_core_hr_national_id_type;
pub mod create_core_hr_offboarding;
pub mod create_core_hr_person;
pub mod create_core_hr_pre_hire;
pub mod create_core_hr_probation_assessment;
pub mod create_core_hr_working_hours_type;
pub mod delete_core_hr_company;
pub mod delete_core_hr_contract;
pub mod delete_core_hr_cost_center;
pub mod delete_core_hr_cost_center_version;
pub mod delete_core_hr_department;
pub mod delete_core_hr_employee_type;
pub mod delete_core_hr_employment;
pub mod delete_core_hr_job;
pub mod delete_core_hr_job_data;
pub mod delete_core_hr_job_family;
pub mod delete_core_hr_job_level;
pub mod delete_core_hr_leave_granting_record;
pub mod delete_core_hr_location;
pub mod delete_core_hr_national_id_type;
pub mod delete_core_hr_person;
pub mod delete_core_hr_pre_hire;
pub mod delete_core_hr_probation_assessment;
pub mod delete_core_hr_working_hours_type;
pub mod download_core_hr_person_file;
pub mod enable_disable_core_hr_probation_assessment;
pub mod get_core_hr_authorization;
pub mod get_core_hr_authorization_list;
pub mod get_core_hr_company;
pub mod get_core_hr_company_list;
pub mod get_core_hr_contract;
pub mod get_core_hr_contract_list;
pub mod get_core_hr_country_region;
pub mod get_core_hr_country_region_list;
pub mod get_core_hr_currency;
pub mod get_core_hr_currency_list;
pub mod get_core_hr_custom_field;
pub mod get_core_hr_custom_field_list;
pub mod get_core_hr_custom_field_object_api_name_list;
pub mod get_core_hr_department;
pub mod get_core_hr_department_list;
pub mod get_core_hr_department_parent_list;
pub mod get_core_hr_employee_type;
pub mod get_core_hr_employee_type_list;
pub mod get_core_hr_job;
pub mod get_core_hr_job_data;
pub mod get_core_hr_job_data_list;
pub mod get_core_hr_job_family;
pub mod get_core_hr_job_family_list;
pub mod get_core_hr_job_level;
pub mod get_core_hr_job_level_list;
pub mod get_core_hr_job_list;
pub mod get_core_hr_job_list_v2;
pub mod get_core_hr_job_v2;
pub mod get_core_hr_leave_balance_list;
pub mod get_core_hr_leave_request_history_list;
pub mod get_core_hr_leave_type_list;
pub mod get_core_hr_location;
pub mod get_core_hr_location_list;
pub mod get_core_hr_national_id_type;
pub mod get_core_hr_national_id_type_list;
pub mod get_core_hr_offboarding_list;
pub mod get_core_hr_person;
pub mod get_core_hr_pre_hire;
pub mod get_core_hr_pre_hire_list;
pub mod get_core_hr_process;
pub mod get_core_hr_process_form_variable_data;
pub mod get_core_hr_process_list;
pub mod get_core_hr_security_group_bp;
pub mod get_core_hr_security_group_list;
pub mod get_core_hr_subdivision;
pub mod get_core_hr_subdivision_list;
pub mod get_core_hr_subregion;
pub mod get_core_hr_subregion_list;
pub mod get_core_hr_transfer_reason_list;
pub mod get_core_hr_transfer_type_list;
pub mod get_core_hr_working_hours_type;
pub mod get_core_hr_working_hours_type_list;
pub mod get_core_hrbp_by_department;
pub mod get_core_hrbp_list;
pub mod match_core_hr_compensation_standard;
pub mod query_core_hr_job_data;
pub mod search_core_hr_assigned_user;
pub mod search_core_hr_bank;
pub mod search_core_hr_bank_branch;
pub mod search_core_hr_city;
pub mod search_core_hr_contract;
pub mod search_core_hr_cost_center;
pub mod search_core_hr_country_region;
pub mod search_core_hr_country_region_subdivision;
pub mod search_core_hr_currency;
pub mod search_core_hr_department;
pub mod search_core_hr_district;
pub mod search_core_hr_employee;
pub mod search_core_hr_job_change;
pub mod search_core_hr_nationality;
pub mod search_core_hr_offboarding;
pub mod search_core_hr_probation;
pub mod update_core_hr_company;
pub mod update_core_hr_contract;
pub mod update_core_hr_cost_center_version;
pub mod update_core_hr_department;
pub mod update_core_hr_employee_type;
pub mod update_core_hr_employment;
pub mod update_core_hr_job;
pub mod update_core_hr_job_data;
pub mod update_core_hr_job_family;
pub mod update_core_hr_job_level;
pub mod update_core_hr_national_id_type;
pub mod update_core_hr_person;
pub mod update_core_hr_pre_hire;
pub mod update_core_hr_probation_assessment;
pub mod update_core_hr_working_hours_type;
pub mod upload_core_hr_person_file;
