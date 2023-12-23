use std::collections::{HashMap, HashSet};
use std::env::current_dir;
use std::fs::{self};
use std::io::Read;
use std::path::PathBuf;

use serde_json::{self, Map, Value};
use sqlx::database::HasArguments;
use sqlx::types::Json;
use sqlx::{Pool, Postgres};

use crate::entities::post::model::PostData;
use crate::entities::user::model::UserData;

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

                match first_part {
                  "users" => println!("users model"),
                  "posts" => println!("posts model"),
                  _ => println!("others model"),
                }
                for json_value in json_data {
                  if let Some(field_value) = json_value.get(first_part) {
                    // 여기서 [{},{},{}] 구조로 만들어짐
                    let arr_field_value = field_value.as_array().unwrap();

                    // 여기서 [{},{},{}] 구조로 만들어진 것을 하나씩 다시 돌린다
                    for each in arr_field_value {
                      let mut field_names: HashSet<String> = HashSet::new();

                      // json 필드 이름 추출
                      extract_field_names(each, &mut field_names);

                      // .collect::<Vec<_>>()에서 collect() 함수는 반복자를 콜렉션으로
                      // 변환하는 메서드입니다. 여기서는 Vec<_>으로 명시된 벡터로 변환하고
                      // 있습니다.

                      // ::<Vec<_>> 부분은 Rust의 제네릭 타입
                      // 매개변수를 명시하는 부분입니다. _는 컴파일러에게 해당 부분의
                      // 타입을 추론하도록 하는 플레이스홀더입니다. 따라서
                      // collect::<Vec<_>>()는 반환될 벡터의 타입을 추론하게 합니다.

                      // 그리고 .join(", ")은 벡터의 요소들을 문자열로 결합하는
                      // 메서드입니다. 여기서 ", "는 각 요소를 구분하는 구분자입니다.

                      // collect::<Vec<_>>()는 벡터로 값을 수집하고, join(", ")은 벡터의
                      // 요소들을 쉼표와 공백으로 구분하여 하나의 문자열로 연결합니다.
                      let columns: String =
                        field_names.into_iter().collect::<Vec<_>>().join(", ");
                      let values: String = field_names
                        .iter()
                        .map(|field| format!(":${}", field))
                        .collect::<Vec<_>>()
                        .join(", ");

                      let query = format!(
                        "insert into {} ({}) values ({})",
                        first_part, columns, values
                      );

                      // 필드 값 추출 및 쿼리 실행
                      let mut tx = pool.begin().await.unwrap();

                      let mut query = sqlx::query(&query);

                      for field_name in field_names {
                        if let Some(field_name) = each.get(&field_names) {
                          query = query.bind(Json(field_name.clone()));
                        }
                      }

                      query.execute(&mut tx).await.unwrap();
                      tx.commit().await.unwrap();
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

fn extract_field_names(json: &Value, field_names: &mut HashSet<String>) {
  match json {
    Value::Object(map) => {
      for (key, value) in map {
        field_names.insert(key.to_owned()); // 필드 이름을 벡터에 추가, Vec의 경우는 push 이나 HashSet은 insert 를 사용해야
                                            // 한다
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
