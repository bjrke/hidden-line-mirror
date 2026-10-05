use crate::calcctontext::*;
use crate::drawcontext::*;
use crate::dreidext::*;
use crate::float::*;
use crate::matrix::*;
use crate::vec2::*;
use crate::vec3::*;
use std::collections::HashMap;

pub struct AppContext {
    eye: Vector3,
    view: Vector3,
    iv: Vector3,
    jv: Vector3,
    pub scene3: Scene3,
    back_face: bool,
    frame: Frame,
}

impl AppContext {
    pub fn new() -> AppContext {
        let eye = Vector3::new(1.5, 2.0, 2.5);
        let view = eye / -2.0;

        AppContext {
            scene3: SceneBuilder::new().scene3,
            eye,
            view,
            iv: Vector3::new(1.0, 0.0, 0.0),
            jv: Vector3::new(0.0, 0.0, 1.0),
            back_face: false,
            frame: 0,
        }
    }

    pub fn render<C: ColorContext, D: DrawContext<C>>(&mut self, dctx: &mut D) {
        let unit_vec_len = 0.4 * self.view.len();
        self.iv = self.view.cross(&self.jv).normalize() * unit_vec_len;
        self.jv = self.iv.cross(&self.view).normalize() * unit_vec_len;

        let scene2 = Scene2::new(&self, &self.scene3);

        console_log!("#triangle: {:?}", scene2.triangles.len());
        console_log!("#lines: {:?}", scene2.lines.len());
        console_log!("eye: {:?}", self.eye);
        console_log!("view: {:?}", self.view);
        console_log!("up {:?}", self.jv);

        let tree = create_tree(&self.scene3, &scene2);

        for (&color, lines) in scene2.lines.iter() {
            let mut color_context = dctx.color_context(color);
            hidden_line(&tree, &self.scene3, &scene2, lines, &mut color_context);
            dctx.draw(self.frame, color_context);
        }

        dctx.finish(self.frame);
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
        console_log!("len1 = 0");
    }
    if len2 == 0.0 {
        console_log!("len2 = 0");
    }

    let f1 = len1 / (len2 * rot_len);
    let f2 = -len2 / (len1 * rot_len);

    *to_rot1 = copy1 * rot_inc + copy2 * f1;
    *to_rot2 = copy1 * f2 + copy2 * rot_inc;
}

pub struct Scene2 {
    pub points: Vec<Vector2>,
    pub triangles: Vec<(usize, usize, usize)>,
    pub lines: HashMap<Color, Vec<(usize, usize)>>,
    pub eye: Vector3,
}

fn perspektive(actx: &AppContext, o: &Vector3) -> Vector2 {
    let mut k = Matrix3::new(actx.iv, actx.jv, actx.eye - *o);

    let kd = k.determinant();
    if kd.abs() > EPSILON2 {
        k.a = -actx.view;
        let x = k.determinant() / kd;
        k.b = k.a;
        k.a = actx.iv;
        let y = k.determinant() / kd;
        Vector2::new(x, y)
    } else {
        Vector2::new(0.0, 0.0)
    }
}

