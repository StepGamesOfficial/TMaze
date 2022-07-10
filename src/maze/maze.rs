use self::CellWall::*;
use crate::maze::cell::{Cell, CellWall};
use crate::core::*;

pub struct Maze {
    pub(crate) cells: Vec<Vec<Vec<Cell>>>,
    pub(crate) width: usize,
    pub(crate) height: usize,
    pub(crate) depth: usize,
}

impl Maze {
    pub fn size(&self) -> Dims3D {
        (self.width as i32, self.height as i32, self.depth as i32)
    }

    pub fn is_in_bounds(&self, pos: Dims3D) -> bool {
        0 <= pos.0
            && pos.0 < self.width as i32
            && 0 <= pos.1
            && pos.1 < self.height as i32
            && 0 <= pos.2
            && pos.2 < self.depth as i32
    }

    pub fn is_valid_neighbor(&self, cell: Dims3D, off: Dims3D) -> bool {
        (off.0 == -1 || off.0 == 1 || off.0 == 0)
            && (off.1 == -1 || off.1 == 1 || off.1 == 0)
            && (off.2 == -1 || off.2 == 1 || off.2 == 0)
            && ((off.0 == 1 || off.0 == -1) as u8
                + (off.1 == 1 || off.1 == -1) as u8
                + (off.2 == 1 || off.2 == -1) as u8)
                == 1
            && self.is_in_bounds(cell)
            && self.is_in_bounds((cell.0 + off.0, cell.1 + off.1, cell.2 + off.2))
    }

    pub fn is_valid_wall(&self, cell: Dims3D, wall: CellWall) -> bool {
        let neighbor_offset = wall.to_coord();
        self.is_valid_neighbor(cell, neighbor_offset)
    }

    pub fn which_wall(cell: Dims3D, cell2: Dims3D) -> CellWall {
        match (cell.0 - cell2.0, cell.1 - cell2.1, cell.2 - cell2.2) {
            (-1, 0, 0) => Right,
            (1, 0, 0) => Left,
            (0, -1, 0) => Bottom,
            (0, 1, 0) => Top,
            (0, 0, 1) => Down,
            (0, 0, -1) => Up,
            _ => panic!(),
        }
    }

    pub fn get_neighbors(&self, cell: Dims3D) -> Vec<&Cell> {
        let offsets = [
            (-1, 0, 0),
            (1, 0, 0),
            (0, -1, 0),
            (0, 1, 0),
            (0, 0, -1),
            (0, 0, 1),
        ];
        offsets
            .into_iter()
            .filter(|off| self.is_valid_neighbor(cell, *off))
            .map(|off| {
                &self.cells[(cell.2 + off.2) as usize][(cell.1 + off.1) as usize]
                    [(cell.0 + off.0) as usize]
            })
            .collect()
    }

    pub fn remove_wall(&mut self, cell: Dims3D, wall: CellWall) {
        if !self.is_valid_wall(cell, wall) {
            return;
        }

        self.cells[cell.2 as usize][cell.1 as usize][cell.0 as usize].remove_wall(wall);
        let neighbor_offset = wall.to_coord();
        {
            let x2 = (cell.0 + neighbor_offset.0) as usize;
            let y2 = (cell.1 + neighbor_offset.1) as usize;
            let z2 = (cell.2 + neighbor_offset.2) as usize;
            self.cells[z2][y2][x2].remove_wall(wall.reverse_wall());
        }
    }

    pub fn get_cells(&self) -> &[Vec<Vec<Cell>>] {
        &self.cells
    }
}
