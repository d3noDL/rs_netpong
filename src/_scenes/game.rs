use crate::_scenes::menu;
use async_recursion::async_recursion;
use macroquad::prelude::*;

#[async_recursion]
pub async fn start() {
    update().await;
}

async fn update() {
    loop {
        clear_background(RED);

        if is_key_pressed(KeyCode::Escape) {
            menu::start().await;
        }
        next_frame().await;
    }
}