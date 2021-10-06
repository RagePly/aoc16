use std::fs;
use std::collections::HashMap;
use std::cmp::Ordering;

#[allow(dead_code)]
const FILENAME: &str = "data/day4.txt";

fn order_by_frequency(words: &[&str]) -> String {
    let mut freq_map: HashMap<char, i64> = HashMap::new();
    for &word in words {
        for c in word.chars() {
            *freq_map.entry(c).or_insert(0) += 1;
        }
    }

    let mut temp_vec: Vec<_> = freq_map.into_iter().collect();
    temp_vec.sort_by(|a, b| match a.1.cmp(&b.1) {
        Ordering::Equal => { // sort alphabetically instead
            a.0.cmp(&b.0)
        },
        order => order.reverse() 
    } ); // sort in decreasing order

    temp_vec.into_iter().map(|c| c.0).take(5).collect()
}
fn parse_line(line: &str) -> (String, i64, String) {
    let split_line: Vec<&str> = line.split("-").collect();
    let encrypt_name = &split_line[0..(split_line.len() - 1)];
    let remainder: Vec<_> = (*split_line.last().unwrap()).splitn(2, "[").collect();
    let id: i64 = String::from(*remainder.get(0).unwrap()).parse().unwrap();
    let temp = String::from(*remainder.last().unwrap());
    let checksum: String = String::from(&temp[0..temp.len() - 1]);
    (order_by_frequency(encrypt_name), id, checksum)
}

#[allow(dead_code)]
pub fn part1() -> (i64, u128) {
    let source = fs::read_to_string(FILENAME).unwrap();
    let now = std::time::Instant::now();
    let mut sum = 0;
    for line in source.split("\r\n") {
        let (oom, id, checksum) = parse_line(line);
        sum += if oom.eq(&checksum) {
            id
        } else {
            0
        };
    }
    (sum, now.elapsed().as_micros())
}

fn rotate_char(c: u8, n: i64) -> u8 {
    if 97 <= c && c <= 122 {
        (((c as i64 - 97) + n) % 26 + 97) as u8
    } else {
        c
    }
}
fn rotate_string(s: String, n: i64) -> String {
    s.into_bytes().iter().map(|c| rotate_char(*c, n) as char).collect()
}

fn parse_line2(line: &str) -> (String, i64, String, Vec<String>) {
    let split_line: Vec<&str> = line.split("-").collect();
    let encrypt_name = &split_line[0..(split_line.len() - 1)];
    let oof = order_by_frequency(encrypt_name);
    let words: Vec<_> = Vec::from(encrypt_name).iter().map(|s| String::from(*s)).collect();
    let remainder: Vec<_> = (*split_line.last().unwrap()).splitn(2, "[").collect();
    let id: i64 = String::from(*remainder.get(0).unwrap()).parse().unwrap();
    let temp = String::from(*remainder.last().unwrap());
    let checksum: String = String::from(&temp[0..temp.len() - 1]);
    (oof, id, checksum, words)
}

#[allow(dead_code)]
pub fn part2() -> (i64, u128) {
    let source = fs::read_to_string(FILENAME).unwrap();
    let now = std::time::Instant::now();
    let mut north_pole_id: i64 = 0;
    for line in source.split("\r\n") {
        let (oof, id, checksum, words) = parse_line2(line);
        if oof.eq(&checksum) {
            let joined_string = words.join(" ");
            let new_str = rotate_string(joined_string, id);
            if new_str.eq("northpole object storage") {
                north_pole_id = id;
                break
            }
        }
    }
    (north_pole_id, now.elapsed().as_micros())
}