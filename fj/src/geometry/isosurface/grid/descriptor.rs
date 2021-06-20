use std::ops::Range;

use itertools::Itertools as _;
use nalgebra::Point;

use crate::geometry::aabb::Aabb;

use super::{Cube, Index};

/// Describes a uniform grid for isosurface extraction
///
/// A grid consists of uniformly laid out vertices that form a number of cubes.
#[derive(Debug)]
pub struct Descriptor {
    /// The axis-aligned bounding box of the isosurface
    ///
    /// The uniform grid will extend beyond this bounding box, i.e. there will
    /// be grid vertices outside of the bounding box. See `Self::vertices` for
    /// details.
    pub aabb: Aabb<3>,

    /// The resolution of the grid, i.e. the distance between grid vertices
    pub resolution: f32,
}

impl Descriptor {
    /// Compute the grid cubes
    ///
    /// The grid extends beyond `self.aabb`, so that the center of the outermost
    /// grid cells are outside of, or on, the isosurface.
    pub fn cubes(&self) -> impl Iterator<Item = Cube> + '_ {
        let min = self.aabb.min;
        let max = self.aabb.max;

        let indices_x = cube_indices(min.x, max.x, self.resolution);
        let indices_y = cube_indices(min.y, max.y, self.resolution);
        let indices_z = cube_indices(min.z, max.z, self.resolution);

        let indices = indices_x
            .cartesian_product(indices_y)
            .cartesian_product(indices_z)
            .map(|((x, y), z)| Index::from([x, y, z]));

        let cubes = indices.map(move |index| Cube {
            min_index: index,
            min_position: index.to_coordinates(min, self.resolution),
            resolution: self.resolution,
        });

        cubes
    }

    /// Compute the grid vertices
    ///
    /// The grid extends beyond `self.aabb`, so that the center of the outermost
    /// grid cells are outside of, or on, the isosurface.
    pub fn vertices(
        &self,
    ) -> impl Iterator<Item = (Index, Point<f32, 3>)> + '_ {
        let min = self.aabb.min;
        let max = self.aabb.max;

        let indices_x = indices(min.x, max.x, self.resolution);
        let indices_y = indices(min.y, max.y, self.resolution);
        let indices_z = indices(min.z, max.z, self.resolution);

        let indices = indices_x
            .cartesian_product(indices_y)
            .cartesian_product(indices_z)
            .map(|((x, y), z)| Index::from([x, y, z]));

        let vertices = indices.map(move |index| {
            (index, index.to_coordinates(min, self.resolution))
        });

        vertices
    }
}

fn cube_indices(min: f32, max: f32, resolution: f32) -> Range<usize> {
    let lower = 0;
    let upper = ((max - min) / resolution).ceil() as usize + 1;

    lower..upper
}

fn indices(min: f32, max: f32, resolution: f32) -> Range<usize> {
    let lower = 0;
    let upper = ((max - min) / resolution).ceil() as usize + 2;

    lower..upper
}

#[cfg(test)]
mod tests {
    use crate::geometry::{aabb::Aabb, isosurface::grid};

    use super::Descriptor;

    #[test]
    fn cubes_should_return_grid_cubes() {
        let grid = Descriptor {
            aabb: Aabb {
                min: [0.0, 0.0, 0.5].into(),
                max: [0.5, 1.0, 1.5].into(),
            },
            resolution: 1.0,
        };

        let cubes: Vec<_> = grid.cubes().collect();

        assert_eq!(
            cubes,
            vec![
                grid::Cube {
                    min_index: [0, 0, 0].into(),
                    min_position: [-0.5, -0.5, 0.0].into(),
                    resolution: 1.0,
                },
                grid::Cube {
                    min_index: [0, 0, 1].into(),
                    min_position: [-0.5, -0.5, 1.0].into(),
                    resolution: 1.0,
                },
                grid::Cube {
                    min_index: [0, 1, 0].into(),
                    min_position: [-0.5, 0.5, 0.0].into(),
                    resolution: 1.0,
                },
                grid::Cube {
                    min_index: [0, 1, 1].into(),
                    min_position: [-0.5, 0.5, 1.0].into(),
                    resolution: 1.0,
                },
                grid::Cube {
                    min_index: [1, 0, 0].into(),
                    min_position: [0.5, -0.5, 0.0].into(),
                    resolution: 1.0,
                },
                grid::Cube {
                    min_index: [1, 0, 1].into(),
                    min_position: [0.5, -0.5, 1.0].into(),
                    resolution: 1.0,
                },
                grid::Cube {
                    min_index: [1, 1, 0].into(),
                    min_position: [0.5, 0.5, 0.0].into(),
                    resolution: 1.0,
                },
                grid::Cube {
                    min_index: [1, 1, 1].into(),
                    min_position: [0.5, 0.5, 1.0].into(),
                    resolution: 1.0,
                },
            ]
        );
    }

    #[test]
    fn vertices_should_return_grid_vertices() {
        let grid = Descriptor {
            aabb: Aabb {
                min: [0.0, 0.0, 0.5].into(),
                max: [0.5, 1.0, 1.5].into(),
            },
            resolution: 1.0,
        };

        let vertices: Vec<_> = grid.vertices().collect();

        assert_eq!(
            vertices,
            vec![
                ([0, 0, 0].into(), [-0.5, -0.5, 0.0].into()),
                ([0, 0, 1].into(), [-0.5, -0.5, 1.0].into()),
                ([0, 0, 2].into(), [-0.5, -0.5, 2.0].into()),
                ([0, 1, 0].into(), [-0.5, 0.5, 0.0].into()),
                ([0, 1, 1].into(), [-0.5, 0.5, 1.0].into()),
                ([0, 1, 2].into(), [-0.5, 0.5, 2.0].into()),
                ([0, 2, 0].into(), [-0.5, 1.5, 0.0].into()),
                ([0, 2, 1].into(), [-0.5, 1.5, 1.0].into()),
                ([0, 2, 2].into(), [-0.5, 1.5, 2.0].into()),
                ([1, 0, 0].into(), [0.5, -0.5, 0.0].into()),
                ([1, 0, 1].into(), [0.5, -0.5, 1.0].into()),
                ([1, 0, 2].into(), [0.5, -0.5, 2.0].into()),
                ([1, 1, 0].into(), [0.5, 0.5, 0.0].into()),
                ([1, 1, 1].into(), [0.5, 0.5, 1.0].into()),
                ([1, 1, 2].into(), [0.5, 0.5, 2.0].into()),
                ([1, 2, 0].into(), [0.5, 1.5, 0.0].into()),
                ([1, 2, 1].into(), [0.5, 1.5, 1.0].into()),
                ([1, 2, 2].into(), [0.5, 1.5, 2.0].into()),
                ([2, 0, 0].into(), [1.5, -0.5, 0.0].into()),
                ([2, 0, 1].into(), [1.5, -0.5, 1.0].into()),
                ([2, 0, 2].into(), [1.5, -0.5, 2.0].into()),
                ([2, 1, 0].into(), [1.5, 0.5, 0.0].into()),
                ([2, 1, 1].into(), [1.5, 0.5, 1.0].into()),
                ([2, 1, 2].into(), [1.5, 0.5, 2.0].into()),
                ([2, 2, 0].into(), [1.5, 1.5, 0.0].into()),
                ([2, 2, 1].into(), [1.5, 1.5, 1.0].into()),
                ([2, 2, 2].into(), [1.5, 1.5, 2.0].into()),
            ]
        );
    }
}
