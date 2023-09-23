use proconio::input;
use std::collections::HashSet;

fn main() {
    input! {
        n: usize,
        k: i64,
        a: [i64; n],
        b: [i64; n],
        c: [i64; n],
        d: [i64; n],
    }

    let mut p = HashSet::new();
    let mut q = HashSet::new();

    for x in &a {
        for y in &b {
            p.insert(x + y);
        }
    }

    for x in &c {
        for y in &d {
            q.insert(x + y);
        }
    }

    let mut ans = "No";
    for x in &p {
        if q.contains(&(k - x)) {
            ans = "Yes";
        }
    }
    println!("{}", ans);
}
