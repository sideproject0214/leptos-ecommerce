use chrono::prelude::*;
use sqlx::FromRow;
use uuid::Uuid;

#[derive(FromRow, Clone, Debug)]
pub struct UserQueryResult {
  pub id: i64,
  pub uuid: Uuid,
  pub name: String,
  pub email: String,
  pub password: String,
  pub google_id: String,
  pub naver_id: String,
  pub kakao_id: String,
  pub is_admin: bool,
  pub email_token: String,
  pub is_verified: bool,
  pub pw_email_address: String,

  pub created_at: DateTime<Utc>,
  pub updated_at: DateTime<Utc>,
}

#[derive(FromRow, Clone, Debug)]
pub struct UserCreate {
  pub id: i64,
  pub uuid: Uuid,
  pub name: String,
  pub email: String,
  pub password: String,
  pub google_id: String,
  pub naver_id: String,
  pub kakao_id: String,
  pub is_admin: bool,
  pub email_token: String,
  pub is_verified: bool,
  pub pw_email_address: String,
}
