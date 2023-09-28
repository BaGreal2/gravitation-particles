use ggez::{
    graphics::{self, Canvas, Color},
    Context,
};
use nalgebra::Vector2;

use crate::{
    circle::Circle,
    particle::Particle,
    utils::{screen_to_world_coords, world_to_screen_coords},
};

#[derive(Clone)]
pub struct Rectangle {
    pub top_left_pos: Vector2<f32>,
    pub w: f32,
    pub h: f32,
}

impl Rectangle {
    pub fn new(top_left_pos: Vector2<f32>, w: f32, h: f32) -> Self {
        Self { top_left_pos, w, h }
    }

    pub fn contains(&self, particle: &Particle) -> bool {
        self.top_left_pos.x <= particle.pos.x
            && self.top_left_pos.y <= particle.pos.y
            && self.top_left_pos.x + self.w > particle.pos.x
            && self.top_left_pos.y + self.h > particle.pos.y
    }

    pub fn intersects_circle(&self, circle: &Circle) -> bool {
        let closest_x = (circle.center.x)
            .max(self.top_left_pos.x)
            .min(self.top_left_pos.x + self.w);
        let closest_y = (circle.center.y)
            .max(self.top_left_pos.y)
            .min(self.top_left_pos.y + self.h);

        let distance_x = circle.center.x - closest_x;
        let distance_y = circle.center.y - closest_y;

        let distance_squared = (distance_x * distance_x) + (distance_y * distance_y);
        distance_squared <= (circle.radius * circle.radius)
    }

    pub fn show(
        &self,
        canvas: &mut Canvas,
        ctx: &mut Context,
        offset: Vector2<f32>,
        zoom: f32,
        color: &mut Color,
    ) {
        color.a = 0.3;
        let rect = graphics::Rect {
            x: world_to_screen_coords(self.top_left_pos, offset, zoom).x,
            y: world_to_screen_coords(self.top_left_pos, offset, zoom).y,
            w: self.w * zoom,
            h: self.h * zoom,
        };
        let rect_mesh = graphics::Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::Stroke(graphics::StrokeOptions::DEFAULT),
            rect,
            *color,
        )
        .unwrap();

        canvas.draw(&rect_mesh, graphics::DrawParam::default());
    }
}
