use proconio::input;

fn main() {
    input!{
        n: usize,
        mut ab:[(usize, usize); n],
        mut cd:[(usize, usize); n],
    }
    // cdをx座標でソートし、順に取り出す（pとする）
    // abからp.xを下回っているものの中でyが最も高いものを取り出す
    // println!("{:?}", cd);
    cd.sort_by(|a, b| a.0.cmp(&b.0));
    // println!("{:?}", cd);

    // abはyで降順にソート
    ab.sort_by(|a, b| b.1.cmp(&a.1));

    let mut red_used = vec![false; n];

    for &(bx, by) in &cd {
        for (i, &(rx, ry)) in ab.iter().enumerate() {
            if red_used[i] == false && rx < bx && ry < by {
                red_used[i] = true;
                // println!("blue=({}, {}) red=({}, {})", bx, by, rx, ry);
                break;
            }
        }
    }

    // println!("{:?}", red_used);

    let mut ans = 0;
    for x in red_used {
        if x {ans+=1};
    }
    println!("{}", ans);


    

}

