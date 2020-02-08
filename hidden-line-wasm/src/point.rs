use crate::drawcontext::*;
use crate::float::*;
use crate::vec2::*;
use crate::vec3::*;

pub struct point {
  pub b: Vector2,
  pub gz: u8,
}

type punkt = point;

impl point {
  pub fn new0() -> point {
    point::new(Vector2::new(0.0, 0.0), 0)
  }
  pub fn new(bv: Vector2, ls: u8) -> point {
    point { b: bv, gz: ls }
  }

  pub fn done() {}

  pub fn draw(&self, ctx: &mut dyn DrawContext, c: Color) {
    ctx.circle(self.b.x, -self.b.y, 2.0, c);
  }
}

pub struct punkt3d {
  pub b: punkt,
  pub o: Vector3,
}

impl punkt3d {
  pub fn new(ax: Float, ay: Float, az: Float) -> punkt3d {
    punkt3d {
      o: Vector3::new(ax, ay, az),
      b: point::new0(),
    }
  }
}

pub fn gleicheseite(s: point, p2: point, p3: point, p4: point) -> Float {
  (((p3.b.y - s.b.y) * (p2.b.x - s.b.x) - (p3.b.x - s.b.x) * (p2.b.y - s.b.y))
    * ((p4.b.y - s.b.y) * (p2.b.x - s.b.x) - (p4.b.x - s.b.x) * (p2.b.y - s.b.y)))
}

// type
//   lset=set of 1..3;

//   punktliste=^pliste;
//   pliste=object
//     private
//       aktuell,Anker,Last:ppunkt3d;
//     public
//       function addo(ax,ay,az:float):ppunkt3d;
//       procedure add(p:ppunkt3d);
//       function first:ppunkt3d;
//       function next:ppunkt3d;
//       constructor init;
//   end;

//

// var
//   points:punktliste;

// implementation

// procedure pliste.add;
// begin
//   if anker=nil then
//     anker:=p;
//   if last=nil then
//     last:=p
//   else begin
//     last^.next:=p;
//     last:=p
//   end
// end;

// function pliste.addo;
// var h:ppunkt3d;
// begin
//   new(h, init(ax,ay,az));
//   add(h);
//   addo:=h
// end;

// function pliste.first;
// begin
//   aktuell:=anker;
//   first:=aktuell;
// end;

// function pliste.next;
// begin
//   if aktuell<>nil then
//     aktuell:=aktuell^.next;
//   next:=aktuell
// end;

// constructor pliste.init;
// begin
//   Last:=nil;
//   Anker:=nil;
// end;

// begin
// end.
