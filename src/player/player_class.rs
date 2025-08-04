use macroquad::prelude::*;

// Your custom vector struct
#[derive(Debug, Copy, Clone)]
pub struct SimpleVec2 {
    pub x: f32,
    pub y: f32,
}

impl SimpleVec2 {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
}

#[repr(u8)]
pub enum PlayerState {
    Moving = 0,
    Jumping = 1,
    NotMoving = 2,
}

pub struct Player {
    pub position_vector: Vec2,
    pub movement_speed: f32,
    pub jump_height: f32,
    pub gravity: f32,
    pub acceleration: f32,
    pub is_grounded: bool
}

pub trait PlayerLogic {
    fn new(
        initial_position: Vec2,
        movement_speed: f32,
        jump_height: f32,
        gravity: f32,
        acceleration: f32,
        is_grounder: bool
    ) -> Self;

    // mutable self so position_vector can be updated
    fn control_rectangle(&mut self, dt: f32, speed: Option<&f32>);

    fn draw_player(&self, height: f32, width: f32, color: Color);

    fn apply_gravity(&mut self);
}

impl PlayerLogic for Player {
    fn new(
        initial_position: Vec2,
        movement_speed: f32,
        jump_height: f32,
        gravity: f32,
        acceleration: f32,
        is_grounded: bool
    ) -> Self {
        Player {
            position_vector: initial_position,
            movement_speed,
            jump_height,
            gravity,
            acceleration,
            is_grounded
        }
    }

    fn control_rectangle(&mut self, dt: f32, speed: Option<&f32>) {
        let actual_speed = *speed.unwrap_or(&self.movement_speed);

        if is_key_down(KeyCode::Right) {
            self.position_vector.x += actual_speed * dt;
        }
        if is_key_down(KeyCode::Left) {
            self.position_vector.x -= actual_speed * dt;
        }

        // if !self.is_grounded && is_key_down(KeyCode::Space) {
        //     self.position_vector.y += self.jump_height;
        // }
    }

    fn draw_player(&self, height: f32, width: f32, color: Color) {
        draw_rectangle(
            self.position_vector.x,
            self.position_vector.y,
            width,
            height,
            color,
        );
    }

    fn apply_gravity(&mut self) {
        self.position_vector.y += &self.gravity;
    }

}
