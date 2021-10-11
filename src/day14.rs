use md5::*;



fn find_triple(b: &Digest) -> Option<u8> {
    for i in 0..15 { // only 15, since last byte only has two
        let lhs1 = (b[i] >> 4) & 0xf;
        let rhs1 = b[i] & 0xf;
        let lhs2 = (b[i+1] >> 4) & 0xf;
        let rhs2 = b[i+1] & 0xf;

        if lhs1 == rhs1 && rhs1 == lhs2  {
            return Some(lhs1);
        } else if rhs1 == lhs2 && lhs2 == rhs2 {
            return Some(rhs1);
        }
    }
    None
}

fn has_quintuple(b: &Digest, c: u8) -> bool {
    for i in 0..16 - 2  { 
        let lhs = (b[i] >> 4) & 0xf;
        let rhs = b[i] & 0xf;

        if (lhs == c && rhs == c && b[i] == b[i+1] && c == ((b[i+2] >> 4) & 0xf )) || 
           (rhs == c && (b[i + 1] & 0xf) == c && ((b[i + 1] >> 4) & 0xf) == c &&  b[i+1] == b[i+2])
        {
            return true;
        }
    }
    false
}

fn has_any_quintuple(b: &Digest) -> bool {
    for i in 0..16 - 2  { 
        let lhs = (b[i] >> 4) & 0xf;
        let rhs = b[i] & 0xf;

        if (lhs ==  rhs && b[i] == b[i+1] && rhs == ((b[i+2] >> 4) & 0xf )) || 
           ((b[i + 1] & 0xf) == rhs && ((b[i + 1] >> 4) & 0xf) == rhs &&  b[i+1] == b[i+2])
        {
            return true;
        }
    }
    false
}

#[derive(Clone, Copy)]
struct HashListItem{
    is_active: bool,
    hash: u8,
    index: usize
}

const SIZE: usize = 1001;

pub fn part1(source: String) -> usize {
    // automatically disable the one at the current index
    let mut circular_hash_list: [HashListItem; SIZE] = [HashListItem {is_active: false, hash: 0, index: 0}; SIZE];
    let mut current_index: usize = 0;
    let mut valid_key_index: [usize; 64] = [0; 64];
    let mut key_count: usize = 0;
    let mut hash_count: usize = 0;
    
    loop {
        circular_hash_list[current_index].is_active = false;

        // generate new hash
        let digest = md5::compute(format!("{}{}", source, hash_count));

        // find triplet
        match find_triple(&digest) {
            Some(h) => {
                circular_hash_list[current_index].hash = h;
                circular_hash_list[current_index].is_active = true;
                circular_hash_list[current_index].index = hash_count;
                if hash_count > 11250 && hash_count < 11400  {
                }
            },
            None => ()
        };

        // look for quintuples, starting with the oldest in the queue
        if has_any_quintuple(&digest) {
            for i in current_index+1..SIZE {
                if circular_hash_list[i].is_active && has_quintuple(&digest, circular_hash_list[i].hash) {
                    valid_key_index[key_count] =circular_hash_list[i].index;
                    key_count += 1;
                    if key_count == 64 {
                        valid_key_index.sort_unstable();
                        return valid_key_index[63];
                    }
                    circular_hash_list[i].is_active = false;
                }
            }

            for i in 0..current_index {
                if circular_hash_list[i].is_active && has_quintuple(&digest, circular_hash_list[i].hash) {
                    valid_key_index[key_count] =circular_hash_list[i].index;
                    key_count += 1;
                    if key_count == 64 {
                        valid_key_index.sort_unstable();
                        return valid_key_index[63];
                    }
                    circular_hash_list[i].is_active = false;
                }
            }
        }

        current_index = (current_index + 1) % 1001;
        hash_count += 1;
    } 
}

pub fn part2(source: String) -> usize {
    // automatically disable the one at the current index
    let mut circular_hash_list: [HashListItem; SIZE] = [HashListItem {is_active: false, hash: 0, index: 0}; SIZE];
    let mut current_index: usize = 0;
    let mut valid_key_index: Vec<usize> = Vec::new();
    let mut hash_count: usize = 0;
    let mut final1000_flag: bool = false;
    let mut final1000_index: usize = 0;
    
    loop {
        circular_hash_list[current_index].is_active = false;

        // generate new hash
        let mut s: String = format!("{}{}", source, hash_count);
        let mut digest = md5::compute(&s);
        for _ in 0..2016 { // TODO: another thread?? would be a good exercise
            s = format!("{:x?}", digest);
            digest = md5::compute(&s);
        } 

        // find triplet
        match find_triple(&digest) {
            Some(h) => {
                circular_hash_list[current_index].hash = h;
                circular_hash_list[current_index].is_active = true;
                circular_hash_list[current_index].index = hash_count;
                if hash_count > 11250 && hash_count < 11400  {
                }
            },
            None => ()
        };

        // look for quintuples, starting with the oldest in the queue
        if has_any_quintuple(&digest) {
            for i in current_index+1..SIZE {
                if circular_hash_list[i].is_active && has_quintuple(&digest, circular_hash_list[i].hash) {
                    valid_key_index.push(circular_hash_list[i].index);
                    if valid_key_index.len() == 64 {
                        final1000_flag = true;
                        final1000_index = hash_count + 1000;
                    }
                    circular_hash_list[i].is_active = false;
                }
            }

            for i in 0..current_index {
                if circular_hash_list[i].is_active && has_quintuple(&digest, circular_hash_list[i].hash) {
                    //println!("{}: {:x?}", circular_hash_list[i].index, digest);
                    valid_key_index.push(circular_hash_list[i].index);
                    if valid_key_index.len() == 64 {
                        final1000_flag = true;
                        final1000_index = hash_count + 1000;
                    }
                    circular_hash_list[i].is_active = false;
                }
            }
        }
        current_index = (current_index + 1) % 1001;
        hash_count += 1;
        if final1000_flag && final1000_index == hash_count {
            valid_key_index.sort_unstable();
            return valid_key_index[63]; 
        }
    } 
}

// !! I think this is it: index n and m have triples and matching N and M (M < N). In this configuration, even though n is the last, m sneaks in first.
// !! make sure to take at least 1000 more to guarantee that n fits in. Sort the list and take the 64th element.
// !! Yes, it was