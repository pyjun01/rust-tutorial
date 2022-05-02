fn main() {
    let arr = [1,2,3,4,5,6];

    let avg: i32 = (arr.to_vec().iter().sum()) / arr.len();
    
    println!("{}", avg);
}
