use crate::core::{Lark, LarkInner};

pub struct TaskService<'client, Store, Client> {
    #[allow(dead_code)]
    pub(crate) cli: &'client LarkInner<Store, Client>,
}

#[cfg(feature = "test-util")]
impl<Store, Client> TaskService<'_, Store, Client> {
    pub fn mock(&self) -> TaskServiceMocker<Store, Client> {
        TaskServiceMocker { cli: self.cli }
    }
}

#[cfg(feature = "test-util")]
pub struct TaskServiceMocker<'client, Store, Client> {
    #[allow(dead_code)]
    pub(crate) cli: &'client LarkInner<Store, Client>,
}

impl<Store, Client> Lark<Store, Client> {
    pub fn task(&self) -> TaskService<Store, Client> {
        TaskService { cli: &self.inner }
    }
}
pub mod batch_delete_task_collaborator;
pub mod batch_delete_task_follower;
pub mod complete_task;
pub mod create_task;
pub mod create_task_collaborator;
pub mod create_task_comment;
pub mod create_task_follower;
pub mod create_task_reminder;
pub mod delete_task;
pub mod delete_task_collaborator;
pub mod delete_task_comment;
pub mod delete_task_follower;
pub mod delete_task_reminder;
pub mod get_task;
pub mod get_task_collaborator_list;
pub mod get_task_comment;
pub mod get_task_comment_list;
pub mod get_task_follower_list;
pub mod get_task_list;
pub mod get_task_reminder_list;
pub mod uncomplete_task;
pub mod update_task;
pub mod update_task_comment;
