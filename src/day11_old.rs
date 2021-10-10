use std::collections::HashSet;
use itertools::Itertools;

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
enum Component {
    Generator(String),
    Microchip(String)
}

#[derive(Debug, Clone)]
struct Floor {
    generators: Generators,
    microchips: Microchips
}

type Generators = HashSet<String>;
type Microchips = HashSet<String>;
type Floors = Vec<Floor>;

fn parse_component(token: &str) -> Component {
    // can contain an and at the start => len == 4
    // either a type-compatible microship
    // or a type generator
    let temp: Vec<_> = token.split_ascii_whitespace().collect();
    let tokens: Vec<_> = if temp.len() == 4 { // contains an and
        temp.into_iter().skip(1).collect()
    } else {
        temp
    };

    match tokens[2] {
        "generator" | "generator." => Component::Generator(String::from(tokens[1])),
        "microchip" | "microchip." => {
            let temp: Vec<_> = tokens[1].split("-").collect();
            Component::Microchip(String::from(temp[0]))
        },
        _ => panic!("unknwon component \"{}\"", tokens[2])
    }
}

fn parse_line(line: &str) -> Floor {
    let temp: Vec<_> = line.splitn(5, " ").collect();
    match temp[4] {
        "nothing relevant." => Floor { generators: Generators::new(), microchips: Microchips::new() },
        _ => {
            let mut microchips = Microchips::new();
            let mut generators = Generators::new();
            for comp in temp[4].split(", ").map(|component| parse_component(component)) {
                match comp {
                    Component::Generator(gen) => generators.insert(gen),
                    Component::Microchip(mic) => microchips.insert(mic)
                };
            };
            Floor { generators: generators, microchips: microchips }
        }
    }
}

#[derive(Debug, Clone)]
enum ItemBag {
    OneItem(Component),
    TwoItems(Component, Component)
}
#[derive(Debug)]
struct Movement {
    new_floor: usize,
    items: ItemBag
}

fn move_is_valid_for_component(comp: &Component, prev_floor: &Floor, new_floor: &Floor) -> bool {
    use Component::*;
    match comp {
        Microchip(mic) => {
            new_floor.generators.is_empty() || // there are no generators
            new_floor.generators.contains(mic) // there are generators but mic is compatible with at least one
        },
        Generator(gen) => {
            (prev_floor.microchips.is_empty() ||    // the move has no effect 
            prev_floor.generators.len() == 1 ||     // removed the last generator
            !prev_floor.microchips.contains(gen))   // there are more generators remaining but the compatible microchip was not present
            &&
            (new_floor.microchips.is_empty() || // there are no microchips
            (new_floor.microchips.len() == 1 && new_floor.microchips.contains(gen)) || // if only compatible microchip is present here it implies that no other generators are,
            // since no valid move would generate that case
            new_floor.microchips.is_subset(&new_floor.generators)) // every microchip that exists is already paired
        }
    }
}

fn move_is_valid_for_components(comp1: &Component, comp2: &Component, prev_floor: &Floor, new_floor: &Floor) -> bool {
    use Component::*;
    match (comp1, comp2) {
        (Microchip(mic), Generator(gen)) | (Generator(gen), Microchip(mic)) => {
            if mic != gen {
                false // generators and microchips can only be moved in pairs if they are equal
            } else {
                new_floor.microchips.is_empty() || // no microchips to fry
                new_floor.microchips.is_subset(&new_floor.generators) // microchips are already paired
            }
        },
        (Microchip(mic1), Microchip(mic2)) => {
            new_floor.generators.is_empty() || // no generators that can affect them
            (new_floor.generators.contains(mic1) && new_floor.generators.contains(mic2)) // compatible generators exist
        },
        (Generator(gen1), Generator(gen2)) => {
            (prev_floor.microchips.is_empty() || // no microchips affected
                prev_floor.generators.len() == 2 || // removed the last generators
                !prev_floor.microchips.contains(gen1) && !prev_floor.microchips.contains(gen2) // generators can't leave compatible microchips
            ) 
            && 
            (new_floor.microchips.is_empty() || // no microchips affected
                new_floor.microchips.is_subset(&new_floor.generators) || // all are already protected
                {
                    let gen1_matching = new_floor.microchips.contains(gen1);
                    let gen2_matching = new_floor.microchips.contains(gen2);
                    (gen1_matching || gen2_matching) && // there can't be microchips existing that don't match
                    (new_floor.microchips.len() == 1 || // the one that matched is the only one
                        (gen1_matching && gen2_matching && new_floor.microchips.len() == 2) // the two existing are matching
                    )
                }
            )
        }
    }
}

