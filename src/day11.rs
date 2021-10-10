use std::collections::HashSet;
use std::collections::VecDeque;
use itertools::Itertools;
use std::collections::HashMap;


type ComponentType = u8;

// #########################################################################
// ! MICROCHIP IS HELD IN THE FOUR __MOST__ SIGNIFICANT BITS
// ! GENERATOR IS HELD IN THE FOUR __LEAST__ SIGNIFICANT BITS
// #########################################################################
fn extract_gen(c_t: ComponentType) -> ComponentType {
    c_t & 0xf
}
fn extract_mic(c_t: ComponentType) -> ComponentType {
    (c_t >> 4) & 0xf
}

fn insert_mic(old: ComponentType, new: ComponentType) -> ComponentType {
    //println!("comp {} insert mic {} -> {}", old, new, (old & 0x0f) | (new << 4));
    (old & 0x0f) | (new << 4)
}

fn insert_gen(old: ComponentType, new: ComponentType) -> ComponentType {
    //println!("comp {} insert gen {} -> {}", old, new, (old & 0x0f) | new );
    (old & 0xf0) | new 
}




type ComponentState = Vec<ComponentType>;

fn is_valid(c_state: &ComponentState) -> bool {
    //VALID("############### IS VALID ###############");
    let mut generator_tally: [usize; 4] = [0; 4];
    let mut microchip_tally: [usize; 4] = [0; 4];
    let mut pair_tally: [usize; 4] = [0; 4];

    for comp in c_state.iter() {
        let gen_floor = extract_gen(*comp) as usize;
        let mic_floor = extract_mic(*comp) as usize;
        //VALID("# Comp {} => gen({}), mic({})", comp, gen_floor, mic_floor); 

        generator_tally[gen_floor] += 1;
        microchip_tally[mic_floor] += 1;
        pair_tally[mic_floor] += if gen_floor == mic_floor { 1 } else { 0 };
    }

    //VALID("# mic tally {:?}\ngen tally {:?}\npair tally{:?}", microchip_tally, generator_tally, pair_tally);

    for floor in 0..4 {
        let generators = generator_tally[floor];
        let microchips = microchip_tally[floor];
        let pairs = pair_tally[floor];

        // floor is valid if there are no generators or no microchips or if every microchip is protected by a pair
        if !(generators == 0 || microchips == 0 || microchips == pairs) {
            //VALID("# floor {} was invalid", floor);
            //VALID("# RESULT: invalid\n########################################");
            return false
        }
        //VALID("# floor {} was valid", floor);

    }
    //VALID("# RESULT: valid\n########################################");
    true
}

#[allow(dead_code)]
fn visualize(s: &State) {
    let mut generators: [Vec<usize>; 4] = [Vec::new(), Vec::new(), Vec::new(), Vec::new()];
    let mut microchips: [Vec<usize>; 4] = [Vec::new(), Vec::new(), Vec::new(), Vec::new()];

    for (i, cmp) in s.comps.iter().enumerate() {
        let generator = extract_gen(*cmp) as usize;
        let microchip = extract_mic(*cmp) as usize;
        generators[generator].push(i);
        microchips[microchip].push(i);
    }

    println!("steps {}", s.steps);
    for floor_temp in 0..4 {
        let floor = 3 - floor_temp;
        print!("F{} ", floor);
        if s.floor == floor {
            print!("E  : ");
        } else {
            print!("   : ");
        }

       for i in 0..s.comps.len() {
            print!("{}", if generators[floor].contains(&i) {
                "G"
            } else {
                " "
            });
            print!("{}", if microchips[floor].contains(&i) {
                " M  : "
            } else {
                "    : "
            });
       } 
       println!();
    }
}

fn is_finished(a_sorted: &ComponentState) -> bool {
    for comp in a_sorted.iter() {
        if *comp != 0x33 {
            return false
        }
    }
    true
}

#[derive(Clone)]
struct State {
    floor: usize,
    steps: usize,
    comps: ComponentState
}

#[derive(PartialEq)]
struct ComparisonState {
    comps: ComponentState,
    floor: usize
}

#[derive(Clone, Copy, Debug)]
enum ComponentIndex {
    Microchip(usize),
    Generator(usize)
}

struct StateMove<'a> {
    components: Vec<ComponentIndex>,
    direction: i32,
    previous_state: &'a State
}

