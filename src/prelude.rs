//! Prelude

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
    draw_target::{DrawTarget, DrawTargetExt},
    image::ImageDrawable,
    image::ImageDrawableExt,
    iterator::PixelIteratorExt,
    Drawable,
};
#[doc(no_inline)]
pub use crate::{
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
