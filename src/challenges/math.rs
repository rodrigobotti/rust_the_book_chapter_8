use std::collections::HashMap;

fn is_even(x: &usize) -> bool {
    x % 2 == 0
}

pub fn median(xs: &[i32]) -> f64 {
    let length = xs.len();
    assert!(length > 0, "xs must have elements");

    let mut to_sort = xs.to_vec();

    to_sort.sort();

    let middle_idx = (length as f64 / 2.0).floor() as usize;

    if is_even(&length) {
        let mid_1 = to_sort.get(middle_idx - 1).cloned().unwrap();
        let mid_2 = to_sort.get(middle_idx).cloned().unwrap();
        return f64::from(mid_1 + mid_2) / 2.0;
    }

    let median = to_sort.get(middle_idx).cloned().unwrap();
    f64::from(median)
}

pub fn mode(xs: &[i32]) -> i32 {
    assert!(xs.len() > 0, "xs must have elements");

    let mut frequencies: HashMap<i32, u32> = HashMap::new();

    for &x in xs {
        let frequency = frequencies.entry(x).or_insert(0);
        *frequency += 1;
    }

    let mut mode: i32 = 0;
    let mut highest_frequency: u32 = 0;

    for (num, frequency) in frequencies {
        if frequency > highest_frequency {
            highest_frequency = frequency;
            mode = num;
        }
    }

    mode
}
