use api::config::{EnvConfig, EnvValue};
use api::entities::index::{get_db_conn, DbRepo};
use axum::routing::get;
use axum::Router;

#[tokio::main]
async fn main() {
  let mut my_env_value = EnvValue::new();
  my_env_value.load_config();

  let _db_repo = DbRepo::init(&my_env_value).await;
  println!(
    "env_path: {:?}\n  \n api_address: {:?}",
    &my_env_value.env_path, &my_env_value.api_address
  );

  tracing_subscriber::fmt::init();
  let _pool = get_db_conn(&my_env_value);

  let app = Router::new().route("/api", get(root));

  tracing::debug!("Listening on {}", String::from(&my_env_value.api_address));

  println!(
    "🚀 Axum server is running at http://{:?}",
    &my_env_value.api_address
  );

  let listener = tokio::net::TcpListener::bind(&my_env_value.api_address)
    .await
    .unwrap();

  axum::serve(listener, app).await.unwrap();
}

// basic handler that responds with a static string
async fn root() -> &'static str {
  "🚀 Hello, Axum World!!!!!!!"
}
