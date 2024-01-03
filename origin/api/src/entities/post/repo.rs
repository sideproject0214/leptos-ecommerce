use actix_web::{get, web, HttpResponse, Responder};

use crate::entities::post::model::Post;
use crate::AppState;
use serde::Deserialize;
use serde_json::json;

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
) -> impl Responder {
	println!("pagination: {:?}", page.pagination);
	let query_result =
		sqlx::query_as!(Post, "select * from posts")
			.fetch_all(&data.db)
			.await;

	if query_result.is_err() {
		let message = "Something bad happened while fetching all note items";
		return HttpResponse::InternalServerError()
			.json(json!({"status": "error","message": message}));
	}
	let notes = query_result.unwrap();

	let json_response = serde_json::json!({
			"status": "success",
			"results": notes.len(),
			"notes": notes
	});
	HttpResponse::Ok().json(json_response)
}
