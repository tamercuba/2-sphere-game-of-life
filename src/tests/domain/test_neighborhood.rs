#[cfg(test)]
use crate::domain::neighborhood::Neighborhood;
use crate::domain::cell::Cell;

#[test]
fn test_new_neighborhood() {
    let cell = Cell::new(0, 0, 10).unwrap();
    let neighborhood = Neighborhood::new(&cell);

    assert_eq!(neighborhood.left.x, 9);
    assert_eq!(neighborhood.left.y, 0);
    assert_eq!(neighborhood.right.x, 1);
    assert_eq!(neighborhood.right.y, 0);
    assert_eq!(neighborhood.top.x, 0);
    assert_eq!(neighborhood.top.y, 9);
    assert_eq!(neighborhood.bottom.x, 0);
    assert_eq!(neighborhood.bottom.y, 1);
    assert_eq!(neighborhood.upper_left.x, 9);
    assert_eq!(neighborhood.upper_left.y, 9);
    assert_eq!(neighborhood.upper_right.x, 1);
    assert_eq!(neighborhood.upper_right.y, 9);
    assert_eq!(neighborhood.bottom_left.x, 9);
    assert_eq!(neighborhood.bottom_left.y, 1);
    assert_eq!(neighborhood.bottom_right.x, 1);
    assert_eq!(neighborhood.bottom_right.y, 1);
}

#[test]
fn test_new_neighborhood_bigger_than_grid_size() {
    let cell = Cell::new(9, 9, 10).unwrap();
    let neighborhood = Neighborhood::new(&cell);

    assert_eq!(neighborhood.left.x, 8);
    assert_eq!(neighborhood.left.y, 9);
    assert_eq!(neighborhood.right.x, 0);
    assert_eq!(neighborhood.right.y, 9);
    assert_eq!(neighborhood.top.x, 9);
    assert_eq!(neighborhood.top.y, 8);
    assert_eq!(neighborhood.bottom.x, 9);
    assert_eq!(neighborhood.bottom.y, 0);
    assert_eq!(neighborhood.upper_left.x, 8);
    assert_eq!(neighborhood.upper_left.y, 8);
    assert_eq!(neighborhood.upper_right.x, 0);
    assert_eq!(neighborhood.upper_right.y, 8);
    assert_eq!(neighborhood.bottom_left.x, 8);
    assert_eq!(neighborhood.bottom_left.y, 0);
    assert_eq!(neighborhood.bottom_right.x, 0);
    assert_eq!(neighborhood.bottom_right.y, 0);
}

#[test]
fn test_neighborhood_iter() {
    let cell = Cell::new(0, 0, 10).unwrap();
    let neighborhood = Neighborhood::new(&cell);
    let iter = neighborhood.iter();

    assert_eq!(iter.len(), 8);
    assert_eq!(iter[0].x, 9);
    assert_eq!(iter[0].y, 9);
    assert_eq!(iter[1].x, 0);
    assert_eq!(iter[1].y, 9);
    assert_eq!(iter[2].x, 1);
    assert_eq!(iter[2].y, 9);
    assert_eq!(iter[3].x, 9);
    assert_eq!(iter[3].y, 0);
    assert_eq!(iter[4].x, 1);
    assert_eq!(iter[4].y, 0);
    assert_eq!(iter[5].x, 9);
    assert_eq!(iter[5].y, 1);
    assert_eq!(iter[6].x, 0);
    assert_eq!(iter[6].y, 1);
    assert_eq!(iter[7].x, 1);
    assert_eq!(iter[7].y, 1);
}