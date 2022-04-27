# Step5
- struct(구조체): 튜플과 유사하지만 구성요소가 각자 다른 타입을 지닐 수 있다. (js object?)
- 구조체는 인스턴스를 생성해 사용하며 점(.) 표기법으로 필드에 접근할 수 있다.
  ```rust
  let user1 = User {
    email: String::from("email@email.com"),
    username: String::from("username"),
    active: true,
    count: 1
  }

  // user1.email
  ```
- struct debug를 위해 print시 {} 대신 {:#?}을 사용한다. (파일 상단에 ```#[derive(Debug)]``` 추가)
- struct의 메소드 선언을 위해서는 impl 문을 사용한다.
  ```rust
    struct Rectangle {
        length: u32,
        width: u32,
    }

    impl Rectangle {
        fn area(&self) -> u32 {
            &self.length * &self.width
        }
    }
  ```
- impl 내부에는 self 파라미터를 받지 않는 함수도 선언 가능하다. (연관함수라 부름)
- 연관함수는 새로운 구조체의 인스턴스를 반환하는 역할로 주로 사용된다.
  ```rust
    impl Rectangle {
      fn sqare(size: u32) -> Rectangle {
        Rectangle { length: size, width: size }
      }
    }

    // Rectangle::sqare(3);
  ```