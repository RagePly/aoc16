pub fn part1(source: String) -> u32 {
    let mut ip_ranges: Vec<_> = source.split("\r\n").map(
        |line| {
            let mut rs = line.split("-").map(|v| v.parse::<u32>().unwrap());
            (rs.next().unwrap(), rs.next().unwrap())
        }
    ).collect();

    ip_ranges.sort_unstable_by(|r1, r2| r1.0.cmp(&r2.0));
    let mut upper_bound = 0;
    for (lb, ub) in ip_ranges.into_iter() {
        if upper_bound >= lb { // the following range overlaps
            upper_bound = ub + 1;
        } else {
            return upper_bound;
        }
    }
    0
}

pub fn part2(source: String) -> u64 { // solution only worked while having u64 since u32 would overflow
    let mut ip_ranges: Vec<_> = source.split("\r\n").map(
        |line| {
            let mut rs = line.split("-").map(|v| v.parse::<u64>().unwrap());
            (rs.next().unwrap(), rs.next().unwrap())
        }
    ).collect();

    ip_ranges.sort_unstable_by(|r1, r2| r1.0.cmp(&r2.0));
    let mut upper_bound = 0;
    let mut count = 0;
    for (lb, ub) in ip_ranges.into_iter() {
        if upper_bound < lb { // the following range does not overlap the previous 
            count += lb - upper_bound
        }

        if upper_bound <= ub {
            upper_bound = ub + 1;
        }
    }
    count + (4294967295 - upper_bound) + 1
}