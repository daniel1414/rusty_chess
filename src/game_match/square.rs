#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
pub struct SquarePosition {
    pub x: u8,
    pub y: u8,
}

impl SquarePosition {
    pub fn new(x: u8, y: u8) -> Self {
        Self { x, y }
    }

    pub fn to_index(&self) -> usize {
        (self.y * 8 + self.x) as usize
    }

    pub fn add(&self, x: u8, y: u8) -> Self {
        Self::new(self.x + x, self.y + y)
    }

    pub fn try_add(&self, x: i8, y: i8) -> Option<Self> {
        let new_x = self.x as i8 + x;
        let new_y = self.y as i8 + y;
        if new_x < 0 || new_x > 7 || new_y < 0 || new_y > 7 {
            None
        } else {
            Some(SquarePosition::new(new_x as u8, new_y as u8))
        }
    }
}
