// use actix_web::{get, web, HttpResponse};
// use serde::{Deserialize, Serialize};
// use sqlx::{PgPool, Pool, Postgres};

// #[derive(Deserialize, Debug)]
// pub struct FilterOptions {
// 	pub page: Option<usize>,
// 	pub limit: Option<usize>,
// }

// #[allow(unused)]
// #[derive(Deserialize, Debug)]
// pub struct ParamOption {
// 	pub pagination: i32,
// }

// #[derive(Serialize, Deserialize, Debug)]
// pub struct CreatePostSchema {
// 	pub title: String,
// 	pub content: String,
// 	#[serde(skip_serializing_if = "Option::is_none")]
// 	pub category: Option<String>,
// 	#[serde(skip_serializing_if = "Option::is_none")]
// 	pub published: Option<bool>,
// }

// #[get("/{pagination}")]
// pub async fn get_posts_pagination(
// 	page: web::Path<FilterOption>,
// 	conn: web::Data<PgPool>,
// 	// conn: web::Data<PgPool>,
// ) -> HttpResponse {
// 	println!("pagination: {:?}", page.pagination);
// 	let query = format!("select * from users");
// 	let result = sqlx::query(&query).fetch_all(&**conn).await;

// 	HttpResponse::Ok().body("show users 22")
// }
