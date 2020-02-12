use crate::float::*;
use crate::mat3::*;
use crate::point::*;
use crate::polygon::*;
use crate::time::*;
use crate::triangle::*;
use crate::vec3::*;

pub struct AppContext<'a> {
    pub zaehl: ctyp,

    pub dreiecks: Vec<dreieck<'a>>,

    pub Auge: Vector3,
    pub BlickR: Vector3,
    pub iv: Vector3,
    pub jv: Vector3,
    pub colmode: bool,
    pub drawmode: u8,
    pub tiefePerspektive: minmax,
    pub ausgabeInsert: bool,
    pub scene: Scene<'a>,
    pub backface: bool,
}

impl AppContext<'_> {
    pub fn neukamera(&mut self) {
        //   cls;

        let iv = self.BlickR.kreuz(&self.jv);
        //  if iv.betrag3d=0 then outstring('i=0');

        let iv = iv.mul3d(0.4 * iv.invBetrag3d() / self.BlickR.invBetrag3d());
        let jv = iv.kreuz(&self.BlickR);
        //   //  if jv.betrag3d=0 then outstring('j=0');
        let jv = jv.mul3d(0.4 * jv.invBetrag3d() / self.BlickR.invBetrag3d());

        self.iv = iv;
        self.jv = jv;
    }

    pub fn perspektive(&mut self) {
        self.tiefePerspektive = minmax::new();
        for p in self.scene.points.iter_mut() {
            let mut K = Matrix3::new(self.iv, self.jv, self.Auge.sub3d(&p.o));

            let kd = K.det3d();
            if kd.abs() > epsilon2 {
                K.x = self.BlickR.neg3d();
                p.b.b.x = K.det3d() / kd;
                K.y = K.x;
                K.x = self.iv;
                p.b.b.y = K.det3d() / kd;

                if self.drawmode == 5 {
                    self.tiefePerspektive
                        .update(1.0 / p.o.sub3d(&self.Auge).invBetrag3d());
                }
            }
        }
    }

    pub fn rechnung(&mut self) {
        self.perspektive();

        let ED = self.BlickR.skalar(&self.Auge) + epsilon1;

        for j in self.scene.dreiecks.iter_mut() {
            if self.BlickR.skalar(&j.origPoint1.o) > ED  &&
                self.BlickR.skalar(&j.origPoint2.o) > ED  &&
               self.BlickR.skalar(&j.origPoint3.o) > ED  &&
             // test if not behind view plane
               // evtl kann man das mit der Lichtberechnung beim Initialisieren des Polygons kombinieren
               (!self.backface || ((j.delegate.p3.b.x - j.delegate.p1.b.x) *
            (j.delegate.p2.b.y - j.delegate.p1.b.y) + epsilon1 < (j.delegate.p3.b.y - j.delegate.p1.b.y) *
            (j.delegate.p2.b.x - j.delegate.p1.b.x)))
            {
                if j.delegate.flaechentest() {
                    let h = polygon::newpoly(&j);
                    //         polygon.push(h, 1, -2)
                }
            }
        }

        //   {  xscan:=-1e20;}
    }
}

pub struct Scene<'a> {
    points: Vec<Box<punkt3d>>,

    dreiecks: Vec<dreieck<'a>>,
}

impl<'a> Scene<'a> {
    pub fn addo(&mut self, x: Float, y: Float, z: Float) -> usize {
        let p = Box::new(punkt3d::new(x, y, z));
        self.points.push(p);
        self.points.len()
    }

    pub fn add(&'a mut self, p1: usize, p2: usize, p3: usize, ls: u8) {
        let d = dreieck::new(&self.points[p1], &self.points[p2], &self.points[p3], ls);
        self.dreiecks.push(d);
    }

    pub fn quad(&'a mut self, p1: usize, p2: usize, p3: usize, p4: usize) {
        self.dreiecks.push(dreieck::new(
            &self.points[p1],
            &self.points[p2],
            &self.points[p3],
            5,
        ));
        self.dreiecks.push(dreieck::new(
            &self.points[p1],
            &self.points[p3],
            &self.points[p4],
            3,
        ));
        // self.add(p1, p2, p3, 5);
        // self.add(p1, p3, p4, 3);
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
