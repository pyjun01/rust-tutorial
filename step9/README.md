# step9

- recoverable(복구 가능), unrecoverable(복구 불가능) 이렇게 두 종류의 에러가 있다.
- ```panic!``` 매크로가 실행되면 프로그램을 종료한다. (실패 메시지 출력, 스택 되감기 및 청소)
- ```RUST_BACKTRACE=1 cargo run``` RUST_BACKTRACE를 설정해 백트레이스를 할 수 있다.
- recoverable 에러는 Result 타입을 반환한다.
  - 성공하면 Ok 인스턴스, 실패할 경우 Err 인스턴스가 된다.
  ```rust
    enum Result<T, E> {
        Ok(T),
        Err(E),
    }
  ```
- ```io::Error``` 구조체는 ```kind``` 메소드를 제공하고 ```io::ErrorKind``` 값을 얻을 수 있다.
- ```match```에는 매치 가드라는 추가 조건을 추가할 수 있다.
  ```rust
    match f {
      Ok(file) => file,
      Err(ref error) if error.kind() == ErrorKind::NotFound => { // match guard
        match File::create("hello.txt") {
          Ok(fc) => fc,
          Err(e) => {
            panic!("Tried to create file but there was a problem: {:?}", e);
          },
        }
      },
      Err(error) => {
        panic!("There was a problem opening the file: {:?}", error);
      }
    };
  ```
- ```Result``` 값 뒤에 ?를 붙이면 Ok라면 값을 받아오고, Err라면 바로 Err 값을 return 한다.
- ?는 ```Result```를 반환하는 함수 내에서만 사용 가능하다.