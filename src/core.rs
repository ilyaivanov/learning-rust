use std::collections::HashMap;

pub fn get_mode(numbers: &Vec<usize>) -> usize {
    let mut map: HashMap<usize, usize> = HashMap::new();

    for a in numbers {
        let _ = map.entry(*a).and_modify(|a| *a += 1).or_insert(1);
    }

    match map.iter().max_by(|(_, a1), (_, b1)| a1.cmp(b1)) {
        Some((a, _)) => *a,
        _ => 0,
    }
}
pub fn parse_numbers(str: String) -> Vec<usize> {
    str.split(",")
        .map(|f| f.trim().parse::<usize>())
        .filter(|v| !v.is_err())
        .map(|f| f.unwrap())
        .collect()
}

pub fn get_mean(sorted_numbers: &Vec<usize>) -> &usize {
    sorted_numbers.get(sorted_numbers.len() / 2).unwrap_or(&0)
}
