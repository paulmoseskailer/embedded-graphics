//! Prelude
#[cfg(feature = "async_draw_target")]
pub use crate::draw_target::DrawTargetAsync;
#[doc(no_inline)]
pub use crate::{
    draw_target::DrawTarget,
    drawable::Pixel,
    geometry::{Dimensions, OriginDimensions, Point, Size},
    pixelcolor::{
        raw::{RawData, ToBytes as _},
        GrayColor, IntoStorage, PixelColor, RgbColor, WebColors,
    },
    primitives::PointsIter,
};
#[maybe_async_cfg::maybe(
    idents(Drawable, ImageDrawable),
    sync(feature = "draw_target_sync"),
    async(feature = "draw_target_async")
)]
pub use crate::{drawable::Drawable, image::ImageDrawable};