fn generate_new_state(state_move: StateMove) -> State {
    use ComponentIndex::*;
    //GENERATE("|--- Generate New State -------------------");
    //GENERATE("| move instruction:");
    //GENERATE("|    direction: {}", if state_move.direction == 1 { "up" } else { "down" });
    //GENERATE("|    components: {:?}", state_move.components);
    let mut new_state = state_move.previous_state.clone();

    new_state.floor = (new_state.floor as i32 + state_move.direction) as usize;
    new_state.steps += 1;

    for comp in state_move.components.iter() {
        //GENERATE("| applying change for {:?}", comp);
        match comp {
            Microchip(mic_i) => {
                let old_comp = new_state.comps[*mic_i];
                //GENERATE("|    old value: new_state[{}] = {}", *mic_i, old_comp);
                new_state.comps[*mic_i] = insert_mic(old_comp, (extract_mic(old_comp) as i32 + state_move.direction) as u8);
                //GENERATE("|    new value: {}", new_state.comps[*mic_i]);
            },
            Generator(gen_i) => {
                let old_comp = new_state.comps[*gen_i];
                //GENERATE("|    old value: new_state[{}] = {}", *gen_i, old_comp);
                new_state.comps[*gen_i] = insert_gen(old_comp, (extract_gen(old_comp) as i32 + state_move.direction) as u8);
                //GENERATE("|    new value: {}", new_state.comps[*gen_i]);
            }
        }
    }
    //GENERATE("| final state:");
    //GENERATE("|     floor: {}", new_state.floor);
    //GENERATE("|     steps: {}", new_state.steps);
    //GENERATE("|     comps: {:?}", new_state.comps);
    //GENERATE("|------------------------------------------");
    new_state    
}

fn generate_new_states(current_state: &State) -> Vec<State> {
    use ComponentIndex::*;
    let directions: Vec<i32> = match current_state.floor {
        0 => vec!(1),
        3 => vec!(-1),
        _ => vec!(1, -1)
    };
    //NEW_STATE("==== Generate New States ===================");
    //NEW_STATE("= Original state:");
    //NEW_STATE("=     floor: {}", current_state.floor);
    //NEW_STATE("=     steps: {}", current_state.steps);
    //NEW_STATE("=     comps: {:?}", current_state.comps);
    //NEW_STATE("= Directions to move in: {}", if directions.len() == 2 { "up and down" } else { if directions[0] == 1 { "up" } else { "down" }});
    //NEW_STATE("= Finding movable components:");

    let mut available_components: Vec<ComponentIndex> = Vec::new();
    for (i, comp) in current_state.comps.iter().enumerate() {
        if extract_mic(*comp) as usize == current_state.floor {
            //NEW_STATE("=     microchip at index {}", i);
            available_components.push(Microchip(i));
        } 
        if extract_gen(*comp) as usize == current_state.floor {
            //NEW_STATE("=     generator at index {}", i);
            available_components.push(Generator(i));
        }
    }


    //NEW_STATE("= Generating item moves:");
    let mut valid_states: Vec<State> = Vec::new();
    for direction in directions.iter() {
        // TODO prune away moving down to empty floors

        let mut single_moves: Vec<StateMove> = Vec::new();
        for comp in available_components.iter() {
            //NEW_STATE("=    Move {:?} {}", comp, if *direction == 1 { "up" } else { "down" });
            single_moves.push( StateMove {
                    direction: *direction,
                    components: vec!(*comp),
                    previous_state: current_state 
                }
            );
        }


        let double_item_states: Vec<_> = single_moves.iter().combinations(2).map(
            |group| StateMove {
                direction: *direction,
                components: vec!(group[0].components[0], group[1].components[0]),
                previous_state: current_state
            }
        ).map(
            |state_move| {
                //NEW_STATE("=    Move {:?} and {:?} {}", state_move.components[0], state_move.components[1], if *direction == 1 { "up" } else { "down" });
                generate_new_state(state_move)
            }
        ).filter(
            |new_state| is_valid(&new_state.comps)
        ).collect();

        single_moves.into_iter().map(
            |state_move| generate_new_state(state_move)
        ).filter(
            |state| is_valid(&state.comps)
        ).for_each(
            |valid_state| valid_states.push(valid_state)
        );

        double_item_states.into_iter().for_each(
            |valid_state| valid_states.push(valid_state)
        );

    }

    //NEW_STATE("============================================");
    valid_states
}


enum ComponentParsing {
    Microchip(String),
    Generator(String)
}

fn parse_component(token: &str) -> ComponentParsing {
    if token.contains("compatible") {
        ComponentParsing::Microchip(String::from(token.split("-").next().unwrap()))
    } else {
        ComponentParsing::Generator(String::from(token))
    }
}


