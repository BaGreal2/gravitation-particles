use ggez::{
    graphics::{self, Canvas, Color},
    Context,
};
use nalgebra::Vector2;

use crate::{particle::Particle, utils::world_to_screen_coords};

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

    pub fn intersects(&self, rect: &Self) -> bool {
        let up = rect.top_left_pos.y + rect.h < self.top_left_pos.y;
        let down = rect.top_left_pos.y > self.top_left_pos.y + self.h;
        let left = rect.top_left_pos.x + rect.w < self.top_left_pos.x;
        let right = rect.top_left_pos.x > self.top_left_pos.x + self.w;
        !(up || down || left || right)
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
            x: world_to_screen_coords(self.top_left_pos, &offset, zoom).x,
            y: world_to_screen_coords(self.top_left_pos, &offset, zoom).y,
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
