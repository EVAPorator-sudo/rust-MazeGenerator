use crate::Maze::Grid::Grid;
use std::fs;

pub fn grid_draw(grid: &Grid) -> String {
    let mut svg = String::new();

    let cell_size = 20;
    let wall_thickness = 1;
    let cols = grid.width;
    let rows = grid.height;

    let image_width = cols * cell_size + 2 * wall_thickness;
    let image_height = rows * cell_size + 2 * wall_thickness;

    svg.push_str(&format!(
        r#"<svg xmlns="http://www.w3.org/2000/svg" width="{}" height="{}">"#,
        image_width, image_height
    ));
    svg.push_str(&format!(
        r#"<rect width="{}" height="{}" fill="white"/>"#,
        image_width, image_height
    ));

    svg.push_str(&format!(
        r#"<rect x="0" y="0" width="{}" height="{}" fill="black"/>"#,
        image_width,
        2 * wall_thickness
    ));
    svg.push_str(&format!(
        r#"<rect x="0" y="0" width="{}" height="{}" fill="black"/>"#,
        2 * wall_thickness,
        image_height
    ));
    svg.push_str(&format!(
        r#"<rect x="0" y="{}" width="{}" height="{}" fill="black"/>"#,
        image_height - 2 * wall_thickness,
        image_width,
        2 * wall_thickness
    ));
    svg.push_str(&format!(
        r#"<rect x="{}" y="0" width="{}" height="{}" fill="black"/>"#,
        image_width - 2 * wall_thickness,
        2 * wall_thickness,
        image_height
    ));

    for row in grid.row_list.iter() {
        for cell in row.cell_list.iter() {
            let pos = cell.position;
            let walls = &cell.walls;

            let x = wall_thickness + pos[0] * cell_size;
            let y = wall_thickness + pos[1] * cell_size;

            if walls[0] {
                svg.push_str(&format!(
                    r#"<rect x="{}" y="{}" width="{}" height="{}" fill="black"/>"#,
                    x,
                    y,
                    wall_thickness,
                    cell_size + wall_thickness
                ));
            }

            if walls[1] {
                svg.push_str(&format!(
                    r#"<rect x="{}" y="{}" width="{}" height="{}" fill="black"/>"#,
                    x - wall_thickness,
                    y + cell_size - wall_thickness,
                    cell_size + 2 * wall_thickness,
                    2 * wall_thickness
                ));
            }

            if walls[2] {
                svg.push_str(&format!(
                    r#"<rect x="{}" y="{}" width="{}" height="{}" fill="black"/>"#,
                    x + cell_size - wall_thickness,
                    y,
                    2 * wall_thickness,
                    cell_size + wall_thickness
                ));
            }
        }
    }

    svg.push_str("</svg>");

    return svg;
}
