# Step6

- rust의 enum은 ts의 enum과 유사하다.
- 대신 variant 선택은 ::으로 한다.
  ```rust
    enum IpAddrKind {
      V4,
      V6,
    }

    // IpAddrKind::V4
  ```
- enum variant에 데이터를 직접 넣어서 관리할 수 있다.
  ```rust
    enum IpAddrKind {
      V4(String),
      V6(String),
    }

    // IpAddrKind::V4(String::from("127.0.0.1")));
  ```
- 각 variant에 다른 타입, 다른 양이 지정 가능하다.
- 실제 IpAddr 표준 라이브러리는 아래처럼 정의한다.
  ```rust
    struct Ipv4Addr {
        // details elided
    }

    struct Ipv6Addr {
        // details elided
    }

    enum IpAddr {
        V4(Ipv4Addr),
        V6(Ipv6Addr),
    }
  ```
- enum에도 impl 문을 사용해 메소드 선언이 가능하다.
- rust에는 Null이 없다.
- nullable 개념을 Option 으로 대신 사용하는 듯 하다.
- switch case와 비슷한 match 연산자가 존재한다.
- match의 enum에서 variant에 선언된 데이터는 함수처럼 괄호를 열어서 사용할 수 있다.
- match는 모든 경우의 수에 대응해야 한다.
- 만약 일부 경우의 수만 대응하고 나머지는 대응하고 싶지 않다면 _ 언더바 패턴을 사용할 수 있다. _는 모든 값과 매칭된다.
  ```rust
    let some_u8_value = 0u8;
    match some_u8_value {
      1 => println!("one"),
      3 => println!("three"),
      5 => println!("five"),
      7 => println!("seven"),
      _ => (),
    }
  ```
- ```if let``` 문은 한가지 경우에만 대응하는 match를 사용하고 싶을때 사용할 수 있다.
  ```rust
    let some_u8_value = Some(0u8);

    match some_u8_value {
      Some(3) => println!("three"),
      _ => (),
    }

    // 위 코드와 동일하게 동작
    if let Some(0u8) = some_u8_value {
      println!("three");
    }
  ```
- ```if let```에는 ```else```도 추가 가능하다.
  ```rust
    let some_value = Some(100);

    if let Some(0) = some_value {
      println!("Yes");
    } else {
      println!("Nob");

    }
  ```