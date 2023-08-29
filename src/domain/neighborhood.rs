use std::fmt;

use super::cell::Cell;

#[derive(Debug)]
pub struct Neighborhood {
  pub left: Option<Cell>,
  pub right: Option<Cell>,
  pub top: Option<Cell>,
  pub bottom: Option<Cell>,
  pub upper_left: Option<Cell>,
  pub upper_right: Option<Cell>,
  pub bottom_left: Option<Cell>,
  pub bottom_right: Option<Cell>,
}

impl Neighborhood {
  pub fn new(cell: &Cell) -> Self {
    Neighborhood {
      left: cell.get_left(),
      right: cell.get_right(),
      top: cell.get_top(),
      bottom: cell.get_bottom(),
      upper_left: cell.get_upper_left(),
      upper_right: cell.get_upper_right(),
      bottom_left: cell.get_bottom_left(),
      bottom_right: cell.get_bottom_right(),
    }
  }

  pub fn iter(&self) -> Vec<Cell> {
    let mut _vec = vec![
      self.upper_left,
      self.top,
      self.upper_right,
      self.left,
      self.right,
      self.bottom_left,
      self.bottom,
      self.bottom_right
    ];
    return _vec
      .iter()
      .filter(|&x| x.is_some())
      .map(|x| x.unwrap())
      .collect();
  }
}

fn _parse(cell: Option<Cell>) -> String {
  return match cell {
    Some(x) => format!("{}", x),
    None => String::from("None"),
  };
}

impl fmt::Display for Neighborhood {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    let left = match self.left {
      Some(ref x) => format!("{}", x),
      None => String::from("None"),
    };
    let top = match self.top {
      Some(ref x) => format!("{}", x),
      None => String::from("None"),
    };
    write!(
      f,
      "{} {} {}\n {} X:X {}\n {} {} {}",
      _parse(self.left),
      _parse(self.top),
      _parse(self.upper_right),
      _parse(self.left),
      _parse(self.right),
      _parse(self.bottom_left),
      _parse(self.bottom),
      _parse(self.bottom_right)
    )
  }
}
