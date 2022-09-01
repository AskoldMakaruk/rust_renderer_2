pub const IMAGE_WIDTH: i32 = 256;
pub const IMAGE_HEIGHT: i32 = 256;
fn main() {
    println!("P3 {0} {1}\n255", IMAGE_WIDTH, IMAGE_HEIGHT);

    for j in 0..=(IMAGE_HEIGHT - 1) {
        for i in 0..IMAGE_WIDTH {
            let j = IMAGE_HEIGHT - j;
            let r = i as f32 / ((IMAGE_WIDTH as f32) - 1.0);
            let g = j as f32 / ((IMAGE_HEIGHT as f32) - 1.0);
            let b = 0.25;

            let ir: i32 = (255.999 * r) as i32;
            let ig: i32 = (255.999 * g) as i32;
            let ib: i32 = (255.999 * b) as i32;

            println!("{0} {1} {2}", ir, ig, ib);
        }
    }
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
