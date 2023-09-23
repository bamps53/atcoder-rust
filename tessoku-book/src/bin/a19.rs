use proconio::input;

fn main() {
    input! {
        n: usize,
        w_max: usize,
        wv: [(usize, usize); n]
    }

    let mut dp = vec![vec![0; w_max+1]; n+1];

    for i in 0..n {
        let (w, v) = wv[i];
        for j in 0..w_max {
            // 選ばないパターン
            dp[i+1][j] = std::cmp::max(dp[i+1][j], dp[i][j]);

            // 選ぶパターン
            if j+w <= w_max {
                dp[i+1][j+w] = std::cmp::max(dp[i+1][j+w], dp[i][j] + v);
            }
        }
    }

    let mut ans = 0;
    for j in 0..w_max+1 {
        ans = std::cmp::max(ans, dp[n][j]);
    }
    println!("{}", ans);
    // println!("{:?}", dp[n]);




}
