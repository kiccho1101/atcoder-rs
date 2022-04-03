use proconio::input;

fn main() {
    input! {
        mut a: String,
        mut b: String,
    }
    a = a.chars().rev().collect::<String>();
    b = b.chars().rev().collect::<String>();
    let mut flag = false;
    for i in 0..a.len() {
        if i < b.len() {
            let a_int = a.chars().nth(i).unwrap().to_digit(10).unwrap();
            let b_int = b.chars().nth(i).unwrap().to_digit(10).unwrap();
            if a_int + b_int >= 10 {
                flag = true;
            }
        }
    }
    if flag {
        println!("Hard");
    } else {
        println!("Easy");
    }
}
