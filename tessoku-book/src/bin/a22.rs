use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n-1],
        b: [usize; n-1],

    }

    let mut dp = vec![0; n+1];
    for i in 0..n-1 {
        dp[a[i]] = dp[a[i]].max(dp[i+1] + 100);
        dp[b[i]] = dp[b[i]].max(dp[i+1] + 150);
    }
    println!("{}", dp[n]);
}
