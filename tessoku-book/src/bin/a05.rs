use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
    }
    let mut count = 0;
    for i in 1..k-1 {
        for j in 1..k-i {
            if (i <= n) & (j <= n) & (k-i-j <= n) {
                count+=1;
            }
        }
    }
    println!("{}", count);
}
