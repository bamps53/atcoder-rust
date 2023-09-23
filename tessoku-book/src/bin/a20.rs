use proconio::{input, marker::Bytes};

fn main() {
    input! {
        s: Bytes,
        t: Bytes
    }

    // println!("{:?}", s);
    // println!("{:?}", t);

    let mut dp = vec![vec![0; t.len() + 1]; s.len() + 1];
    
    for i in 0..s.len() {
        for j in 0..t.len() {
            dp[i+1][j+1] = std::cmp::max(dp[i+1][j+1], dp[i+1][j]);
            dp[i+1][j+1] = std::cmp::max(dp[i+1][j+1], dp[i][j+1]);
            if s[i] == t[j] {
                dp[i+1][j+1] = std::cmp::max(dp[i+1][j+1], dp[i][j] + 1);
            }
            else {
                dp[i+1][j+1] = std::cmp::max(dp[i+1][j+1], dp[i][j]);
            }
        }
    }
    println!("{}", dp[s.len()][t.len()]);
}

// use proconio::{input, marker::Bytes};

// fn main() {
//     input! {
//         s: Bytes,
//         t: Bytes,
//     }
//     let mut dp = vec![vec![0; t.len() + 1]; s.len() + 1];
//     for i in 0..=s.len() {
//         for j in 0..=t.len() {
//             dp[i + 1][j] = dp[i + 1][j].max(dp[i][j]);
//             dp[i][j + 1] = dp[i][j + 1].max(dp[i][j]);
//             if s[i] == t[j] {
//                 dp[i + 1][j + 1] = dp[i + 1][j + 1].max(dp[i][j] + 1);
//             }
//         }
//     }
//     println!("{}", dp[s.len()][t.len()]);
// }
