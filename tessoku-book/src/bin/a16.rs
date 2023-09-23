use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [i64; n-1],
        b: [i64; n-2],
    }
    let mut dp = vec![std::i64::MAX; n];
    dp[0] = 0;
    dp[1] = a[0];
    for i in 2..n {
        dp[i] = std::cmp::min(dp[i], dp[i-1] + a[i-1]);
        dp[i] = std::cmp::min(dp[i], dp[i-2] + b[i-2]);
    }
    println!("{}", dp[n-1]);
}
