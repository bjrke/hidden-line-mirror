use crate::calcctontext::*;
use crate::drawcontext::*;
use crate::dreidext::*;
use crate::float::*;
use crate::matrix::*;
use crate::vec2::*;
use crate::vec3::*;
use std::collections::HashMap;

pub struct AppContext {
    pub eye: Vector3,
    pub view: Vector3,
    pub iv: Vector3,
    pub jv: Vector3,
    pub scene3: Scene3,
    pub back_face: bool,
}

impl AppContext {
    pub fn new() -> AppContext {
        let eye = Vector3(1.5, 2.0, 2.5);
        let view = eye / -2.0;

        AppContext {
            scene3: SceneBuilder::new().scene3,
            eye,
            view,
            iv: Vector3(1.0, 0.0, 0.0),
            jv: Vector3(0.0, 0.0, 1.0),
            back_face: false,
        }
    }

    pub fn render(&mut self, dctx: &mut dyn DrawContext) {
        dctx.cls();

        let unit_vec_len = 0.4 * self.view.len();
        self.iv = self.view.cross(&self.jv).normalize() * unit_vec_len;
        self.jv = self.iv.cross(&self.view).normalize() * unit_vec_len;

        let scene2 = Scene2::new(&self, &self.scene3);

        println!("#triangle: {:?}", scene2.triangles.len());
        println!("#lines: {:?}", scene2.lines.len());
        println!("eye: {:?}", self.eye);
        println!("view: {:?}", self.view);
        println!("up {:?}", self.jv);

        let tree = create_tree(&self.scene3, &scene2);

        hidden_line(&tree, &self.scene3, &scene2, dctx);

        dctx.finish();
    }

    pub fn on_key(&mut self, ch: char) -> bool {
        match ch {
            'a' => self.eye += self.view.normalize(),
            'A' => self.eye += self.view.normalize() * 10.0,
            'y' | 'z' => self.eye -= self.view.normalize(),
            'Y' | 'Z' => self.eye -= self.view.normalize() * 10.0,
            'k' => self.eye -= self.iv.normalize(),
            'K' => self.eye -= self.iv.normalize() * 10.0,
            'l' => self.eye += self.iv.normalize(),
            'L' => self.eye += self.iv.normalize() * 10.0,
            's' => self.eye -= self.jv.normalize(),
            'S' => self.eye -= self.jv.normalize() * 10.0,
            'x' => self.eye += self.jv.normalize(),
            'X' => self.eye += self.jv.normalize() * 10.0,
            'd' => rot_vec(&mut self.view, &mut self.jv, 1.0),
            'D' => rot_vec(&mut self.view, &mut self.jv, 10.0),
            'c' => rot_vec(&mut self.jv, &mut self.view, 1.0),
            'C' => rot_vec(&mut self.jv, &mut self.view, 10.0),
            ',' => rot_vec(&mut self.iv, &mut self.view, 1.0),
            ';' | '<' => rot_vec(&mut self.iv, &mut self.view, 10.0),
            '.' => rot_vec(&mut self.view, &mut self.iv, 1.0),
            ':' | '>' => rot_vec(&mut self.view, &mut self.iv, 10.0),
            'o' => rot_vec(&mut self.jv, &mut self.iv, 1.0),
            'O' => rot_vec(&mut self.jv, &mut self.iv, 10.0),

            'i' => rot_vec(&mut self.iv, &mut self.jv, 1.0),
            'I' => rot_vec(&mut self.iv, &mut self.jv, 10.0),
            'b' | 'B' => self.back_face = !self.back_face,
            _ => return false,
        }
        true
    }
}

fn rot_vec(to_rot1: &mut Vector3, to_rot2: &mut Vector3, t: Float) {
    let rad = t * PI / 180.0;
    let rot_inc = rad.cos() / rad.sin();
    let rot_len = (rot_inc * rot_inc + 1.0).sqrt();
    let rot_inc = rot_inc / rot_len;

    let copy1 = *to_rot1;
    let copy2 = *to_rot2;

    let len1 = to_rot1.len();
    let len2 = to_rot2.len();

    if len1 == 0.0 {
        println!("len1 = 0");
    }
    if len2 == 0.0 {
        println!("len2 = 0");
    }

    let f1 = len1 / (len2 * rot_len);
    let f2 = -len2 / (len1 * rot_len);

    *to_rot1 = copy1 * rot_inc + copy2 * f1;
    *to_rot2 = copy1 * f2 + copy2 * rot_inc;
}

pub struct Scene2 {
    pub points: Vec<Vector2>,
    pub triangles: Vec<(usize, usize, usize)>,
    pub lines: HashMap<(usize, usize), Color>,
    pub eye: Vector3,
}

fn perspektive(actx: &AppContext, o: &Vector3) -> Vector2 {
    let mut k = Matrix3(actx.iv, actx.jv, actx.eye - *o);

    let kd = k.determinant();
    if kd.abs() > EPSILON2 {
        k.0 = -actx.view;
        let x = k.determinant() / kd;
        k.1 = k.0;
        k.0 = actx.iv;
        let y = k.determinant() / kd;
        Vector2(x, y)
    } else {
        Vector2(0.0, 0.0)
    }
}

impl Scene2 {
    pub fn new(actx: &AppContext, scene3: &Scene3) -> Scene2 {
        let mut result = Scene2 {
            points: Vec::new(),
            triangles: Vec::new(),
            lines: HashMap::new(),
            eye: actx.eye,
        };

        for p in scene3.points.iter() {
            result.points.push(perspektive(&actx, &p));
        }

        for t in scene3.triangles.iter() {
            let o1 = scene3.points[t.p1];
            let o2 = scene3.points[t.p2];
            let o3 = scene3.points[t.p3];

            let p1 = result.points[t.p1];
            let p2 = result.points[t.p2];
            let p3 = result.points[t.p3];

            let c = (o1 - o2).cross(&(o3 - o2));
            let cols = float_to_color((actx.view.normalize() * c.normalize()).abs());

            let eye_view_plane_dist = actx.view * actx.eye + EPSILON1;

            // test if not behind view plane
            if actx.view * o1 > eye_view_plane_dist
                && actx.view * o2 > eye_view_plane_dist
                && actx.view * o3 > eye_view_plane_dist
                && (!actx.back_face
                    || ((p3.0 - p1.0) * (p2.1 - p1.1) + EPSILON1 < (p3.1 - p1.1) * (p2.0 - p1.0)))
            // && !colinear(&p1, &p2, &p3)
            {
                result.triangles.push((t.p1, t.p2, t.p3));

                if t.lset & 1 == 1 {
                    result.push_line(t.p1, t.p2, cols);
                }
                if t.lset & 2 == 2 {
                    result.push_line(t.p2, t.p3, cols);
                }
                if t.lset & 4 == 4 {
                    result.push_line(t.p3, t.p1, cols);
                }
            }
        }

        result
    }

    fn push_line(&mut self, p1: usize, p2: usize, col: Color) {
        self.lines
            .entry((p1.min(p2), p1.max(p2)))
            .and_modify(|e| {
                if col > *e {
                    *e = col
                }
            })
            .or_insert(col);
    }
}
