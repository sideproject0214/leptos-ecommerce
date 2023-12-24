# Ecommerce Refactory 

## chapter1 [2023-12-12 09:01:31]
- 도커 컴포즈를 통해서 작동시키려했으나 컴 사양문제로 어려움
  - 러스트가 컴파일이 늦어서 도커로 작동시키면 컴파일에 시간이 너무 걸림
- 따라서 client, api는 그냥 로컬에서 justfile로 작동시킴
- 나머지 파일은 origin에서 복사해서 챕터를 만들기로 함 
- 따라서 개발시에는 gateway 없이 작동시킬 예정

### [api] [2023-12-12 17:54:50]
- api의 경우 models에 스키마를 정의하고 controller 폴더를 따로 만들어 관리했지만
- 여기서는 테이블별로 폴더를 만들고 그곳에서 모델을 정의하고 관련된 controller를 저장할 것이다

### [api][seeder] [2023-12-21 22:53:23]
- seeder는 미리 model 구조가 정의되어야 하고, seed더미파일은 json으로 만들고 필드 이름에 해당 모델이름이 들어가야 한다. 그래서 파일이름 무관하게 json 파일을 읽어오면 json 필드 이름을 기준으로 entities 폴더에서 해당 파일이름.rs 파일에서 구조체를 가져오고 seeder task 폴더에서 파일을 읽어 필드이름을 기준으로 seeder를 for 반복문으로 돌려서 sqlx를 사용해 postgres에 입력한다
- json 파일 형태 {"모델이름":[{},{}, ~ , {}]} 이렇게 만든다
- [2023-12-21 22:59:24] 이전까지는 파일이름을 기준으로 json 파일의 필드이름을 선태해서 값을 가져왔다. Next.js 같은 경우도 폴더이름 등으로 기준으로 하니까... 일단 이방식으로 밀어 붙여봐야겠다. 구상은 새롭게 해봤지만 파일이름 기준으로 하는것도 이미 거의다 만들었으니 나쁘지 않을듯...
- [seeder 작동 조건 / nodejs sequelize 기준]
  - 1. 모델 구조는 미리 정의되어 있을 것
  - 2. db에 이미 해당 테이블이 만들어져 있을 것
    - [o](구현 : 서버 작동시 migration 파일이 있으면 자동으로 만들어짐)
  - 3. seeder는 순수히 json 파일을 그냥 자동으로 db에 migration 하는 기능만 할 것
  - 4. task 폴더에 있는 파일 중 json 확장자만 읽어서 마이그레이션 시키고 완료파일은 success로 옮길 것
- [2023-12-24 22:06:38] seeder 기본작동은 완성. 완성하고 보니 seeder는 상당히 까다로운 작업임. 일단 러스트에서 지원하는 타입에 따라서 postgres 타입도 정해야해서 어려웠음
  - 일단 seeder 작동하는 기본원리는 파일을 열어서 값을 읽어오면 일단 이것들을 string으로 필드 이름과 필드 값을 따로 분리해서 수동으로 postgresql 문을 작성해주고, 이를 sqlx로 실행해주기 위해서는 각 값들을 타입에 맞게 bind 해줘야 하는데 일단 기본적으로 값들은 모두 string이기에 string인 것들은 문제가 안되지만, uuid, 숫자등은 전부 분기해서 순수한 값을 넣어줘야한다.  
  - 대표적으로 처음에는 int로 정한게 나중에 값을 bind할때 string이 아닌 것들은 전부다 순수한 해당 값으로 넣어줘야 하는데, 이때 값의 타입을 확인해서 값을 분기해서 넣어줘야 한다. 이때 러스트에서는 숫자의 경우 as_i64, as_f64, as_u64만 지원해서 postgres에서 int로 정의한 것들을 전부다 bigint로 바꾸고 float로 정의한 것은 DOUBLE PRECISION으로 정의해야 했다
  - (구현힘든곳) 특히 jsonb, text[]값들을 구현하고자 할때 잘 안되어서 정말힘들었다. 기본적으로 chatgpt의 도움을 받았지만 내가 원하는 코드를 100%만들어주지 못하고 대략적인 예시만 보여준다. 그래도 없는 것보다는 도움이 되었다. 
  - (AI에 대한 고찰) 일단 ai 발전이 프로그래머의 직업을 없애것이라는것에는 아직은 시기상조라는 것이 나의 입장이다. 자바스크립트나 파이썬같은 스크립트 계열 언어는 비교적 잘 작성하는 것 같으나, 러스트 같이 강타입언어이면서 컴파일시에 모든게 정해져야 있어야 하는 언어는 10번에 1번정도 90점되는 답을 제시하였다. 물론 그런것에 힌트를 얻어서 개발을 하기에 도움은되었다. 하지만 아이언맨의 자비스같은 기능은 아직은 어렵다. 
    - 일단 ai는 내가 물어본것만 답하는데 질문하는 개발자가 구현하고자 하는 프로그래밍의 구조를 정확히 알지 못하고 질문하면 우리가 원하는 것을 얻지 못했다. 모르는 것을 묻더라도 정확히 무엇을 모르는지를 개발자가 알아야 질문을 통해 힌트라도 얻을 수 있다. 그래서 일반인들이 생각하는 자비스 수준의 ai가 나오지 않는한 개발자직업은 전망이 유망할 것이다. 또한 자비스가 나오더라도 내가 정말 원하는 것을 말로 표현하기란 정말 어렵다. 차라리 코드를 내가 직접 구현하는게 더 빠를 정도다. 
    - 즉, 물어보는 개발자가 정말 자기가 원하는 것이 무엇인지 대부분 모르기때문에 정확한 답을 얻기 어렵다는 것이다. 여러분 개인에게 마음속으로 물어보아라. 무엇을 구현하고자할때 무엇을 구현할지를 정확히 아는지를...(무엇을 구현할지 정확히 안다면 말로 표현하기 보다는 코드로 표현하는게 더 정확할 것이다. 사람의 말에는 중의성과 다양성 등이 존재하기에 말로써는 정확한 코드구현히 정말정말 힘들다. 말로만 코드를 구현하는 능력이 있는 사람은 개발자중에 최상위 수준의 사람만 가능하다고 생각한다. 그만큼 말로 코딩하는 것은 정말 어려운 일이다. 이는 내가 ai도움받아 리팩토링 하면서 느낀점이다)
  - seeder 구현은 정말로 러스트를 포기할까 생각이 많이든 시기였다. 책과 docs만 보고 바로 만들기 시작했다. 간단할줄만 알고 시작했는데 정말 너무 너무 어려웠다. 그래도 참고 하나씩 하나씩 에러를 잡다보니 만들게되었다. 여기서 좀더 나아가면 독립된 crates를 만들수 있다는 생각이 든다. 누가 내 코드를 보고 독립된 crates를 만들면 나를 언급해주면 고맙겠다. 일단 리팩토리에 최선을 다하고 그래도 아무도 안 만들면 독립된 crates를 만들어볼까 생각이 든다. (사실 90% 이상은 만들었기에... ㅎㅎㅎ)