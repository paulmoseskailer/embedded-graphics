//! Prelude

#[doc(no_inline)]
#[cfg(feature = "draw_target_async")]
use crate::draw_target::DrawTargetAsync;
pub use crate::{
    draw_target::DrawTarget,
    geometry::{Angle, AngleUnit, Dimensions, OriginDimensions, Point, Size},
    image::ImageDrawableExt,
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
    idents(DrawTargetExt),
    idents(PixelIteratorExt),
    idents(Drawable),
    sync(feature = "draw_target_sync"),
    async(
        feature = "draw_target_async",
        idents(
            DrawTarget(async = "DrawTargetAsync"),
            ImageDrawable(async = "ImageDrawableAsync")
        )
    )
)]
pub use crate::{
    draw_target::DrawTargetExt, image::ImageDrawable, iterator::PixelIteratorExt, Drawable,
};
