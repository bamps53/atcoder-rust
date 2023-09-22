use proconio::input;

fn main() {
    input!{
        n: usize,
        xs: [f64; n]
    }
    let mut d1: f64 = 0.0;
    let mut d2: f64 = 0.0;
    let mut d3: f64 = 0.0;
    for &x in &xs {
        d1 += x.abs();
        d2 += x * x;
        d3 = d3.max(x.abs());
    }
    let d2_sq = d2.sqrt();
    println!("{}", d1);
    println!("{}", d2_sq);
    println!("{}", d3);

}
