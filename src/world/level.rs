

pub use level::Level;

pub mod level {
    use crate::world::map::Map;
    type Canvas = sdl2::render::Canvas<sdl2::video::Window>;

    pub struct Level {
        map: Map,
    }

    impl Level {
        pub fn new(map: Map) -> Level {
            Level {
                map
            }
        }

        pub fn draw(&self, canvas: &mut Canvas) {

            self.map.draw(canvas);
        }
    }
}