use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sqlx::FromRow;
use uuid::Uuid;

#[derive(FromRow, Clone, Debug, Deserialize, Serialize)]
pub struct UserSeedData {
  pub uuid: Uuid,
  pub name: String,
  pub email: String,
  pub password: String,
  pub google_id: String,
  pub naver_id: String,
  pub kakao_id: String,
  pub is_admin: bool,
  pub created_at: DateTime<Utc>,
  pub updated_at: DateTime<Utc>,
}

pub fn json_data_to_user_data(json: Value) -> Option<UserSeedData> {
  let uuid = Uuid::parse_str(json["uuid"].as_str().unwrap())
    .ok()
    .unwrap();
  let name = json["name"].as_str().unwrap().to_string();
  let email = json["email"].as_str().unwrap().to_string();
  let password = json["password"].as_str().unwrap().to_string();
  let google_id = json["google_id"].as_str().unwrap().to_string();
  let naver_id = json["naver_id"].as_str().unwrap().to_string();
  let kakao_id = json["kakao_id"].as_str().unwrap().to_string();
  let is_admin = json["is_admin"].as_bool().unwrap_or(false);
  let created_at = DateTime::parse_from_rfc3339(json["created_at"].as_str().unwrap())
    .ok()?
    .with_timezone(&Utc);
  let updated_at = DateTime::parse_from_rfc3339(json["updated_at"].as_str().unwrap())
    .ok()?
    .with_timezone(&Utc);

  Some(UserSeedData {
    uuid,
    name,
    email,
    password,
    google_id,
    naver_id,
    kakao_id,
    is_admin,
    created_at,
    updated_at,
  })
}

#[derive(FromRow, Clone, Debug, Deserialize, Serialize)]
pub struct UserCreateInsert {
  pub name: String,
  pub email: String,
  pub password: String,
}
