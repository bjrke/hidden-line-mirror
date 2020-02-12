use crate::appcontext::*;
use crate::float::*;
use crate::mat3::*;
use crate::point::*;

pub fn neukamera(appCtx: &mut AppContext) {
    //   cls;

    let iv = appCtx.BlickR.kreuz(&appCtx.jv);
    //  if iv.betrag3d=0 then outstring('i=0');

    let iv = iv.mul3d(0.4 * iv.invBetrag3d() / appCtx.BlickR.invBetrag3d());
    let jv = iv.kreuz(&appCtx.BlickR);
    //   //  if jv.betrag3d=0 then outstring('j=0');
    let jv = jv.mul3d(0.4 * jv.invBetrag3d() / appCtx.BlickR.invBetrag3d());

    appCtx.iv = iv;
    appCtx.jv = jv;
}

// var
//   backface: boolean;
//   palette: array [0..2] of palettetype;

// implementation

pub fn perspektive(appCtx: &mut AppContext, p: &mut punkt3d) {
    let mut K = Matrix3::new(appCtx.iv, appCtx.jv, appCtx.Auge.sub3d(&p.o));

    let kd = K.det3d();
    if kd.abs() > epsilon2 {
        K.x = appCtx.BlickR.neg3d();
        p.b.b.x = K.det3d() / kd;
        K.y = K.x;
        K.x = appCtx.iv;
        p.b.b.y = K.det3d() / kd;

        if appCtx.drawmode == 5 {
            appCtx
                .tiefePerspektive
                .update(1.0 / p.o.sub3d(&appCtx.Auge).invBetrag3d());
        }
    }
}

pub fn rechnung(appCtx: &mut AppContext) {
    // var
    //   i: ppunkt3d;
    //   j: pdreieck;
    //   ED: float;
    //   h: ppoly;

    //   tiefePerspektive.init;
    //   i := points^.First;
    //   while i <> nil do
    //   begin
    //     perspektive(i);
    //     i := points^.Next;
    //   end;

    //   ED := BlickR.skalar(Auge) + epsilon1;
    //   j := dreiecks.First;
    //   while j <> nil do
    //   begin
    //     if (BlickR.skalar(j^.origPoints[1]^.o) > ED) and
    //       (BlickR.skalar(j^.origPoints[2]^.o) > ED) and
    //       (BlickR.skalar(j^.origPoints[3]^.o) > ED) and
    //       // test if not behind view plane
    //       // evtl kann man das mit der Lichtberechnung beim Initialisieren des Polygons kombinieren
    //       (not backface or ((j^.p[3]^.b.x - j^.p[1]^.b.x) *
    //       (j^.p[2]^.b.y - j^.p[1]^.b.y) + epsilon1 < (j^.p[3]^.b.y - j^.p[1]^.b.y) *
    //       (j^.p[2]^.b.x - j^.p[1]^.b.x))) then
    //     begin
    //       new(h, newpoly(j));
    //       if h^.flaechentest then
    //         polygon.push(h, 1, -2)
    //       else
    //         dispose(h, done);
    //     end;
    //     j := dreiecks.Next;
    //   end;
    //   {  xscan:=-1e20;}
}
// procedure initGraphic;
// var
//   tr, md: integer;
// begin
//   tr := D8bit;
//   md := m1024x768;
//   initgraph(tr, md, '');
// end;

// const
//   colors = 16;

// var
//   p, i: integer;
// begin
//   initGraphic;
//   bmx := getmaxx div 2;
//   bmy := getmaxy div 2;

//   for p := 0 to Length(palette) - 1 do
//   begin
//     for i := 1 to (colors - 1) do
//     begin
//       setpalette(i, i + p * colors);
//     end;
//     getpalette(palette[p]);
//   end;

//   SetAllPalette(palette[0]);
//   for i := 0 to (colors - 1) do
//   begin
//     setcolor(i);
//     setfillstyle(1, i);
//     bar(0, i * 480 div colors, 10, ((i + 1) * 480 div colors) - 1);
//   end;
//   getmem(palleiste, imagesize(0, 0, 10, 479));
//   getimage(0, 0, 10, 479, palleiste^);
// end.
