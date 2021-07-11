

mod paddle;
mod ball;

pub use world::World;

pub mod world {
    use super::paddle::Paddle;
    use super::ball::Ball;
    type Canvas = sdl2::render::Canvas<sdl2::video::Window>;
    use sdl2::EventPump;

    pub struct World {
        paddle: Paddle,
        ball: Ball,
        attached: bool,
    }

    impl World {
        pub(crate) fn new(canvas: &mut Canvas) -> World {
            let vp = canvas.viewport();

            let paddle_width = 100;
            let paddle_height = 20;
            let px = (vp.width()/2 - paddle_width/2) as i32;
            let bottom_margin = 20;
            let py = (vp.height() - paddle_height - bottom_margin) as i32;
            let paddle_speed = 10;
            let pbounds = (0, vp.width() as i32);

            let bradius = 10 as u32;
            let bx = px + paddle_width as i32 / 2;
            let by = py - bradius as i32/2;
            let ball_speed = 10;
            let bbounds = (0, 0, vp.width(), vp.height());

            World {
                paddle: Paddle::new(px,py,paddle_width,paddle_height, paddle_speed, pbounds),
                ball: Ball::new(bx, by, bradius, 0.0, 0.0, ball_speed, bbounds),
                attached: true,
            }
        }

        pub(crate) fn update(&mut self) {
            self.paddle.update();

            if self.attached {
                self.ball.attach_to(&self.paddle);
            } else if self.ball.collides_with(&self.paddle) {
                self.ball.bounce_back(&self.paddle);
            }

            self.ball.update();
        }

        pub(crate) fn draw(&self, canvas: &mut Canvas) {
            self.paddle.draw(canvas);
            self.ball.draw(canvas);
        }

        pub(crate) fn handle_events(&mut self, event_pump: &EventPump) {
            use sdl2::keyboard::Scancode;

            let keyboard = event_pump.keyboard_state();
            let mouse = event_pump.mouse_state();

            let go_left = keyboard.is_scancode_pressed(Scancode::A)
                || keyboard.is_scancode_pressed(Scancode::Left);
            let go_right = keyboard.is_scancode_pressed(Scancode::D)
                || keyboard.is_scancode_pressed(Scancode::Right);

            if !(go_right && go_left) && (go_left || go_right){
                self.paddle.set_direction(if go_right {
                    1
                } else {
                    -1
                });
            } else {
                self.paddle.set_direction(0);
            }

            let tolaunch = keyboard.is_scancode_pressed(Scancode::Space)
                || mouse.left();
            if tolaunch && self.attached {
                self.ball.launch();
                self.attached = false;
            }

        }
    }
}