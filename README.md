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