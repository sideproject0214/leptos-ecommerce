use sqlx::{migrate, FromRow, Pool, Postgres};
use uuid::Uuid;

use crate::config::EnvValue;

/// 모든 User, Post 테이블의 고유 식별 값
#[derive(FromRow)]
pub struct EntityUuid {
  pub uuid: Uuid,
}

/// 이 구조체가 Postgres 데이터베이스 연결 풀을 나타낸다
/// 여기서 derive 매크로를 통해 Clone 트레이트를 자동으로 구현한 것은
/// DbRepo 구조체를 사용할때 해당 객체를 복사해서 사용하라는 의미이다
/// 따라서 DbRepo 구조체를 사용하게 되면 Pool 트레이트를 복사하게
/// 됨으로써 DB와 연결되는 작업을 할때 Pool을 유지하게 되어 성능을
/// 향상시키게 된다
///
/// (Pool이란 데이터베이스의 리소스 관리방식으로 DB 연결을 효율적으로
/// 관리하고 재사용하는 방식을 의미한다. 일반적으로 데이터베이스
/// 연결은 생성 비용이 높거나 제한된 수의 연결만 허용되는 경우가 많다.
/// 따라서 연결 생성, 삭제하는 과비용(오버헤드, 필요이상으로 들어가는
/// 추가적인 비용이나 부담)을 없애고 시스템 성능을 향상시키기 위해
/// 사전에 DB와 연결하는 Pool 이라는 것을 만들어 놓고, 연결요청이
/// 들어올때마다 연결을 가져와 사용한 후에 반납하여 Pool에 반납하여 재
/// 사용할 수 있게 한다.)
#[derive(Clone)]
pub struct DbRepo {
  my_pool: Pool<Postgres>,
}

// 여기서 type Output;은 type Output은 트레이트 내에
// 존재하는 연관 타입(associated type)을 정의하는 부분으로
// 트레이트가 단순한 메서드 모음 이상의 역할을 해야할때 유용하다
// 이것은 해당 트레이트를 구현하는 타입이나 구조체에서 구체적인 타입을
// 지정해야 하는데, 이 경우 Output이라는 연관 타입을 정의하고
// 있습니다.

// 연관 타입은 트레이트의 일부로 선언되며, **구현체**에서 실제 타입을
// 제공해야 합니다. 트레이트를 사용하는 타입이나 구조체에서 이 연관
// 타입을 적절히 구현하여 사용할 수 있습니다.

// 이것은 트레이트가 반환 타입을 추상화하여 여러 구현체에서 동일한
// 트레이트를 구현하더라도 해당 구현체마다 다른 타입을 반환하도록 하는
// 방법 중 하나입니다. 이것을 통해 트레이트를 사용하는 코드는 구현체가
// 실제로 어떤 타입을 반환하는지에 대해 고려할 필요 없이 일관된
// 방식으로 사용할 수 있습니다.

// 예를 들어, DbPoolGetter 트레이트에서 Output이라는 연관 타입을
// 정의했다면, 이를 구현하는 구조체는 이 연관 타입을 구체적으로
// 지정해주어야 합니다. 구체적인 타입은 해당 트레이트를 구현하는
// 구조체의 특성에 따라 다를 수 있습니다.

// 연관타입 이름은 개발자의 의도에 따라 정하면 되고 데이터베이스 관련
// 작업에서는 관례적으로 결과도출물에 Output 이라는 이름의 연관타입을
// 주로 사용한다
pub trait DbPoolGetter {
  type Output;
  fn get_pool(&self) -> &Self::Output;
}

/// DbRepo 라는 구조체 메서드
impl DbRepo {
  pub async fn init(my_env: &EnvValue) -> Self {
    println!("DbRepo Init!!! {:?}", my_env);
    Self {
      my_pool: get_db_conn(&my_env).await,
    }
  }
}

/// DbPoolGetter 트레이트 구현
/// 일반적으로 트레이트 구현은 impl TraitName for Type 으로 만든다
impl DbPoolGetter for DbRepo {
  type Output = Pool<Postgres>;

  fn get_pool(&self) -> &Self::Output {
    &self.my_pool
  }
}

/// Impl로 구조체(struct)에 메서드 추가한다
/// 메서드는 객체와 짝지어진 함수로 인자를 지정할 필요가 없는 함수
/// 여기서 관용적인 new를 사용하지 않는 이유는 이 함수는 비동기이어야
/// 하기 때문이다 관용적인 new 사용 : impl DbRepo { fn **new()** -> T
/// }

pub async fn get_db_conn(my_env: &EnvValue) -> Pool<Postgres> {
  println!("Get DB Connect Start!");
  let pg_dialect = &my_env.db_dialect;
  let pg_username = &my_env.db_username;
  let pg_password = &my_env.db_password;
  let pg_host = &my_env.db_host;
  let pg_port = &my_env.db_port;
  let pg_database = &my_env.db_database;

  let pg_url = format!(
    "{pg_dialect}://{pg_username}:{pg_password}@{pg_host}:{pg_port}/{pg_database}"
  );
  println!("DB URL : {:?}", &pg_url);
  let my_pool = sqlx::postgres::PgPool::connect(&pg_url).await.unwrap();

  let migrate = migrate!("./src/migrations").run(&my_pool).await;

  match migrate {
    Ok(()) => println!("sqlx migration success"),
    Err(e) => println!("sqlx migration error : {:?}", e),
  }

  my_pool
  // env_extact()
}
