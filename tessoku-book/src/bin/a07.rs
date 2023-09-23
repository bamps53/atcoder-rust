use proconio::input;

fn main() {
    input! {
        d: usize,
        n: usize,
        lr: [(usize, usize); n]
    }
    // 来た日と抜けた日を記録していく
    // l日目に+1, r日目に-1
    let mut cum = vec![0; d+2];
    for &(l, r) in &lr {
        cum[l] += 1;
        cum[r+1] -= 1;
        // println!("{:?}", cum);
    }

    // 累積和を取って出力
    for i in 1..d+1 {
        cum[i] = cum[i] + cum[i-1];
        println!("{}", cum[i]);
    }

}
