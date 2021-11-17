use nalgebra::vector;

use crate::{
    geometry::vertices::Vertices as _,
    math::{Point, Vector},
};

/// Access a shape's faces
pub trait Faces {
    /// Compute triangles to approximate the shape's faces
    ///
    /// `tolerance` defines by how far this triangulation is allowed to deviate
    /// from the faces' actual dimensions.
    fn triangles(&self, tolerance: f32) -> Vec<Triangle>;
}

/// A triangle
///
/// Or more specifically, three points. Currently now validation is done, to
/// ensure those points form an actual triangle.
#[derive(Clone, Copy)]
pub struct Triangle(pub [Point; 3]);

impl Triangle {
    /// Invert the triangle
    ///
    /// Inverts the order of triangle vertices.
    pub fn invert(self) -> Self {
        let [v0, v1, v2] = self.0;
        Self([v0, v2, v1])
    }

    /// Translate the triangle
    ///
    /// Translate all triangle vertices by the given vector.
    pub fn translate(self, vector: Vector) -> Self {
        let vertices = self.0.map(|vertex| vertex + vector);
        Self(vertices)
    }
}

impl From<[Point; 3]> for Triangle {
    fn from(vertices: [Point; 3]) -> Self {
        Self(vertices)
    }
}

impl Faces for fj::Shape {
    fn triangles(&self, tolerance: f32) -> Vec<Triangle> {
        match self {
            Self::Shape2d(shape) => shape.triangles(tolerance),
            Self::Shape3d(shape) => shape.triangles(tolerance),
        }
    }
}

impl Faces for fj::Shape2d {
    fn triangles(&self, tolerance: f32) -> Vec<Triangle> {
        match self {
            Self::Circle(shape) => shape.triangles(tolerance),
            Self::Square(shape) => shape.triangles(tolerance),
        }
    }
}

impl Faces for fj::Shape3d {
    fn triangles(&self, tolerance: f32) -> Vec<Triangle> {
        match self {
            Self::Sweep(shape) => shape.triangles(tolerance),
        }
    }
}

impl Faces for fj::Circle {
    fn triangles(&self, tolerance: f32) -> Vec<Triangle> {
        // To approximate the circle, we use a regular polygon for which the
        // circle is the circumscribed circle. The `tolerance` parameter is the
        // maximum allowed distance between the polygon and the circle. This is
        // the same as the difference between the circumscribed circle and the
        // incircle.
        //
        // Let's figure which regular polygon we need to use, by just trying out
        // some of them until we find one whose maximum error is less than or
        // equal to the tolerance.
        let mut n = 3;
        loop {
            let incircle_radius = self.radius * (PI / n as f32).cos();
            let maximum_error = self.radius - incircle_radius;

            if maximum_error <= tolerance {
                break;
            }

            n += 1;
        }

        let mut vertices = Vec::new();
        for i in 0..n {
            let angle = 2. * PI / n as f32 * i as f32;

            let (sin, cos) = angle.sin_cos();

            let x = cos * self.radius;
            let y = sin * self.radius;

            circumference.push(Point2::new(x, y));
        }

        let mut vertices = Vec::new();

        // let v = self.vertices();

        // triangles.push([v[0], v[1], v[2]].into());
        // triangles.push([v[0], v[2], v[3]].into());

        // triangles

        // TASK: Implement.
        todo!()
    }
}

impl Faces for fj::Square {
    fn triangles(&self, _: f32) -> Vec<Triangle> {
        let mut triangles = Vec::new();

        let v = self.vertices();

        triangles.push([v[0], v[1], v[2]].into());
        triangles.push([v[0], v[2], v[3]].into());

        triangles
    }
}

impl Faces for fj::Sweep {
    fn triangles(&self, tolerance: f32) -> Vec<Triangle> {
        let mut triangles = Vec::new();

        let original_triangles = self.shape.triangles(tolerance);

        // Bottom face
        triangles.extend(
            original_triangles.iter().map(|triangle| triangle.invert()),
        );

        // Top face
        triangles.extend(original_triangles.iter().map(|triangle| {
            triangle.translate(vector![0.0, 0.0, self.length])
        }));

        // In the next step, we're going to collect those pairs of vertices into
        // quads. But we also need to make sure we'll get the last quad, which
        // is made up of the last and first pair.
        let mut vertex_pairs = self.vertices().vertex_pairs();
        vertex_pairs.push(vertex_pairs[0]);

        // Collect all vertices that make up the quads of the side faces.
        //
        // This can be simplified (and made non-panicky), once `array_windows`
        // is stabilized.
        let quads = vertex_pairs.windows(2).map(|window| {
            let [v0, v3] = window[0];
            let [v1, v2] = window[1];

            [v0, v1, v2, v3]
        });

        for [v0, v1, v2, v3] in quads {
            triangles.push([v0, v1, v2].into());
            triangles.push([v0, v2, v3].into());
        }

        triangles
    }
}
