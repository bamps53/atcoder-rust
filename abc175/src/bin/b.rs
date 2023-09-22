use proconio::input;

fn main() {
    input!{
        n: usize,
        l:[usize; n]
    }
    let mut ans = 0;
    for i in 0..n {
        for j in i..n {
            for k in j..n {
                let a = l[i];
                let b = l[j];
                let c = l[k];
                if (a > b + c) | (b > a + c) | (c > a + b) {
                    ans+=1;
                }
            }
        }
    }
    println!("{}", ans);
}

