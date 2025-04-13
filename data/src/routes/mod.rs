mod create_task;
mod create_user;
mod delete_task;
mod get_tasks;
mod guard;
mod hello_world;
mod partial_update_task;
mod update_tasks;

use axum::{
    Extension, Router,
    body::Body,
    middleware,
    routing::{delete, get, patch, post, put},
};
use create_task::create_task;
use create_user::{create_user, login, logout};
use delete_task::delete_task;
use get_tasks::{get_all_tasks, get_one_task};
use guard::guard;
use hello_world::hello_world;
use partial_update_task::partial_update;
use sea_orm::DatabaseConnection;
use update_tasks::atomic_update;

pub fn create_routes(database: DatabaseConnection) -> Router<Body> {
    Router::new()
        .route("/logout", post(logout))
        .route("/", get(hello_world))
        .route_layer(middleware::from_fn(guard))
        .route("/tasks", post(create_task))
        .route("/tasks", get(get_all_tasks))
        .route("/tasks/:id", get(get_one_task))
        .route("/tasks/:id", put(atomic_update))
        .route("/tasks/:id", patch(partial_update))
        .route("/tasks/:id", delete(delete_task))
        .route("/users", post(create_user))
        .route("/login", post(login))
        .layer(Extension(database))
}
