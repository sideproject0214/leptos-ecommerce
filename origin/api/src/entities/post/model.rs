use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Serialize, Deserialize, FromRow, Clone, Debug)]
pub struct SizeJsonB {
	size: String,
	value: u16,
}

#[derive(FromRow, Clone, Debug, Serialize, Deserialize)]
pub struct Post {
	pub id: i64,
	pub uuid: String,
	pub user_id: Uuid,
	pub title: String,
	pub image_src: String,
	pub thumbnail_src: Vec<String>,
	pub description: String,
	pub brand: String,
	pub category: String,
	pub size: serde_json::Value,
	pub price: i64,
	pub count_in_stock: i64,
	pub rating: f64,
	pub num_reviews: i64,
	pub sale: i64,
	pub free_shipping: bool,
	pub delivery_fee: i64,
	pub created_at: DateTime<Utc>,
	pub updated_at: DateTime<Utc>,
}

// Sample 예제 따라하기

#[derive(Deserialize, Debug)]
pub struct FilterOptions {
	pub page: Option<usize>,
	pub limit: Option<usize>,
}

#[derive(Deserialize, Debug)]
pub struct ParamOptions {
	pub id: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateNoteSchema {
	pub title: String,
	pub content: String,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub category: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub published: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UpdateNoteSchema {
	pub title: Option<String>,
	pub content: Option<String>,
	pub category: Option<String>,
	pub published: Option<bool>,
}
