use ggez::mint::Point2;
use rand::Rng;
const CELL_SIZE: i32 = 30;
const MAX_POS: i32 = 15;
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Position{
    pub x:i32,
    pub y:i32,
}

impl Position{
    pub fn new(x:i32,y:i32) ->Self{
        Position{x,y}
    }

    pub fn out_of_bounds(&self) -> bool{
         self.x < 0 || self.x>MAX_POS ||
             self.y < 0 || self.y>MAX_POS
    }

    pub fn random(max_x: i32, max_y: i32) -> Self {
        let mut rng = rand::thread_rng();
        Position::new(rng.gen_range(0..max_x+1), rng.gen_range(0..max_y+1))
    }
}
impl Into<Point2<f32>> for Position {
    fn into(self: Self) -> Point2<f32> {
        Point2::<f32> {
            x: (self.x*CELL_SIZE) as f32,
            y: (self.y*CELL_SIZE) as f32,
        }
    }
}