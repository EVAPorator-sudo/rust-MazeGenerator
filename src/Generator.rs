use rand::RngExt;

use crate::Maze::Grid::Grid;
use crate::directions;

pub fn Ellers(mut grid: Grid) -> Grid {
    if grid.width == 0 || grid.height == 0 {
        return grid;
    }
    let mut rng = rand::rng();

    for row_index in 0..grid.height {
        for cell_index in 1..grid.width {
            if rng.random_bool(0.5) {
                grid.Merge([cell_index, row_index], directions::left);
            }
        }

        let mut sets: Vec<Vec<[usize; 2]>> = vec![vec![]];
        let mut set: usize = 0;
        sets.get_mut(set).unwrap().push([0, row_index]);

        for cell_index in 1..grid.width {
            if grid.get_cell([cell_index, row_index]).walls[0] {
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

            let mut carve_points: Vec<usize> = Vec::new();

            while carve_points.len() < carves {
                let new_carve: usize = rng.random_range(0..working_set.len());
                if !carve_points.contains(&new_carve) {
                    carve_points.push(new_carve);
                }
            }
            for index in 0..carve_points.len() {
                grid.Merge(*working_set.get(index).unwrap(), directions::down);
            }
        }
    }

    for cell_index in 0..(grid.width - 1) {
        grid.Merge([cell_index, grid.height - 1], directions::right);
    }

    grid
}
