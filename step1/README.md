# Step1

- main 함수는 러스트 프로그램의 첫 실행 함수이다.
- 함수에 !를 붙이면(println -> println!) 러스트 매크로가 된다. (매크로가 뭔지는 뒤에서 다룬다.)
- 컴파일과 실행이 분리되어 있다.
- ```rustc main.rs ``` 실행시 main.rs를 컴파일해 main 바이너리 파일이 생성된다. main을 실행하면 Hello, world!가 출력된다.
- 생성된 바이너리 파일은 따로 러스트가 설치되어 있지 않은 컴퓨터에서도 실행할 수 있다.

## Cargo
rust의 빌드 시스템 및 패키지 매니저이다. 이거만 보면 Javscript로 치면 node+npm 느낌인듯 하다.

- ```cargo new [name] --bin``` 명령어로 cargo 애플리케이션을 생성할 수 있다.
- Cargo.toml: cargo의 환경설정 포맷. package.json과 유사한 듯 하다.
- ```cargo build``` 명령어로 target/debug 폴더에 실행 파일을 생성한다.
- ```cargo run``` 명령어로 코드를 컴파일 후 바로 실행할 수 있다.
- ```cargo check```  명령어는 러스트 코드가 컴파일이 가능한지 확인해준다.
- cargo의 추가적인 장점은 운영체제에 제한없이 커맨드가 동일하다는 점
- ```cargo build --release``` 명령어로 target/release 폴더에 배포용 최적화된 실행 파일을 생성한다.