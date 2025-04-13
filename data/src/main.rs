use dotenvy::dotenv;
use dotenvy_macro::dotenv;
use data::run;

#[tokio::main]
async  fn main() {
    dotenv().ok();
    let database_uri = dotenv!("DATABASE_URL");
    run(database_uri.to_owned()).await;
}
