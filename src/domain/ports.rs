pub trait ICell: Sized {
  fn get_x(&self) -> i16;
  fn get_y(&self) -> i16;
  fn new(x: i16, y: i16, size: i16) -> Result<Self, String>;
  fn get_neighborhood(&self) -> Vec<Self>;
}
