use criterion::*;
use embedded_graphics::{geometry::AnchorPoint, pixelcolor::Gray8, prelude::*, primitives::*};
#[maybe_async_cfg::maybe(
    sync(feature = "draw_target_sync"),
    async(
        feature = "draw_target_async",
        idents(Drawable(async = "DrawableAsync"), Triangle(async = "TriangleAsync"))
    )
)]
use embedded_graphics::{primitives::triangle::Triangle, Drawable};

mod common;

use common::Framebuffer;

const BOUNDING_BOX: Rectangle = Rectangle::new(Point::new_equal(32), Size::new_equal(192));

#[maybe_async_cfg::maybe(
    idents(closed_shape_benches(fn)),
    sync(feature = "draw_target_sync"),
    async(feature = "draw_target_async")
)]
fn rectangle(c: &mut Criterion) {
    closed_shape_benches(c, "rectangle", || BOUNDING_BOX);
}

#[maybe_async_cfg::maybe(
    idents(closed_shape_benches(fn)),
    sync(feature = "draw_target_sync"),
    async(
        feature = "draw_target_async",
        idents(RoundedRectangle(async = "RoundedRectangleAsync"))
    )
)]
fn rounded_rectangle(c: &mut Criterion) {
    closed_shape_benches(c, "rounded rectangle", || {
        RoundedRectangle::new(BOUNDING_BOX, CornerRadii::new(Size::new(10, 12)))
    });
}

#[maybe_async_cfg::maybe(
    idents(closed_shape_benches(fn)),
    sync(feature = "draw_target_sync"),
    async(
        feature = "draw_target_async",
        idents(RoundedRectangle(async = "RoundedRectangleAsync"))
    )
)]
fn rounded_rectangle_corners(c: &mut Criterion) {
    closed_shape_benches(c, "rounded rectangle corners", || {
        RoundedRectangle::new(
            BOUNDING_BOX,
            CornerRadii {
                top_left: Size::new(10, 12),
                top_right: Size::new(14, 16),
                bottom_right: Size::new(18, 20),
                bottom_left: Size::new(22, 24),
            },
        )
    });
}

#[maybe_async_cfg::maybe(
    idents(closed_shape_benches(fn)),
    sync(feature = "draw_target_sync"),
    async(
        feature = "draw_target_async",
        idents(Triangle(async = "TriangleAsync"))
    )
)]
fn triangle(c: &mut Criterion) {
    closed_shape_benches(c, "triangle", || {
        Triangle::new(
            BOUNDING_BOX.anchor_point(AnchorPoint::BottomLeft),
            BOUNDING_BOX.anchor_point(AnchorPoint::TopCenter),
            BOUNDING_BOX.anchor_point(AnchorPoint::BottomRight),
        )
    });
}

#[maybe_async_cfg::maybe(
    idents(closed_shape_benches(fn)),
    sync(feature = "draw_target_sync"),
    async(feature = "draw_target_async", idents(Circle(async = "CircleAsync")))
)]
fn circle(c: &mut Criterion) {
    closed_shape_benches(c, "circle", || {
        Circle::new(BOUNDING_BOX.top_left, BOUNDING_BOX.size.width)
    });
}

#[maybe_async_cfg::maybe(
    idents(closed_shape_benches(fn)),
    sync(feature = "draw_target_sync"),
    async(feature = "draw_target_async", idents(Ellipse(async = "EllipseAsync")))
)]
fn ellipse(c: &mut Criterion) {
    closed_shape_benches(c, "ellipse", || {
        Ellipse::with_center(BOUNDING_BOX.center(), BOUNDING_BOX.size - Size::new(0, 20))
    });
}

#[maybe_async_cfg::maybe(
    idents(closed_shape_benches(fn)),
    sync(feature = "draw_target_sync"),
    async(feature = "draw_target_async")
)]
fn sector_150(c: &mut Criterion) {
    closed_shape_benches(c, "sector 150°", || {
        Sector::with_center(
            BOUNDING_BOX.center(),
            BOUNDING_BOX.size.width,
            -(30.0.deg()),
            150.0.deg(),
        )
    });
}

