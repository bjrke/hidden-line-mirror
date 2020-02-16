use crate::appcontext::*;
use crate::float::*;
use crate::line::*;
use crate::point::*;
use crate::polygon::*;
use crate::triangle::*;
use crate::vec2::*;
use std::collections::HashSet;

// var
//   zumalen: set of byte;
//   mtf: longint;
//   ausgabeInsert: boolean;

pub fn abflachen() {
    // type
    //   fettesfeld = array[0..16382] of ppoly;
    // var
    //   pa: ^fettesfeld;
    //   c: int;
    //   p, q: ppoly;
    //   maxtiefe: integer;

    //   function newtree(a, e: int; tiefe: integer): ppoly;
    //   var
    //     h, h1, h2: ppoly;
    //     x: int;
    //   begin
    //     if (tiefe > maxtiefe) then
    //       maxtiefe := tiefe;
    //     x := a + ((e - a) div 2);
    //     h := pa^[x];
    //     h^.so := nil;
    //     h^.su := nil;
    //     if a <= (x - 1) then
    //       h1 := newtree(a, x - 1, tiefe + 1)
    //     else
    //       h1 := nil;
    //     if (x + 1) <= e then
    //       h2 := newtree(x + 1, e, tiefe + 1)
    //     else
    //       h2 := nil;
    //     verbindeso(h, h1);
    //     verbindesu(h, h2);
    //     exit(h);
    //   end;

    // begin
    //   maxtiefe := 0;
    //   if swurzel <> nil then
    //   begin
    //     new(pa);
    //     p := swurzel;
    //     while p <> nil do
    //     begin
    //       q := p;
    //       p := p^.so;
    //     end;
    //     c := 0;
    //     while q <> nil do
    //     begin
    //       pa^[c] := q;
    //       Inc(c);
    //       q := q^.pu;
    //     end;
    //     swurzel := newtree(0, c - 1, 1);
    //     swurzel^.ss := @swurzel;
    //     dispose(pa);
    //   end;
    //   mtf := 2 * maxtiefe + 5;
    // end;
}

pub fn drawtree(xscan: Float) {
    // procedure drawtree(xscan: float);

    //   procedure dp(p: ppoly; dx, dy: integer; f: color);
    //   begin
    //     if (dy < bmy * 2) and (p <> nil) then
    //     begin
    //       p^.cols := f+1;
    //       p^.drx := dx;
    //       p^.dry := dy;
    //       dp(p^.so, dx + bmx shr (dy div 10), dy + 10, f+1);
    //       dp(p^.su, dx - bmx shr (dy div 10), dy + 10, f+1);
    //     end;
    //   end;

    //   procedure verbindung(s, z: ppoly; c: color);
    //   begin
    //     if (z <> nil) and (z^.dry < bmy * 2) then
    //     begin
    //       setcolor(c);
    //       line(s^.drx, s^.dry, z^.drx, z^.dry);
    //       line(bmx - 4 * s^.dry + round(xscan), bmy + round(s^.yscan(xscan)),
    //         bmx - 4 * z^.dry + round(xscan), bmy + round(z^.yscan(xscan)));
    //     end;
    //   end;

    //   procedure zeichne(p: ppoly);
    //   var
    //     s: string;
    //     ys: int;
    //   begin
    //     if p <> nil then
    //     begin
    //       setcolor(p^.cols);
    //       circle(p^.drx, p^.dry, 3);
    //       verbindung(p, p^.so, 1);
    //       verbindung(p, p^.su, 2);
    //       verbindung(p, p^.pu, 4);
    //       verbindung(p, p^.po, 4);
    //       zeichne(p^.so);
    //       zeichne(p^.su);
    //       if p^.cols in zumalen then
    //         p^.draw3(p^.cols)
    //       else
    //         setcolor(p^.cols);
    //       ys := round(p^.yscan(xscan));
    //       str(ys, s);
    //       outtextxy(bmx - 4 * p^.dry + round(xscan) - 4 * length(s), bmy - ys, s);
    //       outtextxy(p^.drx - 12, p^.dry, s);
    //     end;
    //   end;

    // begin
    //   setcolor(white);
    //   line(round(xscan + bmx), 0, round(xscan + bmx), 2 * bmy - 1);
    //   dp(swurzel, bmx, 10, 1);
    //   zeichne(swurzel);
    // end;
}

