//! Prelude
#[cfg(feature = "draw_target_async")]
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
    sync(feature = "draw_target_sync"),
    async(
        feature = "draw_target_async",
        idents(
            Drawable(async = "DrawableAsync"),
            ImageDrawable(async = "ImageDrawableAsync")
        )
    )
)]
pub use crate::{drawable::Drawable, image::ImageDrawable};
