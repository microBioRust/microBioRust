//! # A Heatmap in Rust web assembly calling d3.js
//!
//! You will need to use wasm-pack to build instead of cargo 
//!  wasm-pack build --target web
//! And some way of serving locally 
//!  http-server .
//! It requires the index.html in the static directory
//! Currently working with fixed data
//! and a rusty colour theme

use wasm_bindgen::prelude::*;
use std::error::Error;
use serde_wasm_bindgen;
use serde::{Serialize, Deserialize};
use wasm_bindgen::JsValue;
use web_sys::{window, Document, HtmlCanvasElement, CanvasRenderingContext2d};
use web_sys::console;
use std::rc::Rc;
use std::cell::RefCell;
use wee_alloc;

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

pub fn draw_responsive_heatmap(
    context: &CanvasRenderingContext2d,
    values: Vec<Vec<i32>>,
    x_labels: Vec<String>,
    y_labels: Vec<String>,
    canvas_width: f64,
    canvas_height: f64,
    device_pixel_ratio: f64,
) {
    let rows = values.len();
    let cols = values[0].len();
    console::log_1(&JsValue::from_str(&format!("up in the draw function")));
    // Get canvas dimensions 
    // Calculate dynamic padding and box size
    let adj_canvas_width = canvas_width * device_pixel_ratio;
    let adj_canvas_height = canvas_height * device_pixel_ratio;
    let padding_left = adj_canvas_width * 0.05;
    let padding_top = adj_canvas_height * 0.05;
    let padding_bottom = adj_canvas_height * 0.05;
    let padding_right = adj_canvas_width * 0.05;

  //  let box_width = (adj_canvas_width - padding_left - padding_right) / (cols as f64 * 1.1);
  //  let box_height = (adj_canvas_height - padding_top - padding_bottom) / (rows as f64 * 1.1);
   
    let box_width = 30.0;
    let box_height = 30.0;
    // Clear the canvas
    console::log_1(&JsValue::from_str(&format!("pad left {} pad bottom {}",&padding_left, &padding_bottom)));
    context.clear_rect(0.0, 0.0, adj_canvas_width, adj_canvas_height);
    println!("cleared rec");
    // Draw the heatmap
    for row in 0..rows {
        for col in 0..cols {
            let value = values[row][col];

            // Set color based on value
            let color = match value {
                0 => "#fee0d2",
                1 => "#fc9272",
                2 => "#de2d26",
                _ => "#FFFFFF",
            };
            context.set_fill_style(&JsValue::from_str(color));

            let x = padding_left + (col as f64 * box_width);
            let y = padding_top + (row as f64 * box_height);
            context.fill_rect(x, y, box_width, box_height);

            // Draw box borders
            context.set_stroke_style(&JsValue::from_str("#FFFFFF"));
            context.set_line_width(2.0 / device_pixel_ratio);
            
            if row < rows - 1 {
                context.begin_path();
                context.move_to(x, y + box_height);
                context.line_to(x + box_width, y + box_height);
                context.stroke();
            }

            if col < cols - 1 {
                context.begin_path();
                context.move_to(x + box_width, y);
                context.line_to(x + box_width, y + box_height);
                context.stroke();
            }
        }
    }
    console::log_1(&JsValue::from_str(&format!(
    "after the rows and cols padding bottom: {}, height: {}",
         &padding_bottom,
         &(box_height * rows as f64),
            )));

    // Draw X-axis
    context.begin_path();
    context.set_stroke_style(&JsValue::from_str("#000000"));
    context.move_to(padding_left, (box_height * rows as f64) + padding_bottom);
    context.line_to((box_height * rows as f64) + padding_bottom, (box_height * rows as f64) + padding_left);
    context.stroke();
    
    // Draw Y-axis
    context.begin_path();
    context.move_to(padding_left, padding_top);
    context.line_to(padding_left, (box_height * rows as f64) + padding_bottom);
    context.stroke();

    // Draw X-axis ticks and labels
    let label_font_size = (box_height * 0.3).min(box_width * 0.3).max(12.0);
    context.set_font(&format!("{}px Arial", label_font_size));
    context.set_text_align("center");
    context.set_text_baseline("top");
    
    for col in 0..cols {
        let x = padding_left + col as f64 * box_width + box_width / 2.0;
        let y = (box_height * rows as f64) + padding_bottom + 5.0;  // Position below the heatmap
        context.fill_text(&x_labels[col], x, y).unwrap();

        // Draw ticks
        context.begin_path();
        context.move_to(x, (box_height * rows as f64) + padding_bottom);
        context.line_to(x, (box_height * rows as f64) + padding_bottom + 5.0);
        context.stroke();
    }

    // Draw Y-axis ticks and labels
    context.set_text_align("right");
    context.set_text_baseline("middle");
    
    for row in 0..rows {
        let x = padding_left - 10.0;  // Position to the left of the heatmap
        let y = padding_top + row as f64 * box_height + box_height / 2.0;
        context.fill_text(&y_labels[row], x, y).unwrap();

        // Draw ticks
        context.begin_path();
        context.move_to(padding_left, y);
        context.line_to(padding_left - 5.0, y);
        context.stroke();
    }
    console::log_1(&JsValue::from_str(&format!(
    "at the end of draw funct Canvas width: {}, height: {}",
        &adj_canvas_width,
        &adj_canvas_height
         )));
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct HeatmapData {
   values: Vec<Vec<i32>>,
   x_labels: Vec<String>,
   y_labels: Vec<String>,
}

impl HeatmapData {
    // Constructor method
    pub fn new() -> Self {
        HeatmapData {
            values: vec![vec![0]],
            x_labels: Vec::new(),
            y_labels: Vec::new(),
        }
    }
}


//returns a JsValue to javascript
#[wasm_bindgen(start)]
pub fn start() -> Result<(), JsValue> {
    // Get the window and document
    console::log_1(&JsValue::from_str(&format!("literal start")));
    let window = window().expect("should have a window in this context");
    let window = Rc::new(window);
    let window_clone = Rc::clone(&window);
    let document = window.document().expect("should have a document on window");
    console::log_1(&JsValue::from_str(&format!("up in the start of the function")));
    // Get the canvas element
    let canvas = document
        .get_element_by_id("heatmap")
        .expect("Canvas element not found")
        .dyn_into::<HtmlCanvasElement>()?;
    console::log_1(&JsValue::from_str(&format!("called the canvas")));
    let heatmap_values = vec![
        vec![2, 1, 0, 1, 0],  // row 1
        vec![1, 2, 0, 0, 1],  // row 2
        vec![2, 0, 1, 2, 1],  // row 3
        vec![0, 0, 0, 2, 0],  // row 4
        vec![1, 2, 0, 1, 1], // row 5
    ];
    console::log_1(&JsValue::from_str(&format!("called the heatmap vals")));
    let x_labels: Vec<String> = vec!["A", "B", "C", "D", "E"].iter().map(|s| s.to_string()).collect();
    let y_labels: Vec<String> = vec!["R1", "R2", "R3", "R4", "R5"].iter().map(|s| s.to_string()).collect();
    
    let num_rows = heatmap_values.len();      // Should be 5
    let num_cols = heatmap_values[0].len();   // Should be 5
    let mut heatmap_data = HeatmapData::new();
    heatmap_data = HeatmapData { values: heatmap_values.clone(), x_labels: x_labels.clone(), y_labels: y_labels.clone() };
    let box_size = 100.0;
    let device_pixel_ratio = window.device_pixel_ratio();
    console::log_1(&JsValue::from_str(&format!("num rows are {:?} num cols are {:?}", &num_rows, &num_cols)));
    
    // Dynamically set canvas size based on number of rows and columns
    let canvas_width = num_cols as f64 * box_size;  // 6 columns * 50px
    let canvas_height = num_rows as f64 * box_size; // 6 rows * 50px
    canvas.set_width(canvas_width as u32);
    canvas.set_height(canvas_height as u32);
    console::log_1(&JsValue::from_str(&format!(
        "Canvas width: {}, height: {}",
        canvas.width(),
        canvas.height()
         )));

    let context = canvas
        .get_context("2d")?
        .unwrap()
        .dyn_into::<CanvasRenderingContext2d>()?;

    // Define the heatmap matrix (3x3) with values representing different colors
    context.scale(device_pixel_ratio, device_pixel_ratio);
    
    draw_responsive_heatmap(
            &context,
            heatmap_values.clone(),
            x_labels.clone(),
            y_labels.clone(),
            canvas_width,
            canvas_height,
            device_pixel_ratio,
        );
   
    let closure = Closure::wrap(Box::new(move || {
        let new_width = window_clone.inner_width().unwrap().as_f64().unwrap();
        let new_height = window_clone.inner_height().unwrap().as_f64().unwrap();
    
        // Calculate canvas size based on window size
        let canvas_new_width = (num_cols as f64 * box_size).min(new_width);
        let canvas_new_height = (num_rows as f64 * box_size).min(new_height);
    
        canvas.set_width(canvas_new_width as u32);
        canvas.set_height(canvas_new_height as u32);
    
        // Reset the scaling
        context.set_transform(1.0, 0.0, 0.0, 1.0, 0.0, 0.0).unwrap();
        context.scale(device_pixel_ratio, device_pixel_ratio).unwrap();

        draw_responsive_heatmap(
           &context,
           heatmap_values.clone(),
           x_labels.clone(),
           y_labels.clone(),
           canvas_new_width.into(),
           canvas_new_height.into(),
           device_pixel_ratio,
        );
    }) as Box<dyn FnMut()>);
    
    window.add_event_listener_with_callback("resize", closure.as_ref().unchecked_ref()).unwrap();
    closure.forget();
    //let result = serde_wasm_bindgen::to_value(&heatmap_data).expect("issue with converting the heatmap data to js value");
    Ok(())
}
