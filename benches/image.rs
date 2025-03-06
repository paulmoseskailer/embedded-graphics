use criterion::*;
#[maybe_async_cfg::maybe(
    idents(Drawable),
    sync(feature = "draw_target_sync", idents(Image(sync = "Image"))),
    async(feature = "draw_target_async", idents(Image(async = "ImageAsync")))
)]
use embedded_graphics::{image::Image, Drawable};
use embedded_graphics::{image::ImageRaw, pixelcolor::BinaryColor, prelude::*};

mod common;

use common::Framebuffer;

#[maybe_async_cfg::maybe(
    sync(feature = "draw_target_sync", idents(Image(sync = "Image"))),
    async(feature = "draw_target_async", idents(Image(async = "ImageAsync")))
)]
fn image_1bpp(c: &mut Criterion) {
    c.bench_function("image 4x4px", |b| {
        let bytes = include_bytes!("../assets/patch_1bpp.raw");

        let image: ImageRaw<BinaryColor> = ImageRaw::new(bytes, Size::new(4, 4)).unwrap();
        let object = Image::new(&image, Point::zero());

        let mut framebuffer = Framebuffer::new();
        b.iter(|| object.draw(&mut framebuffer))
    });
}

#[cfg(feature = "draw_target_sync")]
criterion_group!(images, image_1bpp_sync);
#[cfg(feature = "draw_target_async")]
criterion_group!(images, image_1bpp_async);
criterion_main!(images);
