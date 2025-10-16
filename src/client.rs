use crate::{
    apis::configuration::Configuration,
    models::{
        self, AuthKey, AuthKeyWithDetails, Board, BoardList, ChatId, ChatMessage, ChatMessageList,
        Column, ColumnList, Company, CompanyList, CreateBoard, CreateChatMessage, CreateColumn,
        CreateGroupChat, CreateProject, CreateProjectRole, CreateSprintSticker,
        CreateSprintStickerState, CreateStringSticker, CreateStringStickerState, CreateTask,
        CreateUser, CreateWebhook, CredentialsWithCompany, CredentialsWithCompanyOptional,
        CredentialsWithName, FileUpload, GroupChat, GroupChatList, Id, Project, ProjectList,
        ProjectRole, ProjectRoleList, SprintStickerState, SprintStickerWithStates,
        SprintStickerWithStatesList, StickerStateId, StringStickerState, StringStickerWithStates,
        StringStickerWithStatesList, Task, TaskChatSubscribers, TaskList, UpdateBoard,
        UpdateChatMessage, UpdateColumn, UpdateCompany, UpdateGroupChat, UpdateProject,
        UpdateProjectRole, UpdateSprintSticker, UpdateSprintStickerState, UpdateStringSticker,
        UpdateStringStickerState, UpdateTask, UpdateUser, UpdateWebhook, User, UserList, Webhook,
    },
    YougileError,
};
use std::sync::Arc;

/// A client for interacting with the YouGile API.
/// This provides a more idiomatic Rust interface compared to the raw API functions.
#[derive(Clone)]
pub struct YouGileClient {
    configuration: Arc<Configuration>,
}

impl YouGileClient {
    /// Creates a new YouGile API client with the given configuration.
    pub fn new(configuration: Configuration) -> Self {
        Self {
            configuration: Arc::new(configuration),
        }
    }

    /// Returns a reference to the internal configuration.
    pub fn configuration(&self) -> &Configuration {
        &self.configuration
    }

    // Authentication methods
    pub async fn create_auth_key(
        &self,
        credentials: CredentialsWithCompany,
    ) -> Result<AuthKey, YougileError> {
        crate::apis::auth::create_auth_key(&self.configuration, credentials).await
    }

    pub async fn delete_auth_key(&self, key: &str) -> Result<(), YougileError> {
        crate::apis::auth::delete_auth_key(&self.configuration, key).await
    }

    pub async fn search_auth_keys(
        &self,
        credentials: CredentialsWithCompanyOptional,
    ) -> Result<Vec<AuthKeyWithDetails>, YougileError> {
        crate::apis::auth::search_auth_keys(&self.configuration, credentials).await
    }

    pub async fn get_company(&self) -> Result<Company, YougileError> {
        crate::apis::auth::get_company(&self.configuration).await
    }

    pub async fn update_company(&self, update: UpdateCompany) -> Result<Id, YougileError> {
        crate::apis::auth::update_company(&self.configuration, update).await
    }

    pub async fn get_companies(
        &self,
        credentials: CredentialsWithName,
        limit: Option<f64>,
        offset: Option<f64>,
    ) -> Result<CompanyList, YougileError> {
        crate::apis::auth::get_companies(&self.configuration, credentials, limit, offset).await
    }

    // Board methods
    pub async fn create_board(&self, create: CreateBoard) -> Result<Id, YougileError> {
        crate::apis::boards::create_board(&self.configuration, create).await
    }

    pub async fn get_board(&self, id: &str) -> Result<Board, YougileError> {
        crate::apis::boards::get_board(&self.configuration, id).await
    }

    pub async fn search_boards(
        &self,
        include_deleted: Option<bool>,
        limit: Option<f64>,
        offset: Option<f64>,
        title: Option<&str>,
        project_id: Option<&str>,
    ) -> Result<BoardList, YougileError> {
        crate::apis::boards::search_board(
            &self.configuration,
            include_deleted,
            limit,
            offset,
            title,
            project_id,
        )
        .await
    }

    pub async fn update_board(&self, id: &str, update: UpdateBoard) -> Result<Id, YougileError> {
        crate::apis::boards::update_board(&self.configuration, id, update).await
    }

    // Column methods
    pub async fn create_column(&self, create: CreateColumn) -> Result<Id, YougileError> {
        crate::apis::columns::create_column(&self.configuration, create).await
    }

    pub async fn get_column(&self, id: &str) -> Result<Column, YougileError> {
        crate::apis::columns::get_column(&self.configuration, id).await
    }

    pub async fn search_columns(
        &self,
        include_deleted: Option<bool>,
        limit: Option<f64>,
        offset: Option<f64>,
        title: Option<&str>,
        board_id: Option<&str>,
    ) -> Result<ColumnList, YougileError> {
        crate::apis::columns::search_column(
            &self.configuration,
            include_deleted,
            limit,
            offset,
            title,
            board_id,
        )
        .await
    }

