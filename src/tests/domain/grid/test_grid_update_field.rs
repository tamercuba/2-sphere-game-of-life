#[cfg(test)]
use crate::domain::{
  cells::finite_plane_cell::FinitePlaneCell as Cell,
  grid::Grid,
};

#[test]
fn test_grid_update_field() {
  let mut grid = Grid::new(10);

  assert_eq!(grid.get_grid()[5][5], false);
  grid.update_field(Cell::new(5, 5, 10).unwrap());
  assert_eq!(grid.get_grid()[5][5], true);
}

#[test]
fn test_grid_update_many() {
  let mut grid = Grid::new(10);
  let cells = vec![Cell::new(5, 5, 10).unwrap(), Cell::new(5, 6, 10).unwrap()];

  assert_eq!(grid.get_grid()[5][5], false);
  assert_eq!(grid.get_grid()[5][6], false);
  grid.update_many(cells);
  assert_eq!(grid.get_grid()[5][5], true);
  assert_eq!(grid.get_grid()[5][6], true);
}

#[test]
fn test_grid_get_liveness_0_neighbors_dead() {
  let grid = Grid::new(10);
  let cell = Cell::new(5, 5, 10).unwrap();

  assert_eq!(grid.get_liveness(cell), false);
}

#[test]
fn test_grid_get_liveness_0_neighbors_alive() {
  let mut grid = Grid::new(10);
  let cell = Cell::new(5, 5, 10).unwrap();
  grid.update_field(cell);

  assert_eq!(grid.get_liveness(cell), false);
}

#[test]
fn test_grid_get_liveness_1_neighbors_dead() {
  let mut grid = Grid::new(10);
  let cell = Cell::new(5, 5, 10).unwrap();
  grid.update_field(Cell::new(5, 6, 10).unwrap());

  assert_eq!(grid.get_liveness(cell), false);
}

#[test]
fn test_grid_get_liveness_1_neighbors_alive() {
  let mut grid = Grid::new(10);
  let cell = Cell::new(5, 5, 10).unwrap();
  grid.update_many(vec![cell, Cell::new(5, 6, 10).unwrap()]);

  assert_eq!(grid.get_liveness(cell), false);
}

#[test]
fn test_grid_get_liveness_2_neighbors_dead() {
  let mut grid = Grid::new(10);
  let cell = Cell::new(5, 5, 10).unwrap();
  grid.update_many(
    vec![Cell::new(5, 6, 10).unwrap(), Cell::new(6, 6, 10).unwrap()]
  );

  assert_eq!(grid.get_liveness(cell), false);
}

#[test]
fn test_grid_get_liveness_2_neighbors_alive() {
  let mut grid = Grid::new(10);
  let cell = Cell::new(5, 5, 10).unwrap();
  grid.update_many(
    vec![cell, Cell::new(5, 6, 10).unwrap(), Cell::new(6, 6, 10).unwrap()]
  );

  assert_eq!(grid.get_liveness(cell), true);
}

#[test]
fn test_grid_get_liveness_3_neighbors_dead() {
  let mut grid = Grid::new(10);
  let cell = Cell::new(5, 5, 10).unwrap();

  grid.update_many(
    vec![
      Cell::new(5, 6, 10).unwrap(),
      Cell::new(6, 6, 10).unwrap(),
      Cell::new(6, 5, 10).unwrap()
    ]
  );

  assert_eq!(grid.get_liveness(cell), true);
}

#[test]
fn test_grid_get_liveness_3_neighbors_alive() {
  let mut grid = Grid::new(10);
  let cell = Cell::new(5, 5, 10).unwrap();

  grid.update_many(
    vec![
      cell,
      Cell::new(5, 6, 10).unwrap(),
      Cell::new(6, 6, 10).unwrap(),
      Cell::new(6, 5, 10).unwrap()
    ]
  );

  assert_eq!(grid.get_liveness(cell), true);
}

#[test]
fn test_grid_get_liveness_4_neighbors_dead() {
  let mut grid = Grid::new(10);
  let cell = Cell::new(5, 5, 10).unwrap();

  grid.update_many(
    vec![
      Cell::new(5, 6, 10).unwrap(),
      Cell::new(6, 6, 10).unwrap(),
      Cell::new(6, 5, 10).unwrap(),
      Cell::new(4, 5, 10).unwrap()
    ]
  );

  assert_eq!(grid.get_liveness(cell), false);
}

#[test]
fn test_grid_get_liveness_4_neighbors_alive() {
  let mut grid = Grid::new(10);
  let cell = Cell::new(5, 5, 10).unwrap();

  grid.update_many(
    vec![
      cell,
      Cell::new(5, 6, 10).unwrap(),
      Cell::new(6, 6, 10).unwrap(),
      Cell::new(6, 5, 10).unwrap(),
      Cell::new(4, 5, 10).unwrap()
    ]
  );

  assert_eq!(grid.get_liveness(cell), false);
}

#[test]
fn test_grid_get_liveness_5_neighbors_dead() {
  let mut grid = Grid::new(10);
  let cell = Cell::new(5, 5, 10).unwrap();

  grid.update_many(
    vec![
      cell,
      Cell::new(5, 6, 10).unwrap(),
      Cell::new(6, 6, 10).unwrap(),
      Cell::new(6, 5, 10).unwrap(),
      Cell::new(4, 5, 10).unwrap(),
      Cell::new(4, 6, 10).unwrap()
    ]
  );

  assert_eq!(grid.get_liveness(cell), false);
}

#[test]
fn test_grid_get_liveness_5_neighbors_alive() {
  let mut grid = Grid::new(10);
  let cell = Cell::new(5, 5, 10).unwrap();

  grid.update_many(
    vec![
      cell,
      Cell::new(5, 6, 10).unwrap(),
      Cell::new(6, 6, 10).unwrap(),
      Cell::new(6, 5, 10).unwrap(),
      Cell::new(4, 5, 10).unwrap(),
      Cell::new(4, 6, 10).unwrap()
    ]
  );

  assert_eq!(grid.get_liveness(cell), false);
}

#[test]
fn test_grid_get_liveness_6_neighbors_dead() {
  let mut grid = Grid::new(10);
  let cell = Cell::new(5, 5, 10).unwrap();

  grid.update_many(
    vec![
      Cell::new(5, 6, 10).unwrap(),
      Cell::new(6, 6, 10).unwrap(),
      Cell::new(6, 5, 10).unwrap(),
      Cell::new(4, 5, 10).unwrap(),
      Cell::new(4, 6, 10).unwrap(),
      Cell::new(4, 4, 10).unwrap()
    ]
  );

  assert_eq!(grid.get_liveness(cell), false);
}

#[test]
fn test_grid_get_liveness_6_neighbors_alive() {
  let mut grid = Grid::new(10);
  let cell = Cell::new(5, 5, 10).unwrap();

  grid.update_many(
    vec![
      cell,
      Cell::new(5, 6, 10).unwrap(),
      Cell::new(6, 6, 10).unwrap(),
      Cell::new(6, 5, 10).unwrap(),
      Cell::new(4, 5, 10).unwrap(),
      Cell::new(4, 6, 10).unwrap(),
      Cell::new(4, 4, 10).unwrap()
    ]
  );

  assert_eq!(grid.get_liveness(cell), false);
}
