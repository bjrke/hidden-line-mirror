type longint = i32;

use crate::float;
use crate::float::Float;

pub struct dtyp {
  pub insert: longint,
  pub Delete: longint,
  pub maximum: longint,
  pub minimum: longint,
  pub aktuell: longint,
}

impl dtyp {
  pub fn new() -> dtyp {
    dtyp {
      insert: 0,
      Delete: 0,
      maximum: 0,
      minimum: 0,
      aktuell: 0,
    }
  }

  pub fn ins(&mut self) {
    self.insert += 1;
    self.aktuell += 1;
    if self.aktuell > self.maximum {
      self.maximum = self.aktuell;
    }
  }

  pub fn del(&mut self) {
    self.Delete += 1;
    self.aktuell -= 1;
    if self.aktuell < self.minimum {
      self.minimum = self.aktuell;
    }
  }

  pub fn ausgabe(&self, Name: &str) {
    println!("{}", Name);
    if self.insert == self.Delete && self.insert != 0 {
      println!("  Einfügungen = Löschungen: {}", self.insert)
    } else {
      if self.insert != 0 {
        println!("  Einfügungen: {}", self.insert);
      }
      if self.Delete != 0 {
        println!("  Löschungen: {}", self.Delete);
      }
    }
    if self.maximum != 0 && self.maximum != self.aktuell {
      println!("  Höchststand: {}", self.maximum);
    }
    if self.minimum != 0 && self.minimum != self.aktuell {
      println!("  Tiefststand: {}", self.minimum);
    }
    if self.aktuell != self.insert {
      if (self.aktuell != 0) && (self.minimum != self.aktuell) && (self.maximum != self.aktuell) {
        println!("  aktueller Stand: {}", self.aktuell);
      }
      if self.aktuell == self.maximum && self.aktuell != 0 {
        println!("  aktuell(Höchst)Stand: {}", self.aktuell);
      }
      if self.aktuell == self.minimum && self.aktuell != 0 {
        println!("  aktuell(Tiefst)Stand: {}", self.aktuell);
      }
    } else {
      if self.aktuell != 0 && self.minimum != self.aktuell && self.maximum != self.aktuell {
        println!("  aktueller Stand = Einfügungen: {}", self.aktuell);
      }
      if self.aktuell == self.maximum && self.aktuell != 0 {
        println!("  aktuell(Höchst)Stand = Einfügungen: {}", self.aktuell);
      }
      if self.aktuell == self.minimum && self.aktuell != 0 {
        println!("  aktuell(Tiefst)Stand = Einfügungen: {}", self.aktuell);
      }
    }
    println!("--------------------------");
  }
}

pub struct ctyp {
  pub q1: dtyp,
  pub q2: dtyp,
  pub q3: dtyp,
  pub suchbaum: dtyp,
  pub polygons: dtyp,
  pub ptest: longint,
  pub Count: longint,
  pub pp: longint,
}

impl ctyp {
  pub fn init() -> ctyp {
    ctyp {
      q1: dtyp::new(),
      q2: dtyp::new(),
      q3: dtyp::new(),
      suchbaum: dtyp::new(),
      polygons: dtyp::new(),
      Count: 0,
      ptest: 0,
      pp: 0,
    }
  }

  pub fn ausgabe(&self) {
    println!("# insert aufrufe : {}", self.Count);
    println!("# Polytests: {}", self.ptest);
    println!("# Triangulationen: {}", self.pp);
    self.q1.ausgabe("Warteschlange 1");
    self.q2.ausgabe("Warteschlange 2");
    self.q3.ausgabe("Warteschlange 3");
    self.suchbaum.ausgabe("Suchbaum");
    self.polygons.ausgabe("Polygone insgesamt");
    //TODO self.points2d.ausgabe("Punkte in Bildeben");
    //TODO self.points3d.ausgabe("Punkte im Raum");
  }
}

pub struct minmax {
  MinValue: Float,
  MaxValue: Float,
}

impl minmax {
  pub fn new() -> minmax {
    minmax {
      MinValue: float::MAX,
      MaxValue: float::MIN,
    }
  }

  pub fn update(&mut self, f: Float) {
    if f < self.MinValue {
      self.MinValue = f;
    }
    if f > self.MaxValue {
      self.MaxValue = f;
    }
  }

  pub fn toString(&self) -> String {
    format!("min: {} max: {}", self.MinValue, self.MaxValue)
  }

  pub fn relative(&self, f: Float) -> Float {
    (f - self.MinValue) / (self.MaxValue - self.MinValue)
  }
}

/*




  cset = set of char;

function gettime2: longint;
function readkey2(include: cset): char;

implementation

var
  time: longint;








function readkey2;
var
  start: longint;
  ch, res: char;
begin
  repeat
    res := ReadKey;
    if (res = #0) then
      ReadKey
  until (Upcase(res) in include);

  if KeyPressed then
  begin
    start := gettime2;
    repeat
      ch := ReadKey;
      if (ch = #0) then
        ReadKey
    until ((not KeyPressed) or (ch != res) or ((gettime2 - start) > 500));
  end;

  if (Upcase(ch) in include) then
    exit(ch)
  else
    exit(res);
end;

begin
  zaehl.points3d.init;
end.
*/
