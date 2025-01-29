use pyo3::prelude::*;
use pyo3::wrap_pyfunction;
use skia_safe::{Canvas, Paint, Color, Surface, EncodedImageFormat};
use std::fs::File;
use std::io::Write;

#[derive(Debug)]
struct Move {
    color: char,  // 'B' for black, 'W' for white
    x: usize,
    y: usize,
}

fn parse_coord(coord: &str) -> Option<(usize, usize)> {
    if coord.len() != 2 {
        return None;
    }
    let x = (coord.chars().nth(0)? as u8 - b'a') as usize;
    let y = (coord.chars().nth(1)? as u8 - b'a') as usize;
    Some((x, y))
}

fn parse_sgf(sgf: &str) -> Vec<Move> {
    let mut moves = Vec::new();
    let mut chars = sgf.chars().peekable();
    
    while let Some(c) = chars.next() {
        match c {
            'B' | 'W' if chars.peek() == Some(&'[') => {
                let color = c;
                chars.next(); // skip '['
                let mut coord = String::new();
                while let Some(c) = chars.next() {
                    if c == ']' {
                        break;
                    }
                    coord.push(c);
                }
                if let Some((x, y)) = parse_coord(&coord) {
                    moves.push(Move { color, x, y });
                }
            }
            _ => continue,
        }
    }
    moves
}

#[pyfunction]
fn render_sgf(sgf_content: &str, output_path: &str) -> PyResult<()> {
    let width = 800;
    let height = 800;
    let grid_size = 19;
    let cell_size = (width as f32 - 100.0) / (grid_size - 1) as f32;
    let offset = 50.0;

    // Create a new surface
    let mut surface = Surface::new_raster_n32_premul((width, height))
        .ok_or_else(|| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>("Failed to create surface"))?;

    let canvas = surface.canvas();

    // Clear the canvas with white color
    canvas.clear(Color::WHITE);

    // Draw the grid
    let mut paint = Paint::default();
    paint.set_color(Color::BLACK);
    paint.set_stroke_width(2.0);
    paint.set_anti_alias(true);

    // Draw horizontal lines
    for i in 0..grid_size {
        let y = offset + i as f32 * cell_size;
        canvas.draw_line((offset, y), (width as f32 - offset, y), &paint);
    }

    // Draw vertical lines
    for i in 0..grid_size {
        let x = offset + i as f32 * cell_size;
        canvas.draw_line((x, offset), (x, height as f32 - offset), &paint);
    }

    // Draw star points (hoshi)
    let star_points = [(3, 3), (3, 9), (3, 15),
                      (9, 3), (9, 9), (9, 15),
                      (15, 3), (15, 9), (15, 15)];
    
    paint.set_style(skia_safe::paint::Style::Fill);
    for &(x, y) in &star_points {
        let cx = offset + x as f32 * cell_size;
        let cy = offset + y as f32 * cell_size;
        canvas.draw_circle((cx, cy), 5.0, &paint);
    }

    // Parse and draw moves
    let moves = parse_sgf(sgf_content);
    for mv in moves {
        let cx = offset + mv.x as f32 * cell_size;
        let cy = offset + mv.y as f32 * cell_size;
        
        let mut stone_paint = Paint::default();
        stone_paint.set_anti_alias(true);
        
        match mv.color {
            'B' => {
                stone_paint.set_color(Color::BLACK);
            }
            'W' => {
                stone_paint.set_color(Color::WHITE);
                stone_paint.set_style(skia_safe::paint::Style::Fill);
                canvas.draw_circle((cx, cy), cell_size * 0.45, &stone_paint);
                
                // Add black outline for white stones
                stone_paint.set_color(Color::BLACK);
                stone_paint.set_style(skia_safe::paint::Style::Stroke);
                stone_paint.set_stroke_width(1.0);
            }
            _ => continue,
        }
        canvas.draw_circle((cx, cy), cell_size * 0.45, &stone_paint);
    }

    // Save the image to a PNG file
    let image = surface.image_snapshot();
    let png_data = image.encode_to_data(EncodedImageFormat::PNG)
        .ok_or_else(|| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>("Failed to encode image"))?;
    let mut file = File::create(output_path)?;
    file.write_all(png_data.as_bytes())?;

    Ok(())
}

#[pymodule]
fn rust_sgf_renderer(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(render_sgf, m)?)?;
    Ok(())
}
