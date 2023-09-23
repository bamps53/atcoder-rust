use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        a: [[usize; n]; m]
    }
    let mut dp = vec![vec![1000000; (1<<n)+1]; m+1];
    dp[0][0] = 0;

    for i in 0..m {
        let mut on = 0;
        for j in 0..n {
            if a[i][j] == 1 {
                on |= 1<<j;
            }
        }
        // println!("{}", on);
        for j in 0..1<<n {
            // 使わないパターン
            dp[i+1][j] = dp[i+1][j].min(dp[i][j]);

            // 使うパターン
            dp[i+1][j | on] = dp[i+1][j | on].min(dp[i][j] + 1);
        }
    }

    let mut ans = 100000;
    for i in 0..m+1 {
        ans = ans.min(dp[i][(1<<n)-1]);
        // println!("{:?}", dp[i]);
    }
    if ans == 100000 {
        ans = -1;
    }
    println!("{}", ans);
}
