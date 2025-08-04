use macroquad::prelude::*;

pub struct Ball {
    pub position: Vec2,
    pub radius: f32,
    pub velocity: Vec2,
    pub color: Color,
}

pub trait BallLogic {
    fn new(initial_position: Vec2, radius: f32, velocity: Vec2, color: Color) -> Self;

    fn update(&mut self, dt: f32, screen_width: f32, screen_height: f32);

    fn draw_ball(&self);

    fn check_collision_and_bounce(&mut self, player_rect: Rect);
}

impl BallLogic for Ball {
    fn new(initial_position: Vec2, radius: f32, velocity: Vec2, color: Color) -> Self {
        Ball {
            position: initial_position,
            radius,
            velocity,
            color,
        }
    }

    fn update(&mut self, dt: f32, screen_width: f32, screen_height: f32) {
        self.position += self.velocity * dt;

        // Bounce off screen edges
        if self.position.x - self.radius <= 0.0 {
            self.position.x = self.radius;
            self.velocity.x = -self.velocity.x;
        } else if self.position.x + self.radius >= screen_width {
            self.position.x = screen_width - self.radius;
            self.velocity.x = -self.velocity.x;
        }

        if self.position.y - self.radius <= 0.0 {
            self.position.y = self.radius;
            self.velocity.y = -self.velocity.y;
        } else if self.position.y + self.radius >= screen_height {
            self.position.y = screen_height - self.radius;
            self.velocity.y = -self.velocity.y;
        }
    }

    fn draw_ball(&self) {
        draw_circle(self.position.x, self.position.y, self.radius, self.color);
    }

    fn check_collision_and_bounce(&mut self, player_rect: Rect) {
        // Find closest point in player rect to ball center
        let closest_x = self.position.x.clamp(player_rect.x, player_rect.x + player_rect.w);
        let closest_y = self.position.y.clamp(player_rect.y, player_rect.y + player_rect.h);

        let distance_x = self.position.x - closest_x;
        let distance_y = self.position.y - closest_y;

        let distance_squared = distance_x * distance_x + distance_y * distance_y;

        if distance_squared < self.radius * self.radius {
            // Basic response: invert velocity depending on collision normal

            // Compute penetration depth
            let distance = distance_squared.sqrt();
            let penetration_depth = self.radius - distance;

            // Normalize collision vector (handle zero distance)
            let normal = if distance == 0.0 {
                Vec2::new(0.0, -1.0)
            } else {
                Vec2::new(distance_x / distance, distance_y / distance)
            };

            // Push ball out of collision
            self.position += normal * penetration_depth;

            // Reflect velocity vector on the collision normal
            let velocity_dot_normal = self.velocity.dot(normal);
            self.velocity -= 2.0 * velocity_dot_normal * normal;
        }
    }
}
