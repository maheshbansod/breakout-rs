

mod paddle;

pub use world::World;

pub mod world {
    use super::paddle::Paddle;
    type Canvas = sdl2::render::Canvas<sdl2::video::Window>;
    use sdl2::EventPump;

    pub struct World {
        paddle: Paddle,
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
            World {
                paddle: Paddle::new(px,py,paddle_width,paddle_height, paddle_speed),
            }
        }

        pub(crate) fn update(&mut self) {
            self.paddle.update();
        }

        pub(crate) fn draw(&self, canvas: &mut Canvas) {
            self.paddle.draw(canvas);
        }

        pub(crate) fn handle_events(&mut self, event_pump: &EventPump) {
            use sdl2::keyboard::Scancode;

            let keyboard = event_pump.keyboard_state();
            let go_left = keyboard.is_scancode_pressed(Scancode::A);
            let go_right = keyboard.is_scancode_pressed(Scancode::D);

            if !(go_right && go_left) && (go_left || go_right){
                self.paddle.set_direction(if go_right {
                    1
                } else {
                    -1
                });
            } else {
                self.paddle.set_direction(0);
            }

        }
    }
}