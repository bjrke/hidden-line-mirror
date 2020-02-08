use crate::time::*;
use crate::triangle::*;

pub struct AppContext<'a> {
    pub zaehl: ctyp,

    pub dreiecks: Vec<dreieck<'a>>,
}

//   dliste = object
//   public
//     function add(p1, p2, p3: ppunkt3d; ls: lset): pdreieck;
//     function First: pdreieck;
//     function Next: pdreieck;
//     constructor init;
//   private
//     Anker, aktuell, Last: pdreieck;
//   end;
