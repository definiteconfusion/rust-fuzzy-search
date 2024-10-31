use std::env;
fn levenshtein_distance(a: &str, b: &str) -> usize {
    let mut prev_row = (0..=b.len()).collect::<Vec<_>>();
    let mut curr_row = vec![0; b.len() + 1];
    for (i, ca) in a.chars().enumerate() {
        curr_row[0] = i + 1;
        for (j, cb) in b.chars().enumerate() {
            curr_row[j + 1] = curr_row[j]
                .min(prev_row[j + 1] + 1)
                .min(prev_row[j] + (ca != cb) as usize);
        }
        prev_row.copy_from_slice(&curr_row);
    }
    curr_row[b.len()]
}
fn fuzzy_search<'a>(query: &'a str, choices: &'a [&'a str], max_distance: usize) -> Vec<&'a str> {
    choices
        .iter()
        .filter(|&choice| levenshtein_distance(query, choice) <= max_distance)
        .copied()
        .collect()
}
fn main() {
    let args: Vec<String> = env::args().collect();
    let choices = args[2..].iter().map(|s| s.as_str()).collect::<Vec<_>>();
    let query = args[1].as_str();
    let max_distance = 2;
    let results = fuzzy_search(query, &choices, max_distance);
    for result in results {
        println!("{}", result);
    }
}