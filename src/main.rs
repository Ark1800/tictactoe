/*
By: Andrew C
Date: 2025-05-30
Program Details: <Program Description Here>
*/

mod modules;
use crate::modules::grid::draw_grid;
use crate::modules::label::Label;
use crate::modules::text_button::TextButton;
use macroquad::prelude::*;

/// Set up window settings before the app runs
fn window_conf() -> Conf {
    Conf {
        window_title: "tictactoe".to_string(),
        window_width: 1024,
        window_height: 1024,
        fullscreen: false,
        high_dpi: true,
        window_resizable: true,
        sample_count: 4, // MSAA: makes shapes look smoother
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let lbl_title = Label::new("Tic Tac Toe", 250.0, 75.0, 80);
    let btn_start = TextButton::new( 25.0, 150.0, 175.0, 100.0, "Start", BLUE, RED, 40);
     // Create a mutable vector of buttons
    let mut squares = vec![
        TextButton::new(225.0, 150.0, 175.0, 210.0, "", WHITE, GRAY, 30),
        TextButton::new(225.0, 381.0, 175.0, 210.0, "", WHITE, GRAY, 30),
        TextButton::new(225.0, 612.0, 175.0, 210.0, "", WHITE, GRAY, 30),
        TextButton::new(425.0, 150.0, 175.0, 210.0, "", WHITE, GRAY, 30),
        TextButton::new(425.0, 381.0, 175.0, 210.0, "", WHITE, GRAY, 30),
        TextButton::new(425.0, 612.0, 175.0, 210.0, "", WHITE, GRAY, 30),
        TextButton::new(625.0, 150.0, 175.0, 210.0, "", WHITE, GRAY, 30),
        TextButton::new(625.0, 381.0, 175.0, 210.0, "", WHITE, GRAY, 30),
        TextButton::new(625.0, 612.0, 175.0, 210.0, "", WHITE, GRAY, 30),
    ];
    loop {
        // background and assets
        clear_background(WHITE);
        draw_rectangle(0.0, 0.0, 1024.0, 120.0, BLUE);
        draw_grid(25.0, BLACK);
        // draw buttons and labels
        lbl_title.draw();
        // Draw each button in the squares vector
        for btn in squares.iter_mut() {
            if btn.click() {
                btn.normal_color = GRAY;
                btn.set_text("X");
            }
        }
        if btn_start.click() {
        }
        next_frame().await;
    }
}
