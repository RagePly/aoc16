use std::collections::{HashMap, VecDeque};
type BotId = i32;
type Value = i32;

#[derive(Debug)]
enum BotNode {
    Output(BotId),
    Bot(BotId),
}

#[derive(Debug)]
enum ValueNode {
    Partial(Value),
    Full(Value, Value) 
}

enum ParseResult {
    Bot(BotId, BotNode, BotNode),
    Input(BotId, Value)
}


type UpdateInfo = (BotId, Value);
type Bots = HashMap<BotId, (BotNode, BotNode)>;
type BotStates = HashMap<BotId, ValueNode>;
type Outputs = HashMap<BotId, Value>;
type UpdateQueue = VecDeque<UpdateInfo>;

const VALUE_VALUE_INDEX: usize = 1;
const VALUE_BOTID_INDEX: usize = 5;

const BOT_BOTID_INDEX: usize = 1;
const BOT_LOW_TYPE_INDEX: usize = 5;
const BOT_LOW_ID_INDEX: usize = 6;
const BOT_HIGH_TYPE_INDEX: usize = 10;
const BOT_HIGH_ID_INDEX: usize = 11;

const HIGH_VALUE: i32 = 61;
const LOW_VALUE: i32 = 17;

fn parse_bot_node(node_type: &str, node_id: &str) -> BotNode {
    use BotNode::*;
    let id: BotId = node_id.parse().unwrap();
    match node_type {
        "bot" => Bot(id),
        "output" => Output(id),
        _ => panic!("Invalid node type {}", node_type)
    } 
}

fn parse_line(line: &str) -> ParseResult {
    use ParseResult::*;
    let tokens: Vec<_> = line.split_ascii_whitespace().collect();
    match tokens[0] {
        "value" => {
            Input(
                tokens[VALUE_BOTID_INDEX].parse().unwrap(), 
                tokens[VALUE_VALUE_INDEX].parse().unwrap())
        },
        "bot" => {
            Bot(
                tokens[BOT_BOTID_INDEX].parse().unwrap(),
                parse_bot_node(tokens[BOT_LOW_TYPE_INDEX], tokens[BOT_LOW_ID_INDEX]),
                parse_bot_node(tokens[BOT_HIGH_TYPE_INDEX], tokens[BOT_HIGH_ID_INDEX]))
        },
        _ => panic!("Error parsing line {}", line)
    }
}

pub fn part1(source: String) -> i32 {
    use ValueNode::*;
    use BotNode::*;

    let mut bots: Bots = Bots::new();
    let mut bot_states: BotStates = BotStates::new();
    let mut update_queue: UpdateQueue = UpdateQueue::new(); 

    for line in source.split("\r\n") {
        match parse_line(line) {
            ParseResult::Bot(id, lnode, hnode) => {
                bots.insert(id, (lnode, hnode));
            },
            ParseResult::Input(id, val) => {
                let new_value = match bot_states.get(&id) {
                    None => Partial(val),
                    Some(current_value) => {
                        match current_value {
                            Partial(one_val) => {
                                let (min, max) = if *one_val > val { (val, *one_val) } else { (*one_val, val) };
                                Full(min, max)
                            },
                            Full(_, _) => panic!("Unexpected modify of completed node")
                        }
                    }
                };
                bot_states.insert(id, new_value);
            }
        };
    };

    // append all filled bots to the queue
    for (k, v) in bot_states.iter() {
        match v {
            Full(low, high) => {
                if *low == LOW_VALUE && *high == HIGH_VALUE { return *k; }; // cheeky attempt to exit early
                let (low_id, high_id) = bots.get(&k).unwrap();
                match high_id {
                    Bot(id) => {
                        update_queue.push_back((*id, *high));
                    },
                    _ => ()
                };
                match low_id {
                    Bot(id) => {
                        update_queue.push_back((*id, *low));
                    },
                    _ => ()
                };
            },
            Partial(_) => ()
        }
    }

    // incrementally fill the state
    while !update_queue.is_empty() {
        let (id, val) = update_queue.pop_front().unwrap();
        let new_value = match bot_states.get(&id) {
            None => Partial(val),
            Some(current_value) => {
                match current_value {
                    Partial(one_val) => {
                        let (min, max) = if *one_val > val { (val, *one_val) } else { (*one_val, val) };
                        if min == LOW_VALUE && max == HIGH_VALUE { return id; }; // bot responsible found, return its id
                        // is now full and it's low and high can be added to the queue
                        let (low_id, high_id) = bots.get(&id).unwrap();
                        match high_id {
                            Bot(bot_id) => {
                                update_queue.push_back((*bot_id, max));
                            },
                            _ => ()
                        };
                        match low_id {
                            Bot(bot_id) => {
                                update_queue.push_back((*bot_id, min));
                            },
                            _ => ()
                        };
                        Full(min, max)
                    },
                    Full(_, _) => panic!("Unexpected modify of completed node")
                }
            }
        };
        bot_states.insert(id, new_value);
    }
    panic!("task failed, no id of bot that compares {} to {}", LOW_VALUE, HIGH_VALUE);
}

