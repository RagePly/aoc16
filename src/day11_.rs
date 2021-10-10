use std::collections::HashSet;
use std::collections::HashMap;
use std::collections::VecDeque;
use itertools::Itertools;

#[derive(Clone, Debug)]
enum Component {
    Generator(String),
    Microchip(String)
}

fn parse_component(token: &str) -> Component {
    if token.contains("compatible") {
        Component::Microchip(String::from(token.split("-").collect::<Vec<&str>>()[0]))
    } else {
        Component::Generator(String::from(token))
    }
}

type Components = HashMap<String, u8>;
struct State {
    depth: usize,
    floor: usize,
    comps: Components
}

fn are_identical(a: &Components, b: &Components) -> bool {
    for (k, v) in a.iter() {
        if *v != b[k] {
            return false
        }
    } 
    true
}

fn are_equivalent(a: &Components, b: &Components) -> bool {
    let mut a_v: Vec<u8> = a.values().map(|v| *v).collect();
    let mut b_v: Vec<u8> = b.values().map(|v| *v).collect();
    a_v.sort();
    b_v.sort();

    for (i, j) in a_v.into_iter().zip(b_v.into_iter()) {
        if i != j {
            return false;
        }
    }
    true
}

fn is_valid(cmps: &Components) -> bool {
    for floor in 0..4 {
        let mut nr_gen: usize = 0;
        let mut nr_pair: usize = 0;
        let mut nr_mic: usize = 0;

        for c in cmps.values() {
            if (c >> 2 * floor) & 0b11 == 0b11 {
                nr_pair += 1;
            }
            if (c & 0b01 << 2 * floor) > 0 {
                nr_gen += 1;
            }
            if (c & 0b10 << 2 * floor) > 0 {
                nr_mic += 1;
            }
        }

        if !(nr_gen == 0 || // no generators, no worry
            nr_mic == 0  || // no microchips, no worry
            nr_pair == nr_mic) // every microchip is paired
        {
            return false;
        }
    }
    true
}

fn floor_is_empty(comp: &Components, floor: usize) -> bool {
    for c in comp.values() {
        if (c & 0b11 << 2 * floor) > 0{
            return false 
        } 
    }
    true
}

fn generate_new_states(state: &State) {
    let new_floors = match stat.floor {
        0 => vec!(1),
        3 => vec!(2),
        _ => vec!(state.floor - 1, state.floor + 1)
    }

    let new_components: Components = HashMap::new();
    for new_floor in new_floors {
        for comp in state.comps {
            if comp k§ // todo i was planning on checking if either gen or mic was present, then create new by shifting those ´bits to the new floor. Then somehow joining for the two component types gn
        }
    }
}

pub fn part1(source: String) -> usize {
    use Component::*;
    let mut ignore_words: HashSet<&str> = HashSet::new();
    for w in "the first second third fourth floor contains a and generator microchip nothing relevant".split_ascii_whitespace() {
        ignore_words.insert(w);
    }

    let mut orig_comp: Components = HashMap::new();
    for (i, l) in source.split("\r\n").enumerate() {
        let components: Vec<_> = l.replace(",", "").replace(".","").to_lowercase().split_ascii_whitespace()
        .filter(|w| !ignore_words.contains(w))
        .map(|token| parse_component(token)).collect();

        for comp in components {
            match comp {
                Generator(name) => {
                    let mask: u8 = 0 | 0b01 << 2 * i; // i don't know the operator precedence
                    *orig_comp.entry(name).or_insert(0) |= mask;
                },
                Microchip(name) => {
                    let mask: u8 = 0 | 0b10 << 2 * i;
                    *orig_comp.entry(name).or_insert(0) |= mask;
                }
            }
        } 
    }

    let orig_state = State { depth: 0, floor: 0, comps: orig_comp };

    0
}
pub fn part2(_source: String) -> i32 {
    0
}