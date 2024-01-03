use actix_web::{get, web, HttpResponse};

use serde::Deserialize;

use crate::entities::post::model::Post;
use crate::AppState;

#[derive(Deserialize, Debug)]
pub struct FilterOptions {
	pub page: Option<usize>,
	pub limit: Option<usize>,
}

#[allow(unused)]
#[derive(Deserialize, Debug)]
pub struct ParamOption {
	pub pagination: i32,
}

#[get("/{pagination}")]
pub async fn get_posts_pagination(
	page: web::Path<ParamOption>,
	data: web::Data<AppState>,
) -> HttpResponse {
	println!("pagination: {:?}", page.pagination);
	let query_result =
		sqlx::query_as!(Post, "select * from posts")
			.fetch_all(&data.db)
			.await;
	// let result = sqlx::query(&query).fetch_all(&**conn).await;

	HttpResponse::Ok().body("show users 22")
}