pub fn part2(source: String) -> i32 {
    use ValueNode::*;
    use BotNode::*;

    let mut bots: Bots = Bots::new();
    let mut bot_states: BotStates = BotStates::new();
    let mut outputs: Outputs = Outputs::new();
    let mut update_queue: UpdateQueue = UpdateQueue::new(); 

    for line in source.split("\r\n") {
        match parse_line(line) {
            ParseResult::Bot(id, lnode, hnode) => {
                bots.insert(id, (lnode, hnode));
            },
            ParseResult::Input(id, val) => {
                let new_value = match bot_states.get(&id) {
                    None => Partial(val),
                    Some(current_value) => {
                        match current_value {
                            Partial(one_val) => {
                                let (min, max) = if *one_val > val { (val, *one_val) } else { (*one_val, val) };
                                Full(min, max)
                            },
                            Full(_, _) => panic!("Unexpected modify of completed node")
                        }
                    }
                };
                bot_states.insert(id, new_value);
            }
        };
    };

    // append all filled bots to the queue
    for (k, v) in bot_states.iter() {
        match v {
            Full(low, high) => {
                let (low_id, high_id) = bots.get(&k).unwrap();
                match high_id {
                    Bot(id) => {
                        update_queue.push_back((*id, *high));
                    },
                    Output(out_id) => {
                        outputs.insert(*out_id, *high);
                    }
                };
                match low_id {
                    Bot(id) => {
                        update_queue.push_back((*id, *low));
                    },
                    Output(out_id) => {
                        outputs.insert(*out_id, *low);
                    }
                };
            },
            Partial(_) => ()
        }
    }

    // incrementally fill the state
    while !update_queue.is_empty() {
        let (id, val) = update_queue.pop_front().unwrap();
        let new_value = match bot_states.get(&id) {
            None => Partial(val),
            Some(current_value) => {
                match current_value {
                    Partial(one_val) => {
                        let (min, max) = if *one_val > val { (val, *one_val) } else { (*one_val, val) };
                        // is now full and it's low and high can be added to the queue
                        let (low_id, high_id) = bots.get(&id).unwrap();
                        match high_id {
                            Bot(bot_id) => {
                                update_queue.push_back((*bot_id, max));
                            },
                            Output(out_id) => {
                                outputs.insert(*out_id, max);
                            }
                        };
                        match low_id {
                            Bot(bot_id) => {
                                update_queue.push_back((*bot_id, min));
                            },
                            Output(out_id) => {
                                outputs.insert(*out_id, min);
                            }
                        };
                        Full(min, max)
                    },
                    Full(_, _) => panic!("Unexpected modify of completed node")
                }
            }
        };
        bot_states.insert(id, new_value);
    };

   let a = *outputs.get(&0).unwrap();
   let b = *outputs.get(&1).unwrap();
   let c = *outputs.get(&2).unwrap();
   a * b * c
}