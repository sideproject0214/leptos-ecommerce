use std::collections::HashSet;
use std::env::current_dir;
use std::fs::{self};
use std::io::Read;
use std::path::PathBuf;

use axum::Json;
use serde_json::{self, Map, Value};
use sqlx::query::Query;
use sqlx::{Arguments, Pool, Postgres};

use crate::entities::user::model::{json_data_to_user_data, UserSeedData};
use crate::seeders::task::users::user_seed_data::load_user_data;

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

pub async fn seeder(pool: &Pool<Postgres>) {
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

                // let json_data = read_json_file();

                match first_part {
                  "users" => println!("users model"),
                  "posts" => println!("posts model"),
                  _ => println!("others model"),
                }

                let user_data = load_user_data();
                println!("user_data {:?}", user_data);

                for user in user_data {
                  let user = UserSeedData {
                    uuid: user.uuid,
                    name: user.name,
                    email: user.email,
                    password: user.password,
                    google_id: user.google_id,
                    naver_id: user.naver_id,
                    kakao_id: user.kakao_id,
                    is_admin: user.is_admin,
                    created_at: user.created_at,
                    updated_at: user.updated_at,
                  };
                  // sqlx::query(
                  //   "INSERT INTO users (uuid, name, email, password, google_id, \
                  //    naver_id, kakao_id, is_admin, created_at, updated_at) VALUES ($1,
                  // \    $2, $3, $4, $5, $6, $7, $8, $9, $10)",
                  // )
                  // .bind(&user.uuid)
                  // .bind(&user.name)
                  // .bind(&user.email)
                  // .bind(&user.password)
                  // .bind(&user.google_id)
                  // .bind(&user.naver_id)
                  // .bind(&user.kakao_id)
                  // .bind(&user.is_admin)
                  // .bind(&user.created_at)
                  // .bind(&user.updated_at)
                  // .execute(pool)
                  // .await
                  // .unwrap();
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

fn extract_field_names(json: &Value, field_names: &mut Vec<String>) {
  match json {
    Value::Object(map) => {
      for (key, value) in map {
        field_names.push(key.to_owned()); // 필드 이름을 벡터에 추가
        extract_field_names(value, field_names); // 중첩된 값이 있을 경우 재귀적으로 호출
      }
    }
    Value::Array(arr) => {
      for value in arr {
        extract_field_names(value, field_names);
      }
    }
    _ => {}
  }
}

// fn extract_field_values(json: &Value, field_values: &mut Vec<String>) {
//   match json {
//     Value::Object(map) => {
//       for (key, value) in map {
//         field_values.push(value.to_owned()); // 필드 이름을 벡터에 추가
//         extract_field_values(value, field_values); // 중첩된 값이 있을 경우 재귀적으로
//                                                    // 호출
//       }
//     }
//     Value::Array(arr) => {
//       for value in arr {
//         extract_field_values(value, field_values);
//       }
//     }
//     _ => {}
//   }
// }
// fn extract_field_values(json: &Value, field_values: &mut Vec<String>) {
//   match json {
//     Value::Object(map) => {
//       for (key, value) in map {
//         field_values.push(value.to_owned()); // 필드 이름을 벡터에 추가
//         extract_field_values(value, field_values); // 중첩된 값이 있을 경우 재귀적으로
//                                                    // 호출
//       }
//     }
//     Value::Array(arr) => {
//       for value in arr {
//         extract_field_values(value, field_values);
//       }
//     }
//     _ => {}
//   }
// }
