use proconio::input;

fn main() {
    input!{
        n: usize,
        ab: [(usize, usize); n]
    }
    let mut ans = 0;
    for &(a, b) in &ab {
        ans += (b - a + 1) * (a + b) / 2;
    }
    println!("{}", ans);
}

