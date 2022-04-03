use proconio::input;

fn main() {
    input! {
        field: [String; 2]
    }

    let mut flag = true;

    // 0,0
    if field[0].chars().nth(0).unwrap() == '#' {
        if field[0].chars().nth(1).unwrap() == '.' && field[1].chars().nth(0).unwrap() == '.' {
            flag = false;
        }
    }

    // 0,1
    if field[0].chars().nth(1).unwrap() == '#' {
        if field[0].chars().nth(0).unwrap() == '.' && field[1].chars().nth(1).unwrap() == '.' {
            flag = false;
        }
    }

    // 1,0
    if field[1].chars().nth(0).unwrap() == '#' {
        if field[0].chars().nth(0).unwrap() == '.' && field[1].chars().nth(1).unwrap() == '.' {
            flag = false;
        }
    }

    // 1,1
    if field[1].chars().nth(1).unwrap() == '#' {
        if field[0].chars().nth(1).unwrap() == '.' && field[1].chars().nth(0).unwrap() == '.' {
            flag = false;
        }
    }

    if flag {
        println!("Yes");
    } else {
        println!("No");
    }
}
