use std::env;
use std::path::PathBuf;

#[derive(Debug)]
pub struct EnvValue {
	pub env_path: PathBuf,
	pub api_address: String,
	pub db_dialect: String,
	pub db_username: String,
	pub db_password: String,
	pub db_host: String,
	pub db_port: String,
	pub db_database: String,
	pub new_field: String,
	pub db_url: String,
}
// 구조체는 값을 추상화한다면 trait는 함수를 추상화 한다
pub trait EnvConfig {
	fn new() -> Self;
	fn load_config(&mut self);
	// 새로운 필드에 대한 getter/setter 메서드 추가
	fn get_new_field(&self) -> &str;
	fn set_new_field(&mut self, value: &str);
}
// 러스트는 호출 대상이 되는 값을 메서드의 첫번째 인수로
// 전달하는데,이 인수는 self 라는 특별한 이름을 가져야 한다. 또한
// Self는 현재 구현이 적용된 타입을 나타내는 Rust의 예약어
// 여기서 Self는 EnvValue 타입을 가리키며, new() 메서드는 EnvValue의
// 새로운 인스턴스를 반환한다

impl EnvConfig for EnvValue {
	fn new() -> Self {
		EnvValue {
			env_path: PathBuf::new(),
			api_address: String::new(),
			db_dialect: String::new(),
			db_username: String::new(),
			db_password: String::new(),
			db_host: String::new(),
			db_port: String::new(),
			db_database: String::new(),
			new_field: String::new(),
			db_url: String::new(),
		}
	}

	// self는 현재 객체의 인스턴스를 전달한다
	// self를 제거하면 해당 메서드들은 객체와의 연관성을 잃게 되어
	// 객체의 상태를 조작할 수 없게 되며, 일반적인 함수로써 동작한다
	fn load_config(&mut self) {
		let rust_env = dotenv::var("RUST_ENV")
			.unwrap_or_else(|_| "development".to_string());

		// "development"는 &str(스트링 슬라이스, 불변 참조자)
		// `&str`과 `String`은 Rust에서 문자열을 다루는 두 가지 주요한
		// 타입입니다.

		// - **`&str`**: '스트링 슬라이스 (string slice)'로, 불변한 문자열 조각에 대한
		//   참조자입니다. 메모리 상에 있는 문자열 데이터의 일부분을 가리키며, 고정된 크기의
		//   포인터와 길이 정보를 가지고 있습니다. `&str`은 특정한 메모리 공간을 차지하지
		//   않으며, 주로 문자열 조각을 참조할 때 사용됩니다. 여기서 특정한 메모리 공간을 차지
		//   하지 않는다는 것은 소유하지 않는다는 의미일 뿐이다. 실제로는 '포인터(문자가
		//   저장된 주소)','길이'정보만을 저장하기에 고정된 크기를 가진다. 반면 String은
		//   '문자열 데이터'와 함께 '용량','길이'도 함께 저장하기에 '문자열 데이터', '용량'은
		//   언제든지 변화가 되므로 가변적이다

		// - **`String`**: Heap에 할당된 가변적인 문자열 타입입니다. `String`은 길이가
		//   가변적이며 변경 가능하며, 데이터를 소유합니다. 즉, 메모리를 할당하고 문자열
		//   데이터를 저장합니다. `String`은 유연한 크기를 가지며, 필요에 따라 문자열을
		//   변경하고 조작할 수 있습니다.

		// 주요 차이점:
		// - `&str`은 불변적인 문자열 참조이며, `String`은 가변적이며 소유권을 가진
		//   문자열입니다.
		// - `&str`은 크기가 고정되어 있고, 메모리를 소유하지 않습니다. `String`은 가변적이고
		//   길이가 동적으로 조절되며, 메모리를 소유합니다.
		// - `String`은 힙(heap)에 할당된 문자열이므로, 데이터의 소유권을 가집니다. 반면
		//   `&str`은 그저 참조이므로 소유권은 없습니다.

		// Rust에서는 문자열 조각을 효율적으로 참조하고 가변적으로 다루기
		// 위해 `&str`과 `String`을 함께 사용합니다. `String`은 동적으로
		// 변하는 문자열을 저장하고 조작하는 데 사용되며, 필요에 따라
		// `&str`로 변환하여 문자열 조각을 참조하거나 조작할 수 있습니다.
		match rust_env.as_str() {
			"development" => {
				// and_then : 결과가 존재하면 Ok, 그렇지 않으면 Err를 반환
				// current_dir()은 이 함수가 작동되는 곳의 위치를 의미한다.
				// 따라서 main.rs 에서 함수를 부를 경우 main.rs를 기준으로
				// 작동하게 된다.
				self.env_path = env::current_dir()
					.and_then(|a| Ok(a.join("src/config/.env.dev")))
					.unwrap();
				// println!("current folder : {:?}", &self.env_path)
			}
			_ => {
				self.env_path = env::current_dir()
					.and_then(|a| Ok(a.join("src/config/.env.prod")))
					.unwrap();
			}
		}

		dotenv::from_filename(&self.env_path).ok();
		self.api_address = env::var("API_ADDRESS")
			.expect("API_ADDRESS Not Exist");
		self.db_dialect =
			env::var("DB_DIALECT").expect("DB_DIALECT Not Exist");
		self.db_username = env::var("DB_USERNAME")
			.expect("DB_USERNAME Not Exist");
		self.db_password = env::var("DB_PASSWORD")
			.expect("DB_PASSWORD Not Exist");
		self.db_host =
			env::var("DB_HOST").expect("DB_HOST Not Exist");
		self.db_port =
			env::var("DB_PORT").expect("DB_PORT Not Exist");
		self.db_database = env::var("DB_DATABASE")
			.expect("DB_DATABASE Not Exist");
	}

	fn get_new_field(&self) -> &str {
		&self.new_field
	}

	fn set_new_field(&mut self, value: &str) {
		self.new_field = value.to_string();
	}
}
