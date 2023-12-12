fn main() {
    let a = [1; 5];

    println!("{:?}", a[2..4].iter().sum::<i32>());
}
