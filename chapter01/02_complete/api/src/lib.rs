pub mod entities {
	pub mod post {
		pub mod model;
		pub mod repo;
		pub mod routes;
	}
}

use actix_web::{
	guard, middleware::Logger, web, App, HttpResponse,
	HttpServer,
};
use dotenv::dotenv;
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};
use sqlx_pg_seeder::seeder;

#[allow(unused)]
pub async fn get_root() -> &'static str {
	"Hello World!!!"
}

pub struct AppState {
	pub db: Pool<Postgres>,
}

pub async fn run() -> std::io::Result<()> {
	if std::env::var_os("RUST_LOG").is_none() {
		std::env::set_var("RUST_LOG", "actix_web=info");
	}

	dotenv().ok();

	let database_url = std::env::var("DATABASE_URL")
		.expect("DATABASE_URL must be set");

	let api_address = std::env::var("API_ADDRESS")
		.expect("API_ADDRESS must be set");

	let pool = match PgPoolOptions::new()
		.max_connections(10)
		.connect(&database_url)
		.await
	{
		Ok(pool) => {
			println!(
				"✅ Connection to the database is successful!"
			);
			pool
		}
		Err(err) => {
			println!(
				"🔥 Failed to connect to the database: {:?}",
				err
			);
			std::process::exit(1);
		}
	};

	let _seeder = seeder(&pool).await;

	println!(
		"🚀 Actix server is running at http://{:?}",
		api_address
	);

	env_logger::init_from_env(
		env_logger::Env::new().default_filter_or("debug"),
	);

	let result = HttpServer::new(move || {
		App::new()
			.wrap(Logger::default())
			.app_data(web::Data::new(AppState {
				db: pool.clone(),
			}))
			.route("/api", web::get().to(get_root))
		
			.default_service(
				web::route()
					.guard(guard::Not(guard::Get()))
					.to(HttpResponse::MethodNotAllowed),
			)
	})
	.bind(api_address)?
	.run()
	.await;

	result
}