    pub async fn update_column(&self, id: &str, update: UpdateColumn) -> Result<Id, YougileError> {
        crate::apis::columns::update_column(&self.configuration, id, update).await
    }

    // Task methods
    pub async fn create_task(&self, create: CreateTask) -> Result<Id, YougileError> {
        crate::apis::tasks::create_task(&self.configuration, create).await
    }

    pub async fn get_task(&self, id: &str) -> Result<Task, YougileError> {
        crate::apis::tasks::get_task(&self.configuration, id).await
    }

    pub async fn get_task_chat_subscribers(&self, id: &str) -> Result<Vec<String>, YougileError> {
        crate::apis::tasks::get_task_chat_subscribers(&self.configuration, id).await
    }

    pub async fn search_tasks(
        &self,
        include_deleted: Option<bool>,
        limit: Option<f64>,
        offset: Option<f64>,
        title: Option<&str>,
        column_id: Option<&str>,
        assigned_to: Option<&str>,
        sticker_id: Option<&str>,
        sticker_state_id: Option<&str>,
    ) -> Result<TaskList, YougileError> {
        crate::apis::tasks::search_task(
            &self.configuration,
            include_deleted,
            limit,
            offset,
            title,
            column_id,
            assigned_to,
            sticker_id,
            sticker_state_id,
        )
        .await
    }

    pub async fn search_tasks_reversed(
        &self,
        include_deleted: Option<bool>,
        limit: Option<f64>,
        offset: Option<f64>,
        title: Option<&str>,
        column_id: Option<&str>,
        assigned_to: Option<&str>,
        sticker_id: Option<&str>,
        sticker_state_id: Option<&str>,
    ) -> Result<TaskList, YougileError> {
        crate::apis::tasks::search_task_reversed(
            &self.configuration,
            include_deleted,
            limit,
            offset,
            title,
            column_id,
            assigned_to,
            sticker_id,
            sticker_state_id,
        )
        .await
    }

    pub async fn update_task(&self, id: &str, update: UpdateTask) -> Result<Id, YougileError> {
        crate::apis::tasks::update_task(&self.configuration, id, update).await
    }

    pub async fn update_task_chat_subscribers(
        &self,
        id: &str,
        task_chat_subscribers: TaskChatSubscribers,
    ) -> Result<Id, YougileError> {
        crate::apis::tasks::update_task_chat_subscribers(
            &self.configuration,
            id,
            task_chat_subscribers,
        )
        .await
    }

    // User methods
    pub async fn create_user(&self, create: CreateUser) -> Result<Id, YougileError> {
        crate::apis::users::create_user(&self.configuration, create).await
    }

    pub async fn delete_user(&self, id: &str) -> Result<Id, YougileError> {
        crate::apis::users::delete_user(&self.configuration, id).await
    }

    pub async fn get_user(&self, id: &str) -> Result<User, YougileError> {
        crate::apis::users::get_user(&self.configuration, id).await
    }

    pub async fn search_users(
        &self,
        limit: Option<f64>,
        offset: Option<f64>,
        email: Option<&str>,
        project_id: Option<&str>,
    ) -> Result<UserList, YougileError> {
        crate::apis::users::search_user(&self.configuration, limit, offset, email, project_id).await
    }

    pub async fn update_user(&self, id: &str, update: UpdateUser) -> Result<Id, YougileError> {
        crate::apis::users::update_user(&self.configuration, id, update).await
    }

    // Project methods
    pub async fn create_project(&self, create: CreateProject) -> Result<Id, YougileError> {
        crate::apis::projects::create_project(&self.configuration, create).await
    }

    pub async fn get_project(&self, id: &str) -> Result<Project, YougileError> {
        crate::apis::projects::get_project(&self.configuration, id).await
    }

    pub async fn search_projects(
        &self,
        include_deleted: Option<bool>,
        limit: Option<f64>,
        offset: Option<f64>,
        title: Option<&str>,
    ) -> Result<ProjectList, YougileError> {
        crate::apis::projects::search_project(
            &self.configuration,
            include_deleted,
            limit,
            offset,
            title,
        )
        .await
    }

    pub async fn update_project(
        &self,
        id: &str,
        update: UpdateProject,
    ) -> Result<Id, YougileError> {
        crate::apis::projects::update_project(&self.configuration, id, update).await
    }

    // Project roles methods
    pub async fn create_project_role(
        &self,
        project_id: &str,
        create_project_role: CreateProjectRole,
    ) -> Result<Id, YougileError> {
        crate::apis::projects::create_project_role(
            &self.configuration,
            project_id,
            create_project_role,
        )
        .await
    }

