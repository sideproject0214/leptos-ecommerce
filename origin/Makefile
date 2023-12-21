.PHONY: init-db check-db init quit-docker quit-check-db q 
# -z는 Bash 스크립트에서 사용되는 조건문 테스트 옵션 중 하나입니다. 이 옵션은
# 문자열의 길이가 0인지 여부를 확인하는데 사용됩니다.

# 옵션 -z는 주어진 변수의 값이 비어있으면(길이가 0이면) 참(true)으로 판단됩니다.
# 즉, 변수에 아무런 값이나 문자열이 없을 때 참으로 인식됩니다.

# -n은 Bash 조건문에서 문자열이 비어있지 않은지 확인하는 조건 테스트 옵션입니다.
# 이 옵션은 문자열의 길이가 0이 아니라면 참(true)으로 판단됩니다. 즉, 문자열이
# 비어있지 않으면 참으로 인식됩니다.

# 참고로 if와 [ 사이에는 공백이 있어야 합니다. 그렇지 않으면 Bash는 if [ ] 각
# 사이에 공백을 주지 않으면 이를 하나의 명령어로 해석하여 제대로 조건을 인식하지 못한다

# Makefile에서 명령을 실행할 때 출력을 숨기려면 명령 앞에 @를 사용합니다.
# 하지만 @은 모든 코드가 성공적으로 실행되었을때만 마지막에 추가하도록 한다. 
# 그렇지 않으면 에러가 발생했을 때 에러 메시지를 출력하지 않기 때문에 디버깅이 어렵다.
init-db:
	@container_id=$$(docker ps -aqf "name=db" --filter "status=running"); \
	if [ -n "$$container_id" ]; then \
		echo "❌ DB is running. docker run pass!"; \
	else \
		container_exist=$$(docker ps -aqf "name=db"); \
		if [ -n "$$container_exist" ]; then \
			echo "✅ DB is not running, Docker Run starting..."; \
			docker start $$(docker ps -aqf "name=db"); \
		else \
			echo "✅ DB is not exist, building and starting..."; \
			cd ./db && docker build -t db:v1 -f Dockerfile.dev . && docker run -p 5432:5432 --name db db:v1; \
		fi; \
	fi;

# Makefile에서 $$는 Makefile 내에서 사용되는 특별한 이스케이프 시퀀스입니다. 
# 보통은 $ 문자를 사용하여 셸 변수를 나타내지만, Makefile은 자체적으로 $ 문자를 특별하게 해석합니다. 
# 따라서 셸 스크립트에서는 $를 사용하여 변수를 참조하거나 연산을 할 때, 
# Makefile은 이를 인식하기 위해 $$로 이스케이프 처리합니다.
# 즉, $$는 Makefile에서 셸 스크립트를 실행할 때 $ 문자를 셸에게 전달하기 위한 방법입니다. 
# 만약 단순히 $로 사용하게 되면 Makefile은 이를 변수 참조로 해석하여 셸에게 전달하지 않고 
# 자체적으로 해석하려고 하기 때문에 이스케이프 처리를 해주어야 합니다.

# `docker ps -aqf "name=db"` 명령은 Docker에서 컨테이너를 검색하는데 사용됩니다.
# 여기서 사용된 각 옵션의 의미는 다음과 같습니다:

# - `docker ps`: 현재 실행 중인 Docker 컨테이너 목록을 표시합니다.
# - `-a`: 모든 컨테이너를 표시합니다. (실행 중이지 않은 컨테이너 포함)
# - `-q`: 컨테이너 ID만 표시합니다. 이 옵션을 사용하면 컨테이너 ID만 출력되므로,
#   컨테이너 ID를 다른 명령어의 입력으로 사용할 수 있습니다.
# - `-f "name=db"`: 필터 옵션으로, 특정 조건에 따라 컨테이너를 필터링합니다.
#   여기서는 컨테이너의 이름이 "db"인 것만 선택합니다.

# 따라서 `docker ps -aqf "name=db"` 명령은 이름이 "db"인 모든 Docker 컨테이너의
# ID를 출력합니다. 이 명령은 주로 셸 스크립트에서 컨테이너의 ID를 가져와 다른
# 명령어에 사용하거나 조작하는 데에 활용됩니다.

# exit: 프로그램이나 스크립트를 종료하는 명령어입니다. 보통은 상태 코드(종료
# 코드)를 함께 사용합니다. 이 상태 코드는 프로그램이나 스크립트가 성공적으로
# 종료되었는지 여부를 나타냅니다. 보통 0은 성공, 그 외의 값은 실패를 의미합니다.

# break: 주로 루프를 중단시키는 제어문입니다. break는 현재 실행 중인 반복문(for,
# while 등)을 종료하고 다음 코드로 이동합니다.

# exit 0: 프로그램이나 스크립트가 성공적으로 종료되었음을 나타내는 상태
# 코드(종료 코드) 0을 반환합니다. 일반적으로 이는 성공적인 종료를 의미합니다.

# exit 1: 프로그램이나 스크립트가 실패로 종료되었음을 나타내는 상태 코드(종료
# 코드) 1을 반환합니다. 일반적으로 이는 오류로 인한 종료를 나타냅니다.
check-db:
	@i=0;\
	while [ $$i -lt 30 ]; do \
		if [ $$(docker ps -qf "name=db" --filter "status=running") ]; then \
			echo "✅ Try $$(($$i+1)) : Database is ready!"; \
			exit 0; \
		else \
			echo "❌ Try $$i : Database is not ready yet. Retrying in 3 seconds..."; \
			sleep 3; \
			i=$$(($$i+1)); \
		fi; \
	done;


# & 기호는 백그라운드에서 명령을 실행하라는 의미입니다. 이를 통해 해당 명령을
# 실행한 후에도 다음 명령을 기다리지 않고 다른 작업을 실행할 수 있습니다.

# 여기서 &&은 하나의 명령어가 성공적으로 실행되면 다음 명령어를 실행하라는 것을
# 나타냅니다. 따라서 cd ./api && cargo watch -x run &의 경우, cd ./api
# 디렉토리로 이동한 다음에 cargo watch -x run을 실행하고 이를 백그라운드에서
# 실행합니다.


init: init-db check-db
	@cd ./api && cargo watch -x run &
	@cd ./client && cargo leptos watch &

# quit 명령어
# 이때 조심할 것은 \ 다음에 공백이 없어야 한다는 것입니다. 공백이 있으면 에러가 난다.
quit-docker:
	@container_exist=$$(docker ps -aqf "name=db" --filter "status=running"); \
	if [ -z "$$container_exist" ]; then \
		echo "✅ DB already stop..."; \
	else \
		echo "✅ DB stop success"; \
		docker stop db; \
	fi;

quit-check-db:
	@i=0; \
	while [ $$i -lt 30 ]; do \
		if [ $$(docker ps -qf "name=db" --filter "status=running") ]; then \
			echo "❌ Try $$i : DB is running... Retrying in 1 seconds..."; \
			sleep 1; \
			i=$$(($$i+1)); \
		else \
			echo "✅ Try $$(($$i+1)) : DB stop"; \
			exit 0; \
		fi; \
	done;

q: quit-docker quit-check-db
	@pkill -f "cargo-watch" & 
	@pkill -f "leptos" & 

