use actix_web::web;

use super::repo::{get_posts_pagination, get_search_posts};

pub fn post_routes() -> actix_web::Scope {
	web::scope("")
		.service(get_posts_pagination)
		.service(get_search_posts)
}
