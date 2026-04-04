use std::collections::HashSet;

use rand::RngExt;

use crate::Maze::Grid::Grid;
use crate::directions;
use crate::directions::*;

pub fn Ellers(mut grid: Grid) -> Grid {
    if grid.width == 0 || grid.height == 0 {
        return grid;
    }
    let mut rng = rand::rng();

    for row_index in 0..(grid.height - 1) {
        for cell_index in 1..grid.width {
            if rng.random_bool(0.5) {
                grid.Merge([cell_index, row_index], &directions::left);
            }
        }

        let mut sets: Vec<Vec<[usize; 2]>> = vec![vec![]];
        let mut set: usize = 0;
        sets.get_mut(set).unwrap().push([0, row_index]);

        for cell_index in 1..grid.width {
            if grid.get_cell(&[cell_index, row_index]).walls[0] {
                sets.push(vec![]);
                set += 1;
            }
            sets.get_mut(set).unwrap().push([cell_index, row_index]);
        }

        for working_set in sets {
            let carves;
            if working_set.len() < 2 {
                carves = 1;
            } else {
                carves = rng.random_range(1..working_set.len());
            }

            let mut indexes: Vec<usize> = (0..working_set.len()).collect();

            for i in 0..carves {
                let j = rng.random_range(i..indexes.len());
                indexes.swap(i, j);
            }

            let carve_points = &indexes[..carves];

            for carve_point in carve_points {
                grid.Merge(*working_set.get(*carve_point).unwrap(), &directions::down);
            }
        }
    }

    for cell_index in 0..(grid.width - 1) {
        grid.Merge([cell_index, grid.height - 1], &directions::right);
    }

    grid
}

fn direction_find(coords: &[usize; 2], direction: &directions) -> [usize; 2] {
    match direction {
        left => [coords[0] - 1, coords[1]],
        right => [coords[0] + 1, coords[1]],
        up => [coords[0], coords[1] - 1],
        down => [coords[0], coords[1] + 1],
    }
}

pub fn Growing_Tree(mut grid: Grid, weighting: f32) -> Grid {
    if grid.width == 0 || grid.height == 0 {
        return grid;
    }
    let mut rng = rand::rng();

    let mut active_list: Vec<[usize; 2]> = Vec::new();
    let mut visited: HashSet<[usize; 2]> = HashSet::new();

    active_list.push([
        rng.random_range(0..(grid.width)),
        rng.random_range(0..(grid.height)),
    ]);

    while !active_list.is_empty() {
        let active_cell: [usize; 2];
        if rng.random_range(0.00..1.00) < weighting && active_list.len() > 1 {
            active_cell = active_list[rng.random_range(0..(active_list.len()))];
        } else {
            active_cell = active_list[active_list.len() - 1]
        }

        visited.insert(active_cell);
        let neighbours = grid.find_neighbours(&active_cell, &visited);

        if neighbours.is_empty() {
            let active_index = active_list.iter().position(|x| *x == active_cell).unwrap();
            active_list.swap_remove(active_index);
        } else if neighbours.len() == 1 {
            let neighbour = direction_find(&active_cell, &neighbours[0]);
            grid.Merge(active_cell, &neighbours[0]);
            if !visited.contains(&neighbour) {
                active_list.push(neighbour);
                visited.insert(neighbour);
            }
        } else {
            let direction = &neighbours[rng.random_range(0..(neighbours.len()))];
            grid.Merge(active_cell, direction);
            let neighbour = direction_find(&active_cell, direction);
            if !visited.contains(&neighbour) {
                active_list.push(neighbour);
                visited.insert(neighbour);
            }
        }
    }
    grid
}
