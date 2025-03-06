#[maybe_async_cfg::maybe(
    idents(Scanline, Triangle),
    sync(feature = "draw_target_sync"),
    async(feature = "draw_target_async")
)]
use crate::primitives::{common::Scanline, triangle::Triangle};

#[maybe_async_cfg::maybe(
    idents(ScanlineIterator),
    sync(feature = "draw_target_sync"),
    async(feature = "draw_target_async")
)]
use crate::{
    geometry::{Dimensions, Point},
    primitives::{common::StrokeOffset, triangle::scanline_iterator::ScanlineIterator},
};

/// Iterator over all points inside the triangle.
#[maybe_async_cfg::maybe(
    idents(Scanline, ScanlineIterator),
    sync(feature = "draw_target_sync"),
    async(feature = "draw_target_async")
)]
#[derive(Clone, Eq, PartialEq, Hash, Debug)]
#[cfg_attr(feature = "defmt", derive(::defmt::Format))]
pub struct Points {
    scanline_iter: ScanlineIterator,
    current_line: Scanline,
}

#[maybe_async_cfg::maybe(
    idents(Scanline, ScanlineIterator, Triangle),
    sync(feature = "draw_target_sync"),
    async(feature = "draw_target_async")
)]
impl Points {
    pub(in crate::primitives) fn new(triangle: &Triangle) -> Self {
        let scanline_iter = ScanlineIterator::new(
            triangle,
            0,
            StrokeOffset::None,
            true,
            &triangle.bounding_box(),
        );

        let current_line = Scanline::new_empty(0);

        Self {
            scanline_iter,
            current_line,
        }
    }
}

#[maybe_async_cfg::maybe(
    sync(feature = "draw_target_sync"),
    async(feature = "draw_target_async")
)]
impl Iterator for Points {
    type Item = Point;

    fn next(&mut self) -> Option<Self::Item> {
        self.current_line.next().or_else(|| {
            self.current_line = self.scanline_iter.next()?.0;

            self.current_line.next()
        })
    }
}

#[maybe_async_cfg::maybe(
    idents(Triangle),
    sync(feature = "draw_target_sync"),
    async(feature = "draw_target_async")
)]
#[cfg(test)]
mod tests {
    use super::Triangle;
    use crate::{
        geometry::Point,
        pixelcolor::BinaryColor,
        primitives::{PointsIter, Primitive, PrimitiveStyle},
        transform::Transform,
        Pixel,
    };

    #[test]
    fn points_iter() {
        let triangle = Triangle::new(Point::new(5, 10), Point::new(15, 20), Point::new(10, 15));

        let styled_points = triangle
            .clone()
            .into_styled(PrimitiveStyle::with_fill(BinaryColor::On))
            .pixels()
            .map(|Pixel(p, _)| p);

        assert!(triangle.points().eq(styled_points));
    }

    #[test]
    fn off_screen_still_draws_points() {
        let off_screen = Triangle::new(Point::new(10, 10), Point::new(20, 20), Point::new(30, -30));
        let on_screen = off_screen.translate(Point::new(0, 35));

        assert!(off_screen
            .points()
            .eq(on_screen.points().map(|p| p - Point::new(0, 35))));
    }

    #[test]
    fn it_draws_unfilled_tri_line_y() {
        let mut tri = Triangle::new(Point::new(2, 2), Point::new(2, 4), Point::new(2, 4)).points();

        assert_eq!(tri.next(), Some(Point::new(2, 2)));
        assert_eq!(tri.next(), Some(Point::new(2, 3)));
        assert_eq!(tri.next(), Some(Point::new(2, 4)));
        assert_eq!(tri.next(), None);
    }

    #[test]
    fn it_draws_unfilled_tri_line_x() {
        let mut tri = Triangle::new(Point::new(2, 2), Point::new(4, 2), Point::new(4, 2)).points();

        assert_eq!(tri.next(), Some(Point::new(2, 2)));
        assert_eq!(tri.next(), Some(Point::new(3, 2)));
        assert_eq!(tri.next(), Some(Point::new(4, 2)));
        assert_eq!(tri.next(), None);
    }
}
