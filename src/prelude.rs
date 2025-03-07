//! Prelude

#[doc(no_inline)]
#[cfg(feature = "draw_target_async")]
pub use crate::draw_target::DrawTargetAsync;
pub use crate::{
    draw_target::DrawTarget,
    geometry::{Angle, AngleUnit, Dimensions, OriginDimensions, Point, Size},
    iterator::ContiguousIteratorExt,
    pixelcolor::{
        raw::{RawData, ToBytes as _},
        GrayColor, IntoStorage, PixelColor, RgbColor, WebColors,
    },
    primitives::{ContainsPoint, OffsetOutline, PointsIter, Primitive},
    transform::Transform,
    Pixel,
};
#[maybe_async_cfg::maybe(
    idents(DrawTargetExt, PixelIteratorExt, ImageDrawableExt),
    sync(feature = "draw_target_sync"),
    async(
        feature = "draw_target_async",
        idents(
            Drawable(async = "DrawableAsync"),
            DrawTarget(async = "DrawTargetAsync"),
            ImageDrawable(async = "ImageDrawableAsync")
        )
    )
)]
pub use crate::{
    draw_target::DrawTargetExt, image::ImageDrawable, image::ImageDrawableExt,
    iterator::PixelIteratorExt, Drawable,
};
