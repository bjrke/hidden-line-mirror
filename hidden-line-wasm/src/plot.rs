use crate::dreidext::*;
use crate::float::*;

use crate::vec3::*;

pub fn init_scene<F: Fn(Float, Float) -> Float>(f: F) -> Scene3 {
    let ad = 1.0;
    let sw = 1.0 / 50.0;
    let mut xx = -ad;

    let mut scene_builder = SceneBuilder::new();
    while xx < ad {
        let mut qs = QuadStrip::init(
            scene_builder,
            xx,
            -ad,
            f(xx, -ad),
            xx + sw,
            -ad,
            f(xx + sw, -ad),
        );

        let mut yy = -ad + sw;
        while yy <= ad {
            qs.add(
                Vector3(xx, yy, f(xx, yy)),
                Vector3(xx + sw, yy, f(xx + sw, yy)),
            );
            yy += sw;
        }
        xx += sw;
        scene_builder = qs.build();
    }

    scene_builder.scene3
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::appcontext::AppContext;
    use crate::drawcontext::*;
    use crate::vec2::Vector2;

    struct TestDrawContext {
        lines: Vec<(Vector2, Vector2, Color)>,
    }

    struct TestColorContext {
        lines: Vec<(Vector2, Vector2)>,
        color: Color,
    }

    impl ColorContext for TestColorContext {
        fn line(&mut self, p1: Vector2, p2: Vector2) {
            self.lines.push((p1, p2));
        }
    }

    impl DrawContext<TestColorContext> for TestDrawContext {
        fn draw(&mut self, _frame: Frame, color_ctx: TestColorContext) {
            for (a, e) in color_ctx.lines {
                self.lines.push((a, e, color_ctx.color));
            }
        }

        fn color_context(&mut self, color: u8) -> TestColorContext {
            TestColorContext {
                lines: vec![],
                color,
            }
        }
    }

    impl TestDrawContext {
        fn new() -> TestDrawContext {
            TestDrawContext { lines: vec![] }
        }
    }
    #[test]
    fn integration() {
        let mut actx = AppContext::new();

        actx.scene3 = init_scene(|x, y| {
            let mut h = 0.0;
            let step = PI / 36.0;
            let mut a = 0.0;
            while a < PI {
                let c = a.cos();
                let s = a.sin();
                let xd = x * c - y * s;
                let yd = x * s + y * c;
                h += (a * (xd * xd * 25.0 + yd * yd * 100.0).sqrt()).cos() / (PI + a);
                a += step;
            }
            h / 10.0
        });

        let mut dctx = TestDrawContext::new();
        actx.render(&mut dctx);

        assert!(!dctx.lines.is_empty());
        for &(a, e, _color) in dctx.lines.iter() {
            assert!(a.0.is_finite() && a.1.is_finite());
            assert!(e.0.is_finite() && e.1.is_finite());
        }
    }

    #[test]
    fn flat_plane_renders_lines() {
        let mut actx = AppContext::new();
        actx.scene3 = init_scene(|_x, _y| 0.0);

        let mut dctx = TestDrawContext::new();
        actx.render(&mut dctx);

        assert!(!dctx.lines.is_empty());
        for &(a, e, _color) in dctx.lines.iter() {
            assert!(a.0.is_finite() && a.1.is_finite());
            assert!(e.0.is_finite() && e.1.is_finite());
        }
    }
}
