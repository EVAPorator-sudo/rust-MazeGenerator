use crate::Maze::Grid::Grid;
use std::{collections::HashMap, i32};

pub fn dijkstra(start: [usize; 2], end: [usize; 2], grid: &Grid) -> Vec<[usize; 2]> {
    let mut g_score: HashMap<[usize; 2], i32> = HashMap::new();
    let mut previous: HashMap<[usize; 2], [usize; 2]> = HashMap::new();
    let mut queue: Vec<[usize; 2]> = Vec::new();

    for pos in grid.get_all_cells() {
        g_score.insert(pos, i32::MAX);
    }

    g_score.insert(start, 0);
    queue.push(start);

    while !queue.is_empty() {
        let current = lowest_g_score(&mut queue, &g_score);

        if current == end {
            break;
        }

        for neighbour in grid.find_movable_neighbours(&current) {
            let neighbour_distance = g_score[&current] + 1;

            if neighbour_distance < *g_score.get(&neighbour).unwrap() {
                g_score.insert(neighbour, neighbour_distance);
                previous.insert(neighbour, current);

                queue.retain(|&x| x != neighbour);
                queue.push(neighbour);
            }
        }
    }

    let mut path = Vec::new();
    let mut step = end;

    if !previous.contains_key(&end) && start != end {
        return path;
    }

    while step != start {
        path.push(step);
        step = previous[&step];
    }
    path.push(start);
    path.reverse();

    path
}

fn lowest_g_score(queue: &mut Vec<[usize; 2]>, map: &HashMap<[usize; 2], i32>) -> [usize; 2] {
    let mut lowest = 0;
    for i in 1..queue.len() {
        if map[&queue[i]] < map[&queue[lowest]] {
            lowest = i;
        }
    }
    queue.remove(lowest)
}

fn lowest_f_score(queue: &mut Vec<[usize; 2]>, map: &HashMap<[usize; 2], i32>) -> [usize; 2] {
    let mut lowest = 0;
    for i in 1..queue.len() {
        if map[&queue[i]] < map[&queue[lowest]]{
            lowest = i;
        }
    }

    queue.remove(lowest)
}

fn manhattan(start: [usize; 2], end: [usize; 2]) -> i32 {
    ((end[0] as i32 - start[0] as i32).abs() + (end[1] as i32 - start[1] as i32)).abs()
}

pub fn Astar(start: [usize; 2], end: [usize; 2], grid: &Grid) -> Vec<[usize; 2]> {
    let mut g_score: HashMap<[usize; 2], i32> = HashMap::new();
    let mut f_score: HashMap<[usize; 2], i32> = HashMap::new();
    let mut previous: HashMap<[usize; 2], [usize; 2]> = HashMap::new();
    let mut queue: Vec<[usize; 2]> = Vec::new();

    for pos in grid.get_all_cells() {
        g_score.insert(pos, i32::MAX);
        f_score.insert(pos, i32::MAX);
    }

    g_score.insert(start, 0);
    f_score.insert(start, manhattan(start, end));
    queue.push(start);

    while !queue.is_empty() {
        let current = lowest_f_score(&mut queue, &f_score);

        if current == end {
            break;
        }

        for neighbour in grid.find_movable_neighbours(&current) {
            let neighbour_distance = g_score[&current] + 1;

            if neighbour_distance < *g_score.get(&neighbour).unwrap() {
                g_score.insert(neighbour, neighbour_distance);
                f_score.insert(neighbour, neighbour_distance + manhattan(neighbour, end));
                previous.insert(neighbour, current);

                queue.retain(|&x| x != neighbour);
                queue.push(neighbour);
            }
        }
    }

    let mut path = Vec::new();
    let mut step = end;

    if !previous.contains_key(&end) && start != end {
        return path;
    }

    while step != start {
        path.push(step);
        step = previous[&step];
    }
    path.push(start);
    path.reverse();

    path
}