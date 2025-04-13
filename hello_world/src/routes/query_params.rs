use axum::{extract::Query, Json};
use serde::{Deserialize, Serialize};



#[derive(Serialize, Deserialize)]
pub struct QueryParams{
    message: String,
    id: i32
}

pub async fn query_param(Query(query): Query<QueryParams>) -> Json<QueryParams>{
    Json(query)
}