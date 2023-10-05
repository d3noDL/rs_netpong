use crate::_scenes::*;
use crate::_network::*;

use std::thread;
use macroquad::{prelude::*, ui::root_ui};
use async_recursion::async_recursion;

#[async_recursion]
pub async fn start() {
    update().await;
}

async fn update() {
    let mut ip = String::from("127.0.0.1:8888");

    loop {
        clear_background(BLACK);
        
        root_ui().input_text(0, "Ip and port", &mut ip);
        
        if root_ui().button(vec2(screen_width() / 2.0, screen_height() / 2.0 + 20.0), "Join") {
            thread::spawn(move || {
                client::join(&ip.clone())
            });
            game::start().await;
            break;
        }
        if root_ui().button(vec2(screen_width() / 2.0, screen_height() / 2.0), "Host") {
            thread::spawn(move || {
                server::host(&ip);
            });
            game::start().await;
            break;
        }
        

        next_frame().await;
    }
}