#[maybe_async_cfg::maybe(
    idents(closed_shape_benches(fn)),
    sync(feature = "draw_target_sync"),
    async(feature = "draw_target_async")
)]
fn sector_360(c: &mut Criterion) {
    closed_shape_benches(c, "sector 360°", || {
        Sector::with_center(
            BOUNDING_BOX.center(),
            BOUNDING_BOX.size.width,
            -(30.0.deg()),
            150.0.deg(),
        )
    });
}

#[maybe_async_cfg::maybe(
    idents(open_shape_benches(fn)),
    sync(feature = "draw_target_sync"),
    async(feature = "draw_target_async")
)]
fn line(c: &mut Criterion) {
    open_shape_benches(c, "line", || {
        Line::new(
            BOUNDING_BOX.anchor_point(AnchorPoint::TopLeft),
            // move point up a bit, because non 45° lines might be slower
            BOUNDING_BOX.anchor_point(AnchorPoint::BottomRight) - Point::new(0, 20),
        )
    });
}

#[maybe_async_cfg::maybe(
    idents(open_shape_benches(fn)),
    sync(feature = "draw_target_sync"),
    async(
        feature = "draw_target_async",
        idents(Polyline(async = "PolylineAsync"))
    )
)]
fn polyline(c: &mut Criterion) {
    let points = [
        BOUNDING_BOX.anchor_point(AnchorPoint::BottomLeft),
        BOUNDING_BOX.anchor_point(AnchorPoint::TopCenter) - Point::new(20, 0),
        BOUNDING_BOX.anchor_point(AnchorPoint::BottomCenter),
        BOUNDING_BOX.anchor_point(AnchorPoint::TopRight),
        BOUNDING_BOX.anchor_point(AnchorPoint::BottomRight),
    ];

    open_shape_benches(c, "polyline", || Polyline::new(&points));
}

#[maybe_async_cfg::maybe(
    idents(open_shape_benches(fn)),
    sync(feature = "draw_target_sync"),
    async(feature = "draw_target_async", idents(Arc(async = "ArcAsync")))
)]
fn arc_150(c: &mut Criterion) {
    open_shape_benches(c, "arc 150°", || {
        Arc::with_center(
            BOUNDING_BOX.center(),
            BOUNDING_BOX.size.width,
            -(30.0.deg()),
            150.0.deg(),
        )
    });
}

#[maybe_async_cfg::maybe(
    idents(open_shape_benches(fn)),
    sync(feature = "draw_target_sync"),
    async(feature = "draw_target_async", idents(Arc(async = "ArcAsync")))
)]
fn arc_360(c: &mut Criterion) {
    open_shape_benches(c, "arc 360°", || {
        Arc::with_center(
            BOUNDING_BOX.center(),
            BOUNDING_BOX.size.width,
            0.0.deg(),
            360.0.deg(),
        )
    });
}

#[cfg(feature = "draw_target_sync")]
criterion_group!(
    primitives,
    rectangle_sync,
    rounded_rectangle_sync,
    rounded_rectangle_corners_sync,
    triangle_sync,
    circle_sync,
    ellipse_sync,
    line_sync,
    polyline_sync,
    sector_150_sync,
    sector_360_sync,
    arc_150_sync,
    arc_360_sync,
);
#[cfg(feature = "draw_target_async")]
criterion_group!(
    primitives,
    rectangle_async,
    rounded_rectangle_async,
    rounded_rectangle_corners_async,
    triangle_async,
    circle_async,
    ellipse_async,
    line_async,
    polyline_async,
    sector_150_async,
    sector_360_async,
    arc_150_async,
    arc_360_async,
);
criterion_main!(primitives);

