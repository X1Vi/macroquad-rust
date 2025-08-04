use macroquad::prelude::*;

// Declare the player module so Rust knows to load `player/player_class.rs`
mod player;
use player::player_class::{Player, PlayerLogic, SimpleVec2};

fn window_conf() -> Conf {
    Conf {
        window_title: "Spawn Rectangles".to_owned(),
        window_width: 1280,
        window_height: 720,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let movement_speed: f32 = 140.0;
    let jump_height: f32 = 8.0;
    let acceleration: f32 = 0.8;
    const GRAVITY: f32 = 0.8;

    let length: f32 = 40.0;
    let x = 1280.0 / 2.0 - length / 2.0;
    let y = 720.0 / 2.0 - length / 2.0;

    let simple_vec = SimpleVec2::new(x, y);
    let position_vector = Vec2::new(simple_vec.x, simple_vec.y);
    let is_grounded: bool = false;
    // Player must be mutable to update position
    let mut player = Player::new(
        position_vector,
        movement_speed,
        jump_height,
        GRAVITY,
        acceleration,
        is_grounded
    );

    loop {
        clear_background(WHITE);
        let dt = get_frame_time();

        // Move and update player position
        player.control_rectangle(dt, None);

        // Draw player at updated position
        player.draw_player(length, length, BLUE);

        player.apply_gravity();

        next_frame().await;
    }
}
