use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        a: [usize; n]
    }

    let is_ok = |left: usize, right: usize| -> bool {
        if right >= n {
            return false;
        }
        if a[right] - a[left] <= k {
            return true;
        }
        else {
            return false;
        }
    };

    let mut ans = 0;
    let mut right = 0;
    for left in 0..n-1 {
        if left > right {
            right = left;
        }
        while is_ok(left, right+1) {
            right += 1;
        }
        ans += right - left;
        // println!("{} {} {}", left, right, right - left);

    }
    println!("{}", ans);
}
