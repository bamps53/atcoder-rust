use proconio::input;

fn main() {
    input!{
        n: usize
    }
    let x = n % 10;

    let ans = match x {
        2 | 4 | 5 | 7 | 9 => "hon",
        3 => "bon",
        _ => "pon"
    };
    println!("{}", ans);
}

