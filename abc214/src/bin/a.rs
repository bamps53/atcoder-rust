use proconio::input;

fn main() {
    input!{
        n: usize,
    }
    let ans = match n {
        x if x < 126 => 4,
        x if x < 212 => 6,
        _ => 8
    };
    println!("{}", ans);
}

