use proconio::input;

const A_MAX: usize = 1500000;

fn check(idx: &Vec<usize>, n: usize, m: usize) -> bool {
    if idx.len() == 0 {
        return true;
    }
    if idx[0] >= m {
        return true;
    }
    if n - idx[idx.len() - 1] > m {
        return true;
    }
    for i in 0..idx.len() - 1 {
        if idx[i + 1] - idx[i] > m {
            return true;
        }
    }
    false
}

fn main() {
    input! {
        n: usize,
        m: usize,
        a: [i32; n],
    }
    // idx[x]: list of i where a[i] = x.
    let mut idx = vec![vec![]; A_MAX + 1];
    for i in 0..n {
        idx[a[i] as usize].push(i);
    }
    let mut ans = A_MAX + 1;
    for i in 0..A_MAX + 1 {
        if check(&idx[i], n, m) {
            ans = i;
            break;
        }
    }
    println!("{}", ans);
}
