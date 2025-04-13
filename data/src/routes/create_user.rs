use crate::database::users;
use crate::database::users::{Entity as Users, Model};
use crate::utils::jwt::create_jwt;
use axum::{Extension, Json, http::StatusCode};
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, IntoActiveModel, QueryFilter,
    Set,
};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct RequestUser {
    username: String,
    password: String,
}

#[derive(Serialize, Deserialize)]
pub struct ResponseUser {
    username: String,
    id: i32,
    token: Option<String>,
}

pub async fn create_user(
    Json(data): Json<RequestUser>,
    Extension(database): Extension<DatabaseConnection>,
) -> Result<Json<ResponseUser>, StatusCode> {
    let jwt_token = create_jwt()?;
    let new_user = users::ActiveModel {
        username: Set(data.username),
        password: Set(hash_password(data.password)?),
        token: Set(Some(jwt_token)),
        ..Default::default()
    }
    .save(&database)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(ResponseUser {
        username: new_user.username.unwrap(),
        id: new_user.id.unwrap(),
        token: new_user.token.unwrap(),
    }))
}

pub async fn login(
    Json(data): Json<RequestUser>,
    Extension(database): Extension<DatabaseConnection>,
) -> Result<Json<ResponseUser>, StatusCode> {
    let db_user = Users::find()
        .filter(users::Column::Username.eq(data.username))
        .one(&database)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    if let Some(db_user) = db_user {
        if !verify_passowrd(data.password, &db_user.password)? {
            return Err(StatusCode::UNAUTHORIZED);
        }
        let new_token = create_jwt()?;
        let mut user = db_user.into_active_model();
        user.token = Set(Some(new_token));

        let saved_user = user
            .save(&database)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

        Ok(Json(ResponseUser {
            username: saved_user.username.unwrap(),
            id: saved_user.id.unwrap(),
            token: saved_user.token.unwrap(),
        }))
    } else {
        Err(StatusCode::NOT_FOUND)
    }
}

pub async fn logout(
    Extension(database): Extension<DatabaseConnection>,
    Extension(user): Extension<Model>,
) -> Result<(), StatusCode> {
    let mut user = user.into_active_model();
    user.token = Set(None);
    user.save(&database)
        .await
        .map_err(|_error| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(())
}

fn hash_password(password: String) -> Result<String, StatusCode> {
    bcrypt::hash(password, 14).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}

fn verify_passowrd(password: String, hash: &str) -> Result<bool, StatusCode> {
    bcrypt::verify(password, hash).map_err(|_error| StatusCode::INTERNAL_SERVER_ERROR)
}
