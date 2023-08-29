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
      return Err(format!("({}, {}) is out of bounds", x, y));
    }
    return Ok(Cell { x, y, size });
  }

  pub fn get_left(&self) -> Option<Cell> {
    if self.x == 0 {
      return None;
    }

    return Some(Cell::new(self.x - 1, self.y, self.size).unwrap());
  }

  pub fn get_right(&self) -> Option<Cell> {
    if self.x == self.size - 1 {
      return None;
    }

    return Some(Cell::new(self.x + 1, self.y, self.size).unwrap());
  }

  pub fn get_top(&self) -> Option<Cell> {
    if self.y == 0 {
      return None;
    }

    return Some(Cell::new(self.x, self.y - 1, self.size).unwrap());
  }

  pub fn get_bottom(&self) -> Option<Cell> {
    if self.y == self.size - 1 {
      return None;
    }

    return Some(Cell::new(self.x, self.y + 1, self.size).unwrap());
  }

  pub fn get_upper_left(&self) -> Option<Cell> {
    if self.x == 0 || self.y == 0 {
      return None;
    }

    return Some(Cell::new(self.x - 1, self.y - 1, self.size).unwrap());
  }

  pub fn get_upper_right(&self) -> Option<Cell> {
    if self.x == self.size - 1 || self.y == 0 {
      return None;
    }

    return Some(Cell::new(self.x + 1, self.y - 1, self.size).unwrap());
  }

  pub fn get_bottom_left(&self) -> Option<Cell> {
    if self.x == 0 || self.y == self.size - 1 {
      return None;
    }

    return Some(Cell::new(self.x - 1, self.y + 1, self.size).unwrap());
  }

  pub fn get_bottom_right(&self) -> Option<Cell> {
    if self.x == self.size - 1 || self.y == self.size - 1 {
      return None;
    }

    return Some(Cell::new(self.x + 1, self.y + 1, self.size).unwrap());
  }

  pub fn get_neighborhood(&self) -> Vec<Cell> {
    let mut neighbors: Vec<Cell> = Vec::new();

    if let Some(cell) = self.get_upper_left() {
      neighbors.push(cell);
    }
    if let Some(cell) = self.get_top() {
      neighbors.push(cell);
    }
    if let Some(cell) = self.get_upper_right() {
      neighbors.push(cell);
    }
    if let Some(cell) = self.get_left() {
      neighbors.push(cell);
    }
    if let Some(cell) = self.get_right() {
      neighbors.push(cell);
    }
    if let Some(cell) = self.get_bottom_left() {
      neighbors.push(cell);
    }
    if let Some(cell) = self.get_bottom() {
      neighbors.push(cell);
    }
    if let Some(cell) = self.get_bottom_right() {
      neighbors.push(cell);
    }

    return neighbors;
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
      size: self.size,
    }
  }
}
