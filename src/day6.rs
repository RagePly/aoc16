use std::fs;
use std::collections::HashMap;

#[allow(dead_code)]
const FILENAME: &str = "data/day6.txt";

#[allow(dead_code)]
pub fn part1() -> (String, u128) {
    let source = fs::read_to_string(FILENAME).unwrap();
    let now = std::time::Instant::now();

    let mut freq_counters: Vec<HashMap<u8, i32>> = Vec::new();
    for line in source.split("\r\n") {
        let byte_array = line.as_bytes();
        for i in 0..line.len() {
            if i >= freq_counters.len() {
                freq_counters.push(HashMap::new());
            }
            let c = byte_array[i];
            *freq_counters[i].entry(c).or_insert(0) += 1; 
        }
    }

    let mut message_bytes: Vec<u8> = Vec::new();
    for freq_counter in freq_counters {
        let mut temp_vec: Vec<_> = freq_counter.into_iter().collect();
        temp_vec.sort_by(|a, b| a.1.cmp(&b.1).reverse());
        message_bytes.push(temp_vec[0].0);
    }
    (String::from_utf8(message_bytes).unwrap(), now.elapsed().as_micros())
}

#[allow(dead_code)]
pub fn part2() -> (String, u128) {
    let source = fs::read_to_string(FILENAME).unwrap();
    let now = std::time::Instant::now();

    let mut freq_counters: Vec<HashMap<u8, i32>> = Vec::new();
    for line in source.split("\r\n") {
        let byte_array = line.as_bytes();
        for i in 0..line.len() {
            if i >= freq_counters.len() {
                freq_counters.push(HashMap::new());
            }
            let c = byte_array[i];
            *freq_counters[i].entry(c).or_insert(0) += 1; 
        }
    }

    let mut message_bytes: Vec<u8> = Vec::new();
    for freq_counter in freq_counters {
        let mut temp_vec: Vec<_> = freq_counter.into_iter().collect();
        temp_vec.sort_by(|a, b| a.1.cmp(&b.1)); // sort in increasing order instead
        message_bytes.push(temp_vec[0].0);
    }
    (String::from_utf8(message_bytes).unwrap(), now.elapsed().as_micros())
}