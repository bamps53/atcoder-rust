use proconio::input;
use proconio::marker::Chars;

fn main() {
    input!{
        s: Chars
    }
    let mut max_r = 0;
    let mut nr = 0;
    for &c in &s {
        if c == 'R' {
            nr+=1;
        }
        else {
            max_r = std::cmp::max(max_r, nr);
            nr = 0;
        }
    }
    println!("{}", std::cmp::max(max_r, nr));

}

