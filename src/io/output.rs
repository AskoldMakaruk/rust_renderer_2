use std::iter::Iterator;

use crate::math::vec3::*;
pub trait Output {
    fn write<'a, I>(it: I)
    where
        I: Iterator<Item = &'a Color>;
}

pub struct ConsoleOutput;

impl Output for ConsoleOutput {
    fn write<'a, I>(it: I)
    where
        I: Iterator<Item = &'a Color>,
    {
        println!("P3 {0} {1}\n255", crate::IMAGE_WIDTH, crate::IMAGE_HEIGHT);
        for color in it {
            println!("{}", color.write_color());
        }
    }
}

#[cfg(test)]
mod output_test {
    use super::ConsoleOutput;
    use super::Output;
    use crate::math::vec3::Color;

    #[test]
    fn console_output_test() {
        ConsoleOutput::write(vec![Color::new(1.0, 1.0, 1.0)].iter())
    }
}
