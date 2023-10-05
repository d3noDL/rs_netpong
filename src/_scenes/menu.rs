use crate::_scenes::*;
use crate::_network::*;


use macroquad::{prelude::*, ui::root_ui};
use async_recursion::async_recursion;

#[async_recursion]
pub async fn start() {
    update().await;
}

async fn update() {
    let mut ip = String::from("127.0.0.1:8888");

    loop {
        clear_background(GOLD);
        
        
        root_ui().input_text(0, "Ip and port", &mut ip);
        
        if root_ui().button(vec2(screen_width() / 2.0, screen_height() / 2.0 + 20.0), "Join") {
            client::join(&ip, "Dino");
            break;
        }
        if root_ui().button(vec2(screen_width() / 2.0, screen_height() / 2.0), "Host") {
            server::host(&ip, "Pepa");
            break;
        }
        
        next_frame().await;
    }

    game::start().await;
}


