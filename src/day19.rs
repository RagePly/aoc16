fn get_winner(lower_bound: u32, upper_bound: u32, total: u32, index: u32) -> u32 {
    if total == 1 {
        lower_bound
    } else if total % 2 == 0 {
        get_winner(lower_bound, upper_bound - 2u32.pow(index), total/2, index + 1)
    } else {
        get_winner(lower_bound + 2u32.pow(index + 1u32), upper_bound, (total + 1)/2 - 1, index + 1)
    }
}

pub fn part1(source: String) -> u32 { // for some reason this code is RIDICULOUSLY fast, taking around few microseconds
    let total = source.parse().unwrap();
    get_winner(1, total, total, 0)
}



pub fn part2(source: String) -> usize {
    let final_size: usize = source.parse().unwrap();
    let mut i: usize = 0;
    // the solution is based on figuring out what position a player in the current round would have had in the previous round
    // all positions are relative to the thief, meaning a player had at least +1 the position index in the previous round, since the
    // role of the thief moves each round
    // if the player is to the left of the current "victim", their position was an additional +1 in the previous round
    // however if the current number of players is even, the current victim was 2 positions to the left of the
    // previous victim meaning that the player directly to the right of the current victim had an additional +1 in 
    // index in the previous round, overruling the previous condition.
    // the thief in the final round wins, thereof the index 0 when size is 2
    // also modulo the calculated index by the size of the previous round
    // in the first round, player 1 was thief at index 0, so adding 1 to the resulting index of the winner is the answer
    for size in 2..final_size {
        if size % 2 == 0 && i == size/2 - 1 {
            i += 2
        } else if i > size/2{
            i += 2;
        } else {  // i < size/2 
            i += 1;
        }
        i %= size + 1;
    }
    i + 1
}