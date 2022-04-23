# step2

- ```use``` 문을 사용해서 명시적으로 타입을 가져올 수 있다.
- io(input, output) lib은 std라 불리는 표준 라이브러리에 존재한다. (std::io)
- 러스트에서 변수는 기본적으로 불변이다.
- 가변변수는 변수 앞에 ```mut```이 붙는다. ```let mut var```
- 빈 String 인스턴스 생성: ```String::new()```
- read_line의 return 값은 io::Result이다.
- read_line의 Result에는 마지막에 친 Enter값 \n도 포함된다.
- Result의 variants는 Ok와 Err이다. Ok: success result, Err: fail reason
- 문자열에 값을 넣을때는 ```{}``` 변경자를 사용한다. ```println!("You guessed: {}", guess);```
- Crate: 러스트 package
- 크레이트는 실행가능한 binary create, 다른 프로그램을 위한 library create가 있다.
- crates.io: npmjs.com
- Cargo.lock: yarn.lock
- ```cargo update``` 명령어는 크레이트 업데이트를 위해 사용된다.
- ```extern crate [name];```으로 외부 의존 크레이트를 명시한다. ```use [name];``` 으로도 표기할 수 있다.
  - 명시 후에는 코드 내에서 ```[name]::```를 앞에 붙여 해당 패키지의 모든 걸 호출할 수 있다.
- ```use rand::Rng```, Rng는 정수 생성기 메소드를 정의한 trait이고 해당 메소드를 사용하려면 반드시 스코프 내에 있어야 한다. trait이 뭔지는 뒤에서 설명
- ```rand::thread_rng```, os가 시드를 정하고 현재 스레드에서만 사용하는 정수생성기를 만든다. ```gen_range``` 호출 시 파라미터의 2개 숫자 사이의 임의의 숫자를 생성한다. (min은 이상, max는 미만)
- ```cargo doc --open``` 명령어는 현재 프로젝트의 의존 패키지들의 문서를 브라우저에 표시해준다. (target/doc에 html이 생성됨)
- ```cmp``` 메소드는 두 값을 비교하는 메소드로 모든 비교 가능한 것들에서 호출 가능하다.
- ```match``` 문을 이용해 guess와 secret_number를 비교한 결과인 Ordering 값에 따라 뭘 할지 결정한다. (switch case 개념)
  ```rust
    match guess.cmp(&secret_number) {
      Ordering::Less => 작음,
      Ordering::Greater => 큼,
      Ordering::Equal => 같음,
    }
  ```
- 러스트는 강한 정적 타입 시스템을 가지고 있다.
- 러스트는 Shadowing이라는 개념이 있어서 이전에 선언한 값을 재선언 하는 걸 허용한다. 변수 값을 다른 타입으로 변환하고 싶을때 종종 사용된다.
- 문자열을 숫자로 변환하려면 String.parse 메소드를 호출하면 된다.
- parse 메소드도 Result를 반환한다.
- 변수에 타입을 명시할때는 변수명 뒤에 ```: [type]```을 추가한다. ```let guess: u32```
- ```loop``` 문은 무한루프를 제공한다.
- ```expect``` 메소드 호출을 ```match``` 문으로 변경하는건 종료에서 처리로 바꾸는 제일 일반적인 방법이다.
- parse 메소드가 Result(Ok 혹은 Err)를 return 하면 match 문으로 해당하는 케이스에 대응한다. 각 case에서 return 하는 값이 match 문의 return 값이 되는듯 하다.
  ```rust
    let guess: u32 = match guess.trim().parse() {
        Ok(num) => num,
        Err(_) => continue,
    };
  ```