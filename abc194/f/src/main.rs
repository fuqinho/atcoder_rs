use proconio::input;

const MOD: i64 = 1000000007;

fn main() {
    input! {
        in_s: String,
        k: usize,
    };
    let s: Vec<i64> = in_s
        .chars()
        .map(|c| c.to_digit(16).unwrap() as i64)
        .collect();

    // dp[i][j]: 以下の条件を満たす上位i桁の数
    // - Nの上位i桁より小さい
    // - ちょうどj種類の数字を使っている
    // - 0ではない
    let mut dp = vec![vec![0; 17]; s.len() + 1];
    let mut used_nums: u32 = 0;
    for i in 0..s.len() {
        for j in 0..=16 {
            // 今まで既に使ってた数字を使う
            dp[i + 1][j] += dp[i][j] * j as i64 % MOD;
            // 今まで使ってなかった数字を使う
            if j + 1 <= 16 {
                dp[i + 1][j + 1] += dp[i][j] * (16 - j) as i64 % MOD;
            }
        }
        // i桁目まではNと一致してる場合
        for x in 0..s[i] {
            if i == 0 && x == 0 {
                continue;
            }
            let kinds = (used_nums | 1 << x).count_ones() as usize;
            dp[i + 1][kinds] += 1;
        }
        // i桁目まで0の場合
        if i != 0 {
            dp[i + 1][1] += 15;
        }
        used_nums |= 1 << s[i];
    }
    let mut ans = dp[s.len()][k];
    if used_nums.count_ones() as usize == k {
        ans += 1;
    }
    println!("{}", ans % MOD);
}
