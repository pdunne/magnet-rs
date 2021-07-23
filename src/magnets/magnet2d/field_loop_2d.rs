use crate::magnets::{GetField, MagnetType2D};
use crate::points::{Point2, Points2};
use crate::MagnetError;
use anyhow::Result;

/// Loops over every 2D Magnet and gets the field at a point `point` .
///
/// By using an enum `MagnetType2D` we can match each magnet type to access the underlying methods.
pub fn loop_field_2d(
    magnet_list: &Vec<MagnetType2D>,
    point: &Point2,
) -> Result<Point2, MagnetError> {
    let mut local_field = Point2::zero();

    // loop over magnets in list
    for mag in magnet_list {
        // sum fields for each magnet type
        match mag {
            &MagnetType2D::Rectangle(magnet) => local_field += magnet.field(point)?,
            &MagnetType2D::Circle(magnet) => local_field += magnet.field(point)?,
        }
    }
    Ok(local_field)
}

pub fn get_field_2d(
    magnet_list: &Vec<MagnetType2D>,
    point: (&f64, &f64),
) -> Result<(f64, f64), MagnetError> {
    let mut local_field = Point2::zero();
    let point = Point2::new(*point.0, *point.1);

    // loop over magnets in list
    for mag in magnet_list {
        // sum fields for each magnet type
        match mag {
            &MagnetType2D::Rectangle(magnet) => local_field += magnet.field(&point)?,
            &MagnetType2D::Circle(magnet) => local_field += magnet.field(&point)?,
        }
    }
    Ok(local_field.as_tuple())
}

#[cfg(test)]
mod tests {
    use crate::{magnets::Rectangle, utils::conversions::Angle};

    use super::*;

    #[test]
    fn test_loop_2d() {
        let mut magnet_list = Vec::<MagnetType2D>::new();

        // Create Magnets
        let m1 = Rectangle::new(
            1.0,
            1.0,
            (-0.5, -0.5),
            Angle::Degrees(0.0),
            1.0,
            Angle::Degrees(90.0),
        );
        magnet_list.push(MagnetType2D::Rectangle(m1));

        let m2 = Rectangle::new(
            1.0,
            1.0,
            (0.5, -0.5),
            Angle::Degrees(0.0),
            -1.0,
            Angle::Degrees(90.0),
        );
        magnet_list.push(MagnetType2D::Rectangle(m2));

        // Create Test Point
        let point = Point2::new(0.0, 0.01);

        // Get local field for all magnets in the registry
        let local_field = loop_field_2d(&magnet_list, &point).unwrap();
        println!("Total field is {} at point {}", local_field, point);
        assert_eq!(local_field.x, 1.357145077959237);
    }
}
