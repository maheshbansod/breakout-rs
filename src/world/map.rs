
pub use map::Map;

mod map {
    type Canvas = sdl2::render::Canvas<sdl2::video::Window>;
    use sdl2::rect::Rect;
    use sdl2::pixels::Color;

    const M: usize = 7;
    const N: usize = 10;

    pub struct Map {
        map: [i32; M*N],
        width: u32,
        height: u32,
        margin: u32,
    }

    // struct Brick {
    //     btype: BrickType,
    //     strength: i32,
    // }

    // enum BrickType {
    //     NormalBrick,
    //     HardBrick,
    // }

    impl Map {
        pub fn new(bricks: [i32; M*N], width: u32, height: u32) -> Map {
            Map {
                map: bricks,
                width: width,
                height: height,
                margin: 2,
            }
        }

        pub fn draw(&self, canvas: &mut Canvas) {
            let width_for_one = self.width as u32/N as u32;
            let height_for_one = self.height as u32/M as u32;
            let bwidth = width_for_one - self.margin;
            let bheight = height_for_one - self.margin;
            canvas.set_draw_color(Color::RGB(255,120,120));
            for i in 0..M {
                for j in 0..N {
                    if self.map[i*N + j] != 0 {
                        canvas.fill_rect(Rect::new(j as i32*width_for_one as i32,
                            i as i32*height_for_one as i32,
                            bwidth, bheight)).unwrap();
                    }
                }
            }
        }
    }
}