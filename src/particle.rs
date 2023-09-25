use ggez::{
    graphics::{self, Canvas, Color},
    mint::Point2,
    Context,
};
use nalgebra as na;

use crate::consts::G;

#[derive(Clone, PartialEq, Debug)]
pub struct Particle {
    pub pos: na::Vector2<f32>,
    pub vel: na::Vector2<f32>,
    pub net_force: na::Vector2<f32>,
    pub mass: f32,
    pub radius: f32,
    pub color: Option<Color>,
    pub index: usize,
}

impl Particle {
    pub fn new(
        pos: na::Vector2<f32>,
        vel: na::Vector2<f32>,
        mass: f32,
        radius: f32,
        color: Option<Color>,
        index: usize,
    ) -> Self {
        Self {
            pos,
            vel,
            net_force: na::Vector2::new(0.0, 0.0),
            mass,
            radius,
            color,
            index,
        }
    }

    pub fn get_attraction_force(&mut self, another_particle: &Particle) -> na::Vector2<f32> {
        let r = self.pos.metric_distance(&another_particle.pos);
        let dir = (another_particle.pos - self.pos).normalize();
        let magnitude = G * ((self.mass * another_particle.mass) / r.powi(2));
        let force = dir * magnitude;

        return force;
    }

    pub fn get_distance_to(&self, object: &na::Vector2<f32>) -> f32 {
        let x_dist = (object.x - self.pos.x).abs();
        let y_dist = (object.y - self.pos.y).abs();
        return (x_dist.powi(2) + y_dist.powi(2)).sqrt();
    }

    pub fn show(&self, canvas: &mut Canvas, ctx: &mut Context) {
        let dot_mesh = graphics::Mesh::new_circle(
            ctx,
            graphics::DrawMode::fill(),
            Point2 {
                x: self.pos.x,
                y: self.pos.y,
            },
            self.radius, // Adjust the radius of the dot
            0.1,         // Line width (not applicable for a filled circle)
            self.color.unwrap(),
        )
        .unwrap();

        canvas.draw(&dot_mesh, graphics::DrawParam::default());
    }
}
