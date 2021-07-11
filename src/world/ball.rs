
pub use ball::Ball;

mod ball {
    type Bounds = (i32, i32, u32, u32);
    type Canvas = sdl2::render::Canvas<sdl2::video::Window>;
    use sdl2::rect::Rect;
    use sdl2::pixels::Color;

    use crate::world::paddle::Paddle;

    pub struct Ball {
        x: i32,
        y: i32,
        dx: f32,
        dy: f32,
        speed: u32,
        radius: u32,

        bounds: Bounds,
    }

    enum OutOfBounds {
        Left,
        Right,
        Top,
        Bottom,
    }

    impl Ball {
        pub fn new(x: i32, y: i32, radius: u32, dx: f32, dy: f32,
            speed: u32, bounds: Bounds) -> Ball {
            Ball {
                x, y, dx, dy, speed, radius, bounds,
            }
        }

        pub fn update(&mut self) {
            let x = (self.x as f32 + self.dx * self.speed as f32) as i32;
            let y = (self.y as f32 + self.dy * self.speed as f32) as i32;

            if let Some(oob_dir) = is_out_of_bounds(x, y, self.radius, self.bounds) {
                match oob_dir {
                    OutOfBounds::Left | OutOfBounds::Right => {
                        self.dx = -self.dx;
                    },
                    OutOfBounds::Top | OutOfBounds::Bottom => {
                        self.dy = -self.dy;
                    }
                }
            } else {
                self.x = x;
                self.y = y;
            }
        }

        pub fn attach_to(&mut self, paddle: &Paddle) {
            self.x = paddle.center_x();
            self.y = paddle.y() - self.radius as i32; 
            self.dx = 0.0;
            self.dy = 0.0;
        }

        pub fn collides_with(&self, paddle: &Paddle) -> bool {
            let ubx = paddle.upper_bound_x();
            let lbx = paddle.lower_bound_x();
            let uby = paddle.upper_bound_y();
            let lby = paddle.lower_bound_y();

            // println!("{} {} {} {}", uby, lby, self.y + self.radius as i32,self.y - self.radius as i32);

            let blbx = self.x - self.radius as i32;
            let bubx = self.x + self.radius as i32;
            let blby = self.y - self.radius as i32;
            let buby = self.y + self.radius as i32;

            ((blbx > lbx && blbx < ubx) || (bubx > lbx && bubx < ubx))
             && ((blby > lby && blby < uby) || (buby > lby && buby < uby))
        }

        pub fn bounce_back(&mut self, paddle: &Paddle) {
            let cx = paddle.center_x();
            self.y = paddle.y() - self.radius as i32 - 1;
            self.dx = (self.x -  cx) as f32/paddle.width() as f32;
            self.dy = -self.dy;
        }

        pub fn launch(&mut self) {
            self.dy = -1.0;
        }

        pub fn draw(&self, canvas: &mut Canvas) {
            canvas.set_draw_color(Color::RGB(255,0,0));
            let x = self.x - self.radius as i32;
            let y = self.y - self.radius as i32;
            let size = 2*self.radius;
            canvas.fill_rect(Rect::new(x, y, size, size)).unwrap();
        }
    }

    fn is_out_of_bounds(x: i32, y: i32, r: u32, bounds: Bounds) -> Option<OutOfBounds> {
        let (bx, by, bwidth, bheight) = bounds;

        let lbx = bx;
        let ubx = bx + bwidth as i32;
        
        let lby = by;
        let uby = by + bheight as i32;

        if x + r as i32 > ubx {
            Some(OutOfBounds::Right)
        } else if (x - r as i32) < lbx {
            Some(OutOfBounds::Left)
        } else if y + r as i32 > uby {
            Some(OutOfBounds::Bottom)
        } else if (y - r as i32) < lby {
            Some(OutOfBounds::Top)
        } else {
            None
        }
    }
}