use macroquad::prelude::*;
use macroquad::ui::*;



pub async fn start() {
    update().await;
}

pub async fn update() {
    loop {
        clear_background(BLACK);

        next_frame().await
    }
}

