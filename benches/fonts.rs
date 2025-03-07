use criterion::*;
use embedded_graphics::{
    geometry::Point,
    mono_font::{
        ascii::{FONT_10X20, FONT_6X9},
        MonoTextStyle, MonoTextStyleBuilder,
    },
    pixelcolor::Gray8,
    prelude::*,
};
#[maybe_async_cfg::maybe(
    sync(feature = "draw_target_sync"),
    async(
        feature = "draw_target_async",
        idents(Drawable(async = "DrawableAsync"), Text(async = "TextAsync"))
    )
)]
use embedded_graphics::{text::Text, Drawable};

mod common;

use common::Framebuffer;

#[maybe_async_cfg::maybe(
    sync(feature = "draw_target_sync", idents(Text(sync = "Text"))),
    async(feature = "draw_target_async", idents(Text(async = "TextAsync")))
)]
fn one_line<S>(style: S) -> Text<'static, S> {
    Text::new("Hello world!", Point::new_equal(20), style)
}

#[maybe_async_cfg::maybe(
    sync(feature = "draw_target_sync", idents(Text(sync = "Text"))),
    async(feature = "draw_target_async", idents(Text(async = "TextAsync")))
)]
fn three_lines<S>(style: S) -> Text<'static, S> {
    Text::new("line 1\nl2\nThis is line 3", Point::new_equal(20), style)
}

#[maybe_async_cfg::maybe(
    idents(one_line(fn), three_lines(fn)),
    sync(feature = "draw_target_sync"),
    async(feature = "draw_target_async")
)]
fn font_6x9(c: &mut Criterion) {
    let mut group = c.benchmark_group("font 6x9");

    let style = MonoTextStyle::new(&FONT_6X9, Gray8::WHITE);
    let style_with_bg = MonoTextStyleBuilder::new()
        .font(&FONT_6X9)
        .text_color(Gray8::WHITE)
        .background_color(Gray8::BLACK)
        .build();

    group.bench_function("one line", |b| {
        let object = one_line(style);

        let mut framebuffer = Framebuffer::new();
        b.iter(|| object.draw(&mut framebuffer))
    });

    group.bench_function("one line with background", |b| {
        let object = one_line(style_with_bg);

        let mut framebuffer = Framebuffer::new();
        b.iter(|| object.draw(&mut framebuffer))
    });

    group.bench_function("three lines", |b| {
        let object = three_lines(style);

        let mut framebuffer = Framebuffer::new();
        b.iter(|| object.draw(&mut framebuffer))
    });

    group.bench_function("three lines with background)", |b| {
        let object = three_lines(style_with_bg);

        let mut framebuffer = Framebuffer::new();
        b.iter(|| object.draw(&mut framebuffer))
    });

    group.finish();
}

#[maybe_async_cfg::maybe(
    idents(one_line(fn), three_lines(fn)),
    sync(feature = "draw_target_sync"),
    async(feature = "draw_target_async")
)]
fn font_10x20(c: &mut Criterion) {
    let mut group = c.benchmark_group("font 10x20");

    let style = MonoTextStyle::new(&FONT_10X20, Gray8::WHITE);
    let style_with_bg = MonoTextStyleBuilder::new()
        .font(&FONT_10X20)
        .text_color(Gray8::WHITE)
        .background_color(Gray8::BLACK)
        .build();

    group.bench_function("one line", |b| {
        let object = one_line(style);

        let mut framebuffer = Framebuffer::new();
        b.iter(|| object.draw(&mut framebuffer))
    });

    group.bench_function("one line with background", |b| {
        let object = one_line(style_with_bg);

        let mut framebuffer = Framebuffer::new();
        b.iter(|| object.draw(&mut framebuffer))
    });

    group.bench_function("three lines", |b| {
        let object = three_lines(style);

        let mut framebuffer = Framebuffer::new();
        b.iter(|| object.draw(&mut framebuffer))
    });

    group.bench_function("three lines with background)", |b| {
        let object = three_lines(style_with_bg);

        let mut framebuffer = Framebuffer::new();
        b.iter(|| object.draw(&mut framebuffer))
    });

    group.finish();
}

#[cfg(feature = "draw_target_sync")]
criterion_group!(fonts, font_6x9_sync, font_10x20_sync);
#[cfg(feature = "draw_target_async")]
criterion_group!(fonts, font_6x9_async, font_10x20_async);
criterion_main!(fonts);
