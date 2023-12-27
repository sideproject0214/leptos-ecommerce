use chrono::prelude::*;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Serialize, Deserialize, FromRow, Clone, Debug)]
pub struct SizeJsonB {
  size: String,
  value: u16,
}

#[derive(FromRow, Clone, Debug, Serialize, Deserialize)]
pub struct PostData {
  // post fields
  pub id: i64,
  pub uuid: String,
  pub title: String,
  pub image_src: String,
  pub thumbnail_src: String,
  pub description: String,
  pub brand: String,
  pub category: String,
  pub size: serde_json::Value,
  pub price: i64,
  pub count_in_stock: i32,
  pub rating: f32,
  pub num_reviews: i32,
  pub sale: i32,
  pub free_shipping: bool,
  pub delivery_fee: i32,

  // user fields
  pub user_id: i64,

  pub created_at: String,
  pub updated_at: String,
}

pub trait PostDataFn {
  fn new() -> Self;

  /// where Self: Sized 제약은 트레이트를 구현하는 타입이 'Sized'인 경우에만 해당 메서드가
  /// 호출될 수 있음을 의미합니다. Rust에서 기본적으로는 모든 타입은 'Sized'이지만, 특정
  /// 조건 아래에서 'Sized'가 아닌 타입을 표현하기 위해 이러한 제약을 사용할 수 있습니다.
  /// Sized 타입: 컴파일 타임에 크기가 결정되어 메모리에 정확히 알려진 타입입니다.
  /// 대부분의 Rust 타입은 'Sized'입니다. 예를 들어, 정수형, 부동 소수점 수, 구조체 등이
  /// 'Sized'입니다.

  // Unsized 타입: 크기가 컴파일 타임에 알려지지 않은 타입입니다. 이러한 타입은 다른
  // 크기의 데이터를 가질 수 있는 동적 크기의 데이터를 가질 수 있습니다. 대표적으로
  // 슬라이스([T])나 트레이트 객체(trait Trait)가 있습니다. 이러한 타입들은 일반적으로
  // 포인터로 참조되며, 동적인 크기를 가질 수 있기 때문에 컴파일러는 이들의 크기를 정확히
  // 알 수 없습니다.

  // Rust는 대부분의 경우 타입이 'Sized'인 것을 기본으로 가정합니다. 따라서 함수의
  // 매개변수, 반환 값, 구조체의 필드 등은 대부분 'Sized'여야 합니다.

  // 하지만 슬라이스와 트레이트 객체와 같은 'unsized' 타입은 바로 값을 가지는 것이 아니라
  // 포인터를 통해 참조되기 때문에 크기가 고정되지 않습니다. 이러한 'unsized' 타입은 보통
  // 힙에 할당되고, 그에 대한 포인터를 통해 접근됩니다.

  // 예를 들어, &str 타입은 문자열 슬라이스로, 실제 데이터의 크기를 컴파일 시점에 알 수
  // 없기 때문에 'unsized'입니다. 따라서 함수의 매개변수로 직접 전달할 수 없고, 대신
  // 참조자(&str)를 통해 사용됩니다.

  // 이러한 차이로 인해 Rust는 컴파일 타임에 안전성을 보장하면서도 유연성을 제공합니다.
  // 'Sized'와 'unsized'의 개념을 이해하면 Rust에서 메모리의 효율적인 관리와 안전한 코드를
  // 작성하는 데 도움이 됩니다.
  ///
  fn create_post(data: &[&str]) -> Vec<Self>
  where
    Self: Sized;
}

impl PostDataFn for PostData {
  fn new() -> Self {
    PostData {
      // post fields
      id: 0,
      uuid: String::new(),
      title: String::new(),
      image_src: String::new(),
      thumbnail_src: String::new(),
      description: String::new(),
      brand: String::new(),
      category: String::new(),
      size: serde_json::Value::Null,
      price: 0,
      count_in_stock: 0,
      rating: 0.0,
      num_reviews: 0,
      sale: 0,
      free_shipping: false,
      delivery_fee: 0,

      // user fields
      user_id: 0,

      created_at: String::new(),
      updated_at: String::new(),
    }
  }

  // Size (크기): size = 24 (0x18)는 해당 데이터 타입이 메모리에서 차지하는 크기를 바이트
  // 단위로 나타냅니다. 이것은 해당 타입이 메모리에서 얼마나 많은 공간을 차지하는지를
  // 의미합니다.

  // Alignment (정렬): align = 0x8은 데이터 타입이 메모리에서 정렬되는 방식을 나타냅니다.
  // Rust는 데이터를 특정 바운더리에 맞추어 메모리에 배치하는데, 이 때의 바운더리를
  // "정렬(alignment)"이라고 합니다. 이 경우 0x8은 8바이트에 해당하는 정렬을 의미합니다.
  // 즉, 해당 데이터 타입의 메모리 주소는 8의 배수 위치에 정렬됩니다.

  // 이러한 정보는 Rust 컴파일러가 데이터 타입을 메모리에 배치할 때 고려하는 요소입니다.
  // 정렬은 CPU가 메모리를 효율적으로 읽고 쓰기 위해 사용하는 방법 중 하나이며, 많은
  // 하드웨어에서는 특정 바운더리에 맞춘 정렬된 데이터를 더 빠르게 처리할 수 있습니다.
  fn create_post(data: &[&str]) -> Vec<Self> {
    let mut posts: Vec<PostData> = Vec::new();

    for &json_str in data {
      let post: PostData = serde_json::from_str(json_str).expect("JSON parsing failed");
      posts.push(post)
    }
    posts
  }
}