fn move_is_valid(movement: &Movement, previous_floor: usize, floors: &Floors) -> bool {
    use ItemBag::*;
    if movement.new_floor > 3 && movement.new_floor != previous_floor {
        false
    } else {
        let prev_floor = floors.get(previous_floor).unwrap();
        let new_floor = floors.get(movement.new_floor).unwrap();
        match &movement.items {
            OneItem(comp) => {
                move_is_valid_for_component(comp, prev_floor, new_floor)
            },
            TwoItems(comp1, comp2) => {
                move_is_valid_for_components(comp1, comp2, prev_floor, new_floor)
            }
        }
    }
}

// TODO: This causes stack overflow, since there are too many paths to try
fn find_path(floors: Floors, elevator_position: usize, steps_here: usize, total_items: usize, mut previous_floors: Vec<String>) -> usize {
    if floors[3].generators.len() + floors[3].microchips.len() == total_items {
        return steps_here
    } 

    let floor_ident = unique_identifier(&floors);
    println!("at depth {}: {}", steps_here, floor_ident);
    if previous_floors.contains(&floor_ident) {
        return usize::MAX
    }
    previous_floors.push(floor_ident);


    let single_item: Vec<_> = floors[elevator_position].generators.iter()
    .map(|s| Component::Generator(String::from(s)))
    .chain(
        floors[elevator_position].microchips.iter()
        .map(|s| Component::Microchip(String::from(s)))
    ).collect();

    let two_items: Vec<Vec<Component>> = single_item.iter().cloned().combinations(2).collect();

    let temp: Vec<_> = single_item
    .into_iter()
    .map(
        |c| ItemBag::OneItem(c)
    ).chain(
        two_items
        .into_iter()
        .map(
            | cs | ItemBag::TwoItems(cs[0].clone(), cs[1].clone())
        )
    ).collect();

    let nr_options = temp.len();
    let best_path: Option<usize> = temp.into_iter()
    .cycle()
    .zip(
        std::iter::repeat(elevator_position + 1).take(nr_options)
        .chain(
            std::iter::repeat(elevator_position - 1).take(nr_options)
        )
    )
    .map(
        | (i_bag, f) | Movement {
            new_floor: f,
            items: i_bag
        }
    )
    .filter(
        | m | move_is_valid(&m, elevator_position, &floors)
    ) 
    .map(|valid_move|
        {
            let mut new_floors: Vec<_> = floors.iter().cloned().collect();
            let item_v: Vec<_> = match valid_move.items {
                ItemBag::OneItem(c) => vec!(c),
                ItemBag::TwoItems(c1, c2) => vec!(c1,c2)
            };

            for item in item_v {
                match item {
                    Component::Microchip(mic) => {
                        new_floors[elevator_position].microchips.remove(&mic);
                        new_floors[valid_move.new_floor].microchips.insert(mic);
                    },
                    Component::Generator(gen) => {
                        new_floors[elevator_position].generators.remove(&gen);
                        new_floors[valid_move.new_floor].generators.insert(gen);
                    }
                }
            };

            find_path(new_floors, valid_move.new_floor, steps_here+1, total_items, previous_floors.clone())
        }
    )
    .min();

    match best_path {
        None => usize::MAX,
        Some(v) => v
    }
}

fn unique_identifier(floors: &Floors) -> String {
    let mut ident = String::from("");
    for (i, floor) in floors.iter().enumerate() {
        let mut generators: Vec<_> = floor.generators.iter().map(|s| s.clone()).collect();
        let mut microchips: Vec<_> = floor.microchips.iter().map(|s| s.clone()).collect();
        generators.sort();
        microchips.sort();
        ident += format!("F{} ", i).as_str();
        ident += "G";
        ident += generators.join(",").as_str();
        ident += " M";
        ident += microchips.join(",").as_str();
    };
    ident
}

// !NOTE: you have to add a "," before the and otherwise the parsing wont work
pub fn part1(source: String) -> usize {
    let floors: Floors = source.split("\r\n").map(|line| parse_line(line)).collect();
    let nr_items: usize = floors.iter().map(|f| f.generators.len() + f.microchips.len()).sum();
    find_path(floors, 0, 0, nr_items, Vec::new())
}

pub fn part2(source: String) -> i32 {
    0
}