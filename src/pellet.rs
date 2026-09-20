use crate::coords::Grid;
use blitkit::geometry::quad::Quad;

pub struct Pellet {
  pub quad: Quad,
  pub visible: bool,
}

impl Pellet {
  pub fn new() -> Pellet {
    Pellet {
      quad: Quad::new((0.0, 0.0).into(), (0.0, 0.0).into()),
      visible: false,
    }
  }

  pub fn place(&mut self, grid: &Grid, cell: (i32, i32)) {
    self.quad = Quad::new(grid.center_of(cell), (grid.cell, grid.cell).into());
  }

  pub fn cell(&self, grid: &Grid) -> (i32, i32) {
    grid.cell_at(self.quad.position)
  }
}
