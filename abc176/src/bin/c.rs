use proconio::input;

fn main() {
    input!{
        n: usize,
        a: [usize; n]
    }
    let mut ans = 0;
    let mut max = 0;
    for &x in &a {
        if x > max {
            max = x;
        }
        else {
            ans += max - x;
        }
    }
    println!("{}", ans);
}

