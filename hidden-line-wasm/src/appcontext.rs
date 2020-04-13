use crate::dreidext::*;
use crate::float::*;
use crate::mat3::*;
use crate::minxqueue::*;
use crate::point::*;
use crate::polygon::*;
use crate::time::*;
use crate::triangle::*;
use crate::vec3::*;
use std::collections::BinaryHeap;
use std::rc::Rc;

pub struct AppContext {
    pub zaehl: ctyp,

    pub Auge: Vector3,
    pub BlickR: Vector3,
    pub iv: Vector3,
    pub jv: Vector3,
    pub colmode: bool,
    pub drawmode: u8,
    pub ausgabeInsert: bool,
    pub sceneBuilder: SceneBuilder,
    pub backface: bool,
}

impl AppContext {
    pub fn neukamera(&mut self) {
        self.iv = self.BlickR.cross(&self.jv).normalize() * (0.4 * self.BlickR.len());
        self.jv = self.iv.cross(&self.BlickR).normalize() * (0.4 * self.BlickR.len());
    }

    pub fn rechnung(&mut self) -> BinaryHeap<MinxQueueEntry> {
        let mut scene = Scene::new(&self);

        let ED = self.BlickR * self.Auge + epsilon1;

        let mut polys = BinaryHeap::new();

        for j in scene.dreiecks.iter_mut() {
            // test if not behind view plane
            if self.BlickR * j.origPoint1.o > ED &&
               self.BlickR * j.origPoint2.o > ED &&
               self.BlickR * j.origPoint3.o > ED &&
               // evtl kann man das mit der Lichtberechnung beim Initialisieren des Polygons kombinieren
               (!self.backface || ((j.delegate.p3.b.x - j.delegate.p1.b.x) *
            (j.delegate.p2.b.y - j.delegate.p1.b.y) + epsilon1 < (j.delegate.p3.b.y - j.delegate.p1.b.y) *
            (j.delegate.p2.b.x - j.delegate.p1.b.x)))
            {
                if j.delegate.flaechentest() {
                    let polygon = polygon::newpoly(j.clone());
                    polys.push(MinxQueueEntry { polygon });
                }
            }
        }

        polys
        //   {  xscan:=-1e20;}
    }

    pub fn on_key(&mut self, ch: char) -> bool {
        match ch {
            'a' => self.Auge += self.BlickR.normalize(),
            'A' => self.Auge += self.BlickR.normalize() * 10.0,
            'y' | 'z' => self.Auge -= self.BlickR.normalize(),
            'Y' | 'Z' => self.Auge -= self.BlickR.normalize() * 10.0,
            'k' => self.Auge -= self.iv.normalize(),
            'K' => self.Auge -= self.iv.normalize() * 10.0,
            'l' => self.Auge += self.iv.normalize(),
            'L' => self.Auge += self.iv.normalize() * 10.0,
            's' => self.Auge -= self.jv.normalize(),
            'S' => self.Auge -= self.jv.normalize() * 10.0,
            'x' => self.Auge += self.jv.normalize(),
            'X' => self.Auge += self.jv.normalize() * 10.0,
            'd' => rot_vec(&mut self.BlickR, &mut self.jv, 1.0),
            'D' => rot_vec(&mut self.BlickR, &mut self.jv, 10.0),
            'c' => rot_vec(&mut self.jv, &mut self.BlickR, 1.0),
            'C' => rot_vec(&mut self.jv, &mut self.BlickR, 10.0),
            ',' => rot_vec(&mut self.iv, &mut self.BlickR, 1.0),
            ';' | '<' => rot_vec(&mut self.iv, &mut self.BlickR, 10.0),
            '.' => rot_vec(&mut self.BlickR, &mut self.iv, 1.0),
            ':' | '>' => rot_vec(&mut self.BlickR, &mut self.iv, 10.0),
            'o' => rot_vec(&mut self.jv, &mut self.iv, 1.0),
            'O' => rot_vec(&mut self.jv, &mut self.iv, 10.0),

            'i' => rot_vec(&mut self.iv, &mut self.jv, 1.0),
            'I' => rot_vec(&mut self.iv, &mut self.jv, 10.0),
            'f' | 'F' => self.colmode = !self.colmode,
            // 't' | 'T' => self.tausgabe = !self.tausgabe,
            'b' | 'B' => self.backface = !self.backface,
            //       'p':begin
            //         palettePos := (palettePos + 1) MOD Length(palette);
            //         SetAllPalette(palette[palettePos]);
            //       end;
            //       'P':begin
            //         palettePos := (palettePos + Length(palette) - 1) MOD Length(palette);
            //         SetAllPalette(palette[palettePos]);
            //       end;
            '0' => self.drawmode = 0,
            '1' => self.drawmode = 1,
            '2' => self.drawmode = 2,
            '3' => self.drawmode = 3,
            '4' => self.drawmode = 4,
            '5' => self.drawmode = 5,
            //       '6':begin drawmode=6;zumalen=[0..255];tausgabe=true;end;
            '7' => self.drawmode = 7,
            '8' => self.drawmode = 8,
            '9' => self.drawmode = 9,
            _ => return false,
        }
        true
    }
}

