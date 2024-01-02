pub mod entities {
	pub mod app_state;
	pub mod index;
	pub mod user {
		pub mod model;
		pub mod repo;
	}
	pub mod post {
		pub mod model;
		pub mod repo;
		pub mod routes;
	}
}
pub mod config;
pub mod seeders {
	pub mod sqlx_seeder;
}

use actix_web::{middleware::Logger, web, App, HttpServer};

use sqlx_pg_seeder::seeder;

use crate::{
	config::{EnvConfig, EnvValue},
	entities::{
		app_state::AppState,
		index::{get_db_conn, DbRepo},
		post::routes::post_routes,
	},
};

#[allow(unused)]
pub async fn get_root() -> &'static str {
	"Hello World!!!"
}

pub async fn run() -> std::io::Result<()> {
	let mut my_env_value = EnvValue::new();
	my_env_value.load_config();

	let db_repo = DbRepo::init(&my_env_value).await;
	let app_data = web::Data::new(AppState {
		client: reqwest::Client::new(),
		db_repo,
	});

	let pool = get_db_conn(&my_env_value).await;
	let _seeder = seeder(&pool).await;

	println!(
		"🚀 Actix server is running at http://{:?}",
		&my_env_value.api_address
	);

	env_logger::init_from_env(
		env_logger::Env::new().default_filter_or("info"),
	);

	let result = HttpServer::new(move || {
		App::new()
			.wrap(Logger::default())
			.app_data(app_data.clone())
			.route("/api", web::get().to(get_root))
			.service(
				web::scope("/api/post").service(post_routes()),
			)
	})
	.bind(&my_env_value.api_address)?
	.run()
	.await;

	result
}
