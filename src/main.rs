use std::ops::Add;

use io::output::{ConsoleOutput, Output};
use math::{
    ray::Ray,
    vec3::{Color, Point, Vec3},
};

pub mod io;
pub mod math;

pub const ASPECT_RATIO: f32 = 16.0 / 9.0;

pub const IMAGE_WIDTH: i32 = 400;
pub const IMAGE_HEIGHT: i32 = (IMAGE_WIDTH as f32 / ASPECT_RATIO) as i32;

pub const VIEWPORT_HEIGHT: f32 = 2.0;
pub const VIEWPORT_WIDTH: f32 = ASPECT_RATIO * VIEWPORT_HEIGHT;
pub const FOCAL_LENGHT: f32 = 1.0;

fn ray_color(r: &Ray) -> Color {
    let unit_dir = r.direction.unit_vec();

    let t = 0.5 * (unit_dir.y + 1.0);
    Color::new(1.0, 1.0, 1.0)
        .mul(1.0 - t)
        .add(Color::new(0.5, 0.7, 1.0).mul(t))
}

fn main() {
    ConsoleOutput::write(generate_image().iter());
}

fn generate_image<'a>() -> Vec<Vec3> {
    let origin = Point::new(0.0, 0.0, 0.0);
    let horizontal = Vec3::new(VIEWPORT_WIDTH, 0.0, 0.0);
    let vertical = Vec3::new(0.0, VIEWPORT_HEIGHT, 0.0);
    let lower_left_corner =
        origin - horizontal.div(2.0) - vertical.div(2.0) - Vec3::new(0.0, 0.0, FOCAL_LENGHT as f32);

    (0..=(IMAGE_HEIGHT - 1))
        .map(|j| {
            (0..IMAGE_WIDTH)
                .map(|i| {
                    let u = i as f32 / (IMAGE_WIDTH as f32 - 1.0);
                    let v = j as f32 / (IMAGE_HEIGHT as f32 - 1.0);
                    let r: Ray = Ray {
                        orig: origin,
                        direction: lower_left_corner + horizontal.mul(u) + vertical.mul(v) - origin,
                    };
                    let color = ray_color(&r);

                    color
                })
                .collect::<Vec<Vec3>>()
        })
        .flatten()
        .collect::<Vec<Vec3>>()
}

// fn main() {
//     // Configure the window that you want to draw in. You can add an event
//     // handler to build interactive art. Input handlers for common use are
//     // provided.
//     let canvas = Canvas::new(512, 512)
//         .title("Tile")
//         .state(MouseState::new())
//         .input(MouseState::handle_input);
//     // The canvas will render for you at up to 60fps.
//     canvas.render(|mouse, image| {
//         // Modify the `image` based on your state.
//         let width = image.width() as usize;
//         for (y, row) in image.chunks_mut(width).enumerate() {
//             for (x, pixel) in row.iter_mut().enumerate() {
//                 let dx = x as i32 - mouse.x;
//                 let dy = y as i32 - mouse.y;
//                 let dist = dx * dx + dy * dy;
//                 *pixel = Color {
//                     r: if dist < 128 * 128 { dy as u8 } else { 0 },
//                     g: if dist < 128 * 128 { dx as u8 } else { 0 },
//                     b: x as u8,
//                 }
//             }
//         }
//     });
// }
// trait Renderer {
//     fn name() -> i32 {
//         1
//     }
// }
