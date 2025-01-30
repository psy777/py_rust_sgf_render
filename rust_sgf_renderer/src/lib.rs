use pyo3::prelude::*;
use pyo3::wrap_pyfunction;
use skia_safe::{Canvas, Paint, Color, Surface, EncodedImageFormat, Image, Typeface, Font, Data, Point};
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
fn render_sgf(sgf_content: &str, output_path: &str, theme: &str, board_width: usize, board_height: usize) -> PyResult<()> {
    let canvas_width = 800;
    let canvas_height = 800;
    
    // Calculate cell size based on the larger board dimension to maintain stone size consistency
    let cell_size = ((canvas_width.min(canvas_height) as f32) - 100.0) / (board_width.max(board_height) as f32 - 1.0);
    
    // Calculate offsets to center the board
    let offset_x = (canvas_width as f32 - cell_size * (board_width as f32 - 1.0)) / 2.0;
    let offset_y = (canvas_height as f32 - cell_size * (board_height as f32 - 1.0)) / 2.0;
    // Create a new surface
    let mut surface = Surface::new_raster_n32_premul((canvas_width, canvas_height))
        .ok_or_else(|| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>("Failed to create surface"))?;

    let canvas = surface.canvas();

    // Set the background based on the theme
    match theme {
        "dark" => {
            let data = Data::new_copy(include_bytes!("dark_board.png"));
            if let Some(img) = Image::from_encoded(data) {
                canvas.draw_image(&img, (0, 0), None);
            }
        }
        "light" => {
            let data = Data::new_copy(include_bytes!("light_board.png"));
            if let Some(img) = Image::from_encoded(data) {
                canvas.draw_image(&img, (0, 0), None);
            }
        }
        _ => {
            canvas.clear(Color::WHITE);
        }
    };

    // Draw the grid
    let mut paint = Paint::default();
    paint.set_color(Color::BLACK);
    paint.set_stroke_width(2.0);
    paint.set_anti_alias(true);

    // Draw horizontal lines
    for i in 0..board_height {
        let y = offset_y + i as f32 * cell_size;
        canvas.draw_line(
            (offset_x, y),
            (offset_x + (board_width as f32 - 1.0) * cell_size, y),
            &paint
        );
    }

    // Draw vertical lines
    for i in 0..board_width {
        let x = offset_x + i as f32 * cell_size;
        canvas.draw_line(
            (x, offset_y),
            (x, offset_y + (board_height as f32 - 1.0) * cell_size),
            &paint
        );
    }

    // Draw star points (hoshi) for standard 19x19 board
    if board_width == 19 && board_height == 19 {
        let star_points = [(3, 3), (3, 9), (3, 15),
                          (9, 3), (9, 9), (9, 15),
                          (15, 3), (15, 9), (15, 15)];
        
        paint.set_style(skia_safe::paint::Style::Fill);
        for &(x, y) in &star_points {
            let cx = offset_x + x as f32 * cell_size;
            let cy = offset_y + y as f32 * cell_size;
            canvas.draw_circle((cx, cy), 5.0, &paint);
        }
    }

    // Load the Oswald font for kifu rendering
    let font_data = Data::new_copy(include_bytes!("Oswald-VariableFont_wght.ttf"));
    let typeface = Typeface::from_data(font_data, 0).unwrap();
    let font = Font::new(typeface, cell_size * 0.45); // Increased font size

    // Parse and draw moves with kifu
    let moves = parse_sgf(sgf_content);
    for mv in &moves {
        let cx = offset_x + mv.x as f32 * cell_size;
        let cy = offset_y + mv.y as f32 * cell_size;
        let stone_size = cell_size * 0.5; // Same size for all themes
        
        if theme == "paper" {
            let mut stone_paint = Paint::default();
            stone_paint.set_anti_alias(true);
            
            match mv.color {
                'B' => {
                    stone_paint.set_color(Color::BLACK);
                    canvas.draw_circle((cx, cy), stone_size, &stone_paint);
                }
                'W' => {
                    stone_paint.set_color(Color::WHITE);
                    stone_paint.set_style(skia_safe::paint::Style::Fill);
                    canvas.draw_circle((cx, cy), stone_size, &stone_paint);
                    
                    // Add black outline for white stones
                    stone_paint.set_color(Color::BLACK);
                    stone_paint.set_style(skia_safe::paint::Style::Stroke);
                    stone_paint.set_stroke_width(1.0);
                    canvas.draw_circle((cx, cy), stone_size, &stone_paint);
                }
                _ => continue,
            }
        } else {
            // Use stone images for dark and light themes
            let stone_data = match mv.color {
                'B' => Data::new_copy(include_bytes!("black_glass_stone.png")),
                'W' => Data::new_copy(include_bytes!("white_glass_stone.png")),
                _ => continue,
            };
            
            if let Some(stone_img) = Image::from_encoded(stone_data) {
                let stone_rect = skia_safe::Rect::from_xywh(
                    cx - stone_size,
                    cy - stone_size,
                    stone_size * 2.0,
                    stone_size * 2.0,
                );
                canvas.draw_image_rect(
                    &stone_img,
                    None,
                    stone_rect,
                    &Paint::default(),
                );
            }
        }

        // Draw move number
        let move_number = moves.iter().position(|m| m.x == mv.x && m.y == mv.y).unwrap() + 1;
        let text = move_number.to_string();
        let mut text_paint = Paint::default();
        text_paint.set_color(if mv.color == 'B' { Color::WHITE } else { Color::BLACK });
        text_paint.set_anti_alias(true);
        
        // Center the text on the stone
        let text_blob = skia_safe::TextBlob::new(&text, &font).unwrap();
        let text_bounds = text_blob.bounds();
        let text_x = cx - text_bounds.width() / 2.0;
        let text_y = cy + text_bounds.height() / 3.0; // Adjusted for better vertical centering
        canvas.draw_text_blob(&text_blob, (text_x, text_y), &text_paint);
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
