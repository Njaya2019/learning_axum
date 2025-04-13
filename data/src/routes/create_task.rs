use crate::database::users;
use crate::database::users::Entity as Users;
use axum::{
    Extension, Json, TypedHeader,
    headers::{Authorization, authorization::Bearer},
    http::StatusCode,
};
use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, Set};
use serde::Deserialize;

use crate::database::tasks;

#[derive(Deserialize)]
pub struct RequestTask {
    title: String,
    description: Option<String>,
    priority: Option<String>,
}

pub async fn create_task(
    Extension(database): Extension<DatabaseConnection>,
    Json(data): Json<RequestTask>,
    authorization: TypedHeader<Authorization<Bearer>>,
) -> Result<(), StatusCode> {
    let token = authorization.token();

    let user = if let Some(user) = Users::find()
        .filter(users::Column::Token.eq(Some(token)))
        .one(&database)
        .await
        .map_err(|_error| StatusCode::INTERNAL_SERVER_ERROR)?
    {
        user
    } else {
        return Err(StatusCode::UNAUTHORIZED);
    };

    let new_task = tasks::ActiveModel {
        title: Set(data.title),
        priority: Set(data.priority),
        description: Set(data.description),
        user_id: Set(Some(user.id)),
        ..Default::default()
    };

    let _result = new_task.save(&database).await.unwrap();

    Ok(())
}
