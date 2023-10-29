use crate::utils::world_to_screen_coords;
use ggez::{
    graphics::{self, Canvas, Color},
    mint::Point2,
    Context,
};
use nalgebra::Vector2;

use crate::consts::{G, SOFTENING};

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Particle {
    pub pos: Vector2<f32>,
    pub vel: Vector2<f32>,
    pub net_force: Vector2<f32>,
    pub mass: f32,
    pub radius: f32,
    pub index: usize,
}

impl Particle {
    pub fn new(pos: Vector2<f32>, vel: Vector2<f32>, mass: f32, radius: f32, index: usize) -> Self {
        Self {
            pos,
            vel,
            net_force: Vector2::new(0.0, 0.0),
            mass,
            radius,
            index,
        }
    }

    pub fn get_attraction_force(&mut self, another_particle: &Particle) -> Vector2<f32> {
        let r =
            (self.pos.metric_distance(&another_particle.pos).powi(2) + SOFTENING.powi(2)).sqrt();
        let dir = (another_particle.pos - self.pos).normalize();
        let magnitude = G * ((self.mass * another_particle.mass) / r.powi(2));
        let force = dir * magnitude;

        force
    }

    pub fn get_distance_to(&self, object: &Vector2<f32>) -> f32 {
        f32::hypot(object.x - self.pos.x, object.y - self.pos.y)
    }

    fn get_color(&self, value: f32, left: &Color, right: &Color) -> Color {
        Color::from_rgb(
            (((1.0 - value) * left.r + value * right.r) * 255.0) as u8,
            (((1.0 - value) * left.g + value * right.g) * 255.0) as u8,
            (((1.0 - value) * left.b + value * right.b) * 255.0) as u8,
        )
    }

    pub fn show(
        &self,
        canvas: &mut Canvas,
        ctx: &mut Context,
        offset: Vector2<f32>,
        zoom: f32,
        max_vel: f32,
        min_vel: f32,
    ) {
        let mut new_radius: f32;
        if self.radius < 1.0 {
            new_radius = 0.25 * zoom; // Adjust the radius of the dot
        } else {
            new_radius = self.radius * zoom; // Adjust the radius of the dot
        }
        if new_radius < 0.25 {
            new_radius = 0.25;
        }

        let mid_vel = (max_vel + min_vel) / 2.0;
        let left = Color::BLUE;
        let middle = Color::GREEN;
        let right = Color::RED;
        let norm_vel = self.vel.norm();
        let new_color: Color;

        if norm_vel < min_vel + mid_vel {
            new_color = self.get_color((norm_vel - min_vel) / mid_vel, &left, &right);
        } else {
            new_color = self.get_color((norm_vel - min_vel - mid_vel) / mid_vel, &middle, &right);
        }
        let dot_mesh = graphics::Mesh::new_circle(
            ctx,
            graphics::DrawMode::fill(),
            Point2 {
                x: world_to_screen_coords(self.pos, &offset, zoom).x,
                y: world_to_screen_coords(self.pos, &offset, zoom).y,
            },
            new_radius,
            0.1,
            new_color, // self.color.unwrap(),
        )
        .unwrap();

        canvas.draw(&dot_mesh, graphics::DrawParam::default());
    }
}
