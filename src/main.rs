mod circle;
mod consts;
mod particle;
mod quadtree;
mod rectangle;
mod utils;

use consts::{HEIGHT, WIDTH, WORLD_HEIGHT, WORLD_WIDTH};
use ggez::event::{self, EventHandler};
use ggez::graphics::{self, Color};
use ggez::input::keyboard::{KeyCode, KeyInput};
use ggez::GameError;
use ggez::{conf, Context, ContextBuilder, GameResult};
use nalgebra::Vector2;
use particle::Particle;
use quadtree::QuadTree;
use rectangle::Rectangle;
use utils::{
    calculate_new_position, create_galaxy, create_quadtree, screen_to_world_coords,
     rename_images, convert_to_video, clean_cache_images, move_on_mouse, zoom_world, save_screen
};
use std::{fs, env};

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

    match ctx.fs.create_dir("/image-cache") {
        Ok(_) => println!("Created initial cache folder"),
        Err(creating_error) => eprintln!("Error creating folder: {:?}", creating_error),
    }
    let directory_name = "results";
    let current_dir = env::current_dir().expect("Failed to get current directory");
    let new_directory_path = current_dir.join(directory_name);
    match fs::create_dir(&new_directory_path) {
        Ok(_) => {
            println!("Created initial results folder");
        }
        Err(e) => {
            eprintln!("Error creating directory: {}", e);
        }
    }

    let my_game = MyGame::new(&mut ctx);

    event::run(ctx, event_loop, my_game);
}

struct MyGame {
    screen: graphics::ScreenImage,
    qt: QuadTree,
    particles: Vec<Particle>,
    keysdown: Vec<KeyCode>,
    origin: Vector2<f32>,
    zoom: f32,
    frame_count: u32,
    recording: bool,
}

impl MyGame {
    pub fn new(ctx: &mut Context) -> MyGame {
        let origin = Vector2::new(0.0, 0.0);
        let zoom = 0.5;
        let screen =
            graphics::ScreenImage::new(ctx, graphics::ImageFormat::Rgba8UnormSrgb, 1., 1., 1);
        let qt = QuadTree::new(Rectangle::new(
            Vector2::new(0.0, 0.0),
            WORLD_WIDTH,
            WORLD_HEIGHT,
        ));
        let mut particles: Vec<Particle> = Vec::new();
        create_galaxy(
            &mut particles,
            screen_to_world_coords(Vector2::new(100.0, 100.0), &origin, zoom),
            Vector2::new(0.0, 0.0),
            200.0,
            10000.0,
            0.001,
            1000,
            &mut Color::from_rgb(255, 0, 0),
        );
        create_galaxy(
            &mut particles,
            screen_to_world_coords(Vector2::new(WIDTH - 100.0, HEIGHT - 100.0), &origin, zoom),
            Vector2::new(0.0, 0.0),
            200.0,
            10000.0,
            0.001,
            1000,
            &mut Color::from_rgb(0, 255, 0),
        );

        particles.sort_by_key(|item| item.mass as i32);

        MyGame {
            screen,
            qt,
            particles,
            keysdown: Vec::new(),
            origin,
            zoom,
            frame_count: 0,
            recording: false,
        }
    }
}

impl EventHandler for MyGame {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        self.qt = create_quadtree(&self.particles);
        for i in 0..self.particles.len() {
            calculate_new_position(&mut self.particles[i], &mut self.qt);
        }
        move_on_mouse(ctx, &mut self.origin, self.zoom);
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let bg_color = Color::BLACK;
        let mut canvas = graphics::Canvas::from_screen_image(ctx, &mut self.screen, bg_color);
        self.qt
            .show(&mut canvas, ctx, self.origin, self.zoom, false);

        if self.recording {
            self.frame_count += 1;
            save_screen(ctx, &mut self.screen, self.frame_count);
        }
        canvas.finish(ctx)?;
        ctx.gfx.present(&self.screen.image(ctx))?;
        Ok(())
    }
    fn key_down_event(
        &mut self,
        ctx: &mut Context,
        keyinput: KeyInput,
        _repeat: bool,
    ) -> Result<(), GameError> {
        if let Some(keycode) = keyinput.keycode {
            self.keysdown.push(keycode);
            self.keysdown.dedup_by_key(|x| *x);

            if keycode == KeyCode::R {
                self.recording = true;
                println!("Recording!");
            }
            if keycode == KeyCode::S {
                self.recording = false;
                println!("Saving video to prject folder (results)...");
                rename_images(ctx);
                convert_to_video(ctx);
                clean_cache_images(ctx);
                println!("Saved!");
            }
        }
        Ok(())
    }

    fn key_up_event(&mut self, _ctx: &mut Context, keyinput: KeyInput) -> Result<(), GameError> {
        if let Some(keycode) = keyinput.keycode {
            self.keysdown.retain(|&x| x != keycode);
        }
        Ok(())
    }

    fn mouse_wheel_event(&mut self, ctx: &mut Context, _x: f32, y: f32) -> Result<(), GameError> {
        zoom_world(ctx, &mut self.origin, &mut self.zoom, y);

        Ok(())
    }
}
