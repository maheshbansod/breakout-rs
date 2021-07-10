

use sdl2::pixels::Color;
use sdl2::rect::Point;
use sdl2::rect::Rect;

type Canvas = sdl2::render::Canvas<sdl2::video::Window>;


pub struct Paddle {
    x: i32,
    speed: u32,
    width: u32,
    height: u32,
    y: i32,
    //Maybe add power ups too?
}

impl Paddle {
    pub fn new() -> Paddle {
        Paddle {
            x: 0,
            speed: 7,
            width: 200,
            height: 20,
            y: 0,
        }
    }

    pub fn release(&self, ball: &mut Ball) {
        let bspeed = 10;
        ball.set_velocity(0, -bspeed);
        ball.release();
    }
    
    pub fn update(&mut self, vp: Rect, keyboard: &sdl2::keyboard::KeyboardState) {
        use sdl2::keyboard::Scancode;

        self.y = (vp.height() - self.height) as i32;

        if keyboard.is_scancode_pressed(Scancode::A) {
            self.x -= self.speed as i32;
        }
        if keyboard.is_scancode_pressed(Scancode::D) {
            self.x += self.speed as i32;
        }

        let vpw = vp.width() as i32;

        if self.x+(self.width as i32) > vpw {
            self.x = vpw - self.width as i32;
        } else if self.x < 0 {
            self.x = 0;
        }
    }

    pub fn draw(&self, canvas: &mut Canvas) {
        self.draw_paddle(canvas);
//        canvas.fill_rect(Rect::new(x, y, self.width, self.height)).expect("Error bruh");
    }

    fn draw_paddle(&self, canvas: &mut Canvas) {
        let n = 50;
        let pi = std::f32::consts::PI;
        let theta0 = 0.0;//3.0*pi/8.0;
        let step = (pi/2.0 - theta0)/(n as f32);
        let r = self.width as f32 / (2.0 * f32::cos(theta0));
        let y0 = (self.y + self.height as i32) as f32;
        let dh = self.height as f32/n as f32;
        let cx = self.x as f32 + self.width as f32/2.0;

        for i in 0..n {
            let theta = theta0 + step*i as f32;
            let rect_width = 2.0 * r * f32::cos(theta);
            let rect_x = cx - rect_width/2.0;
            let rect_y = y0 - dh*i as f32;
            let c = (255 as f32*(i as f32/n as f32)) as u8;
            let paddle_color = Color::RGB(c,c,c);
            canvas.set_draw_color(paddle_color);
            canvas.fill_rect(Rect::new(rect_x as i32, rect_y as i32, rect_width as u32, dh as u32)).unwrap();
        }
    }

    pub fn to_rect(&self) ->  Rect {
        Rect::new(
            self.x,
            self.y,
            self.width,
            self.height,
        )
    }
}

pub struct Ball {
    radius: u32,
    x: i32, /* center x */
    y: i32, /* center y */
    dx: i32,
    dy: i32,
    attached: bool,
    bang: Option<BangAnim>,
}

impl Ball {
    pub fn new() -> Ball {
        Ball {
            radius: 10,
            x: 0,
            y: 0,
            dx: 0,
            dy: 0,
            attached: true,
            bang: None,
        }
    }

    pub fn set_velocity(&mut self, dx: i32, dy: i32) {
        self.dx = dx;
        self.dy = dy;
    }

    pub fn release(&mut self) {
        self.attached = false;
    }

    pub fn draw(&self, canvas: &mut Canvas) {
        let ball_color = Color::RGB(255, 0, 0);
        canvas.set_draw_color(ball_color);
        fill_circle(canvas, Point::new(self.x, self.y), self.radius);
        if let Some(bang) = &self.bang {
            let bang_color = Color::RGB(255,255,0);
            canvas.set_draw_color(bang_color);
            bang.draw(canvas);
        }
    }

    pub fn update(&mut self, vp: Rect, paddle_rect: Rect) {

        if let Some(bang) = &mut self.bang {
            bang.update();
            if bang.isdead() {
                self.bang = None;
            }
        }

        if !self.attached {
            self.x += self.dx;
            self.y += self.dy;

            let vpw = vp.width() as i32;
            let vph = vp.height() as i32;
            let radius = self.radius as i32;

            if self.x + radius > vpw {
                self.x = vpw - radius;
                self.dx = -self.dx;
            } else if self.x - radius < 0 {
                self.x = radius;
                self.dx = -self.dx;
            }

            let py = paddle_rect.y();//vph - paddle_rect.height() as i32;

            if self.y + radius > py {
               let lb = paddle_rect.x() - radius;
               let paddle_width = paddle_rect.width() as i32;
               let ub = paddle_rect.x() + paddle_width + radius;
               if self.x > lb && self.x < ub { //bounce on paddle
                   self.y = py - radius;
                   self.dy = -self.dy;
                   let paddle_center = (lb + ub)/2;
                   let acceleration_ratio = (self.x - paddle_center) as f32/(paddle_width/2) as f32;

                   self.dx = (10 as f32* acceleration_ratio) as i32;
                   self.bang = Some(BangAnim::new(self.x, self.y+(paddle_rect.height()/2) as i32, self.radius, 30));
               }
            }

            if self.y + radius > vph {
                //die? yes die.
                self.y = vph - radius;
                self.dy = -self.dy;
            } else if self.y - radius < 0 {
                self.y = radius;
                self.dy = -self.dy;
            }

        } else {
            self.x = paddle_rect.x() + (paddle_rect.width()/2) as i32;
            self.y = paddle_rect.y() - self.radius as i32;
        }
    }
}

/**
 * Draw a weird fuckin circle
 * */
fn fill_circle(canvas: &mut Canvas, center: Point, radius: u32) {
    let n = 22;
    let pi = std::f32::consts::PI;
    let step = 2 as f32 * pi/(n as f32);
    let radius = radius as f32;
    for i in 0..n {
        canvas.draw_line(center, Point::new(center.x() + (radius*f32::cos(step*(i as f32))) as i32, center.y() + (radius*f32::sin(step*(i as f32))) as i32)).unwrap();
    }
}

struct BangAnim {
    x: i32,
    y: i32,
    radius: u32,
    life: u32,
}

impl BangAnim {
    fn new(x: i32, y: i32, radius: u32, life: u32) -> BangAnim {
        BangAnim {
            x, y, radius, life
        }
    }

    fn update(&mut self) {
        self.life -= 1;
    }

    fn isdead(&self) -> bool {
        self.life <= 0
    }

    fn draw(&self, canvas: &mut Canvas) {
        fill_circle(canvas, Point::new(self.x, self.y), self.radius);
    }
}
