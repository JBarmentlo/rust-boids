
use macroquad::prelude::*;
use std::iter;

mod vec2d;
mod boids;

use boids::Boid;


const N_BOIDS: usize = 10;

#[macroquad::main("Boids")]
async fn main() {
    let w = screen_width() as usize;
    let h = screen_height() as usize;

    let mut boids: Vec<Boid> = iter::from_fn(|| {
        Some(Boid::random())
    })
    .take(N_BOIDS)
    .collect();
    // let mut buffer = vec![CellState::Dead; w * h];

    let mut image = Image::gen_image_color(w as u16, h as u16, WHITE);


    let texture = Texture2D::from_image(&image);

    loop {
        clear_background(WHITE);

        let w = image.width();
        let h = image.height();


        for boid in boids.iter() {

            image.set_pixel(
                // boid.position.x % h,
                // boid.position.y % w,
                10,
                10,
                BLACK,
            );
        }


        texture.update(&image);

        draw_texture(&texture, 0., 0., WHITE);
        next_frame().await
    }
}
