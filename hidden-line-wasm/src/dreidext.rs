use crate::float::*;
use crate::vec3::*;
use std::collections::HashMap;

pub struct SceneTriangle {
    pub p1: usize,
    pub p2: usize,
    pub p3: usize,
    pub lset: u8,
}

pub struct Scene3 {
    pub points: Vec<Vector3>,
    pub triangles: Vec<SceneTriangle>,
}
pub struct SceneBuilder {
    pub scene3: Scene3,
    point_index: HashMap<Vector3, usize>,
}

impl SceneBuilder {
    pub fn new() -> SceneBuilder {
        SceneBuilder {
            scene3: Scene3 {
                points: Vec::new(),
                triangles: Vec::new(),
            },
            point_index: HashMap::new(),
        }
    }

    pub fn push(&mut self, p: Vector3) -> usize {
        let index = self.scene3.points.len();

        let result = *self.point_index.entry(p).or_insert(index);

        if result == index {
            self.scene3.points.push(p);
        }
        result
    }

    pub fn triangle(&mut self, p1: usize, p2: usize, p3: usize, lset: u8) -> usize {
        self.scene3
            .triangles
            .push(SceneTriangle { p1, p2, p3, lset });
        self.scene3.triangles.len() - 1
    }

    pub fn quad(&mut self, p1: usize, p2: usize, p3: usize, p4: usize) {
        let o1 = self.scene3.points[p1];
        let o2 = self.scene3.points[p2];
        let o3 = self.scene3.points[p3];
        let o4 = self.scene3.points[p4];

        if (o1 - o3).len_sq() < (o2 - o4).len_sq() {
            self.triangle(p1, p2, p3, 3);
            self.triangle(p3, p4, p1, 3);
        } else {
            self.triangle(p4, p1, p2, 3);
            self.triangle(p2, p3, p4, 3);
        }
    }
}

pub struct QuadStrip {
    l1: usize,
    l2: usize,
    scene_builder: SceneBuilder,
}

impl QuadStrip {
    pub fn init(
        scene_builder: SceneBuilder,
        ax: Float,
        ay: Float,
        az: Float,
        bx: Float,
        by: Float,
        bz: Float,
    ) -> QuadStrip {
        QuadStrip::new(scene_builder, Vector3(ax, ay, az), Vector3(bx, by, bz))
    }

    pub fn new(mut scene_builder: SceneBuilder, a: Vector3, b: Vector3) -> QuadStrip {
        let l2 = scene_builder.push(a);
        let l1 = scene_builder.push(b);
        QuadStrip {
            l1,
            l2,
            scene_builder,
        }
    }

    pub fn add(&mut self, b: Vector3, a: Vector3) {
        let h1 = self.scene_builder.push(a);
        let h2 = self.scene_builder.push(b);

        self.scene_builder.quad(self.l2, self.l1, h1, h2);

        self.l1 = h1;
        self.l2 = h2;
    }

    pub fn build(self) -> SceneBuilder {
        self.scene_builder
    }
}
