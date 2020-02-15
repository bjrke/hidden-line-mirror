use crate::appcontext::*;
use crate::drawcontext::*;
use crate::float::*;
use crate::line::*;
use crate::mat3::*;
use crate::point::*;
use crate::vec2::*;
use crate::vec3::*;
use std::rc::Rc;

pub struct dreiecktyp {
    pub p1: point,
    pub p2: point,
    pub p3: point,
    pub l1: Line,
    pub l2: Line,
    pub l3: Line,
    pub gl: u8,
    pub cols: Color,
}

impl dreiecktyp {
    pub fn l(&self, num: u8) -> &Line {
        match num {
            1 => &self.l1,
            2 => &self.l2,
            3 => &self.l3,
            _ => panic!("no line number {}", num),
        }
    }

    pub fn p(&self, num: u8) -> &point {
        match num {
            1 => &self.p1,
            2 => &self.p2,
            3 => &self.p3,
            _ => panic!("no line number {}", num),
        }
    }

    pub fn draw1(&self, ctx: &mut dyn DrawContext, c: Color) {
        if self.gl & 1 != 0 {
            self.l1.draw(ctx, c);
        }
        if self.gl & 2 != 0 {
            self.l2.draw(ctx, c);
        }
        if self.gl & 4 != 0 {
            self.l3.draw(ctx, c);
        }
    }

    pub fn draw2(&self, ctx: &mut dyn DrawContext, c: Color) {
        ctx.poly(
            &[
                self.p1.b.x,
                -self.p1.b.y,
                self.p2.b.x,
                -self.p2.b.y,
                self.p3.b.x,
                -self.p3.b.y,
            ],
            c,
        );
    }

    pub fn draw3(&self, ctx: &mut dyn DrawContext, c: Color) {
        self.l1.draw(ctx, c);
        self.l2.draw(ctx, c);
        self.l3.draw(ctx, c);
    }

    pub fn linientest(&self, li: &Line, ausgabe: bool) -> bool {
        let pa = self.punkttest(&li.a.b, "linientest1", ausgabe);
        if pa == 0 {
            return true;
        }

        let pe = self.punkttest(&li.e.b, "linientest2", ausgabe);
        if pe == 0 {
            return true;
        }

        if pa < 20 && pe < 20 {
            return self.punkttest(&li.a.b.add2d(&li.e.b).div2d(2.0), "linientest3", ausgabe) == 0;
        }

        let li1 = intersect(li, &self.l1).matched;
        if li1 == 1 {
            return true;
        }

        let li2 = intersect(li, &self.l2).matched;
        if li2 == 1 || li1 == 2 && li2 == 2 {
            return true;
        }

        let li3 = intersect(li, &self.l3).matched;
        li3 == 1 || li3 == 2 && (li2 == 2 || li1 == 2)
    }

    pub fn punkttest(&self, t: &Vector2, caller: &str, ausgabe: bool) -> u8 {
        let b1 = Vector3::new(t.x, t.y, 1.0);

        let mut K = Matrix3 {
            x: Vector3::new(self.p1.b.x, self.p1.b.y, 1.0),
            y: Vector3::new(self.p2.b.x, self.p2.b.y, 1.0),
            z: b1,
        };

        let kd = K.det3d();

        if kd.abs() < epsilon1 {
            println!("nullerdiv {}", K);
            return 0;
        }

        fn testl(l: Float) -> u8 {
            if l.abs() < epsilon0 {
                0 //=0
            } else if (l - 1.0).abs() < epsilon0 {
                1 //=1
            } else if l < 0.0 {
                2
            } else if l > 1.0 {
                3
            } else {
                4 // 0<l<1
            }
        }

        let h = K.x;
        K.x = b1;
        let la1 = K.det3d() / kd;
        K.x = h;
        let l1 = testl(la1);
        let h = K.y;
        K.y = b1;
        let la2 = K.det3d() / kd;
        K.y = h;

        let l2 = testl(la2);
        let h = K.z;
        K.z = b1;
        let la3 = K.det3d() / kd;
        K.z = h;
        let l3 = testl(la3);

        if ausgabe {
            println!("punkttest {} {} {} {}", caller, la1, la2, la3);
        }

        match l1 * 25 + l2 * 5 + l3 {
            124 => 0, // drin
            24 => 1,  // kanten
            104 => 2,
            120 => 3,
            // eckpunkte
            25 => 11,
            5 => 12,
            1 => 13,
            _ => 20, // draußen
        }
    }
    pub fn flaechentest(&self) -> bool {
        colinear(&self.p1.b, &self.p2.b, &self.p3.b)
    }
}

pub struct dreieck {
    pub delegate: dreiecktyp,
    pub origPoint1: Rc<punkt3d>,
    pub origPoint2: Rc<punkt3d>,
    pub origPoint3: Rc<punkt3d>,
    pub planeNorm: Vector3,
    pub planeDist: Float,
}

impl dreieck {
    pub fn new(p1: Rc<punkt3d>, p2: Rc<punkt3d>, p3: Rc<punkt3d>, ls: u8) -> dreieck {
        let planeNorm = p2.o.sub3d(&p1.o).kreuz(&p3.o.sub3d(&p1.o)).normalize();

        let dp1 = p1.b;
        let dp2 = p2.b;
        let dp3 = p3.b;

        let planeDist = planeNorm.skalar(&p1.o);
        dreieck {
            origPoint1: p1,
            origPoint2: p2,
            origPoint3: p3,
            delegate: dreiecktyp {
                p1: dp1,
                p2: dp2,
                p3: dp3,
                gl: ls,
                l1: Line::new(&dp2, &dp3),
                l2: Line::new(&dp3, &dp1),
                l3: Line::new(&dp1, &dp2),
                cols: 0,
            },
            planeNorm,
            planeDist,
        }
    }

    pub fn tiefe(&self, ctx: &AppContext, k: &Vector2) -> Float {
        let bv = ctx
            .BlickR
            .add3d(&ctx.iv.mul3d(k.x))
            .add3d(&ctx.jv.mul3d(k.y));
        let t = self.planeNorm.skalar(&bv);
        if (t.abs() < epsilon2) {
            100000000.0
        } else {
            (self.planeDist - self.planeNorm.skalar(&ctx.Auge)) / (t * bv.invBetrag3d())
        }
    }
}

pub fn calcColor(f: Float) -> Color {
    if f <= 0.0 {
        1
    } else if f >= 1.0 {
        15
    } else {
        (f * 14.0).round() as Color
    }
}
