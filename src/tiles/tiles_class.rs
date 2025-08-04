use macroquad::prelude::*;

use crate::ball::ball_class::Ball;

// Single tile definition
pub struct Tile {
    pub position: Vec2,
    pub size: Vec2,
    pub color: Color,
    pub is_solid: bool,
    pub destroyed: bool,
}

impl Tile {
    pub fn new(x: f32, y: f32, width: f32, height: f32, color: Color, is_solid: bool) -> Self {
        Tile {
            position: Vec2::new(x, y),
            size: Vec2::new(width, height),
            color,
            is_solid,
            destroyed: false,
        }
    }

    pub fn draw(&self) {
        if !self.destroyed {
            draw_rectangle(
                self.position.x,
                self.position.y,
                self.size.x,
                self.size.y,
                self.color,
            );
        }
    }

    pub fn get_rect(&self) -> Rect {
        Rect::new(self.position.x, self.position.y, self.size.x, self.size.y)
    }
}

// The complete tilemap (grid of tiles with gaps/space)
pub struct TileMap {
    pub tiles: Vec<Vec<Tile>>,
    pub rows: usize,
    pub cols: usize,
    pub tile_size: Vec2,
    pub gap: f32,
}

impl TileMap {
    /// Covers only top 2/3 of the screen, with rectangular tiles and gaps in grid
    pub fn new(
        screen_width: f32,
        screen_height: f32,
        tile_width: f32,
        tile_height: f32,
        gap: f32,
    ) -> Self {
        let covered_height = screen_height * 2.0 / 3.0;

        // Calculate as many full tiles as will fit with gaps
        let cols = (screen_width / (tile_width + gap)) as usize;
        let rows = (covered_height / (tile_height + gap)) as usize;

        let mut tiles = vec![];

        for row in 0..rows {
            let mut row_tiles = vec![];
            for col in 0..cols {
                let x = col as f32 * (tile_width + gap) + gap / 2.0;
                let y = row as f32 * (tile_height + gap) + gap / 2.0;

                // You can change the color or solid state per pattern here
                let color = YELLOW;
                let is_solid = true;

                let tile = Tile::new(x, y, tile_width, tile_height, color, is_solid);
                row_tiles.push(tile);
            }
            tiles.push(row_tiles);
        }

        TileMap {
            tiles,
            rows,
            cols,
            tile_size: Vec2::new(tile_width, tile_height),
            gap,
        }
    }

    pub fn draw(&self) {
        for row in &self.tiles {
            for tile in row {
                tile.draw();
            }
        }
    }

    // Destroy tile on collision with ball (call every frame in main loop)
    pub fn handle_ball_collision(&mut self, ball: &mut Ball) {
        for row in &mut self.tiles {
            for tile in row {
                if !tile.destroyed && tile.is_solid {
                    let tile_rect = tile.get_rect();
                    // Ball-rectangle collision (circle-to-rect)
                    let closest_x = ball
                        .position
                        .x
                        .clamp(tile_rect.x, tile_rect.x + tile_rect.w);
                    let closest_y = ball
                        .position
                        .y
                        .clamp(tile_rect.y, tile_rect.y + tile_rect.h);

                    let dist_x = ball.position.x - closest_x;
                    let dist_y = ball.position.y - closest_y;
                    let dist_sq = dist_x * dist_x + dist_y * dist_y;

                    if dist_sq < ball.radius * ball.radius {
                        // Mark the tile as destroyed
                        tile.destroyed = true;
                        // Bounce the ball (invert Y)
                        ball.velocity.y = -ball.velocity.y;

                        // Push ball out of the tile to avoid sticking
                        let dist = dist_sq.sqrt();
                        let penetration = ball.radius - dist;
                        if dist != 0.0 {
                            let normal = vec2(dist_x / dist, dist_y / dist);
                            ball.position += normal * penetration;
                        }
                        // Optionally: break on first collision per tick
                        break;
                    }
                }
            }
        }
    }

    pub fn from_pattern(
        pattern: &[Vec<Option<Color>>],
        tile_width: f32,
        tile_height: f32,
        gap: f32,
    ) -> Self {
        let rows = pattern.len();
        let cols = if rows > 0 { pattern[0].len() } else { 0 };

        let mut tiles = vec![];

        for (row_idx, row) in pattern.iter().enumerate() {
            let mut row_tiles = vec![];

            for (col_idx, &color_option) in row.iter().enumerate() {
                let x = col_idx as f32 * (tile_width + gap) + gap / 2.0;
                let y = row_idx as f32 * (tile_height + gap) + gap / 2.0;

                // Create tile only if pattern cell is Some(color)
                if let Some(color) = color_option {
                    let tile = Tile::new(x, y, tile_width, tile_height, color, true);
                    row_tiles.push(tile);
                } else {
                    // Optional: Push a non-solid/destroyed tile or skip
                    let tile = Tile::new(x, y, tile_width, tile_height, BLACK, false);
                    // Mark tile destroyed or invisible so it won't draw or collide
                    row_tiles.push(Tile {
                        destroyed: true,
                        ..tile
                    });
                }
            }
            tiles.push(row_tiles);
        }

        TileMap {
            tiles,
            rows,
            cols,
            tile_size: Vec2::new(tile_width, tile_height),
            gap,
        }
    }
}
