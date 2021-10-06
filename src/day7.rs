use std::fs;
use std::collections::HashSet;

#[allow(dead_code)]
const FILENAME: &str = "data/day7.txt";

#[allow(dead_code)]
fn is_abba(s: String) -> bool {
    if s.len() == 4 {
        let sub = s.as_bytes();
        sub[0] != sub[1] && sub[0] == sub[3] && sub[1] == sub[2]
    } else {
        false
    }
}
fn has_abba(s: String) -> bool {
    if s.len() >= 4 {
        for i in 0..s.len() - 3 {
            if is_abba(String::from(&s[i..i+4])) {
                return true;
            }
        };
        return false;
    } else {
        false
    }
}


#[allow(dead_code)]
pub fn part1() -> (i64, u128) {
    let source = fs::read_to_string(FILENAME).unwrap();
    let now = std::time::Instant::now();
    let mut count = 0;
    for line in source.split("\r\n") {
        let mut is_hypernet = false;
        let mut flag = true;
        let mut any_outside_hypernet = false; 
        for sequence in  line.split(|c| c == '[' || c == ']') {
            if !(!is_hypernet && any_outside_hypernet) {
                let is_valid = has_abba(String::from(sequence));
                if is_hypernet && is_valid {
                    flag = false;
                    break
                } else if !is_hypernet && is_valid {
                    any_outside_hypernet = true;
                }
            }
            is_hypernet = !is_hypernet;
        }
        count += if flag && any_outside_hypernet {1} else {0};
    }
    (count, now.elapsed().as_micros())
}

fn is_aba(s: &String) -> bool { // reference since you'd want to use the "aba" afterwards
    if s.len() == 3 {
        let sub = s.as_bytes();
        sub[0] != sub[1] && sub[0] == sub[2]
    } else {
        false
    }
}

fn get_bab_from_aba(s: String) -> String {
    let aba = s.as_bytes();
    let bab = [aba[1], aba[0], aba[1]];
    String::from_utf8(Vec::from(bab)).unwrap()
}

fn get_all_aba(s: String) -> Vec<String> {
    if s.len() >= 3 {
        let mut aba_vec: Vec<String> = Vec::new();
        for i in 0..s.len() - 2 {
            let new_slice = String::from(&s[i..i+3]);
            if is_aba(&new_slice) {
                aba_vec.push(new_slice);
            }
        }
        aba_vec
    } else {
        Vec::new()
    }
}

#[allow(dead_code)]
pub fn part2() -> (i32, u128) {
    let source = fs::read_to_string(FILENAME).unwrap();
    let now = std::time::Instant::now();
    let mut count = 0;
    for line in source.split("\r\n") {
        let mut is_hypernet = false;
        let mut flag = false;
        let mut supernets: HashSet<String> = HashSet::new();
        let mut hypernets: HashSet<String> = HashSet::new();
        for sequence in  line.split(|c| c == '[' || c == ']') {
            let abas = get_all_aba(String::from(sequence));
            for aba in abas.into_iter() {
                if is_hypernet {
                    if supernets.contains(&aba) { // attempt to break out if early discovery
                        flag = true;
                        break
                    }
                    hypernets.insert(aba);
                } else {
                    let bab = get_bab_from_aba(aba);
                    if hypernets.contains(&bab) { // same as above ^^
                        flag = true;
                        break
                    }
                    supernets.insert(bab);
                }
            }
            is_hypernet = !is_hypernet;
            if flag {
                break
            }
        }

        count += if flag || !supernets.is_disjoint(&hypernets) {1} else {0};
    }
    (count, now.elapsed().as_micros())
}