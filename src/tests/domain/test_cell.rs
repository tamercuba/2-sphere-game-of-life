#[cfg(test)]
use crate::domain::cell::Cell;

#[allow(dead_code)]
const GRID_SIZE: i16 = 10;
#[test]
fn test_new_cell() {
  let cell = Cell::new(0, 0, GRID_SIZE).unwrap();

  assert_eq!(cell.x, 0);
  assert_eq!(cell.y, 0);
}

#[test]
fn test_new_cell_bigger_than_grid_size() {
  let cell = Cell::new(GRID_SIZE, GRID_SIZE, GRID_SIZE);

  assert!(cell.is_err());
}

#[test]
fn test_cell_display() {
  let cell = Cell::new(0, 0, GRID_SIZE).unwrap();

  assert_eq!(format!("{}", cell), "(x: 0, y: 0)");
}

#[test]
fn test_get_left() {
  let cell = Cell::new(1, 0, GRID_SIZE).unwrap();
  let left = cell.get_left().unwrap();

  assert_eq!(left.x, 0);
  assert_eq!(left.y, 0);
}

#[test]
fn test_get_left_bigger_than_grid_size() {
  let cell = Cell::new(0, 0, GRID_SIZE).unwrap();
  let left = cell.get_left();

  assert!(left.is_none());
}

#[test]
fn test_get_right() {
  let cell = Cell::new(0, 0, GRID_SIZE).unwrap();
  let right = cell.get_right().unwrap();

  assert_eq!(right.x, 1);
  assert_eq!(right.y, 0);
}

#[test]
fn test_get_right_bigger_than_grid_size() {
  let cell = Cell::new(GRID_SIZE - 1, 0, GRID_SIZE).unwrap();
  let right = cell.get_right();

  assert_eq!(right.is_some(), false);
}

#[test]
fn test_get_top() {
  let cell = Cell::new(0, 1, GRID_SIZE).unwrap();
  let top = cell.get_top().unwrap();

  assert_eq!(top.x, 0);
  assert_eq!(top.y, 0);
}

#[test]
fn test_get_top_bigger_than_grid_size() {
  let cell = Cell::new(0, 0, GRID_SIZE).unwrap();
  let top = cell.get_top();

  assert_eq!(top.is_some(), false);
}

#[test]
fn test_get_bottom() {
  let cell = Cell::new(0, 0, GRID_SIZE).unwrap();
  let bottom = cell.get_bottom().unwrap();

  assert_eq!(bottom.x, 0);
  assert_eq!(bottom.y, 1);
}

#[test]
fn test_get_bottom_bigger_than_grid_size() {
  let cell = Cell::new(0, GRID_SIZE - 1, GRID_SIZE).unwrap();
  let bottom = cell.get_bottom();

  assert_eq!(bottom.is_some(), false);
}

#[test]
fn test_get_upper_left() {
  let cell = Cell::new(1, 1, GRID_SIZE).unwrap();
  let upper_left = cell.get_upper_left().unwrap();

  assert_eq!(upper_left.x, 0);
  assert_eq!(upper_left.y, 0);
}

#[test]
fn test_get_upper_left_bigger_than_grid_size() {
  let cell = Cell::new(0, 0, GRID_SIZE).unwrap();
  let upper_left = cell.get_upper_left();

  assert_eq!(upper_left.is_some(), false);
}

#[test]
fn test_get_upper_right() {
  let cell = Cell::new(0, 1, GRID_SIZE).unwrap();
  let upper_right = cell.get_upper_right().unwrap();

  assert_eq!(upper_right.x, 1);
  assert_eq!(upper_right.y, 0);
}

#[test]
fn test_get_upper_right_bigger_than_grid_size() {
  let cell = Cell::new(GRID_SIZE - 1, 0, GRID_SIZE).unwrap();
  let upper_right = cell.get_upper_right();

  assert_eq!(upper_right.is_some(), false);
}

#[test]
fn test_get_bottom_left() {
  let cell = Cell::new(1, 0, GRID_SIZE).unwrap();
  let bottom_left = cell.get_bottom_left().unwrap();

  assert_eq!(bottom_left.x, 0);
  assert_eq!(bottom_left.y, 1);
}

#[test]
fn test_get_bottom_left_bigger_than_grid_size() {
  let cell = Cell::new(0, GRID_SIZE - 1, GRID_SIZE).unwrap();
  let bottom_left = cell.get_bottom_left();

  assert_eq!(bottom_left.is_some(), false);
}

#[test]
fn test_get_bottom_right() {
  let cell = Cell::new(0, 0, GRID_SIZE).unwrap();
  let bottom_right = cell.get_bottom_right().unwrap();

  assert_eq!(bottom_right.x, 1);
  assert_eq!(bottom_right.y, 1);
}

#[test]
fn test_get_bottom_right_bigger_than_grid_size() {
  let cell = Cell::new(GRID_SIZE - 1, GRID_SIZE - 1, GRID_SIZE).unwrap();
  let bottom_right = cell.get_bottom_right();

  assert_eq!(bottom_right.is_some(), false);
}
