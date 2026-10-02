/*
By: <aum markandey>
Date: 2026-10-02
Program Details: basic calculator finished
*/

mod ui;
mod utils;
use std::result;

use crate::ui::text_button::TextButton;
use macroquad::prelude::*;
use crate::ui::text_input::TextInput;
 use crate::ui::label::Label;
 use crate::ui::still_image::StillImage;use crate::utils::preload_image::TextureManager;
  use crate::utils::preload_image::LoadingScreenOptions; // If you want to customize the loading screen
/// Set up window settings before the app runs
fn window_conf() -> Conf {
    Conf {
        window_title: "basic_calculator_finished".to_string(),
        window_width: 1700,
        window_height: 800,
        fullscreen: false,
        high_dpi: true,
        window_resizable: true,
        sample_count: 4, // MSAA: makes shapes look smoother
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut btn_multiples = TextButton::new(50.0, 350.0, 200.0, 60.0, "multiply", WHITE, RED, 30);
    let mut btn_add = TextButton::new(50.0, 450.0, 200.0, 60.0, "add", WHITE, RED, 30);
    let mut btn_subtract = TextButton::new(50.0, 550.0, 200.0, 60.0, "subtract", WHITE, RED, 30);
    let mut btn_divide = TextButton::new(50.0, 650.0, 200.0, 60.0, "divide", WHITE, RED, 30);
    let mut btn_exit = TextButton::new(1450.0, 650.0, 200.0, 60.0, "Exit", WHITE, RED, 30);
    let texture_manager = TextureManager::new();
    texture_manager.preload_with_loading_screen(&["assets/edward.png"], None, None).await;
    let img_edward = StillImage::from_preload(
        texture_manager.get_preload("assets/edward.png").unwrap(),
        1700.0,
        768.0,
        0.0,
        0.0,
        true,
        1.0,
    );
    let mut lbl_text = Label::new(
        "input first number and second number and click on the operation ",
        10.0,
        100.0,
        30,
    );
     let mut input_firstnum = TextInput::new(500.0, 300.0, 150.0, 40.0, 25.0);
    let mut input_secondnum = TextInput::new(900.0, 300.0, 150.0, 40.0, 25.0);
    //let mut num1: f64 = 0.0;
    //let mut num2: f64 = 0.0;
 let mut total: f64 = 0.0;
 let mut edward: bool = false;
    btn_exit.with_text_color(BLACK); // Sets the normal text color
    btn_exit.with_hover_text_color(WHITE);
    btn_add.with_text_color(BLACK); // Sets the normal text color
    btn_add.with_hover_text_color(WHITE);
    btn_subtract.with_text_color(BLACK); // Sets the normal text color
    btn_subtract.with_hover_text_color(WHITE);
    btn_divide.with_text_color(BLACK); // Sets the normal text color
    btn_divide.with_hover_text_color(WHITE);
    btn_multiples.with_text_color(BLACK); // Sets the normal text color
    btn_multiples.with_hover_text_color(WHITE);
    
    let mut btn_exit = TextButton::new(1450.0, 650.0, 200.0, 60.0, "Exit", WHITE, RED, 30);
     btn_exit.with_text_color(BLACK); // Sets the normal text color
    btn_exit.with_hover_text_color(WHITE);
    fn operations(n: i32, value1: f64, value2: f64) -> f64 {
    if n == 1 {
        return value1 + value2;
    } else if n == 2 {
        return value1 - value2;
    } else if n == 3 {
        return value1 * value2;
    } else if n == 4 {
        return value1 / value2;
    } else {
        return 0.0; // Default case, should not happen
        
    }
}
    loop {
        clear_background(WHITE);
        if btn_exit.click() {
            break;
        }
        if btn_subtract.click() {
            let firstnum_text = input_firstnum.get_text();
            let secondnum_text = input_secondnum.get_text();
            if let Ok(parsed_value) = firstnum_text.trim().parse::<f64>() && let Ok(parsed_value2) = secondnum_text.trim().parse::<f64>() {
                 if parsed_value == 2010.0 {
                    edward = true;
                }
              
              let result = operations(2, parsed_value, parsed_value2);
                lbl_text.set_text(&format!("Result: {}", result));
            } else {
                lbl_text.set_text("Please enter valid numbers.");
            }
            
        }
        if btn_multiples.click() {
           let firstnum_text = input_firstnum.get_text();
            let secondnum_text = input_secondnum.get_text();
            if let Ok(parsed_value) = firstnum_text.trim().parse::<f64>() && let Ok(parsed_value2) = secondnum_text.trim().parse::<f64>() {
                 if parsed_value == 2010.0 {
                    edward = true;
                }
                let result = operations(3, parsed_value, parsed_value2);
                lbl_text.set_text(&format!("Result: {}", result));
                
            } else {
                lbl_text.set_text("Please enter valid numbers.");
            }
        }
        if btn_divide.click() {
            let firstnum_text = input_firstnum.get_text();
            let secondnum_text = input_secondnum.get_text();
            if let Ok(parsed_value) = firstnum_text.trim().parse::<f64>() && let Ok(parsed_value2) = secondnum_text.trim().parse::<f64>() {
                 if parsed_value == 2010.0 {
                    edward = true;
                }
                if parsed_value2 != 0.0 {
                    let result = operations(4, parsed_value, parsed_value2);
                    lbl_text.set_text(&format!("Result: {}", result));
                } else {
                    lbl_text.set_text("Error: Division by zero is not allowed.");
                }
            } else {
                lbl_text.set_text("Please enter valid numbers.");
            }
        }
        if btn_add.click() {
          let firstnum_text = input_firstnum.get_text();
            let secondnum_text = input_secondnum.get_text();
            if let Ok(parsed_value) = firstnum_text.trim().parse::<f64>() && let Ok(parsed_value2) = secondnum_text.trim().parse::<f64>() {
                if parsed_value == 2010.0 {
                    edward = true;
                }
                let result = operations(1, parsed_value, parsed_value2);
                lbl_text.set_text(&format!("Result: {}", result));
            } else {
                lbl_text.set_text("Please enter valid numbers.");
            }
        }
        if edward == true {
            img_edward.draw();
            lbl_text.set_text("i dont know him");
            lbl_text.with_colors(WHITE, Some(DARKGRAY));
        } 
        lbl_text.draw();
        input_firstnum.draw();
        input_secondnum.draw();
        next_frame().await;
    }
}
