use crate::drawcontext::*;
use crate::float::*;
use crate::mat2::*;
use crate::point::*;

pub struct Line {
    pub a: point,
    pub e: point,
}

impl Line {
    pub fn new(p1: &point, p2: &point) -> Line {
        if p1.b.x < p2.b.x {
            Line { a: *p1, e: *p2 }
        } else {
            Line { a: *p2, e: *p1 }
        }
    }

    pub fn draw(&self, ctx: &mut DrawContext, c: Color) {
        ctx.line(self.a.b.x, self.a.b.y, self.e.b.x, self.e.b.y, c);
    }
}

pub struct intersectresult {
    pub matched: u8,
    pub lambda: Float,
    pub mue: Float,
}

pub fn intersect(l1: &Line, l2: &Line) -> intersectresult {
    let k = Matrix2::new(l1.e.b.sub2d(&l1.a.b), l2.a.b.sub2d(&l2.e.b));
    let dk = k.det2d();

    let mut r = intersectresult {
        lambda: 0.0,
        matched: 0,
        mue: 0.0,
    };
    if dk.abs() <= epsilon1 {
        // schneiden sich nicht
        r.matched = 0;
        return r;
    }

    let h = l2.a.b.sub2d(&l1.a.b);

    r.lambda = k.withX(h).det2d() / dk;
    r.mue = k.withY(h).det2d() / dk;
    if r.lambda > epsilon1
        && r.lambda < 1.0 - epsilon1
        && r.mue > epsilon1
        && r.mue < 1.0 - epsilon1
    {
        // schneiden sich ordentlich
        r.matched = 1;
    } else if r.lambda > epsilon1
        && r.lambda < 1.0 - epsilon1
        && ((r.mue - 1.0).abs() <= epsilon1 || r.mue.abs() <= epsilon1)
    {
        // min 1 endpunkt2 auf linie 1
        r.matched = 2;
    } else if ((r.lambda - 1.0).abs() <= epsilon1 || r.lambda.abs() <= epsilon1)
        && r.mue > epsilon1
        && r.mue < 1.0 - epsilon1
    {
        // min 1 endpunkt1 auf linie 2
        r.matched = 3;
    } else if ((r.lambda - 1.0).abs() <= epsilon1 || r.lambda.abs() <= epsilon1)
        && ((r.mue - 1.0).abs() <= epsilon1 || r.mue.abs() <= epsilon1)
    {
        // 1 gemeinsamer endpunkt
        r.matched = 4;
    } else {
        // schneiden sich nicht
        r.matched = 0;
    }
    r
}
