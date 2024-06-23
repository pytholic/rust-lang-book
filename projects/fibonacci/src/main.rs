fn main() {
    let res = get_nth_fibonacci(12);
    println!("{res}")
}

fn get_nth_fibonacci(n: i32) -> i32 {
    let mut a: i32 = 0;
    let mut b: i32 = 1;
    if n == 1 { return a; }
    if n == 2 { return b; }
    let mut sum: i32 = 0;
    for _ in 3..=n {
        sum = a + b;
        a = b;
        b = sum;
    }
    sum
}
