use proconio::input;

fn main() {
    input! {
        n: usize,
        mut pa: [(usize, usize); n]
    }
    // println!("{:?}", pa);

    // for (p, a) in &mut pa {
    //     *p -= 1; // 0-index
    // }

    // println!("{:?}", pa);

    let mut dp = vec![vec![0; n+1]; n+1];
    for l in 0..n {
        for r in 0..n {
            // 左から取り除くかどうか
            let (p, a) = pa[l];
            // println!("{} {} {} {}", p-1, a, l, n - 1 - r);
            if l <= p - 1 && n - 1 - r >= p - 1 {
                dp[l+1][r] = std::cmp::max(dp[l+1][r], dp[l][r] + a);
            }
            else {
                dp[l+1][r] = std::cmp::max(dp[l+1][r], dp[l][r]);
            }

            // 右から取り除くかどうか
            let (p, a) = pa[n - 1 - r];
            if l <= p - 1 && n - 1 - r >= p - 1 {
                dp[l][r+1] = std::cmp::max(dp[l][r+1], dp[l][r] + a);
            }
            else {
                dp[l][r+1] = std::cmp::max(dp[l][r+1], dp[l][r]);
            }
        }
        // println!("{:?}", dp[l]);
    }

    let mut ans = 0;
    for i in 0..n {
        ans = std::cmp::max(ans, dp[i][n-1-i]);
    }
    println!("{}", ans);

}
