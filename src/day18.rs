type Row    = Vec<bool>;

fn rule(l: bool, c: bool, r: bool) -> bool {
    (l && c && !r)  ||
    (!l && c && r)  ||
    (l && !c && !r) ||
    (!l && !c && r)
}

fn build(prev_row: &Row) -> Row {
    [false].iter()
    .chain(prev_row.iter())
    .chain([false].iter())
    .map(|b_ref| *b_ref)
    .collect::<Row>()
    .as_slice()
    .windows(3)
    .map(
    |w| rule(w[0], w[1], w[2])
    ).collect()
}

pub fn part1(source: String) -> usize {
    let mut pre_row: Row = source.as_bytes().iter().map(
        |b| if *b == '^' as u8 { true } else { false }
    ).collect();

    let mut count = pre_row.iter().filter(|b| !*b).count();
    for _ in 1..40 {
        pre_row = build(&pre_row);
        count += pre_row.iter().filter(|b| !*b).count();
    }
    count
}

pub fn part2(source: String) -> usize {
    let mut pre_row: Row = source.as_bytes().iter().map(
        |b| if *b == '^' as u8 { true } else { false }
    ).collect();

    let mut count = pre_row.iter().filter(|b| !*b).count();
    for _ in 1..400000 {
        pre_row = build(&pre_row);
        count += pre_row.iter().filter(|b| !*b).count();
    }
    count
}