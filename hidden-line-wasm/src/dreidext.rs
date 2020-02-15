use crate::appcontext::*;
use crate::float::*;
use crate::point::*;
use crate::triangle::*;
use crate::vec2::*;
use crate::vec3::*;

#[derive(Clone, Copy)]
pub struct Triangle {
    pub p1: usize,

    pub p2: usize,

    pub p3: usize,

    pub lset: u8,
}

pub struct SceneBuilder {
    pub points: Vec<Vector3>,

    pub triangles: Vec<Triangle>,
}

impl SceneBuilder {
    pub fn new() -> SceneBuilder {
        SceneBuilder {
            points: Vec::new(),
            triangles: Vec::new(),
        }
    }

    pub fn point(&mut self, x: Float, y: Float, z: Float) -> usize {
        self.push(Vector3::new(x, y, z))
    }

    pub fn push(&mut self, p: Vector3) -> usize {
        self.points.push(p);
        self.points.len() - 1
    }

    pub fn triangle(&mut self, p1: usize, p2: usize, p3: usize, lset: u8) -> usize {
        self.triangles.push(Triangle { p1, p2, p3, lset });
        self.triangles.len() - 1
    }

    pub fn quad(&mut self, p1: usize, p2: usize, p3: usize, p4: usize) {
        self.triangle(p1, p2, p3, 5);
        self.triangle(p1, p3, p4, 3);
        // self.add(p1, p2, p3, 5);
        // self.add(p1, p3, p4, 3);
    }
}

struct TriFan {
    Center: usize,
    Last: usize,
    First: usize,
    scene: SceneBuilder,
}

impl TriFan {
    pub fn Init(
        mut scene: SceneBuilder,
        cx: Float,
        cy: Float,
        cz: Float,
        ax: Float,
        ay: Float,
        az: Float,
        bx: Float,
        by: Float,
        bz: Float,
    ) -> TriFan {
        let Center = scene.point(cx, cy, cz);
        let Last = scene.point(ax, ay, az);
        let First = scene.point(bx, by, bz);
        scene.triangle(Center, First, Last, 7);
        TriFan {
            scene,
            Center,
            Last,
            First,
        }
    }

    pub fn add(&mut self, ax: Float, ay: Float, az: Float) -> usize {
        let help = self.scene.point(ax, ay, az);
        self.scene.triangle(self.Center, self.Last, help, 7);
        self.Last = help;
        self.Last
    }

    pub fn done(mut self) -> SceneBuilder {
        self.scene.triangle(self.Center, self.Last, self.First, 7);
        self.scene
    }
}

pub struct TriStrip {
    l1: usize,
    l2: usize,
    w: bool,
    scene: SceneBuilder,
}

impl TriStrip {
    pub fn Init(
        mut scene: SceneBuilder,
        cx: Float,
        cy: Float,
        cz: Float,
        ax: Float,
        ay: Float,
        az: Float,
        bx: Float,
        by: Float,
        bz: Float,
    ) -> TriStrip {
        let l1 = scene.point(ax, ay, az);
        let l2 = scene.point(bx, by, bz);
        let c = scene.point(cx, cy, cz);
        scene.triangle(c, l1, l2, 7);
        TriStrip {
            l1,
            l2,
            w: true,
            scene,
        }
    }

    pub fn add(&mut self, ax: Float, ay: Float, az: Float) -> usize {
        let help = self.scene.point(ax, ay, az);
        if self.w {
            self.scene.triangle(self.l1, help, self.l2, 7);
        } else {
            self.scene.triangle(self.l1, self.l2, help, 7);
        }
        self.w = !self.w;

        self.l1 = self.l2;
        self.l2 = help;
        self.l2
    }

    pub fn build(self) -> SceneBuilder {
        self.scene
    }
}

pub struct QuadStrip {
    l1: usize,
    l2: usize,
    scene: SceneBuilder,
}

impl QuadStrip {
    pub fn init(
        scene: SceneBuilder,
        ax: Float,
        ay: Float,
        az: Float,
        bx: Float,
        by: Float,
        bz: Float,
    ) -> QuadStrip {
        QuadStrip::new(scene, Vector3::new(ax, ay, az), Vector3::new(bx, by, bz))
    }

    pub fn new(mut scene: SceneBuilder, a: Vector3, b: Vector3) -> QuadStrip {
        let l2 = scene.push(a);
        let l1 = scene.push(b);
        QuadStrip { l1, l2, scene }
    }

    pub fn add(&mut self, bx: Float, by: Float, bz: Float, ax: Float, ay: Float, az: Float) {
        self.addV(Vector3::new(bx, by, bz), Vector3::new(ax, ay, az));
    }
    pub fn addV(&mut self, b: Vector3, a: Vector3) {
        let h1 = self.scene.push(a);
        let h2 = self.scene.push(b);

        self.scene.quad(self.l2, self.l1, h1, h2);

        self.l1 = h1;
        self.l2 = h2;
    }

    pub fn build(self) -> SceneBuilder {
        self.scene
    }
}

// procedure tetraeder(ax, ay, az, bx, by, bz, cx, cy, cz, dx, dy, dz: float);
// procedure kugel(mx, my, mz, r: float; l, b: int);
// procedure kegel(mx, my, mz, r1x, r1y, r1z, r2x, r2y, r2z, hx, hy, hz: float; b: int);
// procedure cube(ex, ey, ez, ax, ay, az, bx, by, bz, cx, cy, cz: float);
// procedure triangle(ax, ay, az, bx, by, bz, cx, cy, cz: float);

