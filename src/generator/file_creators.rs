use crate::error::{CliError, Result};
use super::file_ops::{file_exists, create_boiled_crab_file};
use super::module_updaters::*;

pub fn create_dto_file(name: &str, title: &str) -> Result<()> {
    let path = format!("src/application/dtos/{}.rs", name);
    
    if file_exists(&path)? {
        return Err(CliError::FileAlreadyExists(format!(
            "File already exists: {}",
            path
        )));
    }
    
    let content = format!(
        r#"use serde::{{Deserialize, Serialize}};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct {}Request {{
    // TODO: Add request fields
}}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct {}Response {{
    pub id: Uuid,
    pub created_at: String,
    pub updated_at: String,
    // TODO: Add response fields
}}
"#,
        title, title
    );

    create_boiled_crab_file(&path, &content)?;
    update_application_dtos_mod(name)?;
    Ok(())
}

pub fn create_service_file(name: &str, title: &str) -> Result<()> {
    let path = format!("src/application/services/{}_service.rs", name);
    
    if file_exists(&path)? {
        return Err(CliError::FileAlreadyExists(format!(
            "File already exists: {}",
            path
        )));
    }
    
    let content = format!(
        r#"use crate::application::dtos::{{{title}Request, {title}Response}};
use crate::domain::repositories::{title}Repository;
use crate::domain::DomainError;
use std::sync::Arc;

pub struct {title}Service {{
    repository: Arc<dyn {title}Repository>,
}}

impl {title}Service {{
    pub fn new(repository: Arc<dyn {title}Repository>) -> Self {{
        Self {{ repository }}
    }}

    pub async fn create(&self, req: {title}Request) -> Result<{title}Response, DomainError> {{
        // TODO: Implement create logic
        todo!()
    }}

    pub async fn get_by_id(&self, id: uuid::Uuid) -> Result<{title}Response, DomainError> {{
        // TODO: Implement get logic
        todo!()
    }}

    pub async fn list(&self) -> Result<Vec<{title}Response>, DomainError> {{
        // TODO: Implement list logic
        todo!()
    }}

    pub async fn update(&self, id: uuid::Uuid, req: {title}Request) -> Result<{title}Response, DomainError> {{
        // TODO: Implement update logic
        todo!()
    }}

    pub async fn delete(&self, id: uuid::Uuid) -> Result<(), DomainError> {{
        // TODO: Implement delete logic
        todo!()
    }}
}}
"#,
        title = title
    );

    create_boiled_crab_file(&path, &content)?;
    update_application_services_mod(name)?;
    Ok(())
}

pub fn create_entity_file(name: &str, title: &str) -> Result<()> {
    let path = format!("src/domain/entities/{}.rs", name);
    
    if file_exists(&path)? {
        return Err(CliError::FileAlreadyExists(format!(
            "File already exists: {}",
            path
        )));
    }
    
    let content = format!(
        r#"use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct {title} {{
    pub id: Uuid,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
    // TODO: Add entity fields
}}

impl {title} {{
    pub fn new() -> Self {{
        Self {{
            id: Uuid::new_v4(),
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
        }}
    }}
}}
"#,
        title = title
    );

    create_boiled_crab_file(&path, &content)?;
    update_domain_entities_mod(name)?;
    Ok(())
}

pub fn create_repository_trait_file(name: &str, title: &str) -> Result<()> {
    let path = format!("src/domain/repositories/{}_repository.rs", name);
    
    if file_exists(&path)? {
        return Err(CliError::FileAlreadyExists(format!(
            "File already exists: {}",
            path
        )));
    }
    
    let content = format!(
        r#"use crate::domain::entities::{title};
use crate::domain::DomainError;
use async_trait::async_trait;
use uuid::Uuid;

#[async_trait]
pub trait {title}Repository: Send + Sync {{
    async fn create(&self, entity: {title}) -> Result<{title}, DomainError>;
    async fn get_by_id(&self, id: Uuid) -> Result<{title}, DomainError>;
    async fn list(&self) -> Result<Vec<{title}>, DomainError>;
    async fn update(&self, id: Uuid, entity: {title}) -> Result<{title}, DomainError>;
    async fn delete(&self, id: Uuid) -> Result<(), DomainError>;
}}
"#,
        title = title
    );

    create_boiled_crab_file(&path, &content)?;
    update_domain_repositories_mod(name)?;
    Ok(())
}

