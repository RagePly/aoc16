fn parse_repeat(slice: &str) -> usize {
    let closing = slice.find(")").unwrap();
    let temp: Vec<_> = slice[..closing].split("x").map(|x: &str| -> usize { x.parse().unwrap() }).collect();
    let nr_chrs: usize = temp[0];
    let nr_times: usize = temp[1];
    nr_chrs * nr_times + parse_text(&slice[(closing + 1 + nr_chrs)..])
}

fn parse_text(slice: &str) -> usize {
    match slice.find("(") {
        Some(_) => {
            let cat_tail: Vec<_> = slice.splitn(2, "(").collect();
            cat_tail[0].chars().count() + parse_repeat(cat_tail[1])
        },
        _ => slice.chars().count()
    }
}

fn parse_text_v2(slice: &str) -> usize {
    match slice.find("(") {
        Some(_) => {
            let cat_tail: Vec<_> = slice.splitn(2, "(").collect();
            cat_tail[0].chars().count() + parse_repeat_v2(cat_tail[1])
        },
        _ => slice.chars().count()
    }
}

fn parse_repeat_v2(slice: &str) -> usize {
    let closing = slice.find(")").unwrap();
    let temp: Vec<_> = slice[..closing].split("x").map(|x: &str| -> usize { x.parse().unwrap() }).collect();
    let nr_chrs: usize = temp[0];
    let nr_times: usize = temp[1];
    let sub_slice_size = parse_text_v2(&slice[closing+1..(closing + 1 + nr_chrs)]);
    let post_slice_size = parse_text_v2(&slice[(closing + 1 + nr_chrs)..]);
    nr_times * sub_slice_size + post_slice_size
}
pub fn part1(source: String) -> usize {
    parse_text(source.as_str())
}
pub fn part2(source: String) -> usize {
    parse_text_v2(source.as_str())
}