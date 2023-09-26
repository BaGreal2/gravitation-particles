use crate::particle::Particle;
use nalgebra::Vector2;

pub struct Circle {
    pub center: Vector2<f32>,
    pub radius: f32,
}

impl Circle {
    pub fn new(center: Vector2<f32>, radius: f32) -> Self {
        Self { center, radius }
    }
    pub fn contains(&self, particle: &Particle) -> bool {
        let distance = ((particle.pos.x - self.center.x).powi(2)
            + (particle.pos.y - self.center.y).powi(2))
        .sqrt();
        distance <= self.radius
    }
}
