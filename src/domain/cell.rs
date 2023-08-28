use std::fmt;

#[derive(Debug)]
pub struct Cell {
    pub x: i16,
    pub y: i16,
    size: i16,
}



impl Cell {
    pub fn new(x: i16, y: i16, size: i16) -> Result<Self, String> {
        if x >= size || y >= size {
            return Err(format!("({}, {}) is out of bounds", x, y))
        }
        return Ok(Cell {x, y, size})
    }

    pub fn get_left(&self) -> Cell {
        let new_x: i16;

        if self.x == 0 {
            new_x = self.size - 1
        } else {
            new_x = self.x - 1
        }

        return Cell::new(new_x, self.y, self.size).unwrap()
    }

    pub fn get_right(&self) -> Cell {
        let new_x: i16;
        if self.x == self.size - 1 {
            new_x = 0
        } else {
            new_x = self.x + 1
        }
        return Cell::new(new_x, self.y, self.size).unwrap()
    }

    pub fn get_top(&self) -> Cell {
        let new_y: i16;
        if self.y == 0 {
            new_y = self.size - 1
        } else {
            new_y = self.y - 1
        }
        return Cell::new(self.x, new_y, self.size).unwrap()
    }

    pub fn get_bottom(&self) -> Cell {
        let new_y: i16;
        if self.y == self.size - 1 {
            new_y = 0
        } else {
            new_y = self.y + 1
        }
        return Cell::new(self.x, new_y, self.size).unwrap()
    }

    pub fn get_upper_left(&self) -> Cell {
        let new_x: i16;
        let new_y: i16;

        if self.x == 0 {
            new_x = self.size - 1
        } else {
            new_x = self.x - 1
        }
        if self.y == 0 {
            new_y = self.size - 1
        } else {
            new_y = self.y - 1
        }

        return Cell::new(new_x, new_y, self.size).unwrap()
    }

    pub fn get_upper_right(&self) -> Cell {
        let new_y: i16;
        let new_x: i16;

        if self.x == self.size - 1 {
            new_x = 0
        } else {
            new_x = self.x + 1
        }
        if self.y == 0 {
            new_y = self.size - 1
        } else {
            new_y = self.y - 1
        }

        return Cell::new(new_x, new_y, self.size).unwrap()
    }

    pub fn get_bottom_left(&self) -> Cell {
        let new_y: i16;
        let new_x: i16;

        if self.x == 0 {
            new_x = self.size - 1
        } else {
            new_x = self.x - 1
        }
        if self.y == self.size - 1 {
            new_y = 0
        } else {
            new_y = self.y + 1
        }

        return Cell::new(new_x, new_y, self.size).unwrap()
    }

    pub fn get_bottom_right(&self) -> Cell {
        let new_y: i16;
        let new_x: i16;

        if self.x == self.size - 1 {
            new_x = 0
        } else {
            new_x = self.x + 1
        }
        if self.y == self.size - 1 {
            new_y = 0
        } else {
            new_y = self.y + 1
        }
        return Cell::new(new_x, new_y, self.size).unwrap()
    }
}

impl fmt::Display for Cell {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "(x: {}, y: {})", self.x, self.y)
    }
}

impl Copy for Cell {}

impl Clone for Cell {
    fn clone(&self) -> Self {
        Cell {
            x: self.x,
            y: self.y,
            size: self.size
        }
    }
}
