use proconio::*;

fn bin_search(a: &Vec<u32>, target: u32) -> isize {
    let mut left: isize = -1;
    let mut right = a.len() as isize;
    while right - left > 1 {
        let mid = (left + right) / 2;
        if a[mid as usize] < target {
            left = mid;
        } else {
            right = mid;
        }
    }
    return right;
}

fn main() {
    input! {
        n: usize,
        q: usize,
        mut a: [u32; n],
        xx: [u32; q],
    }
    a.sort();
    for x in xx.iter() {
        let x = bin_search(&a, *x);
        println!("{}", n - x as usize);
    }
}
