use std::env::current_dir;
use std::fs::{self};
use std::io::Read;
use std::path::PathBuf;

use serde_json::{self, Value};
use sqlx::{Pool, Postgres};

use crate::entities::post::model::PostData;

pub struct Seeder {
  file_names: Vec<String>,
  table_names: Vec<String>,
}

pub trait SeederFn {
  fn new() -> Self;
}

impl SeederFn for Seeder {
  fn new() -> Self {
    Seeder {
      file_names: Vec::new(),
      table_names: Vec::new(),
    }
  }
}

pub async fn seeder(pool: &Pool<Postgres>) -> PathBuf {
  let mut new_seeder = Seeder::new();
  // current_dir()은 현재 작업하고 있는 곳의 폴더를 알려준다
  // api 폴더에서 src/main.rs를 실행하면 api 폴더가 프로그램이 실행되는 기준 디렉토리가
  // 되고, current_dir()를 호출하면 해당 디렉토리인 api가 출력될 것입니다.
  let seeder_folder = current_dir()
    .and_then(|a| Ok(a.join("src/seeders/task")))
    .expect("No seed_folder exists");
  // println!("seeder_folder : {:?}", seeder_folder);
  let success_folder = current_dir()
    .and_then(|a| Ok(a.join("src/seeders/success")))
    .expect("No seed_folder exists");

  // fs::read_dir 함수는 지정된 디렉토리의 내용을 읽어들입니다. 이 함수는 디렉토리 내의
  // 엔트리(파일, 디렉토리, 심볼릭 링크 등) 목록을 담고 있는 반복자(iterator)를
  // 반환합니다.
  if let Ok(entries) = fs::read_dir(&seeder_folder) {
    for entry in entries {
      if let Ok(entry) = entry {
        // 여기서 파일 2개 읽었음

        if let Ok(file_name) = entry.file_name().into_string() {
          // file_name은 users_seeder.rs 형태이다.
          new_seeder.file_names.push(file_name.to_string());

          // if let Some(ext) = file_name.split(".").

          if let Some(ext) = file_name.split(".").last() {
            if ext == "json" {
              if let Some(first_part) = file_name.split(".").next() {
                new_seeder.table_names.push(first_part.to_string());
                // println!("table_names aaa: {:?}", first_part);

                let json_data = read_json_file();

                for json_value in json_data {
                  if let Some(field_value) = json_value.get(first_part) {
                    // println!("json_data [{}] {:?}", first_part, field_value)
                    let arr_field_value = field_value.as_array().expect("No Array");

                    for each in arr_field_value {
                      // println!("each json_data {:?}", each);
                      let v = serde_json::to_value(&each).unwrap();
                      // println!("field, value  {:?}", v);
                      // println!("sqlx insert start!");
                      // Deserialize하는 과정에서 JSON 데이터의 키와 값을 해당 Rust
                      // 구조체의 필드와 매핑시켜 구조체로 변환하는 작업을 말해. 이를 통해
                      // Rust 코드에서 쉽게 JSON 데이터를 조작하고 활용할 수 있게 돼.

                      // query_as는 주로 select 일때 사용하고, query는 주로 insert 구문
                      // 실행할 때 사용한다.
                      let insert_result = sqlx::query(&format!(
                        "insert into {} (uuid, user_id, title, image_src, \
                         thumbnail_src, description, brand, category, size, price, \
                         count_in_stock, rating, num_reviews, sale, free_shipping, \
                         delivery_fee, created_at, updated_at) values ($1, $2, $3, $4, \
                         $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17, \
                         $18)",
                        first_part
                      ))
                      .bind(&each["uuid"])
                      .bind(&each["user_id"])
                      .bind(&each["title"])
                      .bind(&each["image_src"])
                      .bind(&each["thumbnail_src"])
                      .bind(&each["description"])
                      .bind(&each["brand"])
                      .bind(&each["category"])
                      .bind(&each["size"])
                      .bind(&each["price"])
                      .bind(&each["count_in_stock"])
                      .bind(&each["rating"])
                      .bind(&each["num_reviews"])
                      .bind(&each["sale"])
                      .bind(&each["free_shipping"])
                      .bind(&each["delivery_fee"])
                      .bind(&each["created_at"])
                      .bind(&each["updated_at"])
                      .execute(pool)
                      .await;
                      // .unwrap();

                      match insert_result {
                        Ok(_) => println!("Insert successful"),
                        Err(e) => eprintln!("Error inserting data: {}", e),
                      };
                    }
                  }
                }
              }
            }
          }
        }
      } else {
        println!("Failed to read the directory.");
      }
    }
  }

  println!(
    "file_name: {:?} parts : {:?}",
    new_seeder.file_names, new_seeder.table_names
  );

  // serde_json::from_str()은 JSON 문자열을 파싱하여 해당하는 타입으로 디코딩하는
  // 함수입니다. 그러나 여기서 file_name은 파일 이름을 나타내는 문자열이고, 이를 JSON
  // 문자열로 파싱할 수 없습니다. 파일 내용을 읽어와 JSON으로 디코딩해야 합니다.
  // for file_name in new_seeder.file_names {
  //   let _ = read_json_file(&file_name);
  // }

  // for
  seeder_folder
}

pub fn read_json_file() -> Vec<Value> {
  let dir_path = current_dir()
    .expect("Can't retreive directory")
    .join("src/seeders/task");

  let mut json_values: Vec<Value> = Vec::new();

  if let Ok(entries) = fs::read_dir(dir_path) {
    // 각 파일을 처리합니다.
    for entry in entries {
      if let Ok(entry) = entry {
        let file_path = entry.path();

        // 파일 확장자 확인 (JSON 파일만 처리하도록)
        if let Some(ext) = file_path.extension() {
          if ext == "json" {
            // 파일을 열어서 읽기 모드로 엽니다
            let mut file = fs::File::open(&file_path).expect("파일을 열 수가 없습니다");

            // 파일 내용을 읽어서 String 으로 저장합니다.
            let mut contents = String::new();
            file
              .read_to_string(&mut contents)
              .expect("파일을 읽는데 문제가 발생했습니다");

            // JSON 문자열(json_data)을 Data 구조체로 deserialize하는 작업을 해.
            // serde_json::from_str은 주어진 JSON 문자열을 Rust의 데이터 구조체로
            // 변환해줘.
            let json_value: Value =
              serde_json::from_str(&contents).expect("JSON 파싱에 실패했습니다");

            // println!("{:?} JSON 데이터: ", json_value);
            json_values.push(json_value);
          }
        }
      }
    }
  }

  json_values
}
