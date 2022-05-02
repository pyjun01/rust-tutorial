# step8

- empty vector: ```let v: Vec<i32> = Vec::new();```
- 자동 타입 유추: ```let v = Vec![1,2,3];```
- Vector에 요소 추가는 push 메소드를 사용한다.
- Vector가 drop될때 요소들도 같이 drop 된다.
- 요소에 대한 접근은 두가지 경우로 가능하다.
- 전자는 프로그램이 죽게되고 후자는 None을 반환한다.
  ```rust
  let th: &i32 = &v[2];
  let th: Option<&i32> = v.get(2);
  ```
- 스트링 리터럴은 to_string 메소드로 String으로 변환할 수 있다.
- String::from과 to_string의 작동은 동일하다.
- 스트링의 합성은 + 연산자로 가능하다.
  ```rust
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // s1은 사용 불가능 해짐
  ```
- \+ 연산자 대신 format! 메크로를 사용하면 가독성도 좋고, ownership도 가져가지 않는다.
  ```rust
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = format!("{}-{}-{}", s1, s2, s3);
  ```
- String에 index 접근은 지원을 안한다.
- String은 Vec<u8>를 wrapping한 컬렉션이다.
- 