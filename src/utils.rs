use crate::consts::{G, HEIGHT, LOWER_BOUND, UPPER_BOUND, WIDTH, WORLD_HEIGHT, WORLD_WIDTH, MAX_ZOOM};
use crate::particle::Particle;
use crate::quadtree::QuadTree;
use crate::rectangle::Rectangle;
use chrono::{DateTime, Local};
use ggez::graphics::{Color, ScreenImage, ImageEncodingFormat};
use ggez::Context;
use nalgebra::Vector2;
use rand::Rng;
use std::fs;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;
use std::process::{Command, Stdio};

fn random_in_circle(radius: f32, center: Vector2<f32>) -> Vector2<f32> {
    let mut rng = rand::thread_rng();
    let angle = rng.gen_range(0.0..2.0 * std::f32::consts::PI);
    let distance = rng.gen_range(10.0..radius);

    Vector2::new(distance * angle.cos(), distance * angle.sin()) + center
}

pub fn create_galaxy(
    particles: &mut Vec<Particle>,
    center: Vector2<f32>,
    initial_vel: Vector2<f32>,
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
        let new_particle = Particle::new(
            pos,
            dir * orbital_vel,
            particle_mass,
            0.00001,
            Some(*particles_color),
            i as usize,
        );
        particles.push(new_particle);
    }

    let sun = Particle::new(
        center,
        initial_vel,
        sun_mass,
        1.5,
        Some(*particles_color),
        particles_amount as usize,
    );
    particles.push(sun);
}

pub fn create_quadtree(particles: &Vec<Particle>) -> QuadTree {
    let mut qt = QuadTree::new(Rectangle::new(
        Vector2::new(0.0, 0.0),
        WORLD_WIDTH,
        WORLD_HEIGHT,
    ));
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

pub fn world_to_screen_coords(
    world_coords: Vector2<f32>,
    origin: &Vector2<f32>,
    zoom: f32,
) -> Vector2<f32> {
    (origin + world_coords) * zoom
}
pub fn screen_to_world_coords(
    screen_coords: Vector2<f32>,
    origin: &Vector2<f32>,
    zoom: f32,
) -> Vector2<f32> {
    screen_coords / zoom - origin
}

pub fn rename_images(ctx: &Context) {
    let data_dir = ctx.fs.user_data_dir();
    for file in fs::read_dir(data_dir.join("image-cache/")).unwrap() {
        let full_path: PathBuf = file.as_ref().unwrap().path();
        let full_path_string: String = full_path.to_string_lossy().to_string();
        let full_name = String::from(
            file.as_ref()
                .unwrap()
                .path()
                .file_name()
                .unwrap()
                .to_string_lossy(),
        );
        let name = full_name[0..full_name.len() - 4].to_owned();
        if name.chars().nth(0).unwrap() == '.' {
            continue;
        }
        let prefix_amount = 6 - name.len();
        let repeated_string: String = std::iter::repeat("0").take(prefix_amount).collect();
        let path = &full_path_string[0..full_path_string.len() - full_name.len()];
        let old_path = String::from(path) + &full_name;
        let new_path = String::from(path) + &repeated_string + &full_name;
        let _ = fs::rename(old_path, new_path);
    }
}

pub fn convert_to_video(ctx: &Context) {
    let data_dir = ctx.fs.user_data_dir().to_string_lossy().to_string();
    let local: DateTime<Local> = Local::now();
    let formatted_date_time = local.format("%Y-%m-%d_%H.%M.%S").to_string();
    let mut cmd = Command::new("ffmpeg")
        .args([
            "-framerate",
            "25",
            "-pattern_type",
            "glob",
            "-i",
            format!("{data_dir}/image-cache/*.png").as_str(),
            "-vf",
            "eq=saturation=2",
            "-c:v",
            "libx264",
            "-pix_fmt",
            "yuv420p",
            format!("results/{formatted_date_time}.mp4").as_str(),
        ])
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();
    {
        let stdout = cmd.stdout.as_mut().unwrap();
        let stdout_reader = BufReader::new(stdout);
        let stdout_lines = stdout_reader.lines();

        for line in stdout_lines {
            println!("Read: {:?}", line);
        }
    }

    cmd.wait().unwrap();
}

pub fn clean_cache_images(ctx: &Context) {
    let data_dir = ctx.fs.user_data_dir();
    for file in fs::read_dir(data_dir.join("image-cache/")).unwrap() {
        let full_path: PathBuf = file.as_ref().unwrap().path();
        let _ = fs::remove_file(full_path);
    }
}

pub fn move_on_mouse(ctx: &mut Context, origin: &mut Vector2<f32>, zoom: f32) {
    const DESIRED_FPS: u32 = 60;

    while ctx.time.check_update_time(DESIRED_FPS) {
        let mouse_position = ctx.mouse.position();

        if mouse_position.x < LOWER_BOUND.x {
            origin.x += 5.0;
        } else if mouse_position.x > UPPER_BOUND.x {
            origin.x -= 5.0;
        }
        if mouse_position.y < LOWER_BOUND.y {
            origin.y += 5.0;
        } else if mouse_position.y > UPPER_BOUND.y {
            origin.y -= 5.0;
        }

        if -origin.x < 0.0 {
            origin.x = 0.0;
        } else if -origin.x + WIDTH / zoom > WORLD_WIDTH {
            origin.x = WIDTH / zoom - WORLD_WIDTH;
        }
        if -origin.y < 0.0 {
            origin.y = 0.0;
        } else if -origin.y + HEIGHT / zoom > WORLD_HEIGHT {
            origin.y = HEIGHT / zoom - WORLD_HEIGHT;
        }
    }
}

pub fn zoom_world(ctx: &Context, origin: &mut Vector2<f32>, zoom: &mut f32, y_diff: f32) {
    let mouse_x = ctx.mouse.position().x;
    let mouse_y = ctx.mouse.position().y;

    let mut mouse_world = screen_to_world_coords(Vector2::new(mouse_x, mouse_y), origin, *zoom);
    if y_diff > 0.0 {
        *zoom *= 1.1;
    } else if y_diff < 0.0 {
        *zoom /= 1.1;
    }
    if *zoom < MAX_ZOOM {
        *zoom = MAX_ZOOM;
    }
    mouse_world = world_to_screen_coords(mouse_world, origin, *zoom);

    origin.x += (mouse_x - mouse_world.x) / *zoom;
    origin.y += (mouse_y - mouse_world.y) / *zoom;
}

pub fn save_screen(ctx: &Context, screen: &mut ScreenImage, frame_count: u32) {
            let output_name = String::from("/image-cache/") + frame_count.to_string().as_str() + ".png";
            match screen
                .image(ctx)
                .encode(ctx, ImageEncodingFormat::Png, output_name) {
            Err(saving_err) => eprintln!("{}", saving_err),
            _ => {}
        }
}
