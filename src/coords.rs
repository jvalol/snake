use cgmath::Vector2;

/// The playfield: a grid of square cells centered in the window.
///
/// Positions are in pixels with the origin at the window's top-left corner and
/// y pointing down, the space blitkit draws quads in.
#[derive(Debug, Copy, Clone)]
pub struct Grid {
  /// Top-left corner of the playfield in pixels.
  pub origin: Vector2<f32>,
  /// Width and height of one cell in pixels.
  pub cell: f32,
  pub cols: i32,
  pub rows: i32,
}

impl Grid {
  /// Fits `rows` rows of square cells into a window of `size` pixels, then fills
  /// the width with as many columns as fit and centers the result.
  pub fn new(size: Vector2<f32>, rows: i32) -> Grid {
    let cell = (size.y / rows as f32).floor().max(1.0);
    let cols = ((size.x / cell).floor() as i32).max(1);
    let rows = ((size.y / cell).floor() as i32).max(1);
    let origin = Vector2::new(
      (size.x - cols as f32 * cell) * 0.5,
      (size.y - rows as f32 * cell) * 0.5,
    );

    Grid {
      origin,
      cell,
      cols,
      rows,
    }
  }

  /// The cell holding `position`. Cells outside the playfield are negative or
  /// past `cols` and `rows`, which is how a crash into a wall is detected.
  pub fn cell_at(&self, position: Vector2<f32>) -> (i32, i32) {
    (
      ((position.x - self.origin.x) / self.cell).floor() as i32,
      ((position.y - self.origin.y) / self.cell).floor() as i32,
    )
  }

  /// The center of `cell` in pixels.
  pub fn center_of(&self, cell: (i32, i32)) -> Vector2<f32> {
    Vector2::new(
      self.origin.x + (cell.0 as f32 + 0.5) * self.cell,
      self.origin.y + (cell.1 as f32 + 0.5) * self.cell,
    )
  }

  pub fn contains(&self, cell: (i32, i32)) -> bool {
    cell.0 >= 0 && cell.0 < self.cols && cell.1 >= 0 && cell.1 < self.rows
  }

  /// Playfield size in pixels, which is the window size minus the leftover edges.
  pub fn size(&self) -> Vector2<f32> {
    Vector2::new(self.cols as f32 * self.cell, self.rows as f32 * self.cell)
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  fn grid() -> Grid {
    // 600 / 32 rounds down to 18 pixel cells: 44 columns and 33 rows, 792 by 594
    Grid::new((800.0, 600.0).into(), 32)
  }

  #[test]
  fn cells_are_square() {
    let grid = grid();

    assert_eq!(grid.cell, 18.0);
    assert_eq!(grid.cols, 44);
    assert_eq!(grid.rows, 33);
  }

  #[test]
  fn playfield_is_centered() {
    let grid = grid();

    assert_eq!(grid.size().x, 792.0);
    assert_eq!(grid.size().y, 594.0);
    assert_eq!(grid.origin.x, 4.0);
    assert_eq!(grid.origin.y, 3.0);
  }

  #[test]
  fn cell_at_finds_the_cell() {
    let grid = grid();

    assert_eq!(grid.cell_at(grid.origin), (0, 0));
    assert_eq!(grid.cell_at((grid.origin.x + 19.0, grid.origin.y + 1.0).into()), (1, 0));
    // a position left of the playfield is outside it
    assert_eq!(grid.cell_at((0.0, 0.0).into()), (-1, -1));
  }

  #[test]
  fn center_of_is_the_middle_of_the_cell() {
    let grid = grid();
    let center = grid.center_of((0, 0));

    assert_eq!(center.x, grid.origin.x + 9.0);
    assert_eq!(center.y, grid.origin.y + 9.0);
    // a cell center maps back to its own cell
    assert_eq!(grid.cell_at(center), (0, 0));
  }

  #[test]
  fn contains_rejects_cells_outside() {
    let grid = grid();

    assert!(grid.contains((0, 0)));
    assert!(grid.contains((grid.cols - 1, grid.rows - 1)));
    assert!(!grid.contains((-1, 0)));
    assert!(!grid.contains((0, -1)));
    assert!(!grid.contains((grid.cols, 0)));
    assert!(!grid.contains((0, grid.rows)));
  }
}
