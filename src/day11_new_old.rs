use std::collections::HashSet;
use std::collections::VecDeque;
use itertools::Itertools;
use std::rc::Rc;


#[derive(Clone, Debug)]
enum Component {
    Generator(String),
    Microchip(String)
}

type Generators = HashSet<String>;
type Microchips = HashSet<String>;

type GenericComponents = HashSet<usize>;

#[derive(Debug)]
struct Floor {
    generators: Generators,
    microchips: Microchips
}

#[derive(Debug)]
struct GenericFloor {
    generators: GenericComponents,
    microchips: GenericComponents
}

type GenericFloors = Vec<GenericFloor>;
struct GenericState {
    steps: usize,
    elevator: usize,
    floors: GenericFloors
}

type Floors = Vec<Floor>;
struct State {
    steps: usize,
    floors: Floors,
    elevator: usize 
}

enum BackwardsTree {
    Nil,
    Node(Rc<State>, Rc<BackwardsTree>) // floors, parent, children
}
type UpdateQueue = VecDeque<Rc<BackwardsTree>>;


fn print_state(state: &State) {
    println!("-- Step {} --", state.steps);
    for (i, floor) in state.floors.iter().enumerate() {
        if i == state.elevator {
            print!("{} E: [", i);
        } else {
            print!("{}  : [ ", i);
        }
        for gen in floor.generators.iter() {
            print!("{} ", gen);
        }
        print!("] {{ ");
        for mic in floor.microchips.iter() {
            print!("{} ", mic);
        }
        print!("}}\n");
    }
}


fn parse_component(token: &str) -> Component {
    if token.contains("compatible") {
        Component::Microchip(String::from(token.split("-").collect::<Vec<&str>>()[0]))
    } else {
        Component::Generator(String::from(token))
    }
}

fn is_same_state(a: &State, b: &State) -> bool {
    if a.elevator != b.elevator {
        false 
    } else {
        for i in 0..4 {
            if  a.floors[i].generators != b.floors[i].generators || 
                a.floors[i].microchips != b.floors[i].microchips {
                return false
            }
        }
        true
    }
}


fn is_equiv_state(a: &State, b: &State) -> bool {

}

fn is_finished(a: &State) -> bool {
    for i in 0..3 {
        if !(a.floors[i].generators.is_empty() && a.floors[i].microchips.is_empty()) {
            return false;
        }
    };
    true
}

fn state_is_unique_to_branch(ref_state: &State, node: &BackwardsTree) -> bool{
    use BackwardsTree::*;
    match node {
        Nil => true,
        Node(state, parent) => {
            if is_same_state(ref_state, state) {
                false
            } else {
                state_is_unique_to_branch(ref_state, parent)
            }
        }
    }
}

fn floor_is_valid(floor: &Floor) -> bool {
    floor.generators.is_empty() ||
    floor.microchips.is_empty() || // there are generators but no microchips
    floor.microchips.is_subset(&floor.generators) // every microchip has a parent 
}

fn state_is_valid(state: &State) -> bool {
    //println!("### Validating state ###");
    //print_state(state);
    
    for floor in &state.floors {
        if !floor_is_valid(&floor) {
    //        println!("result: INVALID\n########################");

            return false
        }
    }
    //println!("result: VALID\n########################");
    true
}

enum ComponentBag {
    OneItem(Component),
    TwoItems(Component, Component)
}

struct StateMovement<'a> {
    new_floor: usize,
    moving_components: ComponentBag,
    prev_state: &'a State,
}

fn generate_state(state_mov: StateMovement) -> State {
    use ComponentBag::*;
    use Component::*;
    
//    println!("=== GENERATE NEW STATE ===");
//    println!("Orig:");
//    print_state(state_mov.prev_state);
//    println!("New floor: {}", state_mov.new_floor);

    let prev_floor = state_mov.prev_state.elevator;
    let mut new_floors: Floors = Vec::new();
    let components: Vec<_> = match state_mov.moving_components {
        OneItem(c) => vec!(c),
        TwoItems(c1, c2) => vec!(c1, c2)
    };

  //  print!("Moved Components: ");
  //  for comp in components.iter() {
  //      print!("{:?} ", comp);
  //  }
  //  println!();

    for (i, floor) in state_mov.prev_state.floors.iter().enumerate() {
        // make copy of old state
        new_floors.push(
            Floor {
                generators: floor.generators.clone(),
                microchips: floor.microchips.clone()
            }
        );

        if i == prev_floor {
            // remove the moved items from that floor
            components.iter().for_each(
                | comp | { match comp {
                    Generator(gen) => new_floors[i].generators.remove(gen),
                    Microchip(mic) => new_floors[i].microchips.remove(mic)
                }; }
            );
        } else if i == state_mov.new_floor {
            // add items to the new floor
            components.iter().for_each(
                | comp | { match comp {
                    Generator(gen) => new_floors[i].generators.insert(gen.clone()),
                    Microchip(mic) => new_floors[i].microchips.insert(mic.clone())
                }; }
            );
        }
    };

    let temp = State {
        steps: state_mov.prev_state.steps + 1,
        floors: new_floors,
        elevator: state_mov.new_floor 
    };

 //   println!("New:");
 //   print_state(&temp);
 //   println!("==========================");
    temp
}


