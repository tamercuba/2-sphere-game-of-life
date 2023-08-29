#[cfg(test)]
use crate::domain::{ neighborhood::Neighborhood, cell::Cell };

#[test]
fn test_new_neighborhood() {
  let cell = Cell::new(5, 5, 10).unwrap();
  let neighborhood = Neighborhood::new(&cell);

  assert_eq!(neighborhood.iter().len(), 8);
}

#[test]
fn test_new_neighborhood_bigger_than_grid_size() {
  let cell = Cell::new(9, 9, 10).unwrap();
  let neighborhood = Neighborhood::new(&cell);

  assert_eq!(neighborhood.iter().len(), 3);
}

#[test]
fn test_neighborhood_zero_x_index() {
  let cell = Cell::new(0, 2, 5).unwrap();
  let neighborhood = Neighborhood::new(&cell);

  assert_eq!(neighborhood.iter().len(), 5);
}

#[test]
fn test_neighborhood_zero_y_index() {
  let cell = Cell::new(2, 0, 5).unwrap();
  let neighborhood = Neighborhood::new(&cell);

  assert_eq!(neighborhood.iter().len(), 5);
}

#[test]
fn test_neighborhood_max_x_index() {
  let cell = Cell::new(4, 2, 5).unwrap();
  let neighborhood = Neighborhood::new(&cell);

  assert_eq!(neighborhood.iter().len(), 5);
}

#[test]
fn test_neighborhood_max_y_index() {
  let cell = Cell::new(2, 4, 5).unwrap();
  let neighborhood = Neighborhood::new(&cell);

  assert_eq!(neighborhood.iter().len(), 5);
}

#[test]
fn test_neighborhood_origin() {
  let cell = Cell::new(0, 0, 5).unwrap();
  let neighborhood = Neighborhood::new(&cell);

  assert_eq!(neighborhood.iter().len(), 3);
}
