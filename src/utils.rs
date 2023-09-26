use crate::circle::Circle;
use crate::consts::{G, HEIGHT, WIDTH};
use crate::particle::Particle;
use crate::quadtree::QuadTree;
use crate::rectangle::Rectangle;
use ggez::graphics::Color;
use nalgebra::Vector2;
use rand::Rng;

fn random_in_circle(radius: f32, center: Vector2<f32>) -> Vector2<f32> {
    let mut rng = rand::thread_rng();
    let angle = rng.gen_range(0.0..2.0 * std::f32::consts::PI);
    let distance = rng.gen_range(0.0..radius);

    Vector2::new(distance * angle.cos(), distance * angle.sin()) + center
}

pub fn create_galaxy(
    particles: &mut Vec<Particle>,
    center: Vector2<f32>,
    radius: f32,
    sun_mass: f32,
    particle_mass: f32,
    particles_amount: i32,
    particles_color: &mut Color,
) {
    for i in 0..particles_amount {
        let pos = random_in_circle(radius, center);
        let distance_to_center = pos.metric_distance(&center);
        let orbital_vel = ((G * sun_mass) / distance_to_center).sqrt();
        let dir = Vector2::new(pos.y - center.y, center.x - pos.x).normalize();
        particles_color.a = 1.0 - distance_to_center / radius + 0.5;
        let new_particle = Particle::new(
            pos,
            dir * orbital_vel,
            particle_mass,
            1.0,
            Some(*particles_color),
            i as usize,
        );
        particles.push(new_particle);
    }

    let sun = Particle::new(
        center,
        Vector2::new(0.0, 0.0),
        sun_mass,
        1.5,
        Some(Color::RED),
        particles_amount as usize,
    );
    particles.push(sun);
}

pub fn detect_collisions(particles: &mut Vec<Particle>, qt: &QuadTree) {
    let len = particles.len();
    if len == 1 {
        return;
    }
    for i in 0..len {
        let collision_particles = qt.query(&Circle::new(
            Vector2::new(particles[i].pos.x, particles[i].pos.y),
            particles[i].radius * 2.0,
        ));

        for collision_particle in collision_particles.clone() {
            if collision_particle.pos == particles[i].pos {
                continue;
            }
            let dx = collision_particle.pos.x - particles[i].pos.x;
            let dy = collision_particle.pos.y - particles[i].pos.y;
            let distance = (dx * dx + dy * dy).sqrt();

            if distance
                < particles[i].radius
                    + collision_particle.radius
                    + particles[i].vel.magnitude()
                    + collision_particle.vel.magnitude()
            {
                let new_vel = (particles[i].vel * particles[i].mass
                    + collision_particle.vel * collision_particle.mass)
                    / (particles[i].mass + collision_particle.mass);
                particles[i].vel = new_vel.clone();
                // let overlap = particles[i].radius + collision_particle.radius - distance;
                // let dir = Vector2::new(
                //     collision_particle.pos.x - particles[i].pos.x,
                //     collision_particle.pos.y - particles[i].pos.y,
                // )
                // .normalize();
                // let corrrection = dir * overlap / 2.0;
                // particles[i].pos -= corrrection;
                // particles[collision_particle.index].pos -= corrrection;
            }
        }
    }
}

pub fn create_quadtree(particles: &Vec<Particle>) -> QuadTree {
    let mut qt = QuadTree::new(Rectangle::new(Vector2::new(0.0, 0.0), WIDTH, HEIGHT));
    for i in 0..particles.len() {
        qt.insert(&particles[i]);
    }
    qt
}

pub fn calculate_new_position(particle: &mut Particle, qt: &mut QuadTree) {
    particle.net_force = Vector2::new(0.0, 0.0);
    qt.calculate_force(particle);

    let acceleration = particle.net_force / particle.mass;
    particle.vel += acceleration;
    let velocity = particle.vel;
    particle.pos += velocity;
}
