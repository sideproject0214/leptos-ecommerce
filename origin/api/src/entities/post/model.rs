use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

// migrations 폴더의 sql 파일에서 not null로 정의안한 것은 반드시 Option 처리해야 함
// 아래 성공
#[derive(FromRow, Clone, Debug, Serialize, Deserialize)]
pub struct Post {
	pub id: i64,
	pub uuid: Uuid,
	pub title: String,
	pub description: String,
	pub created_at: Option<DateTime<Utc>>,
	pub updated_at: Option<DateTime<Utc>>,
}

// try2
// #[derive(FromRow, Clone, Debug, Serialize, Deserialize)]
// pub struct Post {
// 	pub id: i64,
// 	pub uuid: Uuid,
// 	pub title: String,
// 	pub description: String,
// 	pub created_at: DateTime<Utc>,
// 	pub updated_at: DateTime<Utc>,
// }
// pub struct Post {
// 	pub id: i64,
// 	pub uuid: String,
// 	pub user_id: Uuid,
// 	pub title: String,
// 	pub image_src: String,
// 	pub thumbnail_src: Option<Vec<String>>,
// 	pub description: String,
// 	pub brand: String,
// 	pub category: String,
// 	pub size: serde_json::Value,
// 	pub price: i64,
// 	pub count_in_stock: i64,
// 	pub rating: f64,
// 	pub num_reviews: i64,
// 	pub sale: i64,
// 	pub free_shipping: bool,
// 	pub delivery_fee: i64,

// 	pub created_at: DateTime<Utc>,
// 	pub updated_at: DateTime<Utc>,
// }
