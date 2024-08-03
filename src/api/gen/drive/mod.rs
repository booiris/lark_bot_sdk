use crate::core::{ Lark, LarkInner};

pub struct DriveService<'client, Store, Client> {
    #[allow(dead_code)]
    pub(crate) cli: &'client LarkInner<Store, Client>,
}

#[cfg(feature = "test-util")]
impl<Store, Client> DriveService<'_, Store, Client> {
    pub fn mock(&self) -> DriveServiceMocker<Store, Client> {
        DriveServiceMocker { cli: self.cli }
    }
}

#[cfg(feature = "test-util")]
pub struct DriveServiceMocker<'client, Store, Client> {
    #[allow(dead_code)]
    pub(crate) cli: &'client LarkInner<Store, Client>,
}

impl<Store, Client> Lark<Store, Client> {
    pub fn drive(&self) -> DriveService<Store, Client> {
        DriveService { cli: &self.inner }
    }
}
pub mod get_drive_comment_list;
pub mod prepare_upload_drive_file;
pub mod get_drive_member_permission_list;
pub mod get_sheet_condition_format;
pub mod merge_sheet_cell;
pub mod download_drive_export_task;
pub mod move_drive_file;
pub mod batch_get_drive_comment;
pub mod get_drive_public_permission_v2;
pub mod get_drive_folder_children;
pub mod get_sheet_filter;
pub mod finish_upload_drive_file;
pub mod get_sheet_list;
pub mod check_drive_member_permission_old;
pub mod unmerge_sheet_cell;
pub mod delete_wiki_space_member;
pub mod set_sheet_style;
pub mod update_sheet_filter;
pub mod get_drive_public_permission;
pub mod delete_sheet_filter;
pub mod update_drive_doc_content;
pub mod get_docx_document;
pub mod create_sheet_protected_dimension;
pub mod update_sheet_float_image;
pub mod get_drive_file_statistics;
pub mod delete_drive_permission_public_password;
pub mod create_drive_import_task;
pub mod get_drive_file_task;
pub mod get_wiki_node_list;
pub mod create_drive_doc;
pub mod get_sheet_protected_dimension;
pub mod get_drive_folder_meta;
pub mod delete_drive_sheet_file;
pub mod create_drive_file;
pub mod update_sheet_filter_view_condition;
pub mod batch_delete_docx_block;
pub mod get_drive_doc_raw_content;
pub mod move_sheet_dimension;
pub mod update_drive_member_permission;
pub mod get_subscribe_drive_file;
pub mod set_sheet_value;
pub mod prepend_sheet_value;
pub mod search_drive_file;
pub mod get_sheet_filter_view_condition;
pub mod delete_sheet_dimension_range;
pub mod append_sheet_value;
pub mod get_drive_doc_meta;
pub mod delete_drive_file_version;
pub mod part_upload_drive_media;
pub mod part_upload_drive_file;
pub mod create_drive_member_permission_old;
pub mod query_sheet_filter_view;
pub mod prepare_upload_drive_media;
pub mod get_drive_doc_content;
pub mod update_docx_block;
pub mod create_sheet_filter_view;
pub mod get_drive_comment;
pub mod get_drive_member_permission_list_old;
pub mod set_sheet_value_image;
pub mod find_sheet;
pub mod get_drive_export_task;
pub mod batch_get_sheet_value;
pub mod create_drive_export_task;
pub mod update_sheet_data_validation_dropdown;
pub mod download_drive_media;
pub mod upload_drive_file;
pub mod get_drive_file_view_record_list;
pub mod batch_set_sheet_value;
pub mod subscribe_drive_file;
pub mod transfer_drive_member_permission;
pub mod get_wiki_node;
pub mod update_sheet_property;
pub mod get_drive_root_folder_meta;
pub mod get_drive_public_permission_old;
pub mod batch_update_sheet;
pub mod update_wiki_space_setting;
pub mod delete_sheet_protected_dimension;
pub mod unsubscribe_drive_file;
pub mod check_drive_member_permission;
pub mod delete_drive_comment;
pub mod copy_drive_file;
pub mod update_drive_public_permission;
pub mod update_spreadsheet;
pub mod create_drive_file_version;
pub mod finish_upload_drive_media;
pub mod update_sheet_dimension_range;
pub mod get_spreadsheet;
pub mod get_drive_file_version;
pub mod delete_sheet_data_validation_dropdown;
pub mod get_sheet;
pub mod get_drive_file_version_list;
pub mod create_sheet_filter_view_condition;
pub mod delete_drive_member_permission;
pub mod get_sheet_filter_view;
pub mod copy_wiki_node;
pub mod get_drive_import_task;
pub mod get_sheet_meta;
pub mod get_wiki_space;
pub mod get_sheet_float_image;
pub mod move_wiki_node;
pub mod create_sheet_data_validation_dropdown;
pub mod update_drive_comment;
pub mod update_drive_public_permission_v2;
pub mod get_sheet_data_validation_dropdown;
pub mod update_wiki_node_title;
pub mod update_drive_permission_public_password;
pub mod delete_drive_member_permission_old;
pub mod create_drive_file_subscription;
pub mod update_drive_file_subscription;
pub mod create_wiki_node;
pub mod update_sheet_filter_view;
pub mod get_sheet_value;
pub mod create_spreadsheet;
pub mod transfer_drive_owner_permission;
pub mod create_docx_block;
pub mod get_wiki_task;
pub mod create_sheet_float_image;
pub mod delete_sheet_float_image;
pub mod delete_drive_file;
pub mod upload_drive_media;
pub mod query_sheet_filter_view_condition;
pub mod get_drive_file_list;
pub mod get_drive_file_subscription;
pub mod get_wiki_space_list;
pub mod query_sheet_float_image;
pub mod create_drive_file_shortcut;
pub mod create_drive_folder;
pub mod get_docx_document_raw_content;
pub mod download_drive_file;
pub mod batch_get_drive_media_tmp_download_url;
pub mod get_drive_comment_reply_list;
pub mod update_drive_member_permission_old;
pub mod add_wiki_space_member;
pub mod update_sheet_condition_format;
pub mod get_docx_block_list_of_document;
pub mod update_sheet_protected_dimension;
pub mod move_docs_to_wiki;
pub mod get_docx_block;
pub mod create_sheet_filter;
pub mod delete_sheet_condition_format;
pub mod create_docx;
pub mod create_wiki_space;
pub mod delete_sheet_filter_view;
pub mod get_docx_block_list_of_block;
pub mod insert_sheet_dimension_range;
pub mod update_drive_comment_patch;
pub mod get_drive_file_meta;
pub mod create_sheet_condition_format;
pub mod replace_sheet;
pub mod create_drive_comment;
pub mod create_drive_member_permission;
pub mod create_drive_permission_public_password;
pub mod delete_sheet_filter_view_condition;
pub mod batch_set_sheet_style;
pub mod import_sheet;
pub mod add_sheet_dimension_range;
