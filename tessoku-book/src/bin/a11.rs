use proconio::input;

fn main() {
    input! {
        n: usize,
        x: usize,
        a: [usize; n]
    }

    let is_ok = |index: usize, key: usize| -> bool {
        if a[index] >= key {
            return true;
        }
        else {
            return false;
        }
    };
    
    let binary_search = ||-> usize {
        let mut left = 0;
        let mut right = n;
        while right - left > 1 {
            let pos = (left + right) / 2;
            if is_ok(pos, x) {
                right = pos;
            }
            else {
                left = pos;
            }
        }
        return right;
    };

    println!("{}", binary_search()+1);
}