fn rot_vec(to_rot1: &mut Vector3, to_rot2: &mut Vector3, t: Float) {
    let rad = t * PI / 180.0;
    let rot_inc = rad.cos() / rad.sin();
    let rot_len = (rot_inc.sqr() + 1.0).sqrt();
    let rot_inc = (rot_inc / rot_len);

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

    to_rot1.x = rot_inc * copy1.x + copy2.x * f1;
    to_rot1.y = rot_inc * copy1.y + copy2.y * f1;
    to_rot1.z = rot_inc * copy1.z + copy2.z * f1;

    to_rot2.x = f2 * copy1.x + rot_inc * copy2.x;
    to_rot2.y = f2 * copy1.y + rot_inc * copy2.y;
    to_rot2.z = f2 * copy1.z + rot_inc * copy2.z;
}

pub struct Scene {
    points: Vec<Rc<punkt3d>>,

    dreiecks: Vec<Rc<dreieck>>,
}

impl punkt3d {
    pub fn perspektive(mut self, actx: &AppContext) -> Self {
        // self.tiefePerspektive = minmax::new();

        let mut K = Matrix3::new(actx.iv, actx.jv, actx.Auge - self.o);

        let kd = K.det3d();
        if kd.abs() > epsilon2 {
            K.x = -actx.BlickR;
            self.b.b.x = K.det3d() / kd;
            K.y = K.x;
            K.x = actx.iv;
            self.b.b.y = K.det3d() / kd;

            // if self.drawmode == 5 {
            //     self.tiefePerspektive
            //         .update(1.0 / p.o.sub3d(&self.Auge).invBetrag3d());
            // }
        }
        self
    }
}

impl Scene {
    pub fn new(actx: &AppContext) -> Scene {
        let mut result = Scene {
            points: Vec::new(),
            dreiecks: Vec::new(),
        };

        for p in actx.sceneBuilder.points.iter() {
            result
                .points
                .push(Rc::new(punkt3d::newV(p).perspektive(&actx)));
        }

        for t in actx.sceneBuilder.triangles.iter() {
            let p1 = &result.points[t.p1];
            let p2 = &result.points[t.p2];
            let p3 = &result.points[t.p3];

            let c = (p1.o - p2.o).cross(&(p3.o - p2.o));
            let cols = (actx.BlickR.normalize() * c.normalize()).abs();

            result.dreiecks.push(Rc::new(dreieck::new(
                p1.clone(),
                p2.clone(),
                p3.clone(),
                t.lset,
                cols,
            )));
        }
        result
    }
}

//   dliste = object
//   public
//     function First: pdreieck;
//     function Next: pdreieck;
//     constructor init;
//   private
//     Anker, aktuell, Last: pdreieck;
//   end;

// constructor dliste.init;
// begin
//   Anker := nil;
//   Last := nil;
// end;

// function dliste.add;
// var
//   h: pdreieck;
// begin
//   New(h, init(p1, p2, p3, ls));
//   if anker = nil then
//     anker := h;
//   if last = nil then
//     last := h
//   else
//   begin
//     last^.Next := h;
//     last := h;
//   end;
//   exit(h);
// end;

// function dliste.First;
// begin
//   aktuell := anker;
//   exit(aktuell);
// end;

// function dliste.Next;
// begin
//   if aktuell <> nil then
//     aktuell := aktuell^.Next;
//   exit(aktuell);
// end;

// begin
// end.
