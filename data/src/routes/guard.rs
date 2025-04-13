use crate::database::users::Entity as Users;
use crate::utils::app_errors::AppError;
use crate::{database::users, utils::jwt::is_valid};
use axum::{
    headers::{Authorization, HeaderMapExt, authorization::Bearer},
    http::{Request, StatusCode},
    middleware::Next,
    response::Response,
};
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};

pub async fn guard<T>(mut request: Request<T>, next: Next<T>) -> Result<Response, AppError> {
    let token = request
        .headers()
        .typed_get::<Authorization<Bearer>>()
        .ok_or_else(|| AppError::new(StatusCode::BAD_REQUEST, "Missing Bearer token"))?
        .token()
        .to_owned();
    let database = request
        .extensions()
        .get::<DatabaseConnection>()
        .ok_or_else(|| {
            AppError::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                "Hey something went wrong",
            )
        })?;

    let user = Users::find()
        .filter(users::Column::Token.eq(Some(token.clone())))
        .one(database)
        .await
        .map_err(|error| AppError::new(StatusCode::INTERNAL_SERVER_ERROR, error.to_string()))?;
    is_valid(&token)?;

    let Some(user) = user else {
        return Err(AppError::new(
            StatusCode::UNAUTHORIZED,
            "You are not authorized please login or create an account",
        ));
    };

    request.extensions_mut().insert(user);

    Ok(next.run(request).await)
}
