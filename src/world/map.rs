
pub use map::Map;

mod map {

    use std::fs::File;
    use std::io::{BufReader, Read};

    type Canvas = sdl2::render::Canvas<sdl2::video::Window>;
    use sdl2::rect::Rect;
    use sdl2::pixels::Color;

    use crate::world::ball::Ball;

    const M: usize = 7;
    const N: usize = 10;

    pub struct Map {
        map: [i32; M*N],
        width: u32,
        height: u32,
        marginx: u32, //outside margin
        marginy: u32, //outside margin
        brick_margin: u32,
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
        pub fn new(bricks: [i32; M*N], width: u32, height: u32, marginx: u32, marginy: u32) -> Map {
            Map {
                map: bricks,
                width: width,
                height: height,
                marginx: marginx,
                marginy: marginy,
                brick_margin: 2,
            }
        }

        pub fn from_file(filename: String, width: u32, height: u32, marginx: u32, marginy: u32) -> Result<Map, String> {
            let mut data = String::new();
            let f = File::open(filename);
            if let Err(_e) = f {
                return Err(String::from("Unable to open file"));
            }
            let f = f.unwrap();
            let mut br = BufReader::new(f);
            br.read_to_string(&mut data).expect("Unable to read string from file");

            let mut map = [0; M*N];
            data.split(" ").filter_map(|number| {
                match number.parse::<i32>() {
                    Ok(number) => Some(number),
                    Err(_) => None
                }
            }).zip(map.iter_mut())
                .for_each(|(b, df)| *df = b);
            
            Ok(Map::new(
                map,
                width,
                height,
                marginx,
                marginy
            ))
        }

        pub fn is_empty(&self) -> bool {
            !self.map.iter().any(|x| *x>0 )
        }

        pub fn check_collision_with_ball(&mut self, ball: &Ball) -> Option<u32> {
            let x = ball.x();
            let y = ball.y();
            let radius = ball.radius();

            // if x as u32 > self.width + radius || (x as u32) < self.margin
            // || y as u32 > self.height + radius|| (y as u32) < self.margin {
            //     return None;
            // }

            // let lbx = x - radius as i32;
            // let ubx = x + radius as i32;

            let bswidth = self.width/N as u32;
            let bsheight = self.height/M as u32;

            let i = (y - self.marginy as i32)/bsheight as i32;
            let j = (x - self.marginx as i32)/bswidth as i32;

            //now check up, down, left and right

            //TODO: FIX THIS
            if i - 1 >= 0 && i - 1 < M as i32 && j >= 0 && j < N as i32 { // up
                if y as u32 - radius < self.marginy + (i as u32)*bsheight { //upper one is sus
                    let i = i-1;
                    let brick = &mut self.map[i as usize*N + j as usize];
                    if *brick > 0 {
                        *brick -= 1;
                        return Some(1); //vertical hit
                    }
                }
            }
            if j - 1 >= 0 && j - 1 < N as i32 && i >= 0 && i < M as i32 { //left
                if x as u32 - radius < self.marginx+(j as u32 )*bswidth { //left one is sus
                    let j = j-1;
                    let brick = &mut self.map[i as usize*N + j as usize];
                    if *brick > 0 {
                        *brick -= 1;
                        return Some(2); //horizontal hit
                    }
                }
            }

            if i + 1 < M as i32 && j >= 0 && j < N as i32 { //down
                if y as u32 + radius > self.marginy+(i as u32  + 1)*bsheight { //bottom one is sus
                    let i = i+1;
                    let brick = &mut self.map[i as usize*N + j as usize];
                    if *brick > 0 {
                        *brick -= 1;
                        return Some(1); //vertical hit
                    }
                }
            }

            if j + 1 < N as i32 && i >= 0 && i < M as i32 { //right
                if x as u32 + radius > self.marginx + (j as u32  + 1)*bswidth { //right one is sus
                    let j = j + 1;
                    let brick = &mut self.map[i as usize*N + j as usize];
                    if *brick > 0 {
                        *brick -= 1;
                        return Some(2); //horizontal hit
                    }
                }
            }

            None
        }

        pub fn draw(&self, canvas: &mut Canvas) {
            let width_for_one = self.width as u32/N as u32;
            let height_for_one = self.height as u32/M as u32;
            let bwidth = width_for_one - self.brick_margin;
            let bheight = height_for_one - self.brick_margin;
            canvas.set_draw_color(Color::RGB(255,120,120));
            for i in 0..M {
                for j in 0..N {
                    if self.map[i*N + j] != 0 {

                        canvas.set_draw_color(Color::RGB(255,60*self.map[i*N + j] as u8,120));
                        canvas.fill_rect(Rect::new(self.marginx as i32 + j as i32*width_for_one as i32,
                            self.marginy as i32 + i as i32*height_for_one as i32,
                            bwidth, bheight)).unwrap();

                    }
                }
            }
        }
    }
}