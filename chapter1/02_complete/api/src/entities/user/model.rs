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

pub fn json_data_to_user_data(json: Value) -> Option<UserCreateInsert> {
  let name = json["name"].as_str().unwrap().to_string();
  let email = json["email"].as_str().unwrap().to_string();
  let password = json["password"].as_str().unwrap().to_string();
  let is_admin = json["is_admin"].as_bool().unwrap_or(false);

  Some(UserCreateInsert {
    name,
    email,
    password,
    is_admin,
  })
}

#[derive(FromRow, Clone, Debug, Deserialize, Serialize)]
pub struct UserCreateInsert {
  pub name: String,
  pub email: String,
  pub password: String,
  pub is_admin: bool,
}

impl IntoIterator for UserCreateInsert {
  type Item = (String, String, String, bool);
  type IntoIter = std::vec::IntoIter<Self::Item>;

  fn into_iter(self) -> Self::IntoIter {
    vec![(self.name, self.email, self.password, self.is_admin)].into_iter()
  }
}
