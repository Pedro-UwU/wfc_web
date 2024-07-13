use rand::{seq::SliceRandom, thread_rng, Rng};
use std::{
    collections::{HashMap, HashSet, VecDeque},
    error::Error,
    usize::{self, MAX as USIZE_MAX},
};
use tracing::{error, info};

use crate::models::wfc::*;

#[derive(Debug, Copy, Clone)]
enum Dirs {
    North = 0,
    East = 1,
    South = 2,
    West = 3,
}

pub fn start_new_generation(info: BuildInfo, neighbors: HashMap<usize, Neighbors>) {
    // 1) Get the possible values
    let values: Vec<usize> = neighbors.keys().map(|&x| x).collect();
    info!("Possible Values: {:?}", values);
    // 2) Construct grid
    let mut grid = create_full_grid(info.width, info.height, &values);
    info!("Grid Generated");

    while has_uncollapsed(&grid) {
        // 3) Collapse the cell with least entropy
        let collapsed_index = match collapse_random_cell(info.width, info.height, &mut grid) {
            Some(index) => index,
            None => {
                error!("No indices to collapse"); // TODO, check if the grid is full
                return;
            }
        };
        info!(
            "Collapsed cell {} to value {:?}",
            collapsed_index, grid[collapsed_index]
        );

        // 4) Propagate restrictions
        let could_propagate = propagate_restrictions(
            info.width,
            info.height,
            &mut grid,
            &neighbors,
            collapsed_index,
        );

        if let Err(msg) = could_propagate {
            error!(msg);
            return;
        }
    }
    info!("Generation finished");
    info!("Grid: {:?}", grid);
}

fn has_uncollapsed<T>(grid: &Vec<Vec<T>>) -> bool {
    for v in grid {
        if v.len() > 1 {
            return true;
        }
    }
    false
}

fn create_full_grid(width: usize, height: usize, values: &Vec<usize>) -> Vec<Vec<usize>> {
    let mut grid = vec![];
    for _ in 0..(height * width) {
        // Flatten Array to optimize lookup
        grid.push(values.clone());
    }
    grid
}

fn get_random_index_of_least_entropy<T>(len: usize, grid: &mut Vec<Vec<T>>) -> Option<usize> {
    let mut mins: Vec<usize> = vec![];
    let mut min_value = USIZE_MAX;
    for i in 0..len {
        if grid[i].len() > 1 && grid[i].len() < min_value {
            min_value = grid[i].len();
            mins = vec![];
            mins.push(i);
        } else if grid[i].len() == min_value {
            mins.push(i);
        }
    }
    if mins.len() == 0 {
        return None;
    }
    let mut rng = thread_rng();
    Some(mins[rng.gen_range(0..mins.len())])
}

/// Return the collapsed index
fn collapse_random_cell(width: usize, height: usize, grid: &mut Vec<Vec<usize>>) -> Option<usize> {
    let index_to_collapse: usize = match get_random_index_of_least_entropy(width * height, grid) {
        Some(i) => i,
        None => return None,
    };
    let mut rng = thread_rng();
    let random_value: usize = *grid[index_to_collapse].choose(&mut rng).unwrap();
    grid[index_to_collapse] = vec![random_value];
    Some(index_to_collapse)
}

fn propagate_restrictions(
    width: usize,
    height: usize,
    grid: &mut Vec<Vec<usize>>,
    graph: &HashMap<usize, Neighbors>,
    center_of_prop: usize,
) -> Result<(), Box<dyn Error>> {
    let mut queue: VecDeque<usize> = VecDeque::new();
    let mut marked: HashSet<usize> = HashSet::new();
    let mut result: Result<(), Box<dyn Error>> = Ok(());

    // Insert the center
    marked.insert(center_of_prop);

    let (center_x, center_y) = from_index_to_coords(center_of_prop, width, height);
    let center_neighbors = get_valid_neighbors(center_x, center_y, width, height);
    // Push the neighbors from the center
    for n in center_neighbors {
        queue.push_back(n.0);
    }

    // This is basically a BFS
    while !queue.is_empty() {
        // Get the top of the queue
        let current: usize = queue.pop_front().unwrap();
        if marked.contains(&current) {
            continue;
        }

        // Mark that cell as visited
        marked.insert(current);

        let (x, y) = from_index_to_coords(current, width, height);
        // info!("Propagating cell {},{}", x, y);
        let neighbors = get_valid_neighbors(x, y, width, height);
        info!("Neighbors of cell {} are {:?}", current, neighbors);
        let reduced: bool = match reduce_cell_entropy(current, &neighbors, grid, graph) {
            Ok(b) => b,
            Err(msg) => {
                error!("Could not reduce after collapsing {},{}: {}", x, y, msg);
                result = Err("Could not reduce grid".into());
                break;
            }
        };

        if reduced {
            for (i, _) in neighbors {
                if marked.contains(&i) {
                    continue;
                }
                queue.push_back(i);
            }
        }
    }
    result
}

