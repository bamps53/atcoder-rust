use proconio::input;

fn main() {
    input! {
        h: usize,
        w: usize,
        n: usize,
        abcd: [(usize, usize, usize, usize); n]
    }

    let mut cumsum = vec![vec![0; w+2]; h+2];
    for &(a, b, c, d) in &abcd {
        cumsum[a][b] += 1;
        cumsum[c+1][b] -= 1;
        cumsum[a][d+1] -= 1;
        cumsum[c+1][d+1] += 1;
    }

    // 横方向
    for i in 0..h {
        for j in 0..w {
            cumsum[i+1][j+1] += cumsum[i+1][j];
        }
    }

    // 縦方向
    for j in 0..w {
        for i in 0..h {
            cumsum[i+1][j+1] += cumsum[i][j+1];
        }
    }

    // 出力
    for i in 0..h {
        for j in 0..w {
            if j > 0 {print!(" ")};
            print!("{}", cumsum[i+1][j+1]);
        }
        println!("");
    }
}
