use std::collections::HashMap;

fn median(v: &Vec<i32>) -> f32 {
    let mid = v.len() / 2;
    let mut sorted_v = v.clone();
    sorted_v.sort();
    if v.len() % 2 == 1 {
        sorted_v[mid] as f32
    } else {
        let sum = sorted_v[mid - 1] + sorted_v[mid];
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

fn mode_v2(v: &Vec<i32>) -> i32 {
    let mut map = HashMap::new();

    for x in v {
        let count = map.entry(x).or_insert(0);
        *count += 1;
    }

    map.into_iter()
        .max_by_key(|&(_, count)| count)
        .map(|(val, _)| val)
        .copied()
        .unwrap_or(1)
}

pub fn median_mode(v: &Vec<i32>) -> (f32, i32) {
    (median(v), mode(v))
}
