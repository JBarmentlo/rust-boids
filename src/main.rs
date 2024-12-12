
use macroquad::prelude::*;

mod vec2d;
mod boids;

use boids::Boid;
use boids::Flock;

use std::thread;
use std::time::Duration;
use std::time::Instant;


#[macroquad::main("Boids")]
async fn main() {
    let w = screen_width() as usize;
    let h = screen_height() as usize;

    println!("H {}, W {}", h, w);

    // let w: usize = 800;
    // let h: usize = 600;

    let mut flock = Flock::random(5);
    // let mut buffer = vec![CellState::Dead; w * h];

    let mut image = Image::gen_image_color(w as u16, h as u16, WHITE);


    let texture = Texture2D::from_image(&image);

    let w = image.width() as f32;
    let h = image.height() as f32;

    println!("h = {}, w = {}", h, w);

    loop {
        println!("Loope");
        clear_background(WHITE);
        image = Image::gen_image_color(w as u16, h as u16, WHITE);

        let w = image.width() as f32;
        let h = image.height() as f32;

        let draw_start = Instant::now();
        for boid in flock.boids.iter() {
            // boid.position.x = boid.position.x + 10.;
            let x = (boid.position.x % w) as u32;
            let y = (boid.position.y % h) as u32;
    
            image.set_pixel(
                x,
                y,
                BLACK,
            );
            // println!("{}", boid);
        }
    

        texture.update(&image);

        draw_texture(&texture, 0., 0., WHITE);
        let draw_duration = draw_start.elapsed();

        let flock_start = Instant::now();
        flock = flock.next_step();
        let flock_duration = flock_start.elapsed();
        println!("Boid Compute time: {:?}.\nDraw time: {:?}", flock_duration, draw_duration);
        // thread::sleep(Duration::from_millis(100));
        next_frame().await
    }
}
