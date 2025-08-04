use macroquad::{color, prelude::*};

// Declare the player module so Rust knows to load `player/player_class.rs`
mod ball;
mod player;
mod tiles;

use ball::ball_class::{Ball, BallLogic};
use player::player_class::{Player, PlayerLogic, SimpleVec2};
use tiles::tiles_class::{Tile, TileMap};

fn window_conf() -> Conf {
    Conf {
        window_title: "Spawn Rectangles".to_owned(),
        window_width: 1280,
        window_height: 720,
        window_resizable: false,
        ..Default::default()
    }
}

const SCREEN_HEIGHT: f32 = 1280.0;
const SCREEN_WIDTH: f32 = 720.0;

#[macroquad::main(window_conf)]
async fn main() {
    let movement_speed: f32 = 250.0;
    let jump_height: f32 = 8.0;
    let acceleration: f32 = 0.8;
    const GRAVITY: f32 = 0.8;

    let player_width: f32 = 80.0;
    let player_height: f32 = 20.0;

    let w = screen_width();
    let h = screen_height();

    let x = w / 2.0 - player_width / 2.0;
    let y = h - (player_height + 10.0);

    let simple_vec = SimpleVec2::new(x, y);
    let position_vector = Vec2::new(simple_vec.x, simple_vec.y);
    let is_grounded: bool = false;

    let mut player = Player::new(
        position_vector,
        movement_speed,
        jump_height,
        GRAVITY,
        acceleration,
        is_grounded,
        player_height,
        player_width,
    );

    // Create the ball near the player
    let mut ball = Ball::new(
        Vec2::new(
            player.position_vector.x + player_width / 2.0,
            player.position_vector.y + 10.0,
        ),
        15.0,                    // radius
        Vec2::new(200.0, 180.0), // velocity
        RED,                     // color
    );

    // Define a custom tile pattern with Option<Color> representing tiles or gaps (None)
    let creeper_pattern = vec![
        vec![
            None,
            Some(color::DARKGREEN),
            Some(color::DARKGREEN),
            None,
            None,
            Some(color::DARKGREEN),
            Some(color::DARKGREEN),
            None,
            None,
            Some(color::DARKGREEN),
            Some(color::DARKGREEN),
            None,
            None,
            Some(color::DARKGREEN),
            Some(color::DARKGREEN),
            None,
        ],
        vec![
            Some(color::DARKGREEN),
            Some(color::GREEN),
            Some(color::GREEN),
            Some(color::DARKGREEN),
            Some(color::DARKGREEN),
            Some(color::GREEN),
            Some(color::GREEN),
            Some(color::DARKGREEN),
            Some(color::DARKGREEN),
            Some(color::GREEN),
            Some(color::GREEN),
            Some(color::DARKGREEN),
            Some(color::DARKGREEN),
            Some(color::GREEN),
            Some(color::GREEN),
            Some(color::DARKGREEN),
        ],
        vec![
            Some(color::DARKGREEN),
            Some(BLACK),
            Some(BLACK),
            Some(color::DARKGREEN),
            Some(color::DARKGREEN),
            Some(BLACK),
            Some(BLACK),
            Some(color::DARKGREEN),
            Some(color::DARKGREEN),
            Some(BLACK),
            Some(BLACK),
            Some(color::DARKGREEN),
            Some(color::DARKGREEN),
            Some(BLACK),
            Some(BLACK),
            Some(color::DARKGREEN),
        ],
        vec![
            None,
            Some(color::DARKGREEN),
            Some(color::DARKGREEN),
            None,
            None,
            Some(color::DARKGREEN),
            Some(color::DARKGREEN),
            None,
            None,
            Some(color::DARKGREEN),
            Some(color::DARKGREEN),
            None,
            None,
            Some(color::DARKGREEN),
            Some(color::DARKGREEN),
            None,
        ],
        vec![
            None,
            Some(color::DARKGREEN),
            Some(color::DARKGREEN),
            None,
            None,
            Some(color::DARKGREEN),
            Some(color::DARKGREEN),
            None,
            None,
            Some(color::DARKGREEN),
            Some(color::DARKGREEN),
            None,
            None,
            Some(color::DARKGREEN),
            Some(color::DARKGREEN),
            None,
        ],
        vec![
            Some(color::DARKGREEN),
            Some(color::GREEN),
            Some(color::GREEN),
            Some(color::DARKGREEN),
            Some(color::DARKGREEN),
            Some(color::GREEN),
            Some(color::GREEN),
            Some(color::DARKGREEN),
            Some(color::DARKGREEN),
            Some(color::GREEN),
            Some(color::GREEN),
            Some(color::DARKGREEN),
            Some(color::DARKGREEN),
            Some(color::GREEN),
            Some(color::GREEN),
            Some(color::DARKGREEN),
        ],
        vec![
            Some(color::DARKGREEN),
            Some(BLACK),
            Some(BLACK),
            Some(color::DARKGREEN),
            Some(color::DARKGREEN),
            Some(BLACK),
            Some(BLACK),
            Some(color::DARKGREEN),
            Some(color::DARKGREEN),
            Some(BLACK),
            Some(BLACK),
            Some(color::DARKGREEN),
            Some(color::DARKGREEN),
            Some(BLACK),
            Some(BLACK),
            Some(color::DARKGREEN),
        ],
        vec![
            None,
            Some(color::DARKGREEN),
            Some(color::DARKGREEN),
            None,
            None,
            Some(color::DARKGREEN),
            Some(color::DARKGREEN),
            None,
            None,
            Some(color::DARKGREEN),
            Some(color::DARKGREEN),
            None,
            None,
            Some(color::DARKGREEN),
            Some(color::DARKGREEN),
            None,
        ],
        vec![
            None,
            Some(color::DARKGREEN),
            Some(color::DARKGREEN),
            None,
            None,
            Some(color::DARKGREEN),
            Some(color::DARKGREEN),
            None,
            None,
            Some(color::DARKGREEN),
            Some(color::DARKGREEN),
            None,
            None,
            Some(color::DARKGREEN),
            Some(color::DARKGREEN),
            None,
        ],
        vec![
            Some(color::DARKGREEN),
            Some(color::GREEN),
            Some(color::GREEN),
            Some(color::DARKGREEN),
            Some(color::DARKGREEN),
            Some(color::GREEN),
            Some(color::GREEN),
            Some(color::DARKGREEN),
            Some(color::DARKGREEN),
            Some(color::GREEN),
            Some(color::GREEN),
            Some(color::DARKGREEN),
            Some(color::DARKGREEN),
            Some(color::GREEN),
            Some(color::GREEN),
            Some(color::DARKGREEN),
        ],
        vec![
            Some(color::DARKGREEN),
            Some(BLACK),
            Some(BLACK),
            Some(color::DARKGREEN),
            Some(color::DARKGREEN),
            Some(BLACK),
            Some(BLACK),
            Some(color::DARKGREEN),
            Some(color::DARKGREEN),
            Some(BLACK),
            Some(BLACK),
            Some(color::DARKGREEN),
            Some(color::DARKGREEN),
            Some(BLACK),
            Some(BLACK),
            Some(color::DARKGREEN),
        ],
        vec![
            None,
            Some(color::DARKGREEN),
            Some(color::DARKGREEN),
            None,
            None,
            Some(color::DARKGREEN),
            Some(color::DARKGREEN),
            None,
            None,
            Some(color::DARKGREEN),
            Some(color::DARKGREEN),
            None,
            None,
            Some(color::DARKGREEN),
            Some(color::DARKGREEN),
            None,
        ],
        vec![
            None,
            Some(color::DARKGREEN),
            Some(color::DARKGREEN),
            None,
            None,
            Some(color::DARKGREEN),
            Some(color::DARKGREEN),
            None,
            None,
            Some(color::DARKGREEN),
            Some(color::DARKGREEN),
            None,
            None,
            Some(color::DARKGREEN),
            Some(color::DARKGREEN),
            None,
        ],
        vec![
            Some(color::DARKGREEN),
            Some(color::GREEN),
            Some(color::GREEN),
            Some(color::DARKGREEN),
            Some(color::DARKGREEN),
            Some(color::GREEN),
            Some(color::GREEN),
            Some(color::DARKGREEN),
            Some(color::DARKGREEN),
            Some(color::GREEN),
            Some(color::GREEN),
            Some(color::DARKGREEN),
            Some(color::DARKGREEN),
            Some(color::GREEN),
            Some(color::GREEN),
            Some(color::DARKGREEN),
        ],
        vec![
            Some(color::DARKGREEN),
            Some(BLACK),
            Some(BLACK),
            Some(color::DARKGREEN),
            Some(color::DARKGREEN),
            Some(BLACK),
            Some(BLACK),
            Some(color::DARKGREEN),
            Some(color::DARKGREEN),
            Some(BLACK),
            Some(BLACK),
            Some(color::DARKGREEN),
            Some(color::DARKGREEN),
            Some(BLACK),
            Some(BLACK),
            Some(color::DARKGREEN),
        ],
        vec![
            None,
            Some(color::DARKGREEN),
            Some(color::DARKGREEN),
            None,
            None,
            Some(color::DARKGREEN),
            Some(color::DARKGREEN),
            None,
            None,
            Some(color::DARKGREEN),
            Some(color::DARKGREEN),
            None,
            None,
            Some(color::DARKGREEN),
            Some(color::DARKGREEN),
            None,
        ],
    ];

    let tile_width = 80.0;
    let tile_height = 40.0;
    let gap = 5.0;

    // Create tilemap from pattern to cover just that pattern at top left (scaling pattern size)
    let mut tile_map = TileMap::from_pattern(&creeper_pattern, tile_width, tile_height, gap);

    loop {
        clear_background(WHITE);
        let dt = get_frame_time();

        let w = screen_width();

        // Handle ball collisions with tiles which may destroy tiles and bounce ball
        tile_map.handle_ball_collision(&mut ball);

        // Draw tiles first (background)
        tile_map.draw();

        // Control and draw player
        player.control_rectangle(dt, None, w - player_width);
        player.draw_player(BLUE);

        let player_rect: Rect = player.get_player_rect();

        // Update ball, check collision with player and draw
        ball.update(dt, w, screen_height());
        ball.check_collision_and_bounce(player_rect);
        ball.draw_ball();

        next_frame().await;
    }
}
