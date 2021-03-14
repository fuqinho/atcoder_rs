use proconio::input;

fn main() {
    input! { n: i32 };

    let mut e: f64 = 0.0;
    for i in 0..n - 1 {
        e += n as f64 / (n - 1 - i) as f64;
    }
    println!("{}", e);
}
