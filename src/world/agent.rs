use crate::util::{Vec2f, Rect2f};

const DEFAULT_RADIUS: f32 = 4.0;

pub struct Agent {
    pub radius: f32,
    pub pos: Vec2f,
    pub angle: f32, // radians
    pub energy: f32
}

impl Agent {
    pub fn new(pos: Vec2f) -> Agent {
        Agent {
            radius: DEFAULT_RADIUS,
            pos,
            angle: 0.0,
            energy: 1.0
        }
    }

    #[inline]
    pub fn get_bounding_rect(&self) -> Rect2f {
        Rect2f::new(self.pos.x - self.radius, self.pos.y - self.radius, self.radius, self.radius)
    }

    pub fn tick(&mut self, d_time: f32) {
        // TODO: implement movement and energy code
        self.pos.x += d_time;
        self.pos.y += d_time * 3.0;
        self.angle += d_time;
    }
}
