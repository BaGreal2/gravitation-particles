use crate::particle::Particle;
use crate::rectangle::Rectangle;
use ggez::{
    graphics::{Canvas, Color},
    Context,
};
use nalgebra::Vector2;
use rayon::prelude::*;

#[derive(Clone)]
pub struct QuadTree {
    bounds: Rectangle,
    children: Option<[Box<QuadTree>; 4]>,
    particle: Option<Particle>,
    mass: f32,
    m_center_pos: Vector2<f32>,
}

impl QuadTree {
    pub fn new(bounds: Rectangle) -> Self {
        let copy_bounds = bounds.clone();
        Self {
            bounds,
            children: None,
            particle: None,
            mass: 0.0,
            m_center_pos: Vector2::new(
                copy_bounds.top_left_pos.x + copy_bounds.w / 2.0,
                copy_bounds.top_left_pos.y + copy_bounds.h / 2.0,
            ),
        }
    }

    fn is_divided(&self) -> bool {
        !self.children.is_none()
    }

    fn subdivide(&mut self) {
        let (x, y) = (self.bounds.top_left_pos.x, self.bounds.top_left_pos.y);
        let (w, h) = (self.bounds.w, self.bounds.h);
        let topleft = Rectangle::new(Vector2::new(x, y), w / 2.0, h / 2.0);
        // children[0] = Box::new(QuadTree::new(topleft));
        let topright = Rectangle::new(Vector2::new(x + w / 2.0, y), w / 2.0, h / 2.0);
        // children[1] = Box::new(QuadTree::new(topright));
        let bottomleft = Rectangle::new(Vector2::new(x, y + h / 2.0), w / 2.0, h / 2.0);
        // children[2] = Box::new(QuadTree::new(bottomleft));
        let bottomright = Rectangle::new(Vector2::new(x + w / 2.0, y + h / 2.0), w / 2.0, h / 2.0);
        // children[3] = Box::new(QuadTree::new(bottomright));
        self.children = Some([
            Box::new(QuadTree::new(topleft)),
            Box::new(QuadTree::new(topright)),
            Box::new(QuadTree::new(bottomleft)),
            Box::new(QuadTree::new(bottomright)),
        ]);
    }

    pub fn insert(&mut self, particle: &Particle) {
        if !self.bounds.contains(particle) {
            return;
        }

        if self.particle.is_none() {
            self.particle = Some(*particle);
        } else {
            if !self.is_divided() {
                self.subdivide();
            }
            // self.children
            //     .as_mut()
            //     .unwrap()
            //     .par_iter_mut()
            //     .for_each(|leaf| {
            //         leaf.as_mut().insert(particle);
            //     });
            for leaf in self.children.as_mut().unwrap().as_mut() {
                leaf.as_mut().insert(particle);
            }
            self.update_mass();
        }
    }

    pub fn calculate_force(&mut self, particle: &mut Particle) {
        if !self.is_divided() {
            if let Some(existent_particle) = &self.particle {
                if existent_particle.pos != particle.pos {
                    let attraction_force =
                        particle.get_attraction_force(&self.particle.as_ref().unwrap());
                    particle.net_force += attraction_force;
                }
            }
            return;
        }

        let ratio = self.bounds.w / particle.get_distance_to(&self.m_center_pos);
        if ratio < 0.5 {
            let attraction_force = particle.get_attraction_force(&Particle::new(
                self.m_center_pos,
                Vector2::new(0.0, 0.0),
                self.mass,
                1.0,
                10000,
            ));
            particle.net_force += attraction_force;
            return;
        }

        for leaf in self.children.as_mut().unwrap() {
            leaf.calculate_force(particle);
        }
    }

    fn update_mass(&mut self) {
        if !self.is_divided() {
            if self.particle.is_none() {
                return;
            }
            self.mass = self.particle.as_ref().unwrap().mass;
            self.m_center_pos = self.particle.as_ref().unwrap().pos;
            return;
        }
        let mut mass_sum: f32 = 0.0;
        let mut center_x: f32 = 0.0;
        let mut center_y: f32 = 0.0;

        for leaf in self.children.as_mut().unwrap() {
            leaf.as_mut().update_mass();
            mass_sum += leaf.as_ref().mass;
            center_x += leaf.as_ref().m_center_pos.x * leaf.as_ref().mass;
            center_y += leaf.as_ref().m_center_pos.y * leaf.as_ref().mass;
        }
        self.mass = mass_sum;
        center_x /= mass_sum;
        center_y /= mass_sum;
        self.m_center_pos = Vector2::new(center_x, center_y);
    }

    pub fn show(
        &self,
        canvas: &mut Canvas,
        ctx: &mut Context,
        offset: Vector2<f32>,
        zoom: f32,
        particles_to_draw: &Vec<Particle>,
        max_vel: f32,
        min_vel: f32,
        show_bounds: bool,
    ) {
        if show_bounds {
            self.bounds.show(
                canvas,
                ctx,
                offset,
                zoom,
                &mut Color::from_rgb(255, 255, 255),
            );
        }
        match self.children.as_ref() {
            Some(children) => {
                for leaf in children.as_ref() {
                    leaf.show(
                        canvas,
                        ctx,
                        offset,
                        zoom,
                        particles_to_draw,
                        max_vel,
                        min_vel,
                        show_bounds,
                    );
                }
            }
            None => {}
        }

        match &self.particle {
            Some(existent_particle) => {
                if particles_to_draw.contains(&existent_particle) {
                    existent_particle.show(canvas, ctx, offset, zoom, max_vel, min_vel);
                }
            }
            None => {}
        }
    }

    pub fn query(&self, rect: &Rectangle) -> Vec<Particle> {
        let mut results = Vec::new();
        if !self.bounds.intersects(rect) {
            return results;
        }

        if let Some(particle) = &self.particle {
            if rect.contains(particle) {
                results.push(*particle);
            }
        }
        if self.is_divided() {
            for leaf in self.children.as_ref().unwrap().iter() {
                results.extend(leaf.query(rect));
            }
        }

        results
    }
}