fn get_valid_neighbors(x: usize, y: usize, width: usize, height: usize) -> Vec<(usize, Dirs)> {
    let mut neighbors: Vec<(usize, Dirs)> = vec![];
    if x > 0 {
        neighbors.push((from_coords_to_index(x - 1, y, width, height), Dirs::West));
    }
    if x < (width - 1) {
        neighbors.push((from_coords_to_index(x + 1, y, width, height), Dirs::East));
    }
    if y > 0 {
        neighbors.push((from_coords_to_index(x, y - 1, width, height), Dirs::North));
    }
    if y < (height - 1) {
        neighbors.push((from_coords_to_index(x, y + 1, width, height), Dirs::South));
    }
    neighbors
}

fn reduce_cell_entropy(
    index: usize,
    neighbors: &Vec<(usize, Dirs)>,
    grid: &mut Vec<Vec<usize>>,
    graph: &HashMap<usize, Neighbors>,
) -> Result<bool, Box<dyn Error>> {
    let all_possible_values_set = neighbors
        .into_iter()
        // Map each neighbors to an array of possible values
        .map(|(neighbor_index, dir)| {
            match get_possibilities_from_neighbor(*neighbor_index, *dir, grid, graph) {
                Ok(possible_values) => possible_values,
                _ => Vec::new(),
            }
        })
        .map(|vec| vec.into_iter().collect::<HashSet<usize>>())
        // Reduce the array of array to a set of the intersection of all possible_values from neighbors
        .fold(None, |acc: Option<HashSet<usize>>, curr_set| match acc {
            None => Some(curr_set),
            Some(set) => {
                let new_set: HashSet<usize> = set.intersection(&curr_set).cloned().collect();
                Some(new_set)
            }
        });

    let all_possible_values = match all_possible_values_set {
        Some(set) => set,
        None => HashSet::new(),
    };

    if all_possible_values.len() == 0 {
        info!("Cell {} has 0 possible values", index);
    }
    let current_values: HashSet<usize> = grid[index].clone().into_iter().collect();
    let new_values: HashSet<usize> = current_values
        .intersection(&all_possible_values)
        .cloned()
        .collect();
    info!(
        "Current cell {} - Current values: {:?}, New Values: {:?}",
        index, current_values, new_values
    );
    if new_values.len() == current_values.len() {
        info!(
            "No reductions on cell {}. Current Values: {:?}, New Values: {:?}",
            index, current_values, new_values
        );
        return Ok(false);
    }
    info!(
        "Cell {} has reduced it's entropy to {:?}",
        index, new_values
    );

    grid[index] = new_values.into_iter().collect();
    Ok(true)
}

fn get_possibilities_from_neighbor(
    neighbor_index: usize,
    dir: Dirs,
    grid: &Vec<Vec<usize>>,
    graph: &HashMap<usize, Neighbors>,
) -> Result<Vec<usize>, Box<dyn Error>> {
    let possible_neighbor_values: Vec<usize> = grid[neighbor_index].clone();
    Ok(possible_neighbor_values
        .into_iter()
        .map(|val| {
            let vals = get_possibles_from_value(val, dir, graph);
            match vals {
                Ok(arr) => arr,
                _ => Vec::new(),
            }
        })
        .flatten()
        .collect())
}

fn get_opposite_dir(dir: Dirs) -> Dirs {
    match dir {
        Dirs::North => Dirs::South,
        Dirs::South => Dirs::North,
        Dirs::West => Dirs::East,
        Dirs::East => Dirs::West,
    }
}

fn get_possibles_from_value(
    value: usize,
    dir: Dirs,
    graph: &HashMap<usize, Neighbors>,
) -> Result<Vec<usize>, Box<dyn Error>> {
    if !graph.contains_key(&value) {
        return Err("Value not in graph".into());
    }

    let opp = get_opposite_dir(dir);
    match opp {
        Dirs::North => Ok(graph.get(&value).unwrap().n.clone()),
        Dirs::South => Ok(graph.get(&value).unwrap().s.clone()),
        Dirs::East => Ok(graph.get(&value).unwrap().e.clone()),
        Dirs::West => Ok(graph.get(&value).unwrap().w.clone()),
    }
}

fn from_index_to_coords(index: usize, width: usize, _height: usize) -> (usize, usize) {
    let y = index / width;
    let x = index % width;
    (x, y)
}

fn from_coords_to_index(x: usize, y: usize, width: usize, _height: usize) -> usize {
    y * width + x
}
