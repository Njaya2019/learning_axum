use axum::http::StatusCode;
use chrono::{Duration, Utc};
use dotenvy_macro::dotenv;
use jsonwebtoken::{
    Algorithm, DecodingKey, EncodingKey, Header, Validation, decode, encode, errors,
};
use serde::{Deserialize, Serialize};

use super::app_errors::AppError;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    exp: usize,
    iat: usize,
}

pub fn create_jwt() -> Result<String, StatusCode> {
    let mut now = Utc::now();
    let iat = now.timestamp() as usize;

    let expires_in = Duration::seconds(30);
    now += expires_in;
    let exp = now.timestamp() as usize;
    let claim = Claims { exp: exp, iat: iat };
    let key: EncodingKey = EncodingKey::from_secret(dotenv!("JWT_SECRET").as_ref());
    encode(&Header::default(), &claim, &key).map_err(|_error| StatusCode::INTERNAL_SERVER_ERROR)
    // todo!()
}

pub fn is_valid(token: &str) -> Result<bool, AppError> {
    let secret: &'static str = dotenv!("JWT_SECRET");
    let key = DecodingKey::from_secret(secret.as_bytes());
    decode::<Claims>(token, &key, &Validation::new(Algorithm::HS256)).map_err(
        |error| match error.kind() {
            errors::ErrorKind::ExpiredSignature => AppError::new(
                StatusCode::UNAUTHORIZED,
                "Your session has expired, Please login again",
            ),
            _ => AppError::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                "Something went wrong please try again",
            ),
        },
    )?;
    Ok(true)
}
