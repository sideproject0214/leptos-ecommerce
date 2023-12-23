pub mod entities {
  pub mod index;
  pub mod user {
    pub mod model;
    pub mod repo;
  }
  pub mod post {
    pub mod model;
  }
}
pub mod seeders {
  pub mod sqlx_seeder;
}

pub mod config;