    pub async fn delete_project_role(
        &self,
        project_id: &str,
        id: &str,
    ) -> Result<ProjectRole, YougileError> {
        crate::apis::projects::delete_project_role(&self.configuration, project_id, id).await
    }

    pub async fn get_project_role(
        &self,
        project_id: &str,
        id: &str,
    ) -> Result<ProjectRole, YougileError> {
        crate::apis::projects::get_project_role(&self.configuration, project_id, id).await
    }

    pub async fn search_project_roles(
        &self,
        project_id: &str,
        limit: Option<f64>,
        offset: Option<f64>,
        name: Option<&str>,
    ) -> Result<ProjectRoleList, YougileError> {
        crate::apis::projects::search_project_roles(
            &self.configuration,
            project_id,
            limit,
            offset,
            name,
        )
        .await
    }

    pub async fn update_project_role(
        &self,
        project_id: &str,
        id: &str,
        update_project_role: UpdateProjectRole,
    ) -> Result<Id, YougileError> {
        crate::apis::projects::update_project_role(
            &self.configuration,
            project_id,
            id,
            update_project_role,
        )
        .await
    }

    // Chat methods
    pub async fn send_chat_message(
        &self,
        chat_id: &str,
        create_chat_message: CreateChatMessage,
    ) -> Result<ChatId, YougileError> {
        crate::apis::chats::send_chat_message(&self.configuration, chat_id, create_chat_message)
            .await
    }

    pub async fn get_chat_message(
        &self,
        chat_id: &str,
        id: f64,
    ) -> Result<ChatMessage, YougileError> {
        crate::apis::chats::get_chat_message(&self.configuration, chat_id, id).await
    }

    pub async fn search_chat_messages(
        &self,
        chat_id: &str,
        include_deleted: Option<bool>,
        limit: Option<f64>,
        offset: Option<f64>,
        from_user_id: Option<&str>,
        text: Option<&str>,
        label: Option<&str>,
        since: Option<f64>,
        include_system: Option<bool>,
    ) -> Result<ChatMessageList, YougileError> {
        crate::apis::chats::search_chat_messages(
            &self.configuration,
            chat_id,
            include_deleted,
            limit,
            offset,
            from_user_id,
            text,
            label,
            since,
            include_system,
        )
        .await
    }

    pub async fn update_chat_message(
        &self,
        chat_id: &str,
        id: f64,
        update_chat_message: UpdateChatMessage,
    ) -> Result<ChatId, YougileError> {
        crate::apis::chats::update_chat_message(
            &self.configuration,
            chat_id,
            id,
            update_chat_message,
        )
        .await
    }

    // Group chat methods
    pub async fn create_group_chat(
        &self,
        create_group_chat: CreateGroupChat,
    ) -> Result<Id, YougileError> {
        crate::apis::group_chats::create_group_chat(&self.configuration, create_group_chat).await
    }

    pub async fn get_group_chat(&self, id: &str) -> Result<GroupChat, YougileError> {
        crate::apis::group_chats::get_group_chat(&self.configuration, id).await
    }

    pub async fn search_group_chat(
        &self,
        include_deleted: Option<bool>,
        limit: Option<f64>,
        offset: Option<f64>,
        title: Option<&str>,
    ) -> Result<GroupChatList, YougileError> {
        crate::apis::group_chats::search_group_chat(
            &self.configuration,
            include_deleted,
            limit,
            offset,
            title,
        )
        .await
    }

    pub async fn update_group_chat(
        &self,
        id: &str,
        update_group_chat: UpdateGroupChat,
    ) -> Result<Id, YougileError> {
        crate::apis::group_chats::update_group_chat(&self.configuration, id, update_group_chat)
            .await
    }

    // File methods
    pub async fn upload_file(
        &self,
        file_data: Vec<u8>,
        file_name: &str,
    ) -> Result<FileUpload, YougileError> {
        crate::apis::files::upload_file(&self.configuration, file_data, file_name).await
    }

    // Sticker methods (sprint stickers)
    pub async fn create_sprint_sticker(
        &self,
        create_sprint_sticker: CreateSprintSticker,
    ) -> Result<Id, YougileError> {
        crate::apis::stickers::create_sprint_sticker(&self.configuration, create_sprint_sticker)
            .await
    }

    pub async fn get_sprint_sticker(
        &self,
        id: &str,
    ) -> Result<SprintStickerWithStates, YougileError> {
        crate::apis::stickers::get_sprint_sticker(&self.configuration, id).await
    }

    pub async fn search_sprint_stickers(
        &self,
        include_deleted: Option<bool>,
        limit: Option<f64>,
        offset: Option<f64>,
        name: Option<&str>,
        board_id: Option<&str>,
    ) -> Result<SprintStickerWithStatesList, YougileError> {
        crate::apis::stickers::search_sprint_sticker(
            &self.configuration,
            include_deleted,
            limit,
            offset,
            name,
            board_id,
        )
        .await
    }

