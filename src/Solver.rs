use crate::Maze::Grid::Grid;
use std::{cmp::Reverse, collections::{BinaryHeap, HashMap}, i32};

pub fn dijkstra(start: [usize; 2], end: [usize; 2], grid: &Grid) -> Vec<[usize; 2]> {
    let mut g_score: HashMap<[usize; 2], i32> = HashMap::new();
    let mut previous: HashMap<[usize; 2], [usize; 2]> = HashMap::new();
    let mut queue: BinaryHeap<Reverse<(i32, [usize; 2])>> = BinaryHeap::new();

    for pos in grid.get_all_cells() {
        g_score.insert(pos, i32::MAX);
    }

    g_score.insert(start, 0);
    queue.push(Reverse((0, start)));

    while let Some(Reverse((g, current))) = queue.pop() {

        if current == end {
            break;
        } else if g > g_score[&current] {
            continue;
        }

        for neighbour in grid.find_movable_neighbours(&current) {
            let neighbour_distance = g_score[&current] + 1;

            if neighbour_distance < *g_score.get(&neighbour).unwrap() {
                g_score.insert(neighbour, neighbour_distance);
                previous.insert(neighbour, current);

                queue.push(Reverse((neighbour_distance ,neighbour)));
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

fn manhattan(start: [usize; 2], end: [usize; 2]) -> i32 {
    (end[0] as i32 - start[0] as i32).abs() + (end[1] as i32 - start[1] as i32).abs()
}

pub fn Astar(start: [usize; 2], end: [usize; 2], grid: &Grid) -> Vec<[usize; 2]> {
    let mut g_score: HashMap<[usize; 2], i32> = HashMap::new();
    let mut f_score: HashMap<[usize; 2], i32> = HashMap::new();
    let mut previous: HashMap<[usize; 2], [usize; 2]> = HashMap::new();
    let mut queue: BinaryHeap<Reverse<(i32, [usize; 2])>> = BinaryHeap::new();

    for pos in grid.get_all_cells() {
        g_score.insert(pos, i32::MAX);
        f_score.insert(pos, i32::MAX);
    }

    g_score.insert(start, 0);
    f_score.insert(start, manhattan(start, end));
    queue.push(Reverse((manhattan(start, end), start)));

    while let Some(Reverse((f, current))) = queue.pop() {


        if current == end {
            break;
        } else if f > f_score[&current] {
            continue;
        }

        for neighbour in grid.find_movable_neighbours(&current) {
            let neighbour_distance = g_score[&current] + 1;

            if neighbour_distance < *g_score.get(&neighbour).unwrap() {
                g_score.insert(neighbour, neighbour_distance);
                f_score.insert(neighbour, neighbour_distance + manhattan(neighbour, end));
                previous.insert(neighbour, current);

                queue.push(Reverse((manhattan(neighbour, end) + neighbour_distance, neighbour)));
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