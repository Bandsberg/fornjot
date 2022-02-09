use crate::{camera::Camera, math::Point, window::Window};
use nalgebra::distance;
use winit::dpi::PhysicalPosition;

#[derive(PartialEq, Debug)]
pub struct Movement {
    focus_point: Option<Point<3>>,
    cursor: Option<PhysicalPosition<f64>>,
}

impl Movement {
    pub fn new() -> Self {
        Self {
            focus_point: None,
            cursor: None,
        }
    }

    pub fn start(
        &mut self,
        focus_point: Option<Point<3>>,
        cursor: Option<PhysicalPosition<f64>>,
    ) {
        self.focus_point = focus_point;
        self.cursor = cursor;
    }

    pub fn stop(&mut self) {
        self.focus_point = None;
    }

    pub fn apply(
        &mut self,
        cursor: Option<PhysicalPosition<f64>>,
        camera: &mut Camera,
        window: &Window,
    ) {
        if let (Some(previous), Some(cursor)) = (self.cursor, cursor) {
            let previous = camera.cursor_to_model_space(previous, window);
            let cursor = camera.cursor_to_model_space(cursor, window);

            if let Some(focus_point) = self.focus_point {
                let d1 = distance(&camera.position(), &cursor);
                let d2 = distance(&camera.position(), &focus_point);

                let diff = (cursor - previous) * d2 / d1;
                let offset = camera.camera_to_model().transform_vector(&diff);

                camera.translation.x += offset.x;
                camera.translation.y += offset.y;
            }
        }

        self.cursor = cursor;
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use faux::when;
    use nalgebra::{Translation, Vector3};
    use parry3d_f64::bounding_volume::AABB;
    use winit::dpi::PhysicalPosition;

    #[test]
    fn test_new() {
        let expected_movement = Movement {
            focus_point: None,
            cursor: None,
        };
        assert!(Movement::new() == expected_movement);
    }

    #[test]
    fn test_start() {
        let mut movement = Movement::new();
        let focus_point = Some(Point::origin());
        let cursor = Some(PhysicalPosition::new(1.0, 1.0));
        movement.start(focus_point, cursor);

        let expected_movement = Movement {
            focus_point: Some(Point::origin()),
            cursor: Some(PhysicalPosition::new(1.0, 1.0)),
        };

        assert!(movement == expected_movement);
    }

    #[test]
    fn test_stop() {
        let mut movement = Movement {
            focus_point: Some(Point::origin()),
            cursor: None,
        };
        movement.stop();
        assert_eq!(movement.focus_point, None);
    }

    #[test]
    fn test_apply() {
        // Tests struct Movements "apply" method effect on Movement (self)
        // and "translation" in struct Camera

        //cursor
        let cursor: Option<PhysicalPosition<f64>> =
            Some(PhysicalPosition::new(1.0, 1.0));

        //camera
        let min_point = Point::from(Vector3::new(1.0, 1.0, 1.0));
        let max_point = Point::from(Vector3::new(100.0, 100.0, 100.0));
        let aabb = AABB::new(min_point, max_point);
        let mut camera = Camera::new(&aabb);

        //Window. Mock window for consitency(reproducability) and simplicity

        let mut window = Window::faux();
        when!(window.height()).then_return(100);
        when!(window.width()).then_return(100);

        let mut movement = Movement::new();
        movement.start(
            Some(Point::origin()),
            Some(PhysicalPosition::new(100.0, 100.0)),
        );
        movement.apply(cursor, &mut camera, &window);

        println!("movement: {:?}", movement);

        let expected_movement = Movement {
            focus_point: Some(Point::origin()),
            cursor: Some(PhysicalPosition { x: 1.0, y: 1.0 }),
        };
        println!("expected_movement: {:?}", expected_movement);

        assert!(movement == expected_movement);

        //Expected trnaslation of camera. Used output of current test as it is fairly
        //assumed to be the results that is currently desired.

        let expected_translation = Translation::from([
            -521.7068842052822,
            420.70688420528217,
            -400.4023513866121,
        ]);
        assert_eq!(camera.translation, expected_translation);
    }
}
