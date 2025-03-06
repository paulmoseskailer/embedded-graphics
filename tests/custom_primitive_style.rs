#[maybe_async_cfg::maybe(
    idents(StyledDrawable),
    sync(feature = "draw_target_sync"),
    async(
        feature = "draw_target_async",
        idents(
            Drawable(async = "DrawableAsync"),
            DrawTarget(async = "DrawTargetAsync")
        )
    )
)]
use embedded_graphics::{draw_target::DrawTarget, primitives::StyledDrawable, Drawable};
use embedded_graphics::{
    mock_display::MockDisplay, pixelcolor::Rgb888, prelude::*, primitives::Rectangle,
};

struct CheckerboardStyle<C>(C, C);

#[maybe_async_cfg::maybe(
    keep_self,
    idents(StyledDrawable),
    sync(feature = "draw_target_sync"),
    async(
        feature = "draw_target_async",
        idents(DrawTarget(async = "DrawTargetAsync"))
    )
)]
impl<C: PixelColor> StyledDrawable<CheckerboardStyle<C>> for Rectangle {
    type Color = C;
    type Output = ();

    fn draw_styled<D>(
        &self,
        style: &CheckerboardStyle<C>,
        target: &mut D,
    ) -> Result<Self::Output, D::Error>
    where
        D: DrawTarget<Color = Self::Color>,
    {
        target.fill_contiguous(
            self,
            self.points().map(|p| {
                if (p.x % 2 == 0) ^ (p.y % 2 == 0) {
                    style.1
                } else {
                    style.0
                }
            }),
        )
    }
}

#[test]
fn custom_primitive_style() {
    let style = CheckerboardStyle(Rgb888::RED, Rgb888::GREEN);

    let mut display = MockDisplay::new();
    Rectangle::new(Point::zero(), Size::new(4, 3))
        .into_styled(style)
        .draw(&mut display)
        .unwrap();
    display.assert_pattern(&[
        "RGRG", //
        "GRGR", //
        "RGRG", //
    ]);
}
