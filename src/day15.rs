use std::collections::VecDeque;


#[derive(Debug)]
struct Disk {
    freq: i32,
    position: i32
}

const FREQ_INDEX: usize = 3;
const OFFS_INDEX: usize = 11;

fn parse_disk(line: &str) -> Disk {
    let token: Vec<_> = line.split_ascii_whitespace().collect();
    Disk {
        freq: token[FREQ_INDEX].parse().unwrap(),
        position: token[OFFS_INDEX].replace(".", "").parse().unwrap()
    }
}


pub fn part1(source: String) -> i32 {
    let mut disks: VecDeque<_> = source.split("\r\n").map(|line| parse_disk(line)).collect();
    disks.iter_mut().enumerate().for_each(|v| v.1.position += 1 + v.0 as i32); // compensate for lag when falling, now each should be zero at the same time

    let mut prev_disk = Disk { freq: 1, position: 0 };
    let mut offset = 0;

    while !disks.is_empty() {
        let current_disk = disks.pop_front().unwrap();
        let freq = prev_disk.freq;
        let mut position = (current_disk.position + offset) % current_disk.freq;
        while position != 0 {
            position = (position + freq) % current_disk.freq;
            offset += freq;
        } 

        prev_disk = Disk {
            freq: freq*current_disk.freq,
            position: 0 // unused
        }
    }
    offset 
}

pub fn part2(source: String) -> i32 {
    let mut disks: VecDeque<_> = source.split("\r\n").map(|line| parse_disk(line)).collect();
    disks.push_back(Disk { freq: 11, position: 0});
    disks.iter_mut().enumerate().for_each(|v| v.1.position += 1 + v.0 as i32); // compensate for lag when falling, now each should be zero at the same time

    let mut prev_disk = Disk { freq: 1, position: 0 };
    let mut offset = 0;

    while !disks.is_empty() {
        let current_disk = disks.pop_front().unwrap();
        let freq = prev_disk.freq;
        let mut position = (current_disk.position + offset) % current_disk.freq;
        while position != 0 {
            position = (position + freq) % current_disk.freq;
            offset += freq;
        } 

        prev_disk = Disk {
            freq: freq*current_disk.freq,
            position: 0 // unused
        }
    }
    offset 
}