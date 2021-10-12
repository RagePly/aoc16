type BitString = Vec<bool>;

fn checksum(bitstring: &BitString) -> BitString {
    bitstring.as_slice().chunks_exact(2).map(
        |w| if w[0] == w[1] { true } else { false }
    ).collect()
}

pub fn part1(source: String) -> String {
    let mut bitstring: BitString = source.as_bytes().iter().map(|b| if *b == '1' as u8 {true} else {false}).collect();
    while bitstring.len() < 272 {
        let bitstring_copy = bitstring.clone();
        bitstring.push(false);
        bitstring_copy.iter().rev().for_each(
            |bit| bitstring.push(!bit)
        );
    }

    bitstring.resize(272, false); // force truncation
    let mut bitstring_checksum = checksum(&bitstring);
    while bitstring_checksum.len() % 2 == 0 {
        bitstring_checksum = checksum(&bitstring_checksum);
    }
    bitstring_checksum.into_iter().map(|b| if b { '1' } else { '0' }).collect::<String>()
}

pub fn part2(source: String) -> String {
    let mut bitstring: BitString = Vec::with_capacity(35651584 * 2);
    source.as_bytes().iter().for_each(|b| if *b == '1' as u8 {bitstring.push(true) } else { bitstring.push(false)});
    while bitstring.len() < 35651584 {
        let bitstring_copy = bitstring.clone();
        bitstring.push(false);
        bitstring_copy.iter().rev().for_each(
            |bit| bitstring.push(!bit)
        );
    }

    bitstring.resize(35651584, false); // force truncation
    let mut bitstring_checksum = checksum(&bitstring);
    while bitstring_checksum.len() % 2 == 0 {
        bitstring_checksum = checksum(&bitstring_checksum);
    }
    bitstring_checksum.into_iter().map(|b| if b { '1' } else { '0' }).collect::<String>()
}