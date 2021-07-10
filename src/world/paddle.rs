
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
        bounds: (i32, i32),
        //Maybe add power ups too?
    }

    impl Paddle {
        pub fn new(x: i32, y: i32, width: u32, height: u32, speed: u32, bounds: (i32, i32)) -> Paddle {
            let dir = 0;
            Paddle {
                x, y, speed, width, height, dir, bounds
            }
        }

        pub fn center_x(&self) -> i32 {
            self.x + self.width as i32 / 2
        }

        pub fn y(&self) -> i32 {
            self.y
        }

        pub fn width(&self) -> u32 {
            self.width
        }

        pub fn upper_bound_x(&self) -> i32 {
            self.x + self.width as i32
        }

        pub fn upper_bound_y(&self) -> i32 {
            self.y + self.height as i32
        }

        pub fn lower_bound_x(&self) -> i32 {
            self.x
        }

        pub fn lower_bound_y(&self) -> i32 {
            self.y
        }

        pub fn set_direction(&mut self, dir: i8) {
            self.dir = dir;
        }

        pub fn update(&mut self) {
            self.x += self.dir as i32 * self.speed as i32;

            if self.bounds.0 > self.x {
                self.x = self.bounds.0;
            } else if self.bounds.1 < self.x+self.width as i32 {
                self.x = self.bounds.1 - self.width as i32;
            }
        }

        pub fn draw(&self, canvas: &mut Canvas) {
            canvas.set_draw_color(Color::RGB(200,200,200));
            canvas.fill_rect(Rect::new(self.x, self.y, self.width, self.height)).unwrap();
        }
    }
}