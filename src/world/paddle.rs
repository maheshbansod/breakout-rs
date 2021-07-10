
pub use paddle::Paddle;

pub mod paddle {
    type Canvas = sdl2::render::Canvas<sdl2::video::Window>;
    use sdl2::rect::Rect;
    use sdl2::pixels::Color;

    pub struct Paddle {
        x: i32,
        speed: u32,
        width: u32,
        height: u32,
        y: i32,
        dir: i8,
        //Maybe add power ups too?
    }

    impl Paddle {
        pub fn new(x: i32, y: i32, width: u32, height: u32, speed: u32) -> Paddle {
            let dir = 0;
            Paddle {
                x, y, speed, width, height, dir
            }
        }

        pub fn set_direction(&mut self, dir: i8) {
            self.dir = dir;
        }

        pub fn update(&mut self) {
            self.x += self.dir as i32 * self.speed as i32;
        }

        pub fn draw(&self, canvas: &mut Canvas) {
            canvas.set_draw_color(Color::RGB(200,200,200));
            canvas.fill_rect(Rect::new(self.x, self.y, self.width, self.height)).unwrap();
        }
    }
}