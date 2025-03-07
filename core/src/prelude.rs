//! Prelude
#[maybe_async_cfg::maybe(
    sync(feature = "draw_target_sync"),
    async(
        feature = "draw_target_async",
        idents(
            DrawTarget(async = "DrawTargetAsync"),
            Drawable(async = "DrawableAsync"),
            ImageDrawable(async = "ImageDrawableAsync")
        )
    )
)]
pub use crate::{draw_target::DrawTarget, drawable::Drawable, image::ImageDrawable};
#[doc(no_inline)]
pub use crate::{
    drawable::Pixel,
    geometry::{Dimensions, OriginDimensions, Point, Size},
    pixelcolor::{
        raw::{RawData, ToBytes as _},
        GrayColor, IntoStorage, PixelColor, RgbColor, WebColors,
    },
    primitives::PointsIter,
};