impl Scene2 {
    pub fn new(actx: &AppContext, scene3: &Scene3) -> Scene2 {
        let mut points = Vec::new();
        let mut triangles = Vec::new();

        for p in scene3.points.iter() {
            points.push(perspektive(&actx, &p));
        }

        let mut lines_by_index = HashMap::new();

        let eye = actx.eye;
        for t in scene3.triangles.iter() {
            let o1 = scene3.points[t.p1];
            let o2 = scene3.points[t.p2];
            let o3 = scene3.points[t.p3];

            let p1 = points[t.p1];
            let p2 = points[t.p2];
            let p3 = points[t.p3];

            let c = (o1 - o2).cross(&(o3 - o2));
            let cols = float_to_color((actx.view.normalize() * c.normalize()).abs());

            let eye_view_plane_dist = actx.view * eye + EPSILON1;

            // test if not behind view plane
            if actx.view * o1 > eye_view_plane_dist
                && actx.view * o2 > eye_view_plane_dist
                && actx.view * o3 > eye_view_plane_dist
                && (!actx.back_face
                    || ((p3.x - p1.x) * (p2.y - p1.y) + EPSILON1 < (p3.y - p1.y) * (p2.x - p1.x)))
            // && !colinear(&p1, &p2, &p3)
            {
                triangles.push((t.p1, t.p2, t.p3));

                if t.lset & 1 == 1 {
                    push_line(&mut lines_by_index, t.p1, t.p2, cols);
                }
                if t.lset & 2 == 2 {
                    push_line(&mut lines_by_index, t.p2, t.p3, cols);
                }
                if t.lset & 4 == 4 {
                    push_line(&mut lines_by_index, t.p3, t.p1, cols);
                }
            }
        }

        let mut lines: HashMap<Color, Vec<(usize, usize)>> = HashMap::new();

        for (&(a, e), &color) in lines_by_index.iter() {
            lines
                .entry(color)
                .and_modify(|v| v.push((a, e)))
                .or_insert(vec![(a, e)]);
        }

        Scene2 {
            points,
            triangles,
            lines,
            eye,
        }
    }
}

fn push_line(lines: &mut HashMap<(usize, usize), Color>, p1: usize, p2: usize, col: Color) {
    lines
        .entry((p1.min(p2), p1.max(p2)))
        .and_modify(|e| {
            if col > *e {
                *e = col
            }
        })
        .or_insert(col);
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn on_key_known_keys_return_true() {
        let mut actx = AppContext::new();
        let keys = [
            'a', 'A', 'y', 'z', 'Y', 'Z', 'k', 'K', 'l', 'L', 's', 'S', 'x', 'X', 'd', 'D', 'c',
            'C', ',', ';', '<', '.', ':', '>', 'o', 'O', 'i', 'I', 'b', 'B',
        ];
        for k in keys.iter() {
            assert!(actx.on_key(*k), "key {:?} should be handled", k);
        }
    }

    #[test]
    fn on_key_unknown_keys_return_false() {
        let mut actx = AppContext::new();
        assert!(!actx.on_key('1'));
        assert!(!actx.on_key(' '));
        assert!(!actx.on_key('\n'));
    }

    #[test]
    fn on_key_a_moves_eye() {
        let mut actx = AppContext::new();
        let before = actx.eye;
        assert!(actx.on_key('a'));
        assert!(actx.eye != before);
    }

    #[test]
    fn rot_vec_preserves_lengths_and_orthogonality() {
        let mut u = Vector3::new(1.0, 0.0, 0.0);
        let mut v = Vector3::new(0.0, 1.0, 0.0);

        rot_vec(&mut u, &mut v, 10.0);

        assert!((u.len() - 1.0).abs() < 1.0e-5);
        assert!((v.len() - 1.0).abs() < 1.0e-5);
        assert!(u * v < 1.0e-5);
    }

    #[test]
    fn rot_vec_small_angle_keeps_vectors_close() {
        let mut u = Vector3::new(1.0, 0.0, 0.0);
        let mut v = Vector3::new(0.0, 1.0, 0.0);
        let u0 = u;
        let v0 = v;

        rot_vec(&mut u, &mut v, 1.0);

        assert!(u.cross(&u0).len() < 0.1);
        assert!(v.cross(&v0).len() < 0.1);
    }

    #[test]
    fn scene2_projects_axis_point_to_origin() {
        let mut actx = AppContext::new();
        let on_axis = actx.eye - actx.view;
        actx.scene3 = Scene3 {
            points: vec![on_axis],
            triangles: vec![],
        };

        let scene2 = Scene2::new(&actx, &actx.scene3);

        assert_eq!(scene2.points, vec![Vector2::new(0.0, 0.0)]);
    }

    #[test]
    fn scene2_projects_off_axis_point() {
        let mut actx = AppContext::new();
        actx.scene3 = Scene3 {
            points: vec![Vector3::new(1.0, 0.0, 0.0)],
            triangles: vec![],
        };

        let scene2 = Scene2::new(&actx, &actx.scene3);

        assert_eq!(scene2.points, vec![Vector2::new(0.5, 0.0)]);
    }
}
