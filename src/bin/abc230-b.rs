fn main() {
    proconio::input! {
        s: String,
    }
    let t = "oxx".repeat(100);
    if t.contains(s.as_str()) {
        println!("Yes");
    } else {
        println!("No");
    }
}
