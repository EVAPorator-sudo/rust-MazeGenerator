use image::{ImageBuffer, Luma};

use crate::Maze::Grid::Grid;

pub enum img_format {
    svg,
    png,
    jpeg,
}

pub fn grid_draw_svg(grid: &Grid) -> String {
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

    svg
}

pub fn grid_draw_img(grid: &Grid) -> ImageBuffer<Luma<u8>, Vec<u8>> {
    let cell_size = 20;
    let wall_thickness = 1;
    let cols = grid.width as u32;
    let rows = grid.height as u32;

    let image_width = cols * cell_size + 2 * wall_thickness;
    let image_height = rows * cell_size + 2 * wall_thickness;

    let white = Luma([255]);
    let black = Luma([0]);

    let mut img = ImageBuffer::from_pixel(image_width, image_height, white);

    let draw_rect = |img: &mut ImageBuffer<Luma<u8>, Vec<u8>>, x: u32, y: u32, w: u32, h: u32| {
        for px in x..(x + w).min(image_width) {
            for py in y..(y + h).min(image_height) {
                img.put_pixel(px, py, black);
            }
        }
    };
    draw_rect(&mut img, 0, 0, image_width, wall_thickness);
    draw_rect(&mut img, 0, 0, wall_thickness, image_height);
    draw_rect(
        &mut img,
        0,
        image_height - wall_thickness,
        image_width,
        wall_thickness,
    );
    draw_rect(
        &mut img,
        image_width - wall_thickness,
        0,
        wall_thickness,
        image_height,
    );

    for row in grid.row_list.iter() {
        for cell in row.cell_list.iter() {
            let position = &cell.position;
            let walls = &cell.walls;

            let x = wall_thickness + position[0] as u32 * cell_size;
            let y = wall_thickness + position[1] as u32 * cell_size;

            if walls[0] {
                draw_rect(&mut img, x, y, wall_thickness, cell_size + wall_thickness)
            }
            if walls[1] {
                draw_rect(
                    &mut img,
                    x,
                    y + cell_size,
                    cell_size + wall_thickness,
                    wall_thickness,
                )
            }
            if walls[2] {
                draw_rect(
                    &mut img,
                    x + cell_size,
                    y,
                    wall_thickness,
                    cell_size + wall_thickness,
                )
            }
        }
    }

    img
}
