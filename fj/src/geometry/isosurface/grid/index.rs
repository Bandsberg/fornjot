use std::{convert::TryInto as _, ops::Add};

use nalgebra::Point;

/// The index of a vertex within the isosurface extraction grid
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Index([usize; 3]);

impl Index {
    /// Return the x component of the index
    pub fn x(&self) -> usize {
        self.0[0]
    }

    /// Return the y component of the index
    pub fn y(&self) -> usize {
        self.0[1]
    }

    /// Return the z component of the index
    pub fn z(&self) -> usize {
        self.0[2]
    }

    /// Convert the index into a position
    ///
    /// Compute the position of the vertex within the isosurface extraction grid
    /// from `min`, the minimum point of the grid, and `resolution`, which
    /// defines the size of the grid cells.
    pub fn to_position(
        self,
        min: Point<f32, 3>,
        resolution: f32,
    ) -> Point<f32, 3> {
        [
            index_to_coordinate(self.x(), min.x, resolution),
            index_to_coordinate(self.y(), min.y, resolution),
            index_to_coordinate(self.z(), min.z, resolution),
        ]
        .into()
    }
}

impl From<[usize; 3]> for Index {
    fn from(index: [usize; 3]) -> Self {
        Self(index)
    }
}

impl Add<[isize; 3]> for Index {
    type Output = Self;

    fn add(mut self, rhs: [isize; 3]) -> Self::Output {
        self.0[0] = (self.0[0] as isize + rhs[0]).try_into().unwrap();
        self.0[1] = (self.0[1] as isize + rhs[1]).try_into().unwrap();
        self.0[2] = (self.0[2] as isize + rhs[2]).try_into().unwrap();

        self
    }
}

fn index_to_coordinate(index: usize, min: f32, resolution: f32) -> f32 {
    index as f32 * resolution + min - resolution / 2.0
}
