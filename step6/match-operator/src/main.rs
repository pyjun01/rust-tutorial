enum Coin {
    Penny(u32),
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u32 {
    match coin {
        Coin::Penny(v) => {
            println!("{}", v);
            v
        },
        Coin::Nickel => 5,
        _ => 1,
    }
}


fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn main() {
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    println!("{}", value_in_cents(Coin::Penny(111)));

    let some_value = Some(100);

    if let Some(0) = some_value {
      println!("Yes");
    } else {
      println!("Nob");
    }
}
