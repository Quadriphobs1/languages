use std::collections::HashMap;

fn mean(data: &Vec<i32>) -> i32 {
    let sum: i32 = data.iter().sum();
    (sum as f32 / data.len() as f32) as i32
}

fn median(data: &mut Vec<i32>) -> i32 {
    data.sort();
    let mid = data.len() / 2;
    if data.len() % 2 == 0 {
        mean(&mut vec![data[mid - 1], data[mid]]) as i32
    } else {
        data[mid]
    }
}

fn mode(data: &Vec<i32>) -> Vec<i32> {
    let mut map = HashMap::new();

    for i in data {
        let c = map.entry(i).or_insert(0);
        *c += 1;
    }

    let max = map.values().cloned().max().unwrap_or(0);

    map.into_iter()
        .filter(|&(_, v)| v == max)
        .map(|(&k, _)| k)
        .collect()
}

fn main() {
    let mut d = vec![
        2, 4, 5, 1, 4, 6, 4, 6, 3, 9, 8, 5, 6, 3, 4, 2, 5, 1, 10, 3, 5, 10, 45, 5, 6, 6,
    ];
    let mn = mean(&d);
    let md = median(&mut d);
    let mde = mode(&d);

    println!("MEAN:  {}", mn);
    println!("MEDIAN: {}", md);
    println!("MODE: {:?}", mde);
}
