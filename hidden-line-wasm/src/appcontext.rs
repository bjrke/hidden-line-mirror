use crate::dreidext::*;
use crate::float::*;
use crate::mat3::*;
use crate::point::*;
use crate::polygon::*;
use crate::time::*;
use crate::triangle::*;
use crate::vec3::*;
use std::rc::Rc;

pub struct AppContext {
    pub zaehl: ctyp,

    pub Auge: Vector3,
    pub BlickR: Vector3,
    pub iv: Vector3,
    pub jv: Vector3,
    pub colmode: bool,
    pub drawmode: u8,
    pub tiefePerspektive: minmax,
    pub ausgabeInsert: bool,
    pub sceneBuilder: SceneBuilder,
    pub backface: bool,
}

impl AppContext {
    pub fn neukamera(&mut self) {
        let iv = self.BlickR.kreuz(&self.jv);
        //  if iv.betrag3d=0 then outstring('i=0');

        let iv = iv.mul3d(0.4 * iv.invBetrag3d() / self.BlickR.invBetrag3d());
        let jv = iv.kreuz(&self.BlickR);
        //   //  if jv.betrag3d=0 then outstring('j=0');
        let jv = jv.mul3d(0.4 * jv.invBetrag3d() / self.BlickR.invBetrag3d());

        self.iv = iv;
        self.jv = jv;
    }

    pub fn rechnung(&mut self) -> Vec<polygon> {
        let mut scene = Scene::new(&self);

        let ED = self.BlickR.skalar(&self.Auge) + epsilon1;

        let mut polys = Vec::new();

        for j in scene.dreiecks.iter_mut() {
            // test if not behind view plane
            if self.BlickR.skalar(&j.origPoint1.o) > ED &&
               self.BlickR.skalar(&j.origPoint2.o) > ED &&
               self.BlickR.skalar(&j.origPoint3.o) > ED &&
               // evtl kann man das mit der Lichtberechnung beim Initialisieren des Polygons kombinieren
               (!self.backface || ((j.delegate.p3.b.x - j.delegate.p1.b.x) *
            (j.delegate.p2.b.y - j.delegate.p1.b.y) + epsilon1 < (j.delegate.p3.b.y - j.delegate.p1.b.y) *
            (j.delegate.p2.b.x - j.delegate.p1.b.x)))
            {
                if j.delegate.flaechentest() {
                    polys.push(polygon::newpoly(j.clone()));
                    //         polygon.push(h, 1, -2)
                }
            }
        }

        polys
        //   {  xscan:=-1e20;}
    }

    pub fn on_key(&mut self, ch: char) {
        match ch {
            'a' => self.Auge = self.Auge.move3d(&self.BlickR, 1.0),
            'A' => self.Auge = self.Auge.move3d(&self.BlickR, 10.0),
            'y' | 'z' => self.Auge = self.Auge.move3d(&self.BlickR, -1.0),
            'Y' | 'Z' => self.Auge = self.Auge.move3d(&self.BlickR, -10.0),
            'k' => self.Auge = self.Auge.move3d(&self.iv, -1.0),
            'K' => self.Auge = self.Auge.move3d(&self.iv, -10.0),
            'l' => self.Auge = self.Auge.move3d(&self.iv, 1.0),
            'L' => self.Auge = self.Auge.move3d(&self.iv, 10.0),
            's' => self.Auge = self.Auge.move3d(&self.jv, -1.0),
            'S' => self.Auge = self.Auge.move3d(&self.jv, -10.0),
            'x' => self.Auge = self.Auge.move3d(&self.jv, 1.0),
            'X' => self.Auge = self.Auge.move3d(&self.jv, 10.0),
            'd' => RotVec(&mut self.BlickR, &mut self.jv, 1.0),
            'D' => RotVec(&mut self.BlickR, &mut self.jv, 10.0),
            'c' => RotVec(&mut self.jv, &mut self.BlickR, 1.0),
            'C' => RotVec(&mut self.jv, &mut self.BlickR, 10.0),
            ',' => RotVec(&mut self.iv, &mut self.BlickR, 1.0),
            ';' | '<' => RotVec(&mut self.iv, &mut self.BlickR, 10.0),
            '.' => RotVec(&mut self.BlickR, &mut self.iv, 1.0),
            ':' | '>' => RotVec(&mut self.BlickR, &mut self.iv, 10.0),
            'o' => RotVec(&mut self.jv, &mut self.iv, 1.0),
            'O' => RotVec(&mut self.jv, &mut self.iv, 10.0),

            'i' => RotVec(&mut self.iv, &mut self.jv, 1.0),
            'I' => RotVec(&mut self.iv, &mut self.jv, 10.0),
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
            _ => {}
        }
    }
}

pub struct Scene {
    points: Vec<Rc<punkt3d>>,

    dreiecks: Vec<Rc<dreieck>>,
}

impl punkt3d {
    pub fn perspektive(mut self, actx: &AppContext) -> Self {
        // self.tiefePerspektive = minmax::new();

        let mut K = Matrix3::new(actx.iv, actx.jv, actx.Auge.sub3d(&self.o));

        let kd = K.det3d();
        if kd.abs() > epsilon2 {
            K.x = actx.BlickR.neg3d();
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

            let c = p1.o.sub3d(&p2.o).kreuz(&p3.o.sub3d(&p2.o));
            let faktor = actx.BlickR.invBetrag3d() * c.invBetrag3d();
            let cols = calcColor((actx.BlickR.skalar(&c) * faktor).abs());

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