pub fn part1(source: String) -> usize {
    use ComponentParsing::*;
    let banned_words: HashSet<&str> = "the first second third fourth floor contains a generator and microchip nothing relevant".split_ascii_whitespace().collect();

    let mut component_map: HashMap<String, ComponentType> = HashMap::new();
    for (floor, line) in source.to_lowercase().replace(",", "").replace(".", "").split("\r\n").enumerate() {
        for token in line.split_ascii_whitespace().filter(|token| !banned_words.contains(token)) {
            match parse_component(token) {
                Microchip(name) => {
                    let prev = *component_map.get(&name).unwrap_or(&0);
                    //println!("microchip: {} previous {} new {}", name, prev, floor);
                    component_map.insert(name, insert_mic(prev, floor as u8));
                },
                Generator(name) => {
                    let prev = *component_map.get(&name).unwrap_or(&0);
//                    println!("generator: {} previous {} new {}", name, prev, floor);
                    component_map.insert(name, insert_gen(prev, floor as u8));
                }
            }
        }
    }

    let components: Vec<_> = component_map.values().map(|v| *v).collect();

    let original_state = State {
        floor: 0,
        steps: 0,
        comps: components
    };

    let mut past_states: Vec<ComparisonState> = Vec::new();
    let mut update_queue: VecDeque<State> = VecDeque::new();
    let mut current_step: usize = 0;
    update_queue.push_back(original_state);

    while !update_queue.is_empty() {
        let current_state = update_queue.pop_front().unwrap();
        let mut components_compare = current_state.comps.clone();
        components_compare.sort_unstable();
        let comparison_state = ComparisonState{
            comps: components_compare,
            floor: current_state.floor
        };
        //CURRENT_STATE("* ~~~ Current state ~~~~~~~~~~~~");
        //CURRENT_STATE("* current state:");
        //visualize(&current_state);
        if past_states.contains(&comparison_state) {
            //CURRENT_STATE("* has been visited, prune away...");
            //CURRENT_STATE("* ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~");
            continue
        }

        if current_state.steps > current_step {
            //CURRENT_STATE("* step {} is finished", current_step);
            current_step += 1;
        }
   
        if is_finished(&current_state.comps) {
            return current_step
        }

        //CURRENT_STATE("* generating new steps...");
        let new_states = generate_new_states(&current_state);
        past_states.push(comparison_state);
        new_states.into_iter().for_each(
            |state| {
                //visualize(&state); println!();
                update_queue.push_back(state)
            }
        );
        //CURRENT_STATE("* ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~");
    }
    current_step
}
pub fn part2(source: String) -> usize {
    use ComponentParsing::*;
    let banned_words: HashSet<&str> = "the first second third fourth floor contains a generator and microchip nothing relevant".split_ascii_whitespace().collect();

    let mut component_map: HashMap<String, ComponentType> = HashMap::new();
    for (floor, line_temp) in source.to_lowercase().replace(",", "").replace(".", "").split("\r\n").enumerate() {
        let line = if floor == 0 {
            String::from(line_temp) + " a a-compatible b b-compatible"
        } else {
            String::from(line_temp)
        };

        for token in line.split_ascii_whitespace().filter(|token| !banned_words.contains(token)) {
            match parse_component(token) {
                Microchip(name) => {
                    let prev = *component_map.get(&name).unwrap_or(&0);
                    //println!("microchip: {} previous {} new {}", name, prev, floor);
                    component_map.insert(name, insert_mic(prev, floor as u8));
                },
                Generator(name) => {
                    let prev = *component_map.get(&name).unwrap_or(&0);
//                    println!("generator: {} previous {} new {}", name, prev, floor);
                    component_map.insert(name, insert_gen(prev, floor as u8));
                }
            }
        }
    }

    let components: Vec<_> = component_map.values().map(|v| *v).collect();

    let original_state = State {
        floor: 0,
        steps: 0,
        comps: components
    };

    let mut past_states: Vec<ComparisonState> = Vec::new();
    let mut update_queue: VecDeque<State> = VecDeque::new();
    let mut current_step: usize = 0;
    update_queue.push_back(original_state);

    while !update_queue.is_empty() {
        let current_state = update_queue.pop_front().unwrap();
        let mut components_compare = current_state.comps.clone();
        components_compare.sort_unstable();
        let comparison_state = ComparisonState{
            comps: components_compare,
            floor: current_state.floor
        };
        //CURRENT_STATE("* ~~~ Current state ~~~~~~~~~~~~");
        //CURRENT_STATE("* current state:");
        //visualize(&current_state);
        if past_states.contains(&comparison_state) {
            //CURRENT_STATE("* has been visited, prune away...");
            //CURRENT_STATE("* ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~");
            continue
        }

        if current_state.steps > current_step {
            //CURRENT_STATE("* step {} is finished", current_step);
            current_step += 1;
        }
   
        if is_finished(&current_state.comps) {
            return current_step
        }

        //CURRENT_STATE("* generating new steps...");
        let new_states = generate_new_states(&current_state);
        past_states.push(comparison_state);
        new_states.into_iter().for_each(
            |state| {
                //visualize(&state); println!();
                update_queue.push_back(state)
            }
        );
        //CURRENT_STATE("* ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~");
    }
    current_step
}