use proconio::input;

fn main() {
    input! {
        n: usize,
        s: usize,
        a: [usize; n]
    }
    let num = 10000_usize;

    let mut dp = vec![vec![0; num+1]; n];
    dp[0][0] = 1;
    dp[0][a[0]] = 1;
    for i in 1..n {
        for j in 0..num {
            if dp[i-1][j] == 1 {
                // i個目を選ばないパターン
                dp[i][j] = 1; 

                // i個目を選ぶパターン
                if j + a[i] <= num {
                    dp[i][j+a[i]] = 1;
                }
            }
        }
    }

    let mut ans = "No";
    for i in 0..n {
        // println!("{:?}", dp[i]);
        if dp[i][s] == 1 {
            ans = "Yes";
        }
    }
    println!("{}", ans);
}
