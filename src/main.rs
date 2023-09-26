mod circle;
mod consts;
mod particle;
mod quadtree;
mod rectangle;
mod utils;

use consts::{HEIGHT, WIDTH};
use ggez::event::{self, EventHandler};
use ggez::graphics::{self, Color};
use ggez::{conf, Context, ContextBuilder, GameResult};
use nalgebra::Vector2;
use particle::Particle;
use quadtree::QuadTree;
use rectangle::Rectangle;
use utils::{calculate_new_position, create_galaxy, create_quadtree, detect_collisions};

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
        let qt = QuadTree::new(Rectangle::new(Vector2::new(0.0, 0.0), WIDTH, HEIGHT));
        let mut particles: Vec<Particle> = Vec::new();
        // ----- GALAXY COLLISION ------
        let sun_mass = 100000.0;
        let radius: f32 = 200.0;
        let center1 = Vector2::new(radius + 50.0, radius + 50.0);
        create_galaxy(
            &mut particles,
            center1,
            radius,
            sun_mass,
            0.01,
            300,
            &mut Color::from_rgb(250, 219, 132),
        );
        let center2 = Vector2::new(WIDTH - radius - 50.0, HEIGHT - radius - 50.0);
        create_galaxy(
            &mut particles,
            center2,
            radius,
            sun_mass,
            0.01,
            300,
            &mut Color::from_rgb(85, 208, 230),
        );
        let center3 = Vector2::new(WIDTH - radius - 50.0, radius + 50.0);
        create_galaxy(
            &mut particles,
            center3,
            radius,
            sun_mass,
            0.01,
            300,
            &mut Color::from_rgb(132, 250, 205),
        );
        let center4 = Vector2::new(radius + 50.0, HEIGHT - radius - 50.0);
        create_galaxy(
            &mut particles,
            center4,
            radius,
            sun_mass,
            0.01,
            300,
            &mut Color::from_rgb(240, 149, 226),
        );
        // create_galaxy(
        //     &mut particles,
        //     Vector2::new(WIDTH/2.0, HEIGHT/2.0),
        //     400.0,
        //     100000.0,
        //     0.01,
        //     1200,
        //     &mut Color::from_rgb(128, 43, 102),
        // );

        particles.sort_by_key(|item| item.mass as i32);

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
        let bg_color = Color::BLACK;
        let mut canvas = graphics::Canvas::from_frame(ctx, bg_color);
        self.qt.show(&mut canvas, ctx, false);
        canvas.finish(ctx)
    }
}
