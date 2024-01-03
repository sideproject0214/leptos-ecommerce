use super::model::Post;
use actix_web::{get, web, HttpResponse, Responder};

use crate::AppState;
use serde::Deserialize;
use serde_json::json;

#[derive(Deserialize, Debug)]
pub struct PaginationParam {
	pub pagination: Option<i32>,
}

#[get("/pagination/{pagination}")]
pub async fn get_posts_pagination(
	param: web::Path<PaginationParam>,
	data: web::Data<AppState>,
) -> impl Responder {
	println!("pagination: {:?}", param.pagination.unwrap());

	const LIMIT: i64 = 6;
	let offset = param.pagination.unwrap_or(0) * 6;

	// let query_result =
	// 	sqlx::query_as!(Post, "select * from posts")
	// 		.fetch_all(&data.db)
	// 		.await;
	let query_result = sqlx::query_as!(
		Post,
		"select * from posts order by id limit $1 offset $2",
		LIMIT,
		offset as i32
	)
	.fetch_all(&data.db)
	.await;

	if let Err(_) = query_result {
		let message = "Something bad happened while fetching all note items";
		return HttpResponse::InternalServerError()
			.json(json!({"status": "error","message": message}));
	}

	let posts = query_result.unwrap();

	let json_response = serde_json::json!(posts);

	HttpResponse::Ok().json(json_response)
}

#[derive(Deserialize, Debug)]
pub struct SearchParam {
	pub search: String,
}

#[get("/search/{search}")]
pub async fn get_search_posts(
	param: web::Path<SearchParam>,
	data: web::Data<AppState>,
) -> impl Responder {
	let search_value = format!("%{}%", param.search);

	let query_result = sqlx::query_as!(
		Post,
		"SELECT * FROM posts WHERE title ILIKE $1 OR brand ILIKE $1 OR description ILIKE $1 OR category ILIKE $1",
		search_value
)
.fetch_all(&data.db)
.await;

	if let Err(_) = query_result {
		let message = "Something bad happened while fetching all note items";
		return HttpResponse::InternalServerError().json(
			json!({
					"status": "error",
					"message": message
			}),
		);
	}

	let posts = query_result.unwrap();
	let json_response = serde_json::json!(posts);

	HttpResponse::Ok().json(json_response)

	// HttpResponse::Ok().body(response)
}
