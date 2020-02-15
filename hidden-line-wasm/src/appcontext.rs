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

    pub fn rechnung(&mut self) -> Vec<polygon> {
        let mut scene = Scene::new(&self);

        let ED = self.BlickR.skalar(&self.Auge) + epsilon1;

        let mut polys = Vec::new();

        for j in scene.dreiecks.iter_mut() {
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
                    polys.push(polygon::newpoly(j.clone()));
                    //         polygon.push(h, 1, -2)
                }
            }
        }

        polys
        //   {  xscan:=-1e20;}
    }
}

pub struct Scene {
    points: Vec<Rc<punkt3d>>,

    dreiecks: Vec<Rc<dreieck>>,
}

impl punkt3d {
    pub fn perspektive(mut self, appCtx: &AppContext) -> Self {
        // self.tiefePerspektive = minmax::new();

        let mut K = Matrix3::new(appCtx.iv, appCtx.jv, appCtx.Auge.sub3d(&self.o));

        let kd = K.det3d();
        if kd.abs() > epsilon2 {
            K.x = appCtx.BlickR.neg3d();
            self.b.b.x = K.det3d() / kd;
            K.y = K.x;
            K.x = appCtx.iv;
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
    pub fn new(appCtx: &AppContext) -> Scene {
        let mut result = Scene {
            points: Vec::new(),
            dreiecks: Vec::new(),
        };

        for p in appCtx.sceneBuilder.points.iter() {
            result
                .points
                .push(Rc::new(punkt3d::newV(p).perspektive(&appCtx)));
        }

        for t in appCtx.sceneBuilder.triangles.iter() {
            result.dreiecks.push(Rc::new(dreieck::new(
                result.points[t.p1].clone(),
                result.points[t.p2].clone(),
                result.points[t.p3].clone(),
                t.lset,
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
