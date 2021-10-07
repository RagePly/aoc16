use md5::compute;

#[allow(dead_code)]
const FILENAME: &str = "data/day5.txt";

#[allow(dead_code)]
pub fn part1(source: String) -> String {
    let mut i: i64 = 0;
    let mut password: Vec<u8> = Vec::new();
    while password.len() < 8 {
        let test = format!("{}{}",source, i);
        let digest = compute(test);
        if digest[0] == 0 &&
            digest[1] == 0 &&
            digest[2] & 0xf0u8 == 0 {
            let str_repr = format!("{:x}", digest);
            password.push(str_repr.as_bytes()[5]);
        }
        i += 1;
    } 
    String::from_utf8(password).unwrap()
}

#[allow(dead_code)]
pub fn part2(source: String) -> String {
    let mut i: i64 = 0;
    let mut flag: u8 = 0;
    let mut password: [u8; 8] = [0,0,0,0,0,0,0,0];
    while flag != 0xff {
        let test = format!("{}{}",source, i);
        let digest = compute(test);
        if digest[0] == 0 &&
            digest[1] == 0 &&
            digest[2] & 0xf0 == 0 && // ^^ 5 leading zeroes
            digest[2] & 0x0f < 8 { // only consider valid indices, could up to 15
            let index = digest[2] & 0x0fu8;
            if flag >> (7 - index) & 0b1 == 0 { // check if flag is not set
                flag |= 0b1 << (7 - index); // set flag for index position 
                let str_repr = format!("{:x}", digest);
                password[index as usize] = str_repr.as_bytes()[6];
            }
        }
        i += 1;
    } 
    String::from_utf8(Vec::from(password)).unwrap()
}

