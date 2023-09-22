use proconio::input;
use std::process::exit;

fn main() {
    input!{
        x: u64,
        y: u64
    }
    for i in 0..x+1 {
        if (i * 2 + (x-i) * 4 == y)
        {
            println!("Yes");
            exit(0);
        }
    }
    println!("No");
}

