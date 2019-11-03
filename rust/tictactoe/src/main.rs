fn bingo(a: &[i32]) -> String {
    let mut result = true;
    let mut bingo = [2, 9, 14, 7, 15];

    for (_, elem) in bingo.iter_mut().enumerate() {
        if !a.contains(elem) {
            result = false;
            break;
        }
    }
    if result {
        return "WIN".to_string();
    } else {
        return "LOSE".to_string();
    }
}

fn main() {
    let a = bingo(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
    let b = bingo(&[21, 13, 2, 7, 5, 14, 7, 15, 9, 10]);
    println!("{}", a); // LOSE
    println!("{}", b); // WIN
}