pub fn create_repository_impl_file(name: &str, title: &str) -> Result<()> {
    let path = format!("src/infrastructure/database/repository/{}.rs", name);
    
    if file_exists(&path)? {
        return Err(CliError::FileAlreadyExists(format!(
            "File already exists: {}",
            path
        )));
    }
    
    let content = format!(
        r#"use async_trait::async_trait;
use chrono::Utc;
use sea_orm::{{DatabaseConnection, EntityTrait, QueryFilter, ColumnTrait, ActiveModelTrait, IntoActiveModel, QuerySelect}};
use uuid::Uuid;

use crate::domain::entities::{0};
use crate::domain::repositories::{0}Repository;
use crate::domain::DomainError;
use super::super::model;

pub struct {0}RepositoryImpl {{
    db: DatabaseConnection,
}}

impl {0}RepositoryImpl {{
    pub fn new(db: DatabaseConnection) -> Self {{
        Self {{ db }}
    }}
}}

#[async_trait]
impl {0}Repository for {0}RepositoryImpl {{
    async fn create(&self, entity: {0}) -> Result<{0}, DomainError> {{
        // TODO: Implement database create
        todo!()
    }}

    async fn get_by_id(&self, id: Uuid) -> Result<{0}, DomainError> {{
        // TODO: Implement database get
        todo!()
    }}

    async fn list(&self) -> Result<Vec<{0}>, DomainError> {{
        // TODO: Implement database list
        todo!()
    }}

    async fn update(&self, id: Uuid, entity: {0}) -> Result<{0}, DomainError> {{
        // TODO: Implement database update
        todo!()
    }}

    async fn delete(&self, id: Uuid) -> Result<(), DomainError> {{
        // TODO: Implement database delete
        todo!()
    }}
}}
"#,
        title
    );

    create_boiled_crab_file(&path, &content)?;
    update_infrastructure_database_mod(name)?;
    Ok(())
}

pub fn create_database_model_file(name: &str, _title: &str) -> Result<()> {
    let path = format!("src/infrastructure/database/model/{}.rs", name);
    
    if file_exists(&path)? {
        return Err(CliError::FileAlreadyExists(format!(
            "File already exists: {}",
            path
        )));
    }
    
    let table_name = format!("{}s", name);
    let content = format!(
        r#"use sea_orm::entity::prelude::*;
use chrono::{{DateTime, Utc}};
use serde::{{Serialize, Deserialize}};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "{}")]
pub struct Model {{
    #[sea_orm(primary_key)]
    pub id: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    // TODO: Add other fields
}}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {{}}

impl ActiveModelBehavior for ActiveModel {{}}
"#,
        table_name
    );

    create_boiled_crab_file(&path, &content)?;
    Ok(())
}

pub fn create_handler_file(name: &str, title: &str) -> Result<()> {
    let path = format!("src/presentation/handlers/{}_handlers.rs", name);
    
    if file_exists(&path)? {
        return Err(CliError::FileAlreadyExists(format!(
            "File already exists: {}",
            path
        )));
    }
    
    let content = format!(
        r#"use crate::application::dtos::{{{title}Request, {title}Response}};
use crate::presentation::handlers::AppState;
use axum::{{
    extract::{{Path, State}},
    http::StatusCode,
    response::IntoResponse,
    routing::{{delete, get, post, put}},
    Json, Router,
}};
use uuid::Uuid;

pub fn {name}_routes() -> Router<AppState> {{
    Router::new()
        .route(
            "/",
            post(create_{name}).get(list_{name}),
        )
        .route(
            "/:id",
            get(get_{name})
                .put(update_{name})
                .delete(delete_{name}),
        )
}}

// TODO: Implement handlers with proper state and service injection
async fn create_{name}(
    State(_state): State<AppState>,
    Json(_payload): Json<{title}Request>,
) -> impl IntoResponse {{
    (StatusCode::CREATED, Json({title}Response {{
        id: Uuid::new_v4(),
        created_at: chrono::Utc::now().to_rfc3339(),
        updated_at: chrono::Utc::now().to_rfc3339(),
    }}))
}}

async fn get_{name}(
    State(_state): State<AppState>,
    Path(_id): Path<Uuid>,
) -> impl IntoResponse {{
    (
        StatusCode::OK,
        Json({title}Response {{
            id: Uuid::new_v4(),
            created_at: chrono::Utc::now().to_rfc3339(),
            updated_at: chrono::Utc::now().to_rfc3339(),
        }}),
    )
}}

async fn list_{name}(
    State(_state): State<AppState>,
) -> impl IntoResponse {{
    Json(Vec::<{title}Response>::new())
}}

async fn update_{name}(
    State(_state): State<AppState>,
    Path(_id): Path<Uuid>,
    Json(_payload): Json<{title}Request>,
) -> impl IntoResponse {{
    (
        StatusCode::OK,
        Json({title}Response {{
            id: Uuid::new_v4(),
            created_at: chrono::Utc::now().to_rfc3339(),
            updated_at: chrono::Utc::now().to_rfc3339(),
        }}),
    )
}}

async fn delete_{name}(
    State(_state): State<AppState>,
    Path(_id): Path<Uuid>,
) -> impl IntoResponse {{
    StatusCode::NO_CONTENT
}}
"#
    );

    create_boiled_crab_file(&path, &content)?;
    update_presentation_handlers_mod(name)?;
    Ok(())
}
