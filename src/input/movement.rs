use crate::{camera::Camera, math::Point};
use cfg_if::cfg_if;
#[cfg(test)]
use mockall;
use mockall_double;
use nalgebra::distance;
use winit::dpi::PhysicalPosition;

//Setting up mock of Window / use window::Window
//Måske bare mocke i window craten??? og importere den mock og bruge den.
//Så tænker jeg namespace burde være correct.
cfg_if! {

    if #[cfg(test)] {
        use mockall::*;
        mock! {
            pub Window{
                fn height(&self) -> u32;
                fn width(&self) -> u32;
            }
        }
        use MockWindow as Window;

    }

    else {
        use crate::window::Window;
    }


}

#[derive(PartialEq)]
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
/*
#[cfg(test)]
mockall::mock! {pub Window{
    fn height() -> u32;
}}
*/
/*
#[cfg(not(test))]
use foo::Foo;
#[cfg(test)]
use foo::MockFoo as Foo;
*/

//#[double]
//use Window::height;

#[cfg(test)]
mod tests {

    use super::*;
    use crate::{camera::Camera, math::Point};
    use mockall;
    use mockall_double::double;
    use nalgebra::Vector3;
    use parry3d_f64::bounding_volume::AABB;
    use winit::dpi::PhysicalPosition;

    #[test]
    fn test_new() {
        let movement = Movement {
            focus_point: None,
            cursor: None,
        };
        assert!(Movement::new() == movement);
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

    //TODO
    #[test]
    fn test_apply() {
        //cursor
        let cursor: Option<PhysicalPosition<f64>> =
            Some(PhysicalPosition::new(1.0, 1.0));

        //camera: &mut Camera,
        //Camera: pub fn new(aabb: &AABB) -> Self
        //aabb: pub fn new(mins: Point<Real>, maxs: Point<Real>) -> AABB
        //Point:
        let min_point = Point::from(Vector3::new(1.0, 1.0, 1.0));
        let max_point = Point::from(Vector3::new(100.0, 100.0, 100.0));
        let aabb = AABB::new(min_point, max_point);
        let mut camera = Camera::new(&aabb);

        //Window. Mock window for consitency(reproducability) and simplcity
        //Use mockall_double and mockall, hopefully it will work :S
        /*
                mockall::mock! {
                    pub Window{
                        fn height(&self) -> u32;
                        fn width(&self) -> u32;
                    }
                }
                use MockWindow as Window;
        */
        let mut mock = MockWindow::new();
        mock.expect_height().return_const(100u32);

        let movement = Movement::new();
        movement.apply(cursor, &mut camera, &mock);
    }
}
