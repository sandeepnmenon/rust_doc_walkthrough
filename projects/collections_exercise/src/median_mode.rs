use std::collections::HashMap;

fn median(v: &Vec<i32>) -> f32 {
    let len = v.len();
    println!("L: {} l/2 :{}", len, len / 2);
    let mut sorted_v = v.clone();
    sorted_v.sort();
    if len % 2 == 1 {
        sorted_v[len / 2] as f32
    } else {
        let sum = sorted_v[len / 2 - 1] + sorted_v[len / 2];
        (sum as f32) / 2.0
    }
}

fn mode(v: &Vec<i32>) -> i32 {
    let mut map = HashMap::new();

    for x in v {
        let count = map.entry(x).or_insert(0);
        *count += 1;
    }

    let mut mode = v[0];
    let mut max_count = 0;
    for (key, count) in map {
        if count > max_count {
            max_count = count;
            mode = *key;
        }
    }

    mode
}

pub fn median_mode(v: &Vec<i32>) -> (f32, i32) {
    (median(v), mode(v))
}
