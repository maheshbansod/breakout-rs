extern crate sdl2;

use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::time::Duration;

use mylib::Ball;
use mylib::Paddle;

pub fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem.window("Breakout rs", 800, 600)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    canvas.present();
    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut paddle = Paddle::new();
    let mut ball = Ball::new();

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                Event::KeyDown { keycode: Some(Keycode::Space), ..} => {
                    paddle.release(&mut ball);
                },
                _ => {}
            }
        }
        
        paddle.update(canvas.viewport(), &event_pump.keyboard_state());
        ball.update(canvas.viewport(), paddle.to_rect());

        canvas.set_draw_color(Color::RGB(0, 255, 255));
        canvas.clear();

        paddle.draw(&mut canvas);
        ball.draw(&mut canvas);

        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}
