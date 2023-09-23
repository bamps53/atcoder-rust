use proconio::input;

fn main() {
    input! {
        n: isize,
        k: isize,
        a: [isize; n]
    }

    let is_ok = |t: isize| -> bool {
        let mut num = 0;
        for x in &a {
            num += t / x;
        }
        // println!("num={}", num);
        if num >= k {
            return true;
        }
        else {
            return false;
        }
    };

    let binary_search = || -> isize {
        let mut ng: isize = 0;
        let mut ok: isize = 1_000_000_010;
        while (ok - ng).abs() > 1 {
            let mid = (ok + ng) / 2;

            if is_ok(mid) {
                ok = mid;
            }
            else {
                ng = mid;
            }
        }
        return ok;
    };

    println!("{}", binary_search());
}
