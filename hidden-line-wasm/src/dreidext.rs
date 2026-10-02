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

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn push_deduplicates_points() {
        let mut builder = SceneBuilder::new();

        let i1 = builder.push(Vector3(1.0, 2.0, 3.0));
        let i2 = builder.push(Vector3(1.0, 2.0, 3.0));
        let i3 = builder.push(Vector3(4.0, 5.0, 6.0));

        assert_eq!(i1, i2);
        assert_eq!(builder.scene3.points.len(), 2);
        assert!(i3 != i1);
    }

    #[test]
    fn triangle_appends_to_scene() {
        let mut builder = SceneBuilder::new();
        let a = builder.push(Vector3(0.0, 0.0, 0.0));
        let b = builder.push(Vector3(1.0, 0.0, 0.0));
        let c = builder.push(Vector3(0.0, 1.0, 0.0));

        let t = builder.triangle(a, b, c, 7);

        assert_eq!(t, 0);
        assert_eq!(builder.scene3.triangles.len(), 1);
        assert_eq!(builder.scene3.triangles[0].lset, 7);
    }

    #[test]
    fn quad_splits_along_short_diagonal() {
        let mut builder = SceneBuilder::new();
        let p1 = builder.push(Vector3(0.0, 0.0, 0.0));
        let p2 = builder.push(Vector3(2.0, 0.0, 0.0));
        let p3 = builder.push(Vector3(2.0, 1.0, 0.0));
        let p4 = builder.push(Vector3(0.0, 3.0, 0.0));

        builder.quad(p1, p2, p3, p4);

        let Scene3 { points, triangles } = &builder.scene3;
        assert_eq!(points.len(), 4);
        assert_eq!(triangles.len(), 2);
        assert_eq!(triangles[0].p1, p1);
        assert_eq!(triangles[0].p2, p2);
        assert_eq!(triangles[0].p3, p3);
        assert_eq!(triangles[1].p1, p3);
        assert_eq!(triangles[1].p2, p4);
        assert_eq!(triangles[1].p3, p1);
    }

    #[test]
    fn quad_splits_along_other_diagonal() {
        let mut builder = SceneBuilder::new();
        let p1 = builder.push(Vector3(0.0, 0.0, 0.0));
        let p2 = builder.push(Vector3(1.0, 0.0, 0.0));
        let p3 = builder.push(Vector3(2.0, 1.0, 0.0));
        let p4 = builder.push(Vector3(0.0, 1.0, 0.0));

        builder.quad(p1, p2, p3, p4);

        let Scene3 { triangles, .. } = &builder.scene3;
        assert_eq!(triangles.len(), 2);
        assert_eq!(triangles[0].p1, p4);
        assert_eq!(triangles[0].p2, p1);
        assert_eq!(triangles[0].p3, p2);
        assert_eq!(triangles[1].p1, p2);
        assert_eq!(triangles[1].p2, p3);
        assert_eq!(triangles[1].p3, p4);
    }

    #[test]
    fn quad_strip_builds_single_quad() {
        let builder = SceneBuilder::new();
        let mut strip = QuadStrip::new(builder, Vector3(0.0, 0.0, 0.0), Vector3(1.0, 0.0, 0.0));

        strip.add(Vector3(0.0, 1.0, 0.0), Vector3(1.0, 1.0, 0.0));

        let builder = strip.build();

        assert_eq!(builder.scene3.points.len(), 4);
        assert_eq!(builder.scene3.triangles.len(), 2);
    }
}