    pub async fn update_sprint_sticker(
        &self,
        id: &str,
        update_sprint_sticker: UpdateSprintSticker,
    ) -> Result<Id, YougileError> {
        crate::apis::stickers::update_sprint_sticker(&self.configuration, id, update_sprint_sticker)
            .await
    }

    // Sprint sticker states methods
    pub async fn create_sprint_sticker_state(
        &self,
        sticker_id: &str,
        create_sprint_sticker_state: CreateSprintStickerState,
    ) -> Result<StickerStateId, YougileError> {
        crate::apis::stickers::create_sprint_sticker_state(
            &self.configuration,
            sticker_id,
            create_sprint_sticker_state,
        )
        .await
    }

    pub async fn get_sprint_sticker_state(
        &self,
        sticker_id: &str,
        sticker_state_id: &str,
        include_deleted: Option<bool>,
    ) -> Result<SprintStickerState, YougileError> {
        crate::apis::stickers::get_sprint_sticker_state(
            &self.configuration,
            sticker_id,
            sticker_state_id,
            include_deleted,
        )
        .await
    }

    pub async fn update_sprint_sticker_state(
        &self,
        sticker_id: &str,
        sticker_state_id: &str,
        update_sprint_sticker_state: UpdateSprintStickerState,
    ) -> Result<StickerStateId, YougileError> {
        crate::apis::stickers::update_sprint_sticker_state(
            &self.configuration,
            sticker_id,
            sticker_state_id,
            update_sprint_sticker_state,
        )
        .await
    }

    // String sticker methods
    pub async fn create_string_sticker(
        &self,
        create_string_sticker: CreateStringSticker,
    ) -> Result<Id, YougileError> {
        crate::apis::stickers::create_string_sticker(&self.configuration, create_string_sticker)
            .await
    }

    pub async fn get_string_sticker(
        &self,
        id: &str,
    ) -> Result<StringStickerWithStates, YougileError> {
        crate::apis::stickers::get_string_sticker(&self.configuration, id).await
    }

    pub async fn search_string_stickers(
        &self,
        include_deleted: Option<bool>,
        limit: Option<f64>,
        offset: Option<f64>,
        name: Option<&str>,
        board_id: Option<&str>,
    ) -> Result<StringStickerWithStatesList, YougileError> {
        crate::apis::stickers::search_string_sticker(
            &self.configuration,
            include_deleted,
            limit,
            offset,
            name,
            board_id,
        )
        .await
    }

    pub async fn update_string_sticker(
        &self,
        id: &str,
        update_string_sticker: UpdateStringSticker,
    ) -> Result<Id, YougileError> {
        crate::apis::stickers::update_string_sticker(&self.configuration, id, update_string_sticker)
            .await
    }

    // String sticker states methods
    pub async fn create_string_sticker_state(
        &self,
        sticker_id: &str,
        create_string_sticker_state: CreateStringStickerState,
    ) -> Result<StickerStateId, YougileError> {
        crate::apis::stickers::create_string_sticker_state(
            &self.configuration,
            sticker_id,
            create_string_sticker_state,
        )
        .await
    }

    pub async fn get_string_sticker_state(
        &self,
        sticker_id: &str,
        sticker_state_id: &str,
        include_deleted: Option<bool>,
    ) -> Result<StringStickerState, YougileError> {
        crate::apis::stickers::get_string_sticker_state(
            &self.configuration,
            sticker_id,
            sticker_state_id,
            include_deleted,
        )
        .await
    }

    pub async fn update_string_sticker_state(
        &self,
        sticker_id: &str,
        sticker_state_id: &str,
        update_string_sticker_state: UpdateStringStickerState,
    ) -> Result<StickerStateId, YougileError> {
        crate::apis::stickers::update_string_sticker_state(
            &self.configuration,
            sticker_id,
            sticker_state_id,
            update_string_sticker_state,
        )
        .await
    }

    // Webhook methods
    pub async fn create_webhook(&self, create_webhook: CreateWebhook) -> Result<Id, YougileError> {
        crate::apis::webhooks::create_webhook(&self.configuration, create_webhook).await
    }

    pub async fn update_webhook(
        &self,
        id: &str,
        update_webhook: UpdateWebhook,
    ) -> Result<Id, YougileError> {
        crate::apis::webhooks::update_webhook(&self.configuration, id, update_webhook).await
    }

    pub async fn search_webhooks(
        &self,
        include_deleted: Option<bool>,
    ) -> Result<Webhook, YougileError> {
        crate::apis::webhooks::search_webhooks(&self.configuration, include_deleted).await
    }
}
