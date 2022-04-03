fn main() {
    proconio::input! {
        mut n: usize,
    }
    if n >= 42 {
        n += 1;
    }
    println!("{}", format!("AGC{:03}", n));
}
