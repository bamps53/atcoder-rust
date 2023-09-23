use proconio::input;

macro_rules! print_vec {
    ($vec:expr) => {
        let result = $vec.iter()
                         .rev()
                         .map(|x| x.to_string())
                         .collect::<Vec<String>>()
                         .join(" ");
        println!("{}", result);
    };
}

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
    // println!("{}", dp[n-1]);
    // println!("{:?}", dp);
    
    let mut v = Vec::new();
    let mut pos = n-1;
    v.push(pos+1);
    while pos >= 1 {
        if dp[pos] - dp[pos-1] == a[pos-1] {
            pos = pos - 1;
        }
        else {
            pos = pos - 2;
        }
        v.push(pos+1);
    }
    println!("{}", v.len());
    print_vec!(v);
    


}
