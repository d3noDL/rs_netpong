mod scenes;
use scenes::{game, menu};

#[macroquad::main("netPONG!")]
async fn main() {
    menu::start().await;
}
