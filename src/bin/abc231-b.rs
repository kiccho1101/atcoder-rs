use std::collections::HashMap;

use proconio::input;

fn main() {
    input! {
        n: i32,
        ss: [String; n],
    }
    let mut votes: HashMap<String, usize> = HashMap::new();
    for s in ss.iter() {
        *votes.entry(s.to_string()).or_insert(0) += 1;
    }

    let mut votes: Vec<(String, usize)> = votes.into_iter().collect();
    votes.sort_by_key(|x| x.1);
    votes.reverse();

    println!("{}", votes[0].0);
}
