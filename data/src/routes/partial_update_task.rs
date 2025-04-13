use crate::database::tasks::Entity as Tasks;
use axum::{Extension, Json, extract::Path, http::StatusCode};
use sea_orm::{
    ColumnTrait, DatabaseConnection, EntityTrait, IntoActiveModel, QueryFilter, Set,
    prelude::DateTimeWithTimeZone,
};
use serde::{Deserialize, Serialize};

use crate::database::tasks;

#[derive(Serialize, Deserialize)]
pub struct ResponseTask {
    pub id: Option<i32>,
    #[serde(
        default,                                    // <- important for deserialization
        skip_serializing_if = "Option::is_none",    // <- important for serialization
        with = "::serde_with::rust::double_option",
    )]
    pub priority: Option<Option<String>>,
    pub title: Option<String>,
    #[serde(
        default,                                    // <- important for deserialization
        skip_serializing_if = "Option::is_none",    // <- important for serialization
        with = "::serde_with::rust::double_option",
    )]
    pub completed_at: Option<Option<DateTimeWithTimeZone>>,
    #[serde(
        default,                                    // <- important for deserialization
        skip_serializing_if = "Option::is_none",    // <- important for serialization
        with = "::serde_with::rust::double_option",
    )]
    pub description: Option<Option<String>>,
    #[serde(
        default,                                    // <- important for deserialization
        skip_serializing_if = "Option::is_none",    // <- important for serialization
        with = "::serde_with::rust::double_option",
    )]
    pub deleted_at: Option<Option<DateTimeWithTimeZone>>,
}

pub async fn partial_update(
    Path(id): Path<i32>,
    Extension(database): Extension<DatabaseConnection>,
    Json(task): Json<ResponseTask>,
) -> Result<(), StatusCode> {
    let mut db_task = if let Some(task) = Tasks::find_by_id(id)
        .one(&database)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
    {
        task.into_active_model()
    } else {
        return Err(StatusCode::NOT_FOUND);
    };

    if let Some(priority) = task.priority {
        db_task.priority = Set(priority);
    }

    if let Some(description) = task.description {
        db_task.description = Set(description);
    }

    if let Some(title) = task.title {
        db_task.title = Set(title);
    }

    if let Some(completed_at) = task.completed_at {
        db_task.completed_at = Set(completed_at);
    }

    if let Some(deleted_at) = task.deleted_at {
        db_task.deleted_at = Set(deleted_at);
    }

    Tasks::update(db_task)
        .filter(tasks::Column::Id.eq(id))
        .exec(&database)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(())
}
