use core::ops::Range;

#[maybe_async_cfg::maybe(
    idents(PixelIteratorExt, Scanline),
    sync(feature = "draw_target_sync"),
    async(
        feature = "draw_target_async",
        idents(DrawTarget(async = "DrawTargetAsync"))
    )
)]
use crate::{draw_target::DrawTarget, iterator::PixelIteratorExt, primitives::common::Scanline};

/// Scanline with stroke and fill regions.
#[maybe_async_cfg::maybe(
    sync(feature = "draw_target_sync"),
    async(feature = "draw_target_async")
)]
#[derive(Clone, Eq, PartialEq, Hash, Debug)]
#[cfg_attr(feature = "defmt", derive(::defmt::Format))]
pub struct StyledScanline {
    y: i32,
    stroke_range: Range<i32>,
    fill_range: Range<i32>,
}

#[maybe_async_cfg::maybe(
    idents(Scanline),
    sync(feature = "draw_target_sync"),
    async(
        feature = "draw_target_async",
        idents(DrawTarget(async = "DrawTargetAsync"))
    )
)]
impl StyledScanline {
    /// Creates a new styled scanline.
    pub fn new(y: i32, stroke_range: Range<i32>, fill_range: Option<Range<i32>>) -> Self {
        let fill_range = fill_range.unwrap_or_else(|| stroke_range.end..stroke_range.end);

        Self {
            y,
            stroke_range,
            fill_range,
        }
    }

    /// Returns the stroke region on the left side.
    ///
    /// If the scanline contains no fill region the entire scanline will be returned.
    pub const fn stroke_left(&self) -> Scanline {
        Scanline::new(self.y, self.stroke_range.start..self.fill_range.start)
    }

    /// Returns the stroke region on the right side.
    ///
    /// If the scanline contains no fill region an empty scanline will be returned.
    pub const fn stroke_right(&self) -> Scanline {
        Scanline::new(self.y, self.fill_range.end..self.stroke_range.end)
    }

    /// Returns the fill region.
    pub fn fill(&self) -> Scanline {
        Scanline::new(self.y, self.fill_range.clone())
    }

    /// Draws the stroke regions.
    pub async fn draw_stroke<T: DrawTarget>(
        &self,
        target: &mut T,
        stroke_color: T::Color,
    ) -> Result<(), T::Error> {
        self.stroke_left().draw(target, stroke_color).await?;
        self.stroke_right().draw(target, stroke_color).await
    }

    /// Draws the stroke and fill regions.
    pub async fn draw_stroke_and_fill<T: DrawTarget>(
        &self,
        target: &mut T,
        stroke_color: T::Color,
        fill_color: T::Color,
    ) -> Result<(), T::Error> {
        self.stroke_left().draw(target, stroke_color).await?;
        self.fill().draw(target, fill_color).await?;
        self.stroke_right().draw(target, stroke_color).await
    }
}
