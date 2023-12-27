use std::env::current_dir;
use std::error::Error;
use std::fs::{self};
use std::io::Read;

use serde_json::{self, Value};

use sqlx::{Pool, Postgres};

use chrono::{DateTime, FixedOffset};
use sqlx::types::chrono;
use sqlx::types::Uuid;

use config::Config;

struct Seeder {
  file_names: Vec<String>,
  table_names: Vec<String>,
}

trait SeederFn {
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

// Implementation of types defined in external crates like DateTime and FixedOffset from chrono within the current code scope is not possible. This is one of the fundamental rules of Rust, requiring the definition of a new trait specific to these types. To achieve this, you need to enable the 'chrono' module in your Cargo.toml and then use the 'DateTime<FixedOffset>' time-related types. Afterwards, define and use a new type for 'DateTime<FixedOffset>' from chrono. Failing to do so will result in errors because the default DateTime<FixedOffset> lacks a trait for encoding as a timestamp.
trait MyDateTimeEncode {
  fn my_encode(&self) -> String;
}

impl MyDateTimeEncode for DateTime<FixedOffset> {
  fn my_encode(&self) -> String {
    self.to_rfc3339()
  }
}

#[derive(Debug)]
struct SeederConfig {
  task_folder_path: String,
  success_folder_path: String,
  jsonb_name: String,
  array_string_name: String,
  created_at_name: String,
  updated_at_name: String,
}

trait SeederConfigFn {
  fn new() -> Self;
}

impl SeederConfigFn for SeederConfig {
  fn new() -> Self {
    SeederConfig {
      task_folder_path: String::new(),
      success_folder_path: String::new(),
      jsonb_name: String::new(),
      array_string_name: String::new(),
      created_at_name: String::new(),
      updated_at_name: String::new(),
    }
  }
}

fn read_config() -> Result<SeederConfig, config::ConfigError> {
  let mut new_seeder_config = SeederConfig::new();
  let settings = match Config::builder()
    .add_source(config::File::with_name("pg-seeder"))
    .build()
  {
    Ok(config) => config,
    Err(err) => {
      eprintln!("Error: Failed to load configuration: {}", err);
      Config::default() // 또는 원하는 기본값을 리턴하는 함수 호출 등
    }
  };

  match settings.get::<String>("seeders.task_folder") {
    Ok(value) => new_seeder_config.task_folder_path = value,
    Err(_) => new_seeder_config.task_folder_path = "src/seeders/task".to_string(),
  };
  match settings.get::<String>("seeders.success_folder") {
    Ok(value) => new_seeder_config.success_folder_path = value,
    Err(_) => new_seeder_config.success_folder_path = "src/seeders/success".to_string(),
  };
  match settings.get::<String>("seeders.created_at_name") {
    Ok(value) => new_seeder_config.created_at_name = value,
    Err(_) => new_seeder_config.created_at_name = "created_at".to_string(),
  };
  match settings.get::<String>("seeders.updated_at_name") {
    Ok(value) => new_seeder_config.updated_at_name = value,
    Err(_) => new_seeder_config.updated_at_name = "updated_at".to_string(),
  };
  match settings.get::<String>("seeders.jsonb_name") {
    Ok(value) => new_seeder_config.jsonb_name = value,
    Err(_) => new_seeder_config.jsonb_name = "size".to_string(),
  };
  match settings.get::<String>("seeders.array_string_name") {
    Ok(value) => new_seeder_config.array_string_name = value,
    Err(_) => new_seeder_config.array_string_name = "thumbnail_src".to_string(),
  };

  Ok(new_seeder_config)
}

#[warn(unused_variables)]
pub async fn seeder(pool: &Pool<Postgres>) -> Result<(), Box<dyn Error>> {
  let seed_config = read_config().unwrap();
  println!("read config {:?}", seed_config.success_folder_path);

  let mut new_seeder = Seeder::new();
  // current_dir()은 현재 작업하고 있는 곳의 폴더를 알려준다
  // api 폴더에서 src/main.rs를 실행하면 api 폴더가 프로그램이 실행되는 기준 디렉토리가
  // 되고, current_dir()를 호출하면 해당 디렉토리인 api가 출력될 것입니다.
  let seeder_folder = current_dir()
    .and_then(|a| Ok(a.join(seed_config.task_folder_path)))
    .expect("No seed_folder exists");
  // println!("seeder_folder : {:?}", seeder_folder);

  let success_folder = current_dir()
    .and_then(|a| Ok(a.join(seed_config.success_folder_path)))
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
                    // 여기서 [{},{},{}] 구조로 만들어짐
                    let arr_field_value = field_value.as_array().unwrap();

                    // 여기서 [{},{},{}] 구조로 만들어진 것을 하나씩 다시 돌린다
                    for each in arr_field_value {
                      let mut field_names: Vec<&str> = Vec::new();
                      let mut field_values: Vec<String> = Vec::new();

                      // println!("each json_data {:?}", each);

                      // {} 형태로 만들어짐

                      if let Some(json_obj) = each.as_object() {
                        for (key, value) in json_obj {
                          field_names.push(key);

                          field_values.push(value.to_string());
                        }
                      }
                      println!("Field Names: {:?}", &field_names);
                      println!("Field Values: {:?}", &field_values);

                      // 아래는 posts를 돌릴때 필요한 것이다. enumerate는 인덱스를
                      // 만들어준다
                      // let placeholders = (1..=field_values.len())
                      //   .enumerate()
                      //   .map(|(idx, n)| match field_names[idx] {
                      //     "size" => format!("${}::JSONB", n),
                      //     "thumbnail_src" => format!("${}::TEXT[]", n),
                      //     _ => format!("${}", n),
                      //   })
                      //   .collect::<Vec<String>>()
                      //   .join(", ");

                      let mut placeholders = String::new();

                      for (idx, n) in (1..=field_values.len()).enumerate() {
                        let field_name = &field_names[idx];

                        let placeholder = match field_name {
                          _ if field_name == &seed_config.jsonb_name => {
                            format!("${}::JSONB", n)
                          }

                          field if field == &seed_config.array_string_name => {
                            format!("${}::TEXT[]", n)
                          }
                          _ => format!("${}", n),
                        };

                        placeholders.push_str(&placeholder);

                        if idx < field_values.len() - 1 {
                          placeholders.push_str(", ");
                        }
                      }

                      let query = format!(
                        "insert into {} ({}) values ({})",
                        &first_part,
                        &field_names.join(", "),
                        placeholders
                      );

                      println!("postgres query : {:?}", &query);

                      // 쿼리 실행
                      let mut query = sqlx::query(&query);

                      // 개별적으로 값들을 바인딩

                      for (index, value) in field_values.iter().enumerate() {
                        match each.get(field_names[index]) {
                          Some(json_value) => match json_value {
                            Value::Bool(bool_value) => {
                              query = query.bind(bool_value);
                            }
                            Value::Number(int_value) => {
                              if let Some(i64_value) = int_value.as_i64() {
                                query = query.bind(i64_value);
                              } else if let Some(f64_value) = int_value.as_f64() {
                                query = query.bind(f64_value);
                              } else {
                                println!("Number Error")
                              }
                            }

                            Value::String(uuid_string) => {
                              println!("index {:?}", field_names[index]);
                              match Uuid::parse_str(uuid_string) {
                                Ok(uuid_value) => {
                                  query = query.bind(uuid_value);
                                }
                                Err(_) => match field_names[index]
                                  == seed_config.created_at_name
                                  || field_names[index] == seed_config.updated_at_name
                                {
                                  true => {
                                    if let Ok(timestamp) =
                                      chrono::DateTime::parse_from_rfc3339(uuid_string)
                                    {
                                      query = query.bind(timestamp);
                                      println!(
                                        "string: created_at!!! true, filed_name : {:?}",
                                        field_names[index]
                                      )
                                    }
                                  }
                                  false => {
                                    query = query.bind(uuid_string);
                                    println!(
                                      "string: created_at!!! false, filed_name : {:?}",
                                      field_names[index]
                                    )
                                  }
                                },
                              }
                            }
                            Value::Array(array_value) => {
                              println!("array {:?}", field_names[index]);

                              query = query.bind(array_value);
                            }
                            Value::Object(obj_value) => {
                              println!("JSONB!!!! {:?}", field_names[index]);
                              let json_string = serde_json::to_string(obj_value)
                                .expect("Failed to serialize JSON object to string");

                              // Bind the JSON string to the SQL query
                              query = query.bind(json_string);
                            }
                            _ => {
                              query = query.bind(value);
                            }
                          },
                          None => {
                            println!("Seeder Error!")
                          }
                        }
                      }
                      // 쿼리 실행
                      query.execute(pool).await.unwrap();
                    }
                  }
                }
              }
            }
          }
          println!("✅ Seed completed for the {:?}.json", file_name);
        }

        let new_path = success_folder.join(entry.file_name());
        println!("success folder : {:?}", new_path);
        if let Err(err) = fs::rename(entry.path(), new_path) {
          println!("Failed to move file: {}", err);
        };
      } else {
        println!("Failed to read the directory.");
      }
    }
  }

  println!(
    "file_name: {:?} parts : {:?}",
    new_seeder.file_names, new_seeder.table_names
  );

  // seeder_folder
  print!("✅ Seeder Work Success! ✅ \n");
  Ok(())
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
