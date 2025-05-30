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
    let btn_square1 = TextButton::new(225.0, 150.0, 175.0, 210.0, "", WHITE, GRAY, 30);
    let btn_square2 = TextButton::new(225.0, 381.0, 175.0, 210.0, "", WHITE, GRAY, 30);
    let btn_square3 = TextButton::new(225.0, 612.0, 175.0, 210.0, "", WHITE, GRAY, 30);
    let btn_square4 = TextButton::new(425.0, 150.0, 175.0, 210.0, "", WHITE, GRAY, 30);
    let btn_square5 = TextButton::new(425.0, 381.0, 175.0, 210.0, "", WHITE, GRAY, 30);
    let btn_square6 = TextButton::new(425.0, 612.0, 175.0, 210.0, "", WHITE, GRAY, 30);
    let btn_square7 = TextButton::new(625.0, 150.0, 175.0, 210.0, "", WHITE, GRAY, 30);
    let btn_square8 = TextButton::new(625.0, 381.0, 175.0, 210.0, "", WHITE, GRAY, 30);
    let btn_square9 = TextButton::new(625.0, 612.0, 175.0, 210.0, "", WHITE, GRAY, 30);
    loop {
        clear_background(WHITE);
        draw_rectangle(0.0, 0.0, 1024.0, 120.0, BLUE);
        draw_grid(25.0, BLACK);
        lbl_title.draw();
        if btn_square1.click() {}
        if btn_square2.click() {}
        if btn_square3.click() {}
        if btn_square4.click() {}
        if btn_square5.click() {}
        if btn_square6.click() {}
        if btn_square7.click() {}
        if btn_square8.click() {}
        if btn_square9.click() {}
        next_frame().await;
    }
}
