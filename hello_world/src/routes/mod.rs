mod always_errors;
mod custom_json_extractor;
mod get_json;
mod hello_world;
mod middleware_custom_message;
mod middleware_message;
mod mirror_body_json;
mod mirror_body_string;
mod mirror_user_agent;
mod mirror_user_header;
mod path_variables;
mod query_params;
mod returns_201;
mod set_middleware_custom_headers;
mod validate_with_serder;

use axum::{body::Body, http::Method, middleware, routing::{get, post}, Extension, Router};
use tower_http::cors::{Any, CorsLayer};

use always_errors::always_errors;
use custom_json_extractor::custom_json_extractor;
use get_json::get_json;
use hello_world::hello_world;
use middleware_custom_message::middleware_custom_message;
use middleware_message::middleware_message;
use mirror_body_string::mirror_body_string;
use mirror_body_json::mirror_body_json;
use mirror_user_agent::mirror_user_agent;
use mirror_user_header::mirror_user_headers;
use path_variables::{path_variable, hard_coded_path};
use query_params::query_param;
use returns_201::returns_201;
use set_middleware_custom_headers::set_middleware_custom_headers;
use validate_with_serder::validate_data;



#[derive(Clone)]
pub struct SharedData{
    pub message: String
}


pub fn create_routes() -> Router<Body>{
    let cors = CorsLayer::new()
            .allow_methods([Method::GET, Method::POST])
            .allow_origin(Any);
    let shared_data = SharedData{message: "Hello from shared data".to_owned()};

    Router::new()
                 .route("/middleware_custom_message", get(middleware_custom_message))
                 .route_layer(middleware::from_fn(set_middleware_custom_headers))
                 .route("/", get(hello_world))
                 .route("/mirror_body_string", post(mirror_body_string))
                 .route("/mirror_body_json", post(mirror_body_json))
                 .route("/mirror_user_agent", get(mirror_user_agent))
                 .route("/mirror_user_header", get(mirror_user_headers))
                 .route("/things/15", get(hard_coded_path))
                 .route("/things/:id", get(path_variable))
                 .route("/query_params", get(query_param))
                 .route("/middleware_message", get(middleware_message))
                 .layer(cors)
                 .layer(Extension(shared_data))
                 .route("/always_errors", get(always_errors))
                 .route("/returns_201", post(returns_201))
                 .route("/get_json", get(get_json))
                 .route("/validate_data", post(validate_data))
                 .route("/custom_json_extractor", post(custom_json_extractor))
}
