use pyo3::prelude::*;
use pyo3::types::{PyModule, PyList};
use pyo3::wrap_pyfunction;
use skia_safe::{Paint, Color, surfaces, EncodedImageFormat, Image, Font, Data, FontMgr, Point};
use skia_safe::canvas::SrcRectConstraint;
use std::fs::File;
use std::io::Write;

// Import sgfmill module at initialization
static SGFMILL: pyo3::once_cell::GILOnceCell<Py<PyModule>> = pyo3::once_cell::GILOnceCell::new();

fn get_sgfmill(py: Python) -> PyResult<&PyModule> {
    SGFMILL
        .get_or_try_init(py, || {
            let sgfmill = py.import("sgfmill.sgf")?;
            Ok(sgfmill.into())
        })
        .map(|module| module.as_ref(py))
}

#[derive(Debug)]
struct Move {
    x: usize,
    y: usize,
    color: char,
    move_number: usize,
}

#[derive(Debug)]
struct BoardSize {
    width: usize,
    height: usize,
}

/// Parse board size from SGF content, handling both square and rectangular boards.
/// Returns a BoardSize struct with the parsed dimensions.
/// 
/// # Arguments
/// * `sgf_content` - The SGF content to parse
/// 
/// # Examples
/// ```
/// // Square board: SZ[19]
/// let size = parse_board_size("(;SZ[19])").unwrap();
/// assert_eq!(size.width, 19);
/// assert_eq!(size.height, 19);
/// 
/// // Rectangular board: SZ[15:10]
/// let size = parse_board_size("(;SZ[15:10])").unwrap();
/// assert_eq!(size.width, 15);
/// assert_eq!(size.height, 10);
/// ```
fn parse_board_size(sgf_content: &str) -> PyResult<BoardSize> {
    // Extract size value from SZ property
    let sz_value = if let Some(sz_start) = sgf_content.find("SZ[") {
        let sz_content = &sgf_content[sz_start + 3..];
        if let Some(sz_end) = sz_content.find(']') {
            sz_content[..sz_end].to_string()
        } else {
            return Ok(BoardSize { width: 19, height: 19 }); // Default if malformed
        }
    } else {
        return Ok(BoardSize { width: 19, height: 19 }); // Default if no SZ property
    };

    // Parse the size value
    if let Some(_) = sz_value.find(':') {
        // Rectangular board (e.g., "15:10")
        let parts: Vec<&str> = sz_value.split(':').collect();
        if parts.len() != 2 {
            return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(
                "Invalid rectangular board size format"
            ));
        }

        let (width, height) = match (parts[0].parse::<usize>(), parts[1].parse::<usize>()) {
            (Ok(w), Ok(h)) => (w, h),
            _ => return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(
                "Invalid board size format"
            )),
        };

        if width < 2 || width > 25 || height < 2 || height > 25 {
            return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(
                "Board dimensions must be between 2 and 25"
            ));
        }

        Ok(BoardSize { width, height })
    } else {
        // Square board (e.g., "19")
        match sz_value.parse::<usize>() {
            Ok(size) if size >= 2 && size <= 25 => {
                Ok(BoardSize { width: size, height: size })
            }
            Ok(_) => Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(
                "Board size must be between 2 and 25"
            )),
            Err(_) => Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(
                "Invalid board size"
            )),
        }
    }
}

fn get_board_position(py: Python, sgf_content: &str, move_number: Option<usize>) -> PyResult<(BoardSize, Vec<Move>)> {
    // Parse board size using our custom parser
    let board_size = parse_board_size(sgf_content)?;

    // Create a modified SGF with square board size for sgfmill
    let max_size = board_size.width.max(board_size.height);
    let modified_sgf = if let Some(sz_start) = sgf_content.find("SZ[") {
        if let Some(sz_end) = sgf_content[sz_start..].find(']') {
            let before = &sgf_content[..sz_start];
            let after = &sgf_content[sz_start + sz_end + 1..];
            format!("{}SZ[{}]{}", before, max_size, after)
        } else {
            sgf_content.to_string()
        }
    } else {
        sgf_content.to_string()
    };

    // Use sgfmill only for parsing moves, with a square board
    let sgfmill = get_sgfmill(py)?;
    let sgf_game = sgfmill.getattr("Sgf_game")?.call_method1("from_string", (modified_sgf,))?;
    
    // Get the main sequence of moves
    let main_sequence = sgf_game.getattr("get_main_sequence")?.call0()?.downcast::<PyList>()?;
    let total_moves = main_sequence.len();
    let moves_to_process = move_number.unwrap_or(total_moves);
    
    let mut moves = Vec::new();
    for i in 0..moves_to_process {
        if i >= total_moves {
            break;
        }
        
        let node = main_sequence.get_item(i)?;
        let get_move = node.getattr("get_move")?;
        if let Ok(color_move) = get_move.call0()?.extract::<(Option<&str>, Option<(usize, usize)>)>() {
            if let (Some(color), Some((x, y))) = color_move {
                // Convert coordinates safely
                let board_x = y;  // Swap x and y
                let board_y = x;
                moves.push(Move {
                    x: board_x,
                    y: board_y,
                    color: if color == "b" { 'B' } else { 'W' },
                    move_number: i,
                });
            }
        }
    }
    
    Ok((board_size, moves))
}

