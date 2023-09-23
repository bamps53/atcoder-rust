use proconio::input;

fn main() {
    input! {
        n: usize,
    }
    for i in (0..10).rev() {
        let x = n >> i;
        print!("{}", x % 2);
    }
    println!("");
}
