use crate::database::tasks::Entity as Tasks;
use axum::{
    Extension,
    extract::{Path, Query},
    http::StatusCode,
};
use sea_orm::{DatabaseConnection, EntityTrait, IntoActiveModel, Set};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct QueryParams {
    soft: bool,
}

pub async fn delete_task(
    Path(id): Path<i32>,
    Extension(database): Extension<DatabaseConnection>,
    Query(query_params): Query<QueryParams>,
) -> Result<(), StatusCode> {
    if query_params.soft {
        let mut task = if let Some(task) = Tasks::find_by_id(id)
            .one(&database)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        {
            task.into_active_model()
        } else {
            return Err(StatusCode::NOT_FOUND);
        };

        let now = chrono::Utc::now();
        task.deleted_at = Set(Some(now.into()));
        Tasks::update(task)
            .exec(&database)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    } else {
        let task = if let Some(task) = Tasks::find_by_id(id)
            .one(&database)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        {
            task.into_active_model()
        } else {
            return Err(StatusCode::NOT_FOUND);
        };

        Tasks::delete(task)
            .exec(&database)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    }

    Ok(())
}
