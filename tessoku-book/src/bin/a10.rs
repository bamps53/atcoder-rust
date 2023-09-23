use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
        d: usize,
        lr: [(usize, usize); d]
    }

    let mut left_max = vec![0; n+1];
    let mut right_max = vec![0; n+1];

    for i in 0..n {
        left_max[i+1] = std::cmp::max(left_max[i], a[i]);
    }
    for i in (0..n).rev() {
        right_max[i] = std::cmp::max(right_max[i+1], a[i]);
    }

    for &(l, r) in &lr {
        println!("{}", std::cmp::max(left_max[l-1], right_max[r]));
    }

    // println!("{:?}", left_max);
    // println!("{:?}", right_max);
}
