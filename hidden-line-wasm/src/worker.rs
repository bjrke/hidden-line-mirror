use crate::appcontext::{Camera, Scene2};
use crate::calcctontext::{create_tree, hidden_line_records, scene_lines};
use crate::dreidext::{Scene3, SceneTriangle};
use crate::vec3::Vector3;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Worker {
    scene3: Scene3,
}

#[wasm_bindgen]
impl Worker {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Worker {
        Worker {
            scene3: Scene3 {
                points: Vec::new(),
                triangles: Vec::new(),
            },
        }
    }

    pub fn set_scene(&mut self, points: &[f32], triangles: &[u32]) {
        let pts: Vec<Vector3> = points
            .chunks_exact(3)
            .map(|c| Vector3::new(c[0], c[1], c[2]))
            .collect();
        let tris: Vec<SceneTriangle> = triangles
            .chunks_exact(4)
            .map(|c| SceneTriangle {
                p1: c[0] as usize,
                p2: c[1] as usize,
                p3: c[2] as usize,
                lset: c[3] as u8,
            })
            .collect();
        self.scene3 = Scene3 {
            points: pts,
            triangles: tris,
        };
    }

    pub fn compute(&self, camera: &[f32], rank: usize, count: usize) -> Vec<f32> {
        let camera = Camera {
            eye: Vector3::new(camera[0], camera[1], camera[2]),
            view: Vector3::new(camera[3], camera[4], camera[5]),
            iv: Vector3::new(camera[6], camera[7], camera[8]),
            jv: Vector3::new(camera[9], camera[10], camera[11]),
            back_face: camera[12] != 0.0,
        };

        let scene2 = Scene2::new(&camera, &self.scene3);
        let tree = create_tree(&self.scene3, &scene2);
        let lines = scene_lines(&scene2);

        hidden_line_records(&tree, &self.scene3, &scene2, &lines, rank, count)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn compute_returns_visible_segments() {
        let mut worker = Worker::new();
        let points = [0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0, 0.0];
        let triangles = [0u32, 1, 2, 7];
        worker.set_scene(&points, &triangles);

        let camera = [
            0.5, 0.5, 2.0, // eye
            0.0, 0.0, -1.0, // view
            1.0, 0.0, 0.0, // iv
            0.0, 1.0, 0.0, // jv
            0.0, // back_face
        ];

        let records = worker.compute(&camera, 0, 1);

        assert!(!records.is_empty());
        assert_eq!(records.len() % 5, 0);
    }
}