fn generate_new_states(current_state: &State) -> Vec<State> {
    let elevator = current_state.elevator;
    let new_floor_nrs = if elevator == 0 {
        vec!(1)
    } else if elevator == 3 {
        vec!(2)
    } else {
        vec!(elevator-1, elevator+1)
    };

    let mut new_states: Vec<State> = Vec::new();
    for new_floor in new_floor_nrs {
        let mut is_backtrace = true;
        for floor in 0..new_floor+1 {
            if  !current_state.floors[floor].generators.is_empty() || 
                !current_state.floors[floor].microchips.is_empty() {
                    is_backtrace = false;
            }
        }

        if is_backtrace { // you never need to move down if all items are above
            continue
        }

        let singles: Vec<_> = current_state.floors[current_state.elevator].generators.iter()
        .map(
            |mic| Component::Generator(String::from(mic))
        )
        .chain(
            current_state.floors[current_state.elevator].microchips.iter()
            .map(
                |gen| Component::Microchip(String::from(gen))
            )
        )
        .collect();


        singles.iter().combinations(2).map(
            |comps| ComponentBag::TwoItems(comps[0].clone(), comps[1].clone())
        ).chain(
            singles.iter()
            .map(
                |comp| ComponentBag::OneItem(comp.clone())
            )
        )
        .map(
            |c_bag| generate_state(
                StateMovement{ 
                    new_floor: new_floor, 
                    moving_components: c_bag, 
                    prev_state: current_state 
                }
            )
        ) 
        .filter(
            |state| state_is_valid(&state)
        )
        .for_each(
            |state| new_states.push(state)
        );
    }


//    println!("### Newly generated steps from ###");
//    print_state(current_state);
//    println!("----------------------------------");
//    for state in new_states.iter() {
//        print_state(state);
//        println!();
//    }
//    println!("----------------------------------");
    
    new_states
}

pub fn part1(source: String) -> usize {
    use BackwardsTree::*;
    let mut ignore_words: HashSet<&str> = HashSet::new();
    for w in "the first second third fourth floor contains a and generator microchip nothing relevant".split_ascii_whitespace() {
        ignore_words.insert(w);
    }

    let mut orig_state = State{ steps: 0, floors: Vec::new(), elevator: 0};
    for (i, l) in source.split("\r\n").enumerate() {
        let components: Vec<_> = l.replace(",", "").replace(".","").to_lowercase().split_ascii_whitespace()
        .filter(|w| !ignore_words.contains(w))
        .map(|token| parse_component(token)).collect();

        
        orig_state.floors.push(Floor{generators: Generators::new(), microchips: Microchips::new()});
        for component in components {
            match component {
                Component::Generator(gen) => orig_state.floors[i].generators.insert(gen),
                Component::Microchip(mic) => orig_state.floors[i].microchips.insert(mic)
            };
        }
    }

    let root = Rc::new(Node(Rc::new(orig_state), Rc::new(Nil)));
    let mut update_queue = UpdateQueue::new();
    let mut current_step = 0;
    let mut past_states: Vec<Rc<State>> = Vec::new();
    update_queue.push_back(Rc::clone(&root));
    
    while !update_queue.is_empty(){
        let current = update_queue.pop_front().unwrap(); 
//        println!("Current state: ");
        let mut solution_found = false;
        match &*current {
            Node(current_state, parent) => {
 //               print_state(current_state);
                current_step = if current_state.steps > current_step { println!("{}", current_state.steps); current_state.steps } else { current_step };
                let mut do_generate_new = true;
                for old_state in past_states.iter() {
                    if is_same_state(&current_state, &old_state) {
                        do_generate_new = false;
                    }
                }

                if do_generate_new {
                    past_states.push(Rc::clone(current_state));
                    generate_new_states(current_state)
                    .into_iter()
                    .filter(|state| state_is_unique_to_branch(state, &*current))
                    .for_each(
                        |valid_state| {
                            if is_finished(&valid_state) {
                                solution_found = true;
                            }
                            update_queue.push_back(
                                Rc::new(
                                    Node(Rc::new(valid_state), Rc::clone(&parent))
                                )
                            );
                        }
                    );
                };
            },
            _ => panic!("tried to evaluate empty node")
        };

        if solution_found {
            return current_step + 1;
        }
    }
    0
}


pub fn part2(_source: String) -> i32 {
    0
}