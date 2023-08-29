#[cfg(test)]
use crate::domain::{ grid::Grid, cell::Cell };

#[test]
fn test_new_grid() {
  let grid = Grid::new(10);

  assert_eq!(grid.get_grid().len(), 10);
  for i in 0..10 {
    assert_eq!(grid.get_grid()[i].len(), 10);
  }
}

#[test]
fn test_grid_mutate_1() {
  // Before: 0 0 0 0 0  After: 0 0 0 0 0
  //         0 0 1 0 0         0 0 0 0 0
  //         0 0 1 0 0         0 1 1 1 0
  //         0 0 1 0 0         0 0 0 0 0
  //         0 0 0 0 0         0 0 0 0 0

  let mut grid = Grid::new(5);
  grid.update_many(
    vec![
      Cell::new(2, 1, 5).unwrap(),
      Cell::new(2, 2, 5).unwrap(),
      Cell::new(2, 3, 5).unwrap()
    ]
  );

  grid.mutate();

  assert_eq!(grid.get_grid()[1][2], true);
  assert_eq!(grid.get_grid()[2][2], true);
  assert_eq!(grid.get_grid()[3][2], true);

  // assert_eq!(grid.get_grid()[2][1], false);
  // assert_eq!(grid.get_grid()[2][3], false);
}

#[test]
fn test_grid_mutate_2() {
  // Before: 0 0 1 0 0  After: 0 0 0 0 0
  //         0 0 0 0 0         0 1 0 1 0
  //         0 1 1 1 0         0 0 1 0 0
  //         0 0 0 0 0         0 0 1 0 0
  //         0 0 0 0 0         0 0 0 0 0
  let mut grid = Grid::new(5);
  grid.update_many(
    vec![
      Cell::new(0, 2, 5).unwrap(),
      Cell::new(2, 1, 5).unwrap(),
      Cell::new(2, 2, 5).unwrap(),
      Cell::new(2, 3, 5).unwrap()
    ]
  );

  grid.mutate();

  assert!(grid.get_grid()[1][1]);
  assert!(grid.get_grid()[1][3]);
  assert!(grid.get_grid()[2][2]);
  assert!(grid.get_grid()[3][2]);

  assert_eq!(grid.get_grid()[0][2], false);
  assert_eq!(grid.get_grid()[2][1], false);
  assert_eq!(grid.get_grid()[2][3], false);
}

#[test]
fn test_grid_mutate_3() {
  // Before: 0 0 0 0 0  After: 0 0 0 0 0
  //         0 0 0 0 0         1 0 0 0 0
  //         1 1 0 0 1         1 0 0 0 0
  //         0 0 0 0 0         1 0 0 0 0
  //         0 0 0 0 0         0 0 0 0 0

  let mut grid = Grid::new(5);
  grid.update_many(
    vec![
      Cell::new(0, 2, 5).unwrap(),
      Cell::new(1, 2, 5).unwrap(),
      Cell::new(4, 2, 5).unwrap()
    ]
  );

  grid.mutate();

  assert!(grid.get_grid()[0][1]);
  assert!(grid.get_grid()[0][2]);
  assert!(grid.get_grid()[0][3]);

  assert_eq!(grid.get_grid()[1][2], false);
  assert_eq!(grid.get_grid()[4][2], false);
}
