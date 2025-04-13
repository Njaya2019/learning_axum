use crate::database::tasks::Entity as Tasks;
use axum::{Extension, Json, extract::Path, http::StatusCode};
use sea_orm::{
    ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, Set, prelude::DateTimeWithTimeZone,
};
use serde::{Deserialize, Serialize};

use crate::database::tasks;

#[derive(Serialize, Deserialize)]
pub struct ResponseTask {
    pub id: Option<i32>,
    pub priority: Option<String>,
    pub title: String,
    pub completed_at: Option<DateTimeWithTimeZone>,
    pub description: Option<String>,
    pub deleted_at: Option<DateTimeWithTimeZone>,
    pub user_id: Option<i32>,
    pub is_default: Option<bool>,
}

pub async fn atomic_update(
    Path(id): Path<i32>,
    Extension(database): Extension<DatabaseConnection>,
    Json(task): Json<ResponseTask>,
) -> Result<(), StatusCode> {
    let update_task = tasks::ActiveModel {
        id: Set(id),
        priority: Set(task.priority),
        title: Set(task.title),
        completed_at: Set(task.completed_at),
        description: Set(task.description),
        deleted_at: Set(task.deleted_at),
        user_id: Set(task.user_id),
        is_default: Set(task.is_default),
    };

    Tasks::update(update_task)
        .filter(tasks::Column::Id.eq(id))
        .exec(&database)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(())
}
