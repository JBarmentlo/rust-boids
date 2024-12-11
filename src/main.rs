
use macroquad::prelude::*;

mod vec2d;
mod boids;

use boids::Boid;
use boids::Flock;



#[macroquad::main("Boids")]
async fn main() {
    let w = screen_width() as usize;
    let h = screen_height() as usize;


    let mut flock = Flock::random(10);
    // let mut buffer = vec![CellState::Dead; w * h];

    let mut image = Image::gen_image_color(w as u16, h as u16, WHITE);


    let texture = Texture2D::from_image(&image);

    let w = image.width() as f32;
    let h = image.height() as f32;

    println!("h = {}, w = {}", h, w);

    loop {
        println!("Loope");
        clear_background(WHITE);

        let w = image.width() as f32;
        let h = image.height() as f32;

        for boid in flock.boids.iter() {
            // boid.position.x = boid.position.x + 10.;
            let x = (boid.position.x % w) as u32;
            let y = (boid.position.y % h) as u32;
    
            image.set_pixel(
                x,
                y,
                BLACK,
            );
        }
    

        texture.update(&image);

        draw_texture(&texture, 0., 0., WHITE);
        flock = flock.next_step();
        next_frame().await
    }
}
