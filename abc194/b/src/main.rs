use proconio::input;
use std::cmp;

fn main() {
    input! {
        n: i32,
        abs: [(i32, i32); n],
    };

    let mut ans = abs[0].0 + abs[0].1;
    for i in 0..abs.len() {
        for j in 0..abs.len() {
            if i == j {
                ans = cmp::min(ans, abs[i].0 + abs[i].1);
            } else {
                ans = cmp::min(ans, cmp::max(abs[i].0, abs[j].1));
            }
        }
    }
    println!("{}", ans);
}
