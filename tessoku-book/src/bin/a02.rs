use proconio::input;
use std::process::exit;

fn main() {
    input! {
        n: usize,
        x: usize,
        a: [usize; n],
    }
    for &b in &a {
        if b == x {
            println!("Yes");
            exit(0);
        }
    }
    println!("No");
}