pub fn sdelete(xscan: Float, p: &polygon) /* -> &polygon */
{
    // var
    //   h: ppoly;
    // begin
    //   if (p = nil) then
    //   begin
    //     outstring('sdelete(nil)');
    //     exit(nil);
    //   end;

    //   zaehl.suchbaum.del;

    //   h := nil;
    //   if p^.so = nil then
    //     h := p^.su
    //   else if p^.su = nil then
    //     h := p^.so
    //   else
    //   begin
    //     if p^.po = nil then
    //     begin
    //       drawtree(xscan);
    //       outstring('p^.po=nil');
    //     end;
    //     if p^.pu = nil then
    //     begin
    //       drawtree(xscan);
    //       outstring('p^.pu=nil');
    //     end;
    //     rand := not rand;
    //     if rand then
    //     begin
    //       h := p^.pu;
    //       if h^.so <> nil then
    //         outstring('h^.so<>nil');
    //       if h = p^.su then
    //       begin
    //         verbindeso(h, p^.so);
    //       end
    //       else
    //       begin
    //         h^.ss^ := h^.su;
    //         if h^.su <> nil then
    //         begin
    //           h^.su^.ss := h^.ss;
    //         end;
    //         verbindeso(h, p^.so);
    //         verbindesu(h, p^.su);
    //       end;
    //     end
    //     else
    //     begin
    //       h := p^.po;
    //       if h^.su <> nil then
    //         outstring('h^.su<>nil');
    //       if h = p^.so then
    //       begin
    //         verbindesu(h, p^.su);
    //       end
    //       else
    //       begin
    //         h^.ss^ := h^.so;
    //         if h^.so <> nil then
    //         begin
    //           h^.so^.ss := h^.ss;
    //         end;
    //         verbindeso(h, p^.so);
    //         verbindesu(h, p^.su);
    //       end;
    //     end;
    //   end;
    //   p^.ss^ := h;
    //   if h <> nil then
    //     h^.ss := p^.ss;
    //   verbindepu(p^.po, p^.pu);
    //   p^.ss := nil;
    //   p^.po := nil;
    //   p^.pu := nil;
    //   p^.so := nil;
    //   p^.su := nil;

    //   exit(p);
    // end;
}

pub fn lineSetBit(i: u8) -> u8 {
    match i {
        1 => 1,
        2 => 2,
        3 => 4,
        _ => panic!("not more than 3 lines allowed in a triangle {}", i),
    }
}

