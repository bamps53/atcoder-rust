use proconio::input;

fn main() {
    input! {
        h: usize,
        w: usize,
        x: [[usize; w]; h],
        q: usize,
        abcd: [(usize, usize, usize, usize); q],
    }

    let mut cumsum = vec![vec![0; w+1]; h+1];

    // 横方向の累積和
    for i in 0..h {
        for j in 0..w {
            cumsum[i+1][j+1] = cumsum[i+1][j] + x[i][j];
        }
    }

    // 縦方向の累積和
    for j in 0..w {
        for i in 0..h {
            cumsum[i+1][j+1] += cumsum[i][j+1];
        }
    }

    // println!("{:?}", cumsum);

    // cumsumの各マスには、(0, 0)~(i, j)の領域の和が入っている
    for k in 0..q {
        let &(a, b, c, d) = &abcd[k];
        let ans = cumsum[c][d] + cumsum[a-1][b-1] - cumsum[c][b-1] - cumsum[a-1][d] ;
        println!("{}", ans); 
    }
}
