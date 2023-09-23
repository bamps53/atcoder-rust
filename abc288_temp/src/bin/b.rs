use proconio::input;
fn main() {
    input!{
        n: usize,
        k: usize,
        mut s: [String; k],
    }
    s.sort();
    for i in 0..k {
        println!("{}", s[i]);
    }
}
