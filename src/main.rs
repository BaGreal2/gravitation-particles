mod circle;
mod consts;
mod particle;
mod quadtree;
mod rectangle;
mod utils;

use consts::{ HEIGHT, WIDTH};
use ggez::event::{self, EventHandler};
use ggez::graphics::{self, Color};
use ggez::{conf, Context, ContextBuilder, GameResult};
use nalgebra as na;
use particle::Particle;
use quadtree::QuadTree;
use rectangle::Rectangle;
use utils::{create_galaxy, detect_collisions, create_quadtree, calculate_new_position};

fn main() {
    let window_setup = conf::WindowSetup::default().title("Gravity Particles");
    let window_mode = conf::WindowMode::default()
        .dimensions(WIDTH, HEIGHT)
        .fullscreen_type(conf::FullscreenType::Windowed)
        .resizable(true);
    let (mut ctx, event_loop) = ContextBuilder::new("gravity", "xanin")
        .window_setup(window_setup)
        .window_mode(window_mode)
        .build()
        .expect("aieee, could not create ggez context!");

    let my_game = MyGame::new(&mut ctx);

    event::run(ctx, event_loop, my_game);
}

struct MyGame {
    qt: QuadTree,
    particles: Vec<Particle>,
}


impl MyGame {
    pub fn new(_ctx: &mut Context) -> MyGame {
        let qt = QuadTree::new(Rectangle::new(na::Vector2::new(0.0, 0.0), WIDTH, HEIGHT));
        let mut particles: Vec<Particle> = Vec::new();
        let sun_mass = 300000.0;
        let radius: f32 = 500.0;
        let center1 = na::Vector2::new(radius + 50.0, radius + 50.0);
        create_galaxy(
            &mut particles,
            center1,
            radius,
            sun_mass,
            0.01,
            700,
            Color::from_rgb(230, 191, 85),
        );
        let center2 = na::Vector2::new(WIDTH - radius - 50.0, HEIGHT - radius - 50.0);
        create_galaxy(
            &mut particles,
            center2,
            radius,
            sun_mass,
            0.01,
            700,
            Color::from_rgb(85, 208, 230),
        );

        particles.sort_by_key(|item| item.mass as i32);
        // let center3 = na::Vector2::new(WIDTH/2.0, HEIGHT/2.0);
        // create_galaxy(
        //     &mut particles,
        //     center3,
        //     radius,
        //     sun_mass,
        //     0.01,
        //     1000,
        //     Color::from_rgb(85, 208, 230),
        // );

        MyGame { qt, particles }
    }
}

impl EventHandler for MyGame {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        self.qt = create_quadtree(&self.particles);
        for i in 0..self.particles.len() {
            calculate_new_position(&mut self.particles[i], &mut self.qt);
        }
        detect_collisions(&mut self.particles, &self.qt);
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas = graphics::Canvas::from_frame(ctx, Color::BLACK);
        self.qt.show(&mut canvas, ctx, false);
        canvas.finish(ctx)
    }
}


