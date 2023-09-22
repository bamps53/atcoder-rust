use proconio::input;

fn main() {
    input!{
        n: usize,
        mut ab: [(usize, usize); n]
    }
    ab.sort_by(|x, y| x.1.cmp(&y.1));

    let mut t = 0;
    let mut ans = "Yes";
    for &(a, b) in &ab {
        t += a;
        if t > b {
            ans = "No";
            break;
        }
    }
    println!("{}", ans);

}

