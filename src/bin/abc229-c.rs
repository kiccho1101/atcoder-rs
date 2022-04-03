use proconio::input;

fn main() {
    input! {
        n: usize,
        mut w: u64,
        mut p: [(u64, u64); n],
    }
    p.sort_by_key(|p| !p.0);
    let mut ans = 0;
    for (a, b) in p {
        let v = b.min(w);
        ans += v * a;
        w -= v;
    }
    println!("{}", ans);
}
