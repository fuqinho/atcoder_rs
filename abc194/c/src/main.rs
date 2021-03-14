use proconio::input;

fn main() {
    input! {
        n: usize,
        a_s: [i32; n],
    };

    let mut cnt = vec![0; 401];
    for a in a_s {
        cnt[(a + 200) as usize] += 1;
    }
    let mut ans: i64 = 0;
    for i in 0..cnt.len() {
        for j in 0..cnt.len() {
            let diff = i as i64 - j as i64;
            ans += cnt[i] * cnt[j] * diff * diff;
        }
    }
    ans /= 2;
    println!("{}", ans);
}
