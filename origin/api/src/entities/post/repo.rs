use actix_web::{
	delete, get, patch, post, web, HttpResponse, Responder,
};
use chrono::prelude::*;
use serde::{Deserialize, Serialize};
use serde_json::json;
use sqlx::{PgPool, Pool, Postgres};

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
	// conn: web::Data<PgPool>,
) -> HttpResponse {
	println!("pagination: {:?}", page.pagination);
	let query = format!("select * from users");
	// let result = sqlx::query(&query).fetch_all(&**conn).await;

	HttpResponse::Ok().body("show users 22")
}

// 강의 예제 따라하기