// procedure tetraeder;
// var
//   h: ^triStrip;
// begin
//   new(h, init(ax, ay, az, cx, cy, cz, bx, by, bz));
//   h^.add(dx, dy, dz);
//   h^.add(ax, ay, az);
//   h^.add(cx, cy, cz);
//   dispose(h, done);
// end;

// procedure kugel;
// var
//   i, j: int;
//   wl, wb: float;
//   npol, spol: ^trifan;
//   land: ^quadstrip;
// begin
//   if l < 3 then
//     l := 3;
//   if b < 3 then
//     b := 3;
//   wl := 2 * pi / l;
//   wb := pi / b;

//   for i := 1 to l do
//   begin
//     if i = 1 then
//     begin
//       new(npol, init(mx, my, mz + r, mx + r * sin(wb), my, mz + r *
//         cos(wb), mx + r * sin(wb) * cos(wl), my + r * sin(wb) * sin(wl),
//         mz + r * cos(wb)));
//       new(spol, init(mx, my, mz - r, mx + r * sin(wb), my, mz - r *
//         cos(wb), mx + r * sin(wb) * cos(wl), my - r * sin(wb) * sin(wl),
//         mz - r * cos(wb)));
//     end
//     else
//     begin
//       npol^.add(mx + r * sin(wb) * cos(i * wl), my + r * sin(wb) *
//         sin(i * wl), mz + r * cos(wb));
//       spol^.add(mx + r * sin(wb) * cos(i * wl), my - r * sin(wb) *
//         sin(i * wl), mz - r * cos(wb));
//     end;
//     new(land, init(mx + r * sin(wb) * cos(i * wl), my + r * sin(wb) *
//       sin(i * wl), mz + r * cos(wb), mx + r * sin(wb) * cos((i - 1) * wl),
//       my + r * sin(wb) * sin((i - 1) * wl), mz + r * cos(wb), mx +
//       r * sin(2 * wb) * cos(i * wl), my + r * sin(2 * wb) * sin(i * wl),
//       mz + r * cos(2 * wb), mx + r * sin(2 * wb) * cos((i - 1) * wl),
//       my + r * sin(2 * wb) * sin((i - 1) * wl), mz + r * cos(2 * wb)));
//     for j := 3 to b - 1 do
//     begin
//       land^.add(
//         mx + r * sin(j * wb) * cos(i * wl), my + r * sin(j * wb) *
//         sin(i * wl), mz + r * cos(j * wb),
//         mx + r * sin(j * wb) * cos((i - 1) * wl), my + r * sin(j * wb) *
//         sin((i - 1) * wl), mz + r * cos(j * wb));
//     end;
//     dispose(land, done);
//   end;
//   dispose(npol, done);
//   dispose(spol, done);
// end;

// procedure kegel;
// var
//   tr, th: ^trifan;
//   i: int;
//   w, c, s: float;
// begin
//   w := 2 * pi / b;
//   for i := 1 to b do
//   begin
//     c := cos(w * i);
//     s := sin(w * i);
//     if i = 1 then
//     begin
//       new(tr, init(mx, my, mz, mx - r1x, my - r1y, mz - r1z, mx -
//         r1x * c + r2x * s, my - r1y * c + r2y * s, mz - r1z * c + r2z * s));
//       new(th, init(mx + hx, my + hy, mz + hz, mx + r1x, my + r1y, mz +
//         r1z, mx + r1x * c + r2x * s, my + r1y * c + r2y * s, mz + r1z * c + r2z * s));
//     end
//     else
//     begin
//       tr^.add(mx - r1x * c + r2x * s, my - r1y * c + r2y * s, mz - r1z * c + r2z * s);
//       th^.add(mx + r1x * c + r2x * s, my + r1y * c + r2y * s, mz + r1z * c + r2z * s);
//     end;
//   end;
//   dispose(tr, done);
//   dispose(th, done);
// end;

// procedure cube;
// var
//   q: ^quadstrip;
// begin
//   new(q, init(ex, ey, ez, ex + bx, ey + by, ez + bz, ex + ax, ey +
//     ay, ez + az, ex + ax + bx, ey + ay + by, ez + az + bz));
//   q^.add(ex + ax + cx, ey + ay + cy, ez + az + cz, ex + ax + bx + cx,
//     ey + ay + by + cy, ez + az + bz + cz);
//   q^.add(ex + cx, ey + cy, ez + cz, ex + bx + cx, ey + by + cy, ez + bz + cz);
//   dispose(q, done);
//   new(q, init(ex + ax, ey + ay, ez + az, ex + ax + cx, ey + ay + cy,
//     ez + az + cz, ex, ey, ez, ex + cx, ey + cy, ez + cz));
//   q^.add(ex + bx, ey + by, ez + bz, ex + bx + cx, ey + by + cy, ez + bz + cz);
//   q^.add(ex + ax + bx, ey + ay + by, ez + az + bz, ex + ax + bx + cx,
//     ey + ay + by + cy, ez + az + bz + cz);
//   dispose(q, done);
// end;

// procedure triangle;
// var
//   a, b, c: ppunkt3d;
// begin
//   c := points^.addo(cx, cy, cz);
//   a := points^.addo(ax, ay, az);
//   b := points^.addo(bx, by, bz);
//   dreiecks.add(c, a, b, [1, 2, 3]);
// end;

// begin
//   new(points, init);
//   dreiecks.init;
// end.