pub fn polypoly(appCtx: &AppContext, xscan: Float, p1: &dreieck, p2: &polygon) {
    // var
    //   ll: array[1..20] of linie;
    //   am: array[1..20, 1..20] of boolean;
    //   h1, h2, h3, h4, plpos, llpos, i, j, k, pip: int;
    //   intersect: intersectresult;
    //   h: punkt;
    //   li: linie;
    //   w: boolean;

    let mut pl = Vec::new();
    let mut ll = Vec::new();
    let mut am = HashSet::new();

    let mut addpl = |p: &Vector2, ls: u8| {
        pl.push(point::new(p, ls));
        if (appCtx.ausgabeInsert) {
            println!("addpl({}): {}", pl.len(), p);
        }
    };

    for i in 1..3 {
        for j in 1..3 {
            let p1lj = p1.delegate.l(j);
            let p2li = p2.delegate.l(i);
            let intersect = intersect(p1lj, p2li);
            if (intersect.matched == 1) {
                let p1lja = p1lj.a.b;
                let p1lje = p1lj.e.b;
                let p2lia = p2li.a.b;
                let p2lie = p2li.e.b;

                let h = p1lja
                    .mul2d(1.0 - intersect.lambda)
                    .add2d(&p1lje.mul2d(intersect.lambda))
                    .add2d(&p2lia.mul2d(1.0 - intersect.mue))
                    .add2d(&p2lie.mul2d(intersect.lambda))
                    .div2d(2.0);

                addpl(&h, p2.delegate.gl & lineSetBit(i));
            }
        }
    }

    for i in 1..3 {
        let p1pi = p1.delegate.p(i);

        let pip = p2
            .delegate
            .punkttest(&p1pi.b, "polypoly1", appCtx.ausgabeInsert);
        if appCtx.ausgabeInsert {
            println!("pip1 {}", pip);
        }
        match pip {
            0 => addpl(&p1pi.b, 0), // eckpunkte des oberen, die nur im(nicht auf)unteren sind
            1 | 2 | 3 => {
                let h1 = gleicheseite(
                    p1pi,
                    &p2.delegate.l(pip).a,
                    p2.delegate.p(pip),
                    p1.delegate.p(i % 3 + 1),
                );
                let h2 = gleicheseite(
                    p1pi,
                    &p2.delegate.l(pip).a,
                    p2.delegate.p(pip),
                    p1.delegate.p((i + 1) % 3 + 1),
                );
                if h1 == 1.0 || h2 == 1.0 {
                    addpl(&p1pi.b, p2.delegate.gl & lineSetBit(pip));
                }
            }
            11 | 12 | 13 => {
                let p1n = p1.delegate.p(i % 3 + 1);
                let p1l = p1.delegate.p((i + 1) % 3 + 1);
                let p2l = p2.delegate.p((pip - 1) % 3 + 1);
                let p2n = p2.delegate.p(pip % 3 + 1);
                let h1 = gleicheseite(p1pi, p1n, p2l, p1l);
                let h2 = gleicheseite(p1pi, p1l, p2l, p1n);
                let h3 = gleicheseite(p1pi, p1n, p2n, p1l);
                let h4 = gleicheseite(p1pi, p1l, p2n, p1n);
                if h1 == -1.0 || h2 == -1.0 || h3 == -1.0 || h4 == -1.0 {
                    addpl(&p1pi.b, p2.delegate.gl & !lineSetBit(pip - 10));
                }
            }
            _ => { // 3 draußen brauchen wir nich
            }
        }
    }

    for i in 1..3 {
        let p2pi = p2.delegate.p(i);
        let pip = p1
            .delegate
            .punkttest(&p2pi.b, "polypoly2", appCtx.ausgabeInsert);
        if appCtx.ausgabeInsert {
            println!("pip2 {}", pip);
        }

        //eckpunkt unteres dreieck
        match pip {
            1 | 2 | 3 => {
                let h1 = gleicheseite(
                    p2pi,
                    &p1.delegate.l(pip).a,
                    p1.delegate.p(pip),
                    p2.delegate.p(i % 3 + 1),
                );
                let h2 = gleicheseite(
                    p2pi,
                    &p1.delegate.l(pip).a,
                    p1.delegate.p(pip),
                    p2.delegate.p((i + 1) % 3 + 1),
                );
                if h1 == -1.0 || h2 == -1.0 {
                    addpl(&p2pi.b, p2.delegate.gl & !lineSetBit(i));
                }
            }
            20 => addpl(&p2pi.b, p2.delegate.gl & !lineSetBit(i)),
            //eckpunkte des unteren, die alle draußen sind
            _ => {}
        }
    }

    for (i, pli) in pl.iter().enumerate() {
        for (j, plj) in pl.iter().take(i).enumerate() {
            let li = Line::new(pli, plj);
            if !p1.delegate.linientest(&li, appCtx.ausgabeInsert) {
                if !ll.iter().any(|l| match intersect(&li, l).matched {
                    1 | 2 | 3 => true,
                    _ => false,
                }) {
                    ll.push(li);
                    am.insert((i, j));
                }
            }
        }
    }

    for (k, plk) in pl.iter().enumerate() {
        for (j, plj) in pl.iter().take(k).enumerate() {
            for (i, pli) in pl.iter().take(j).enumerate() {
                if am.contains(&(i, j)) && am.contains(&(j, k)) && am.contains(&(k, i)) {
                    //     Inc(zaehl.pp);

                    let h = pli.b.add2d(&plj.b).add2d(&plk.b).div2d(3.0);

                    if p1.delegate.punkttest(&h, "addppl1", appCtx.ausgabeInsert) == 20
                        && !colinear(&pli.b, &plj.b, &plk.b)
                    {
                        let mut ls = 0;
                        if plj.gz & plk.gz != 0 {
                            ls += 1;
                        }
                        if plk.gz & pli.gz != 0 {
                            ls += 2;
                        }
                        if pli.gz & plj.gz != 0 {
                            ls += 4;
                        }

                        let mut ph =
                            polygon::initpoly(pli, plj, plk, ls, p2.originalTriangle.clone());

                        let h = p1
                            .delegate
                            .p1
                            .b
                            .add2d(&p1.delegate.p2.b)
                            .add2d(&p1.delegate.p3.b)
                            .div2d(3.0);

                        if ph.delegate.punkttest(&h, "addppl2", appCtx.ausgabeInsert) == 20
                            && ph.delegate.flaechentest()
                        {
                            if (ph.delegate.p1.b.x >= xscan) {
                                ph.delegate.cols = 2.0 / 16.0; //15
                                                               // push(ph, 1, zaehl.Count);
                            } else {
                                ph.delegate.cols = 3.0 / 16.0; //4
                                                               // push(ph, 2, zaehl.Count);
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn loesche(xscan: Float, p: &polygon) /* -> &polygon */
{
    // var
    //   o, u, h: ppoly;
    // begin
    //   if (p = nil) then
    //   begin
    //     outstring('p ist nil');
    //     exit(nil);
    //   end;

    //   o := p^.po;
    //   u := p^.pu;
    //   sdelete(xscan, p);
    //   del(p, 3);
    //   while (o <> nil) and (u <> nil) do
    //   begin
    //     case polytest(xscan, o, u, True, False) of
    //       1:
    //       begin
    //         h := o;
    //         o := o^.po;
    //         sdelete(xscan, h);
    //         del(h, 3);
    //         push(h, 2, h^.Count);
    //       end;
    //       2: exit(p);
    //       4:
    //       begin
    //         h := o;
    //         o := o^.po;
    //         sdelete(xscan, h);
    //         del(h, 3);
    //         if h <> nil then
    //         begin
    //           polypoly(xscan, u^.originalTriangle, h);
    //           dispose(h, done);
    //         end
    //         else
    //           outstring('h is nil (falls3)');
    //       end;
    //       3:
    //       begin
    //         h := u;
    //         u := u^.pu;
    //         sdelete(xscan, h);
    //         del(h, 3);
    //         if h <> nil then
    //         begin
    //           polypoly(xscan, o^.originalTriangle, h);
    //           dispose(h, done);
    //         end
    //         else
    //           outstring('h is nil (falls4)');
    //       end;
    //       5:
    //       begin
    //         h := o;
    //         o := o^.po;
    //         sdelete(xscan, h);
    //         del(h, 3);
    //         outint('l5 ozähler ', h^.Count);
    //         dispose(h, done);

    //         h := u;
    //         u := u^.pu;
    //         sdelete(xscan, h);
    //         del(h, 3);
    //         dispose(h, done);
    //         outint('l5 uzähler', h^.Count);
    //       end
    //       else
    //     end;
    //   end;
    //   exit(p);
    // end;
}

pub fn insert(xscan: Float, p: &polygon, schnitttest: bool) {

    // var
    //   h, o, u, a: ppoly;
    //   ak: pppoly;
    //   typ: richtung;
    //   ch: char;
    //   tf: int;
    // label
    //   ende;
    // begin
    //   {  schnitttest:=true;}
    //   if p^.flaechentest then
    //   begin
    //     Inc(zaehl.Count);
    //     if drawmode = 6 then
    //     begin
    //       p^.draw2(15);
    //       outint('zähler', zaehl.Count);
    //       outstring(format('insert %p', [p]));
    //     end;
    //     tf := 0;
    //     a := nil;
    //     ak := @swurzel;
    //     while ak^ <> nil do
    //     begin
    //       case polytest(xscan, ak^, p, schnitttest, ausgabeInsert) of
    //         1:
    //         begin
    //           a := ak^;
    //           ak := @ak^^.so;
    //           typ := so;
    //           Inc(tf);
    //           if (ausgabeInsert) then
    //             outstring('f1so');
    //         end;
    //         2:
    //         begin
    //           a := ak^;
    //           ak := @ak^^.su;
    //           typ := su;
    //           Inc(tf);
    //           if (ausgabeInsert) then
    //             outstring('f2su');
    //         end;
    //         3:
    //         begin
    //           if p <> nil then
    //           begin
    //             polypoly(xscan, ak^^.originalTriangle, p);
    //             dispose(p, done);
    //             if (ausgabeInsert) then
    //               outstring('f3u');
    //           end
    //           else
    //             outstring('p is nil (fall3)');
    //           goto ende;
    //         end;
    //         4:
    //         begin
    //           if (ausgabeInsert) then
    //             outstring('f4o');
    //           h := loesche(xscan, ak^);
    //           if h <> nil then
    //           begin
    //             polypoly(xscan, p^.originalTriangle, h);
    //             dispose(h, done);
    //           end
    //           else
    //             outstring('h is nil (fall4)');
    //         end;
    //         5:
    //         begin
    //           outint('i5 pzähler', p^.Count);
    //           dispose(p, done);
    //           p := loesche(xscan, ak^);
    //           outint('i5 akzähler', p^.Count);
    //           dispose(p, done);
    //           goto ende;
    //         end
    //         else
    //       end;
    //     end;
    //     if a <> nil then
    //     begin
    //       if typ = so then
    //       begin
    //         verbindeso(a, p);
    //         o := a^.po;
    //         u := a;
    //       end
    //       else
    //       begin
    //         verbindesu(a, p);
    //         o := a;
    //         u := a^.pu;
    //       end;
    //       verbindepu(o, p);
    //       verbindepu(p, u);
    //       p^.so := nil;
    //       p^.su := nil;
    //       push(p, 3, p^.Count);
    //     end
    //     else
    //     begin
    //       swurzel := p;
    //       p^.po := nil;
    //       p^.pu := nil;
    //       p^.so := nil;
    //       p^.su := nil;
    //       p^.pr := nil;
    //       p^.ss := @swurzel;
    //       push(swurzel, 3, p^.Count);
    //     end;
    //     zaehl.suchbaum.ins;
    //     ende:
    //       if tf > mtf then
    //       begin
    //         abflachen;
    //       end;
    //     if drawmode = 6 then
    //     begin
    //       repeat
    //         drawtree(xscan);
    //         ch := readkey2([#32, #27, '1'..'9', 'a']);
    //         if ch in ['1'..'9'] then
    //         begin
    //           cls;
    //           if (Ord(ch) - Ord('0')) in zumalen then
    //             zumalen := zumalen - [(Ord(ch) - Ord('0'))]
    //           else
    //             zumalen := zumalen + [(Ord(ch) - Ord('0'))];
    //         end;
    //         if ch = 'a' then
    //           abflachen;
    //         if ch = #27 then
    //           drawmode := 1;
    //       until ch in [#32, #27];
    //       cls;
    //     end;
    //   end;
}

pub fn sweep() {
    // var
    //   p: ppoly;
    //   ende: boolean;
    //   xscan: float;
    // begin
    //   mtf := 0;
    //   swurzel := polygon.pop(1);
    //   zaehl.suchbaum.ins;
    //   if swurzel <> nil then
    //   begin
    //     swurzel^.ss := @swurzel;
    //     swurzel^.so := nil;
    //     swurzel^.su := nil;
    //     swurzel^.po := nil;
    //     swurzel^.pu := nil;
    //     push(swurzel, 3, swurzel^.Count);
    //     xscan := swurzel^.p[1]^.b.x;
    //   end;
    //   ende := False;
    //   while not ende do
    //   begin
    //     if First[1] = nil then
    //     begin
    //       if First[3] = nil then
    //         ende := True
    //       else
    //       begin
    //         p := loesche(xscan, First[3]);
    //         p^.cols := 15; {5}
    //         p^.drawpoly;
    //         dispose(p, done);
    //       end;
    //     end
    //     else
    //     begin
    //       p := polygon.pop(1);
    //       xscan := p^.p[1]^.b.x;
    //       insert(xscan, p, True);

    //       if First[1] <> nil then
    //         xscan := First[1]^.p[1]^.b.x;

    //       while (First[3] <> nil) and (First[3]^.p[3]^.b.x <= xscan + epsilon1) do
    //       begin

    //         while First[2] <> nil do
    //         begin
    //           p := polygon.pop(2);
    //           insert(xscan, p, False);
    //         end;
    //         p := loesche(xscan, First[3]);
    //         p^.cols := 15; {6}
    //         p^.drawpoly;
    //         dispose(p, done);
    //       end;
    //     end;
    //     if not ende then
    //     begin
    //       p := polygon.pop(2);
    //       while p <> nil do
    //       begin
    //         insert(xscan, p, False);
    //         p := polygon.pop(2);
    //       end;
    //     end;
    //   end;
    // end;
}
// begin
//   ausgabeInsert := False;
// end.

//   richtung = (li, re, so, su, po, pu, pr, ne);
