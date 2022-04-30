# Step7

- library crate는 --lib 플래그를 사용한다.
- 모듈 정의는 모두 ```mod```로 시작한다.
- 모듈 내부에 정의된 함수는 ::으로 사용한다. ```network::connect()```
- 모듈은 계층형으로 형성된다.
- ```mod [name];``` 형식으로만 선언해두면 러스트가 알아서 해당 name의 파일을 찾아서 모듈을 연결시켜준다.
- 모듈에 서브 모듈을 import 하려면 모듈의 이름으로 하는 dir에 mod.rs라는 이름의 파일로 선언하고, 서브 모듈 파일을 해당 dir에 추가해야 한다.
  - ```bar``` 모듈이 서브모듈을 가지고 있지 않다면 bar.rs 파일에 bar를 선언한다.
  - ```bar``` 모듈이 서브모듈을 가지고 있다면 bar/mod.rs 파일에 bar를 선언한다.
- 하나의 Cargo 프로젝트에서 바이너리 크레이트와 라이브러리 크레이트를 동시에 가지고 있을 수 있다.
- ```extern crate```는 루트 모듈에 있어야 한다.
- 러스트는 private가 기본 상태다.
- ```pub``` 으로 public 선언을 할 수 있다.
- public은 부모 모듈 어디에서든 접근이 가능하다.
- private는 동일한 파일 내에 있는 부모, 자식 모듈만 접근 가능하다.
- ```use```를 통해 모듈 함수 호출의 긴 경로를 줄일 수 있다.
  ```rust
  pub mod a {
      pub mod series {
          pub mod of {
              pub fn nested_modules() {}
          }
      }
  }

  use a::series::of;

  fn main() {
      of::nested_modules();
  }
  ```
- enum 또한 use로 가져올 수 있는데 여러 아이템을 가져올때는 Object destructuring 처럼 가져온다.
  ```rust
    use Enum::{Red, Yellow};
  ```
- 모두(glob) 가져올때는 *을 사용한다.
  ```rust
    use Enum::*;
  ```
- 모듈 경로를 ```::```으로 시작하면 root부터 시작한다.
- 모듈 경로에 ```super```를 붙이면 부모 모듈로 거슬러 올라갈 수 있다.
  ```rust
    ::client::connect();
    super::client::connect();
  ```