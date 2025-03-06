//! Scanline iterator.

#[maybe_async_cfg::maybe(
    idents(Scanline, ScanlineIntersections),
    sync(feature = "draw_target_sync"),
    async(feature = "draw_target_async", idents(Triangle(async = "Triangle")))
)]
use crate::primitives::{
    common::{PointType, Scanline, StrokeOffset},
    triangle::scanline_intersections::ScanlineIntersections,
    Rectangle, Triangle,
};
use core::ops::Range;

/// Iterate over every scanline in the triangle's bounding box.
#[maybe_async_cfg::maybe(
    idents(ScanlineIntersections),
    sync(feature = "draw_target_sync"),
    async(feature = "draw_target_async")
)]
#[derive(Clone, Eq, PartialEq, Hash, Debug)]
#[cfg_attr(feature = "defmt", derive(::defmt::Format))]
pub(in crate::primitives::triangle) struct ScanlineIterator {
    rows: Range<i32>,
    scanline_y: i32,
    intersections: ScanlineIntersections,
}

#[maybe_async_cfg::maybe(
    idents(Scanline, ScanlineIntersections),
    sync(feature = "draw_target_sync"),
    async(
        feature = "draw_target_async",
        idents(Triangle(async = "TriangleAsync"))
    )
)]
impl ScanlineIterator {
    /// New.
    pub fn new(
        triangle: &Triangle,
        stroke_width: u32,
        stroke_offset: StrokeOffset,
        has_fill: bool,
        bounding_box: &Rectangle,
    ) -> Self {
        let triangle = triangle.sorted_clockwise();

        let mut rows = bounding_box.rows();

        if let Some(scanline_y) = rows.next() {
            let intersections = ScanlineIntersections::new(
                &triangle,
                stroke_width,
                stroke_offset,
                has_fill,
                scanline_y,
            );

            Self {
                rows,
                scanline_y,
                intersections,
            }
        } else {
            Self::empty()
        }
    }

    const fn empty() -> Self {
        Self {
            rows: 0i32..0,
            scanline_y: 0,
            intersections: ScanlineIntersections::empty(),
        }
    }
}

#[maybe_async_cfg::maybe(
    idents(Scanline),
    sync(feature = "draw_target_sync"),
    async(feature = "draw_target_async")
)]
impl Iterator for ScanlineIterator {
    type Item = (Scanline, PointType);

    fn next(&mut self) -> Option<Self::Item> {
        self.intersections.next().or_else(|| {
            self.scanline_y = self.rows.next()?;

            self.intersections.reset_with_new_scanline(self.scanline_y);

            self.intersections.next()
        })
    }
}
