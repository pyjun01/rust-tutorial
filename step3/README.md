# Step3

- 변수가 불변성을 지킴으로써 변수의 값이 어떻게 변경될지 찾아볼 필요가 없다.
- 상수 변수 다른점
  1. 상수는 mut을 허용 안한다.
  2. 상수는 타입이 필수다.
- 변수를 재선언 하는걸 shadow라 한다.
  ```rust
    let x = 5;
    let x = x + 1;
  ```
- shadow와 mut을 사용하는것의 차이는 shadow는 값의 타입을 변경할 수 있지만 mut은 변경할 수 없다.
- 타입은 스칼라, 컴파운드 둘로 나뉜다.
- String을 parse해 숫자로 변환하는 경우 처럼 타입의 선택 폭이 넓은 경우는 반드시 타입의 명시를 첨가해야 합니다.
  ```rust
    let guess: u32 = "42".parse().expect("Not a number!"); // Ok
    let guess = "42".parse().expect("Not a number!"); // Err
  ```
- 스칼라는 하나의 값을 표현하는 타입으로 정수, 부동소숫점, 불린, 문자 이렇게 4가지가 있다.
- 정수형: 소수점이 없는 숫자. signed: i8, i16, i32, i64, isize. unsigned: u8, u16, u32, u64, usize
- isize, usize는 컴퓨터 환경의 bit를 따라간다.
- 정수형의 기본 타입은 i32이다.
- 부동 소수점: 소수점을 갖는 숫자. f32, f64가 있다.
- 불린은 bool로 명시한다.
- 튜플과 배열이라는 복합 타입을 가지고 있다.
- 튜플은 요소들이 서로 다른 타입이어도 된다.
  ```rust
    let tup: (i32, f64, u8) = (500, 6.4, 1);

    let (x, y, z) = tup;

    let xx = tup.0;
    let yy = tup.1;
    let zz = tup.2;

    println!("The value of y is: {}", y);
  ```
-  튜플 값의 접근은 ```[var].[index]``` 형식으로 접근한다.
- 배열은 요소가 모두 같은 타입이어야 한다.
- 배열의 길이는 고정이여서 크기를 조정할 수 없다.
- 배열의 길이보다 큰 index의 요소에 접근하면 런타임에서 에러가 발생한다.
- 러스트는 snake case를 사용한다.
- 함수는 선언의 순서와 상관없이 호출해서 사용가능하다.
- 함수의 매개변수는 타입 지정이 필수다.
- 표현식은 세미콜론을 사용하지 않는다. 세미콜론을 사용하면 문으로 변한다.
- block을 식으로 사용하는데 iife 느낌이다.
  ```rust
    let y = { // y = 6
      let x = 5;
      x + 1
    };
  ```
- 함수에서 값을 반환하려면 반환 타입을 명시해야 한다.
- 반환 값은 return의 값 혹은 함수 본문의 마지막 표현식 값이다. (아래 함수는 5를 반환한다.)
  ```rust
    fn five() -> i32 {
      5
    }
  ```
- if 표현식은 괄호 없이 ```if [조건식] { ...``` 형식이다.
- 조건식은 무조건 bool이어야 한다.
- if 식을 변수 선언부에 넣어서 값으로 쓸 수 있다.
  ```rust
    let number = if true {
        5
    } else {
        6
    };
  ```
- 만약 if else가 서로 다른 타입을 반환할경우 에러가 난다.
- loop는 무한루프를 도는 반복문이다.
- for 반복문은 for in loop 형식이다.
  ```rust
    let a = [10, 20, 30, 40, 50];

    for element in a.iter() {
        println!("the value is: {}", element);
    }

    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("LIFTOFF!!!");
  ```