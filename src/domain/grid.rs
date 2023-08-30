use std::fmt;
use super::ports::ICell;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Grid {
  grid: Vec<Vec<bool>>,
  size: i16,
  age: i64,
}

impl Grid {
  pub fn new(size: i16) -> Self {
    Grid {
      grid: vec![vec![false; size as usize]; size as usize],
      size: size,
      age: 0,
    }
  }

  pub fn update_field<T: ICell>(&mut self, cell: T) {
    self.grid[cell.get_x() as usize][cell.get_y() as usize] ^= true;
  }

  pub fn update_many<T: ICell>(&mut self, cells: Vec<T>) {
    for cell in cells {
      self.update_field(cell);
    }
  }

  pub fn get_liveness<T: ICell>(&self, cell: T) -> bool {
    let mut alive_neighbors = 0;

    for neigboor in cell.get_neighborhood().iter() {
      if self.grid[neigboor.get_x() as usize][neigboor.get_y() as usize] {
        alive_neighbors += 1;
      }
    }

    // Any living cell with less than two living neighbors dies of loneliness.
    if
      self.grid[cell.get_x() as usize][cell.get_y() as usize] &&
      alive_neighbors < 2
    {
      return false;
    }

    // Any live cell with more than three live neighbors dies from overpopulation.
    if
      self.grid[cell.get_x() as usize][cell.get_y() as usize] &&
      alive_neighbors > 3
    {
      return false;
    }

    // Any cell with exactly three live neighbors becomes a live cell.
    if alive_neighbors == 3 {
      return true;
    }

    // Any cell with two live neighbors remains in the same state for the next generation.
    if alive_neighbors == 2 {
      return self.grid[cell.get_x() as usize][cell.get_y() as usize];
    } else {
      return false;
    }
  }

  pub fn mutate<T: Copy + ICell>(&mut self) {
    let mut to_mutate: Vec<T> = vec![];

    for x in 0..self.size {
      for y in 0..self.size {
        let cell = ICell::new(x, y, self.size).unwrap();
        let liveness = self.get_liveness(cell);

        if self.grid[x as usize][y as usize] != liveness {
          to_mutate.push(cell);
        }
      }
    }

    self.update_many(to_mutate);
    self.age += 1;
  }

  pub fn get_age(&self) -> i64 {
    self.age
  }

  #[allow(dead_code)]
  pub fn get_grid(&self) -> &Vec<Vec<bool>> {
    &self.grid
  }
}

impl<'a> fmt::Display for Grid {
  // Temporary apresentation of the grid just for testing purposes
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    let mut grid_string =
      String::from("#  ") +
      &String::from("# ").repeat((self.size as usize) + 2);
    grid_string.push_str("\n#  ");

    for row in &self.grid {
      for cell in row {
        if *cell {
          grid_string.push_str("\x1b[94m\u{23F9} \x1b[94m");
        } else {
          grid_string.push_str("\x1b[93m\u{23F9} \x1b[0m");
        }
      }
      grid_string.push_str("  #\n#  ");
    }

    let end = String::from("# ").repeat((self.size as usize) + 2);
    grid_string.push_str(&end);
    write!(f, "{}", grid_string)
  }
}
