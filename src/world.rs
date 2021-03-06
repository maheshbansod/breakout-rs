

mod paddle;
mod ball;
mod level;
mod map;

pub use world::World;

pub mod world {
    use super::paddle::Paddle;
    use super::ball::Ball;
    use super::level::Level;
    use super::map::Map;
    type Canvas = sdl2::render::Canvas<sdl2::video::Window>;
    use sdl2::EventPump;

    pub struct World {
        paddle: Paddle,
        ball: Ball,
        attached: bool,
        lives: u32,
        levels: Vec<Level>,
        current_level: u32,
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

            let mut levels = Vec::new();

            let mut paths: Vec<_> = std::fs::read_dir("./lvls/").unwrap().map(|r| r.unwrap()).collect();
            paths.sort_by_key(|dir| dir.path());
            for path in paths {
                levels.push(Level::new(Map::from_file( path.path().to_str().unwrap().to_string(),
                    vp.width(), vp.height()/3, 10, 10).unwrap()));
            }

            World {
                paddle: Paddle::new(px,py,paddle_width,paddle_height, paddle_speed, pbounds),
                ball: Ball::new(bx, by, bradius, 0.0, 0.0, ball_speed, bbounds),
                attached: true,
                lives: 3,
                levels: levels,
                current_level: 1,
            }
        }

        pub(crate) fn update(&mut self) {
            self.paddle.update();

            if self.attached {
                self.ball.attach_to(&self.paddle);
            } else if self.ball.collides_with(&self.paddle) {
                self.ball.bounce_back(&self.paddle);
            }
            
            if self.ball.handle_collision_with_brick(self.levels[(self.current_level - 1) as usize].map_mutable()) {
                if self.levels[(self.current_level - 1) as usize].is_complete() {
                    if self.levels.len() > self.current_level as usize {
                        println!("You've completed level {}! Congratulations! ^_^", self.current_level);
                        self.current_level += 1;
                        self.attached = true;
                    } else {
                        println!("You've completed the game!!");
                    }
                }
            }

            self.ball.update();

            if self.ball.has_died() {
                self.lives -= 1;
                if self.lives > 0 {
                    self.attached = true;
                    self.ball.revive();
                } else {
                    //game over
                    println!("game over");
                    self.ball.revive(); //dont revive when game over implemented
                    self.lives = 1;
                }
            }
        }

        pub(crate) fn draw(&self, canvas: &mut Canvas) {
            self.levels[(self.current_level-1) as usize].draw(canvas);
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