#[cfg(feature = "draw_target_sync")]
#[maybe_async_cfg::maybe(
    sync(feature = "draw_target_sync"),
    async(
        feature = "draw_target_async",
        idents(Drawable(async = "DrawableAsync"))
    )
)]
fn closed_shape_benches<P>(c: &mut Criterion, name: &str, build: impl Fn() -> P)
where
    P: Primitive,
    Styled<P, PrimitiveStyle<Gray8>>: Drawable<Color = Gray8>,
{
    let mut group = c.benchmark_group(name);

    group.bench_function("fill", |b| {
        let style = PrimitiveStyle::with_fill(Gray8::WHITE);
        let object = build().into_styled(style);

        let mut framebuffer = Framebuffer::new();
        b.iter(|| object.draw(&mut framebuffer))
    });

    group.bench_function("stroke 1px", |b| {
        let style = PrimitiveStyleBuilder::new()
            .stroke_color(Gray8::BLACK)
            .stroke_width(1)
            .stroke_alignment(StrokeAlignment::Inside)
            .build();
        let object = build().into_styled(style);

        let mut framebuffer = Framebuffer::new();
        b.iter(|| object.draw(&mut framebuffer))
    });

    group.bench_function("stroke 10px", |b| {
        let style = PrimitiveStyleBuilder::new()
            .stroke_color(Gray8::BLACK)
            .stroke_width(10)
            .stroke_alignment(StrokeAlignment::Inside)
            .build();
        let object = build().into_styled(style);

        let mut framebuffer = Framebuffer::new();
        b.iter(|| object.draw(&mut framebuffer))
    });

    group.bench_function("stroke 1px and fill", |b| {
        let style = PrimitiveStyleBuilder::new()
            .fill_color(Gray8::WHITE)
            .stroke_color(Gray8::BLACK)
            .stroke_width(1)
            .stroke_alignment(StrokeAlignment::Inside)
            .build();
        let object = build().into_styled(style);

        let mut framebuffer = Framebuffer::new();
        b.iter(|| object.draw(&mut framebuffer))
    });

    group.bench_function("stroke 10px and fill", |b| {
        let style = PrimitiveStyleBuilder::new()
            .fill_color(Gray8::WHITE)
            .stroke_color(Gray8::BLACK)
            .stroke_width(10)
            .stroke_alignment(StrokeAlignment::Inside)
            .build();
        let object = build().into_styled(style);

        let mut framebuffer = Framebuffer::new();
        b.iter(|| object.draw(&mut framebuffer))
    });

    group.finish()
}

#[cfg(feature = "draw_target_sync")]
#[maybe_async_cfg::maybe(
    sync(feature = "draw_target_sync"),
    async(
        feature = "draw_target_async",
        idents(Drawable(async = "DrawableAsync"))
    )
)]
fn open_shape_benches<P>(c: &mut Criterion, name: &str, build: impl Fn() -> P)
where
    P: Primitive,
    Styled<P, PrimitiveStyle<Gray8>>: Drawable<Color = Gray8>,
{
    let mut group = c.benchmark_group(name);

    group.bench_function("stroke 1px", |b| {
        let style = PrimitiveStyle::with_stroke(Gray8::WHITE, 1);
        let object = build().into_styled(style);

        let mut framebuffer = Framebuffer::new();
        b.iter(|| object.draw(&mut framebuffer))
    });

    group.bench_function("stroke 10px", |b| {
        let style = PrimitiveStyle::with_stroke(Gray8::WHITE, 10);
        let object = build().into_styled(style);

        let mut framebuffer = Framebuffer::new();
        b.iter(|| object.draw(&mut framebuffer))
    });

    group.bench_function("stroke 50px", |b| {
        let style = PrimitiveStyle::with_stroke(Gray8::WHITE, 50);
        let object = build().into_styled(style);

        let mut framebuffer = Framebuffer::new();
        b.iter(|| object.draw(&mut framebuffer))
    });

    group.finish()
}
