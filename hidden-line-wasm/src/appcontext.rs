use crate::float::*;
use crate::point::*;
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
