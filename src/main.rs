use io::output::{ConsoleOutput, Output};
use math::vec3::{Color, Vec3};

pub mod io;
pub mod math;

pub const IMAGE_WIDTH: i32 = 256;
pub const IMAGE_HEIGHT: i32 = 256;

fn main() {
    ConsoleOutput::write(generate_image().iter());
}

fn generate_image<'a>() -> Vec<Vec3> {
    (0..=(IMAGE_HEIGHT - 1))
        .map(|j| {
            (0..IMAGE_WIDTH)
                .map(|i| {
                    Color::new(
                        (i as f32) / (IMAGE_WIDTH - 1) as f32,
                        (IMAGE_HEIGHT - j) as f32 / (IMAGE_HEIGHT - 1) as f32,
                        0.25,
                    )
                    .unit_vec()
                    .mul(255.99)
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
