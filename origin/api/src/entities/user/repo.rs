use std::error::Error;

use uuid::Uuid;

use super::model::UserCreateInsert;

pub async fn user_create(
  user: &UserCreateInsert,
  pool: &sqlx::PgPool,
) -> Result<(), Box<dyn Error>> {
  let query = "insert into users (name, email, password) values ($1, $2, $3)";

  sqlx::query(query)
    .bind(&user.name)
    .bind(&user.email)
    .bind(&user.password)
    .execute(pool)
    .await?;

  Ok(())
}
