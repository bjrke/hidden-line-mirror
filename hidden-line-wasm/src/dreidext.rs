use crate::float::*;
use crate::vec3::*;
use std::collections::HashMap;

pub struct SceneTriangle {
    pub p1: usize,
    pub p2: usize,
    pub p3: usize,
    pub lset: u8,
}

pub struct SceneBuilder {
    pub points: Vec<Vector3>,
    pub triangles: Vec<SceneTriangle>,
    point_index: HashMap<Vector3, usize>,
}

impl SceneBuilder {
    pub fn new() -> SceneBuilder {
        SceneBuilder {
            points: Vec::new(),
            triangles: Vec::new(),
            point_index: HashMap::new(),
        }
    }

    pub fn point(&mut self, x: Float, y: Float, z: Float) -> usize {
        self.push(Vector3(x, y, z))
    }

    pub fn push(&mut self, p: Vector3) -> usize {
        let index = self.points.len();
        let Vector3(x, y, z) = p;

        let result = *self.point_index.entry(p).or_insert(index);

        if result == index {
            self.points.push(p);
        }
        result
    }

    pub fn triangle(&mut self, p1: usize, p2: usize, p3: usize, lset: u8) -> usize {
        self.triangles.push(SceneTriangle { p1, p2, p3, lset });
        self.triangles.len() - 1
    }

    pub fn quad(&mut self, p1: usize, p2: usize, p3: usize, p4: usize) {
        let o1 = self.points[p1];
        let o2 = self.points[p2];
        let o3 = self.points[p3];
        let o4 = self.points[p4];

        if (o1 - o3).len_sq() < (o2 - o4).len_sq() {
            self.triangle(p1, p2, p3, 3);
            self.triangle(p3, p4, p1, 3);
        } else {
            self.triangle(p4, p1, p2, 3);
            self.triangle(p2, p3, p4, 3);
        }
    }
}

struct TriFan {
    center: usize,
    last: usize,
    first: usize,
    scene: SceneBuilder,
}

impl TriFan {
    pub fn new(
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
        let center = scene.point(cx, cy, cz);
        let last = scene.point(ax, ay, az);
        let first = scene.point(bx, by, bz);
        scene.triangle(center, first, last, 7);
        TriFan {
            scene,
            center,
            last,
            first,
        }
    }

    pub fn add(&mut self, ax: Float, ay: Float, az: Float) -> usize {
        let help = self.scene.point(ax, ay, az);
        self.scene.triangle(self.center, self.last, help, 7);
        self.last = help;
        self.last
    }

    pub fn build(mut self) -> SceneBuilder {
        self.scene.triangle(self.center, self.last, self.first, 7);
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
    pub fn new(
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
        QuadStrip::new(scene, Vector3(ax, ay, az), Vector3(bx, by, bz))
    }

    pub fn new(mut scene: SceneBuilder, a: Vector3, b: Vector3) -> QuadStrip {
        let l2 = scene.push(a);
        let l1 = scene.push(b);
        QuadStrip { l1, l2, scene }
    }

    pub fn add(&mut self, b: Vector3, a: Vector3) {
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
