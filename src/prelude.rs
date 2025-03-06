//! Prelude

#[maybe_async_cfg::maybe(
    idents(DrawTargetExt),
    sync(feature = "draw_target_sync"),
    async(
        feature = "draw_target_async",
        idents(DrawTarget(async = "DrawTargetAsync"))
    )
)]
use crate::draw_target::{DrawTarget, DrawTargetExt};
#[maybe_async_cfg::maybe(
    idents(ImageDrawable),
    sync(feature = "draw_target_sync"),
    async(feature = "draw_target_async")
)]
use crate::image::ImageDrawable;
#[maybe_async_cfg::maybe(
    idents(PixelIteratorExt),
    sync(feature = "draw_target_sync"),
    async(feature = "draw_target_async")
)]
use crate::iterator::PixelIteratorExt;
#[maybe_async_cfg::maybe(
    idents(Drawable),
    sync(feature = "draw_target_sync"),
    async(feature = "draw_target_async")
)]
use crate::Drawable;
#[doc(no_inline)]
pub use crate::{
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
