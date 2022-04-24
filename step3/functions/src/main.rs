fn main() {
    println!("Hello, world!");

    another_function(5);
    println!("{}", five());
}

fn another_function(x: i32) {
    println!("Another function. x is: {}", x);
}

fn five() -> i32 {
    5
}