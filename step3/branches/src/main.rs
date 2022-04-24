fn main() {
    let number = if true {
        5
    } else {
        6
    };

    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    let a = [10, 20, 30, 40, 50];

    for element in a.iter() {
        println!("the value is: {}", element);
    }

    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("LIFTOFF!!!");
}
