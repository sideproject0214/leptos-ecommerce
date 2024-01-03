use actix_web::{get, web, HttpResponse, Responder};

use crate::entities::post::model::Post;
use crate::AppState;
use serde::Deserialize;
use serde_json::json;

#[derive(Deserialize, Debug)]
pub struct FilterOptions {
	pub page: Option<i32>,
}

#[allow(unused)]
#[derive(Deserialize, Debug)]
pub struct ParamOption {
	pub pagination: i32,
	pub search: i32,
}

#[get("/pagination/{pagination}")]
pub async fn get_posts_pagination(
	page: web::Path<ParamOption>,
	data: web::Data<AppState>,
) -> impl Responder {
	println!("pagination: {:?}", page.pagination);

	const LIMIT: i64 = 6;
	let offset = page.pagination * 6;

	let query_result = sqlx::query_as!(
		Post,
		"select * from posts order by id limit $1 offset $2",
		LIMIT,
		offset as i32
	)
	.fetch_all(&data.db)
	.await;

	if query_result.is_err() {
		let message = "Something bad happened while fetching all note items";
		return HttpResponse::InternalServerError()
			.json(json!({"status": "error","message": message}));
	}

	let posts = query_result.unwrap();

	let json_response = serde_json::json!(posts);

	HttpResponse::Ok().json(json_response)
}

#[get("/search/{search}")]
pub async fn get_search_posts(
	page: web::Path<ParamOption>,
	data: web::Data<AppState>,
) -> impl Responder {
	println!("pagination: {:?}", page.search);

	const LIMIT: i64 = 6;
	let offset = page.search * 6;

	let query_result = sqlx::query_as!(
		Post,
		"select * from posts order by id limit $1 offset $2",
		LIMIT,
		offset as i32
	)
	.fetch_all(&data.db)
	.await;

	if query_result.is_err() {
		let message = "Something bad happened while fetching all note items";
		return HttpResponse::InternalServerError()
			.json(json!({"status": "error","message": message}));
	}

	let posts = query_result.unwrap();

	let json_response = serde_json::json!(posts);

	HttpResponse::Ok().json(json_response)
}
