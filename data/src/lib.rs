mod database;
mod routes;
mod utils;

use routes::create_routes;
use sea_orm::Database;

pub async fn run(database_uri: String) {
    let database = Database::connect(database_uri).await.unwrap();

    let app = create_routes(database);
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap()
}
