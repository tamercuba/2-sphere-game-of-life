use super::cell::Cell;

pub struct Neighborhood {
    pub left: Cell,
    pub right: Cell,
    pub top: Cell,
    pub bottom: Cell,
    pub upper_left: Cell,
    pub upper_right: Cell,
    pub bottom_left: Cell,
    pub bottom_right: Cell
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
            bottom_right: cell.get_bottom_right()
        }
    }

    pub fn iter(&self) -> Vec<Cell> {
        vec![
            self.upper_left,
            self.top,
            self.upper_right,
            self.left,
            self.right,
            self.bottom_left,
            self.bottom,
            self.bottom_right
        ]
    }
}