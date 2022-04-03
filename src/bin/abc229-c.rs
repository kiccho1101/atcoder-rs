use proconio::*;
use superslice::*;

fn main() {
    input! {
        n: usize,
        q: usize,
        mut a: [u32; n],
        x: [u32; q],
    }
    a.sort();
    for x in x {
        let x = a.lower_bound(&x);
        println!("{}", n - x);
    }
}