#[pyfunction]
#[pyo3(signature = (sgf_content, output_path, theme="dark", kifu=false, move_number=None))]
fn render_sgf(sgf_content: &str, output_path: &str, theme: &str, kifu: bool, move_number: Option<usize>) -> PyResult<()> {
    Python::with_gil(|py| {
        // Parse board position using sgfmill
        let (board_size, moves) = get_board_position(py, sgf_content, move_number)?;
        let board_width = board_size.width;
        let board_height = board_size.height;
        let canvas_width = 800;
        let canvas_height = 800;
        
        // Calculate cell size based on board dimensions and desired padding
        let max_board_dim = board_width.max(board_height) as f32 - 1.0;
        let cell_size = (canvas_width.min(canvas_height) as f32) / (max_board_dim + 3.0); // Add 3.0 for 1.5 cells padding on each side
        let margin_x = (canvas_width as f32 - (cell_size * (board_width as f32 - 1.0))) / 2.0;
        let margin_y = (canvas_height as f32 - (cell_size * (board_height as f32 - 1.0))) / 2.0;
        
        // Create surface
        let mut surface = surfaces::raster_n32_premul((canvas_width, canvas_height))
            .ok_or_else(|| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>("Failed to create surface"))?;

        let canvas = surface.canvas();

        // Set background based on theme
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

        // Draw grid
        let mut paint = Paint::default();
        paint.set_anti_alias(true);

        // Draw horizontal lines
        for i in 0..board_height {
            let y = margin_y + i as f32 * cell_size;
            canvas.draw_line(
                (margin_x, y),
                (margin_x + (board_width as f32 - 1.0) * cell_size, y),
                &paint
            );
        }

        // Draw vertical lines
        for i in 0..board_width {
            let x = margin_x + i as f32 * cell_size;
            canvas.draw_line(
                (x, margin_y),
                (x, margin_y + (board_height as f32 - 1.0) * cell_size),
                &paint
            );
        }

        // Draw star points (hoshi)
        if board_width == 19 && board_height == 19 {
            let star_points = [(3, 3), (3, 9), (3, 15),
                             (9, 3), (9, 9), (9, 15),
                             (15, 3), (15, 9), (15, 15)];
            
            paint.set_style(skia_safe::paint::Style::Fill);
            for &(x, y) in &star_points {
                let cx = margin_x + x as f32 * cell_size;
                let cy = margin_y + y as f32 * cell_size;
                canvas.draw_circle((cx, cy), 5.0, &paint);
            }
        }

        // Load font for kifu rendering
        let font_data = Data::new_copy(include_bytes!("Oswald-VariableFont_wght.ttf"));
        let font_mgr = FontMgr::new();
        let typeface = font_mgr.new_from_data(font_data.as_bytes(), 0).unwrap();
        let font = Font::new(typeface, cell_size * 0.6);

        // Draw stones and move numbers
        for mv in &moves {
            let cx = margin_x + mv.x as f32 * cell_size;
            let cy = margin_y + mv.y as f32 * cell_size;
            let stone_size = cell_size * 0.5;

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
                        
                        stone_paint.set_color(Color::BLACK);
                        stone_paint.set_style(skia_safe::paint::Style::Stroke);
                        stone_paint.set_stroke_width(1.0);
                        canvas.draw_circle((cx, cy), stone_size, &stone_paint);
                    }
                    _ => continue,
                }
            } else {
                let stone_data = match mv.color {
                    'B' => Data::new_copy(include_bytes!("black_glass_stone.png")),
                    'W' => Data::new_copy(include_bytes!("white_glass_stone.png")),
                    _ => continue,
                };
                
                if let Some(stone_img) = Image::from_encoded(stone_data) {
                    // Define source rect to flip the image 180 degrees
                    let src_rect = skia_safe::Rect::from_xywh(0.0, 0.0, stone_img.width() as f32, stone_img.height() as f32);
                    let dst_rect = skia_safe::Rect::from_xywh(
                        cx - stone_size,
                        cy - stone_size,
                        stone_size * 2.0,
                        stone_size * 2.0
                    );
                    
                    let mut paint = Paint::default();
                    paint.set_anti_alias(true);
                    
                    canvas.save();
                    canvas.rotate(180.0, Some(Point::new(cx, cy))); // Rotate 180 degrees around stone center
                    canvas.draw_image_rect(
                        &stone_img,
                        Some((&src_rect, SrcRectConstraint::Fast)),
                        dst_rect,
                        &paint,
                    );
                    canvas.restore();
                }
            }

            // Draw move numbers if kifu mode is enabled
            if kifu {
                let text = mv.move_number.to_string();
                
                let mut outline_paint = Paint::default();
                outline_paint.set_style(skia_safe::paint::Style::Stroke);
                outline_paint.set_stroke_width(3.0);
                outline_paint.set_anti_alias(true);
                outline_paint.set_color(if mv.color == 'B' { Color::BLACK } else { Color::WHITE });
                
                let mut fill_paint = Paint::default();
                fill_paint.set_style(skia_safe::paint::Style::Fill);
                fill_paint.set_anti_alias(true);
                fill_paint.set_color(if mv.color == 'B' { Color::WHITE } else { Color::BLACK });
                
                let text_blob = skia_safe::TextBlob::new(&text, &font).unwrap();
                let text_bounds = text_blob.bounds();
                let text_x = text_bounds.width() / 2.0;
                let text_y = text_bounds.height() / 4.0;
                
                canvas.draw_text_blob(&text_blob, (cx - text_x, cy + text_y), &outline_paint);
                canvas.draw_text_blob(&text_blob, (cx - text_x, cy + text_y), &fill_paint);
            }
        }

        // Save image
        let image_data = surface.image_snapshot()
            .encode_to_data(EncodedImageFormat::PNG)
            .ok_or_else(|| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>("Failed to encode image"))?;

        let mut file = File::create(output_path)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyIOError, _>(e.to_string()))?;
        
        file.write_all(image_data.as_bytes())
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyIOError, _>(e.to_string()))?;

        Ok(())
    })
}

#[pymodule]
fn rust_sgf_renderer(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(render_sgf, m)?)?;
    Ok(())
}
