use actix_web::web;

use super::repo::get_posts_pagination;

pub fn post_routes() -> actix_web::Scope {
	web::scope("/pagination").service(get_posts_pagination)
}
