use crate::{apis::configuration::Configuration, YougileError};
use std::sync::Arc;

/// A client for interacting with the YouGile API.
/// This provides a more idiomatic Rust interface compared to the raw API functions.
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
        credentials: crate::models::CredentialsWithCompany,
    ) -> Result<
        crate::models::AuthKeyDto,
        crate::apis::Error<crate::apis::auth::AuthKeyControllerCreateError>,
    > {
        crate::apis::auth::auth_key_controller_create(&self.configuration, credentials).await
    }

    pub async fn delete_auth_key(
        &self,
        key: &str,
    ) -> Result<(), crate::apis::Error<crate::apis::auth::AuthKeyControllerDeleteError>> {
        crate::apis::auth::auth_key_controller_delete(&self.configuration, key).await
    }

    pub async fn search_auth_keys(
        &self,
        credentials: crate::models::CredentialsWithCompanyOptional,
    ) -> Result<
        Vec<crate::models::AuthKeyWithDetailsDto>,
        crate::apis::Error<crate::apis::auth::AuthKeyControllerSearchError>,
    > {
        crate::apis::auth::auth_key_controller_search(&self.configuration, credentials).await
    }

    pub async fn get_company(
        &self,
    ) -> Result<
        crate::models::Company,
        crate::apis::Error<crate::apis::auth::CompanyControllerGetError>,
    > {
        crate::apis::auth::company_controller_get(&self.configuration).await
    }

    pub async fn update_company(
        &self,
        update: crate::models::UpdateCompany,
    ) -> Result<
        crate::models::Id,
        crate::apis::Error<crate::apis::auth::CompanyControllerUpdateError>,
    > {
        crate::apis::auth::company_controller_update(&self.configuration, update).await
    }

    pub async fn get_companies(
        &self,
        credentials: crate::models::CredentialsWithName,
        limit: Option<f64>,
        offset: Option<f64>,
    ) -> Result<crate::models::CompanyList, crate::apis::Error<crate::apis::auth::GetCompaniesError>>
    {
        crate::apis::auth::get_companies(&self.configuration, credentials, limit, offset).await
    }

    // Board methods
    pub async fn create_board(
        &self,
        create: crate::models::CreateBoard,
    ) -> Result<crate::models::Id, YougileError> {
        crate::apis::boards::create_board(&self.configuration, create).await
    }

    pub async fn get_board(
        &self,
        id: &str,
    ) -> Result<
        crate::models::Board,
        crate::apis::Error<crate::apis::boards::BoardControllerGetError>,
    > {
        crate::apis::boards::get_board(&self.configuration, id).await
    }

    pub async fn search_boards(
        &self,
        include_deleted: Option<bool>,
        limit: Option<f64>,
        offset: Option<f64>,
        title: Option<&str>,
        project_id: Option<&str>,
    ) -> Result<
        crate::models::BoardList,
        crate::apis::Error<crate::apis::boards::BoardControllerSearchError>,
    > {
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

    pub async fn update_board(
        &self,
        id: &str,
        update: crate::models::UpdateBoard,
    ) -> Result<
        crate::models::Id,
        crate::apis::Error<crate::apis::boards::BoardControllerUpdateError>,
    > {
        crate::apis::boards::update_board(&self.configuration, id, update).await
    }

    // Column methods
    pub async fn create_column(
        &self,
        create: crate::models::CreateColumn,
    ) -> Result<
        crate::models::Id,
        crate::apis::Error<crate::apis::columns::ColumnControllerCreateError>,
    > {
        crate::apis::columns::create_column(&self.configuration, create).await
    }

    pub async fn get_column(
        &self,
        id: &str,
    ) -> Result<
        crate::models::Column,
        crate::apis::Error<crate::apis::columns::ColumnControllerGetError>,
    > {
        crate::apis::columns::get_column(&self.configuration, id).await
    }

    pub async fn search_columns(
        &self,
        include_deleted: Option<bool>,
        limit: Option<f64>,
        offset: Option<f64>,
        title: Option<&str>,
        board_id: Option<&str>,
    ) -> Result<
        crate::models::ColumnList,
        crate::apis::Error<crate::apis::columns::ColumnControllerSearchError>,
    > {
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

    pub async fn update_column(
        &self,
        id: &str,
        update: crate::models::UpdateColumn,
    ) -> Result<
        crate::models::Id,
        crate::apis::Error<crate::apis::columns::ColumnControllerUpdateError>,
    > {
        crate::apis::columns::update_column(&self.configuration, id, update).await
    }

    // Task methods
    pub async fn create_task(
        &self,
        create: crate::models::CreateTask,
    ) -> Result<crate::models::Id, crate::apis::Error<crate::apis::tasks::TaskControllerCreateError>>
    {
        crate::apis::tasks::task_controller_create(&self.configuration, create).await
    }

    pub async fn get_task(
        &self,
        id: &str,
    ) -> Result<crate::models::Task, crate::apis::Error<crate::apis::tasks::TaskControllerGetError>>
    {
        crate::apis::tasks::task_controller_get(&self.configuration, id).await
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
    ) -> Result<
        crate::models::TaskListDto,
        crate::apis::Error<crate::apis::tasks::TaskControllerSearchError>,
    > {
        crate::apis::tasks::task_controller_search(
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

    pub async fn update_task(
        &self,
        id: &str,
        update: crate::models::UpdateTask,
    ) -> Result<crate::models::Id, crate::apis::Error<crate::apis::tasks::TaskControllerUpdateError>>
    {
        crate::apis::tasks::task_controller_update(&self.configuration, id, update).await
    }

    // User methods
    pub async fn create_user(
        &self,
        create: crate::models::CreateUserDto,
    ) -> Result<crate::models::Id, crate::apis::Error<crate::apis::users::UserControllerCreateError>>
    {
        crate::apis::users::user_controller_create(&self.configuration, create).await
    }

    pub async fn get_user(
        &self,
        id: &str,
    ) -> Result<
        crate::models::UserDto,
        crate::apis::Error<crate::apis::users::UserControllerGetError>,
    > {
        crate::apis::users::user_controller_get(&self.configuration, id).await
    }

    pub async fn search_users(
        &self,
        limit: Option<f64>,
        offset: Option<f64>,
        email: Option<&str>,
        project_id: Option<&str>,
    ) -> Result<
        crate::models::UserListDto,
        crate::apis::Error<crate::apis::users::UserControllerSearchError>,
    > {
        crate::apis::users::user_controller_search(
            &self.configuration,
            limit,
            offset,
            email,
            project_id,
        )
        .await
    }

    pub async fn update_user(
        &self,
        id: &str,
        update: crate::models::UpdateUserDto,
    ) -> Result<crate::models::Id, crate::apis::Error<crate::apis::users::UserControllerUpdateError>>
    {
        crate::apis::users::user_controller_update(&self.configuration, id, update).await
    }

    // Project methods
    pub async fn create_project(
        &self,
        create: crate::models::CreateProject,
    ) -> Result<
        crate::models::Id,
        crate::apis::Error<crate::apis::projects::ProjectControllerCreateError>,
    > {
        crate::apis::projects::project_controller_create(&self.configuration, create).await
    }

    pub async fn get_project(
        &self,
        id: &str,
    ) -> Result<
        crate::models::Project,
        crate::apis::Error<crate::apis::projects::ProjectControllerGetError>,
    > {
        crate::apis::projects::project_controller_get(&self.configuration, id).await
    }

    pub async fn search_projects(
        &self,
        include_deleted: Option<bool>,
        limit: Option<f64>,
        offset: Option<f64>,
        title: Option<&str>,
    ) -> Result<
        crate::models::ProjectList,
        crate::apis::Error<crate::apis::projects::ProjectControllerSearchError>,
    > {
        crate::apis::projects::project_controller_search(
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
        update: crate::models::UpdateProject,
    ) -> Result<
        crate::models::Id,
        crate::apis::Error<crate::apis::projects::ProjectControllerUpdateError>,
    > {
        crate::apis::projects::project_controller_update(&self.configuration, id, update).await
    }
}
