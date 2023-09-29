use crate::utils::world_to_screen_coords;
use ggez::{
    graphics::{self, Canvas, Color},
    mint::Point2,
    Context,
};
use nalgebra::Vector2;

use crate::consts::G;

#[derive(Clone, PartialEq, Debug)]
pub struct Particle {
    pub pos: Vector2<f32>,
    pub vel: Vector2<f32>,
    pub net_force: Vector2<f32>,
    pub mass: f32,
    pub radius: f32,
    pub color: Option<Color>,
    pub index: usize,
}

impl Particle {
    pub fn new(
        pos: Vector2<f32>,
        vel: Vector2<f32>,
        mass: f32,
        radius: f32,
        color: Option<Color>,
        index: usize,
    ) -> Self {
        Self {
            pos,
            vel,
            net_force: Vector2::new(0.0, 0.0),
            mass,
            radius,
            color,
            index,
        }
    }

    pub fn get_attraction_force(&mut self, another_particle: &Particle) -> Vector2<f32> {
        let r = self.pos.metric_distance(&another_particle.pos);
        let dir = (another_particle.pos - self.pos).normalize();
        let magnitude = G * ((self.mass * another_particle.mass) / r.powi(2));
        let force = dir * magnitude;

        force
    }

    pub fn get_distance_to(&self, object: &Vector2<f32>) -> f32 {
        f32::hypot(object.x - self.pos.x, object.y - self.pos.y)
    }

    pub fn show(&self, canvas: &mut Canvas, ctx: &mut Context, offset: Vector2<f32>, zoom: f32) {
        let mut new_radius: f32;
        if self.radius < 1.0 {
            new_radius = 0.25 * zoom; // Adjust the radius of the dot
        } else {
            new_radius = self.radius * zoom; // Adjust the radius of the dot
        }
        if new_radius < 0.25 {
            new_radius = 0.25;
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
            self.color.unwrap(),
        )
        .unwrap();

        canvas.draw(&dot_mesh, graphics::DrawParam::default());
    }
}
