use proconio::input;
use std::collections::BTreeMap;

fn main() {
    input! {
        n: usize,
        a: [i64; n]
    }
    
    let mut map = BTreeMap::new();
    for i in 0..n {
        map.insert(a[i], 0); // あとでvalueはindexで埋めるのでここではなんでもいい
    }
    for (i, (_k, v)) in map.iter_mut().enumerate() {
        *v = i + 1; // 1-index
    }
    for i in 0..n {
        if i > 0 {
            print!(" ");
        }
        print!("{}", map.get(&a[i]).unwrap());
    }
    println!();


}
