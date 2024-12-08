use std::collections::{HashMap, HashSet};

#[derive(PartialEq)]
enum Part {
    One,
    Two,
}

type Coords = (isize, isize); // `(.0, .1)` = (x, y)

const PATH: &str = "src/input/day8/input.txt";
const MAP_UB: isize = 50;

fn within_bounds(antinode: Coords) -> bool {
    -1 < antinode.0 && -1 < antinode.1 && antinode.0 < MAP_UB && antinode.1 < MAP_UB
}

fn find_antinodes(node1: Coords, node2: Coords, part: Part) -> Vec<Coords> {
    let mut antinodes = vec![];
    let dist = (node2.0 - node1.0, node2.1 - node1.1);

    let mut next_antinode = (node2.0 + dist.0, node2.1 + dist.1);
    loop {
        if within_bounds(next_antinode) {
            antinodes.push(next_antinode);
        } else {
            break;
        }
        if part == Part::One {
            break;
        }
        next_antinode = (next_antinode.0 + dist.0, next_antinode.1 + dist.1);
    }

    if part == Part::Two && !antinodes.is_empty() {
        // antinodes exist: actual nodes are antinodes too
        antinodes.push(node1);
        antinodes.push(node2);
    }

    antinodes
}

fn fill_antinodes(antinodes: &mut HashSet<Coords>, node1: Coords, node2: Coords, part: Part) {
    let dist = (node2.0 - node1.0, node2.1 - node1.1);

    let mut next_antinode = (node2.0 + dist.0, node2.1 + dist.1);
    loop {
        if within_bounds(next_antinode) {
            antinodes.insert(next_antinode);
        } else {
            break;
        }
        if part == Part::One {
            break;
        }
        next_antinode = (next_antinode.0 + dist.0, next_antinode.1 + dist.1);
    }

    if part == Part::Two && !antinodes.is_empty() {
        // antinodes exist: actual nodes are antinodes too
        antinodes.insert(node1);
        antinodes.insert(node2);
    }
}

fn main() {
    let mut map: HashMap<char, Vec<Coords>> = HashMap::new();
    let mut antinodes: HashSet<Coords> = HashSet::new();
    let mut antinodes2: HashSet<Coords> = HashSet::new();

    std::fs::read_to_string(PATH)
        .unwrap()
        .char_indices()
        .for_each(|(offset, freq)| {
            if freq != '.' && freq != '\n' {
                // casted to `isize` to steer clear of overflow/underflow issues
                // in `get_antinodes`. `MAP_UB` + 1 `\n` every line
                let node = (offset as isize % (MAP_UB + 1), offset as isize / (MAP_UB + 1));
                map.entry(freq)
                    .and_modify(|nodes| nodes.push(node))
                    .or_insert(vec![node]);
            }
        });

    for (freq, nodes) in &map {
        for (idx1, node1) in nodes.iter().enumerate() {
            for (_, node2) in nodes.iter().enumerate().filter(|(idx2, _)| *idx2 != idx1) {
                antinodes2.extend(find_antinodes(*node1, *node2, Part::One));
                fill_antinodes(&mut antinodes, *node1, *node2, Part::One);
            }
        }
    }

    println!("part1: {:?}", antinodes.len());

    for (freq, nodes) in &map {
        for (idx1, node1) in nodes.iter().enumerate() {
            for (_, node2) in nodes.iter().enumerate().filter(|(idx2, _)| *idx2 != idx1) {
                antinodes2.extend(find_antinodes(*node1, *node2, Part::Two));
                fill_antinodes(&mut antinodes, *node1, *node2, Part::Two);
            }
        }
    }

    println!("part2: {:?}", antinodes.len());
    println!("part2 :: 2: {:?}", antinodes2.len());
}
