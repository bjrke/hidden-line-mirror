use crate::dreidext::*;
use crate::float::*;

use crate::vec3::*;

pub fn init_scene<F: Fn(Float, Float) -> Float>(f: F) -> SceneBuilder {
    let ad = 1.0;
    let sw = 0.04;
    let mut xx = -ad;

    let mut scene = SceneBuilder::new();
    while xx < ad {
        let mut qs = QuadStrip::init(scene, xx, -ad, f(xx, -ad), xx + sw, -ad, f(xx + sw, -ad));

        let mut yy = -ad + sw;
        while yy <= ad {
            qs.add(
                Vector3(xx, yy, f(xx, yy)),
                Vector3(xx + sw, yy, f(xx + sw, yy)),
            );
            yy = yy + sw;
        }
        xx = xx + sw;
        scene = qs.build();
    }

    scene
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::appcontext::AppContext;
    use crate::drawcontext::*;

    struct TestDrawContext {
        lines: Vec<(Float, Float, Float, Float, Color)>,
    }

    impl DrawContext for TestDrawContext {
        fn circle(&mut self, _x: Float, _y: Float, _r: Float, _c: Color) {}

        fn line(&mut self, xa: Float, ya: Float, xe: Float, ye: Float, c: Color) {
            self.lines.push((xa, ya, xe, ye, c));
        }

        fn cls(&mut self) {}
    }

    impl TestDrawContext {
        fn new() -> TestDrawContext {
            TestDrawContext { lines: vec![] }
        }
    }
    #[test]
    fn integration() {
        let mut actx = AppContext::new();

        actx.scene_builder = init_scene(|x, y| {
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
        actx.back_face = false;

        let mut dctx = TestDrawContext::new();
        actx.render(&mut dctx);

        println!("{:?}", dctx.lines);
        println!("{:?}", dctx.lines.len());
    }
}
