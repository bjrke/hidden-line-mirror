use crate::appcontext::*;
use crate::drawcontext::*;
use crate::float::*;
use crate::line::*;
use crate::point::*;
use crate::triangle::*;
use crate::vec2::*;
use rand::Rng;
use std::rc::Rc;

extern crate rand;

pub struct polygon {
    pub delegate: dreiecktyp,
    pub originalTriangle: Rc<dreieck>,
    pub farbe: Color,
    pub ymin: Float,
    pub ymax: Float,
}

impl polygon {
    pub fn initpoly(
        p1: &point,
        p2: &point,
        p3: &point,
        ls: u8,
        originalTriangle: Rc<dreieck>,
    ) -> polygon {
        //   zaehl.polygons.ins;

        //TODO copy?
        let mut p1 = *p1;
        let mut p2 = *p2;
        let mut p3 = *p3;

        let mut gl = ls;

        if p1.b.x > p2.b.x {
            let h = p2;
            p2 = p1;
            p1 = h;
            let mut glneu = gl & 4;
            if gl & 1 != 0 {
                glneu += 2;
            }
            if gl & 2 != 0 {
                glneu += 1;
            }
            gl = glneu;
        }

        if p1.b.x > p3.b.x {
            let h = p3;
            p3 = p1;
            p1 = h;
            let mut glneu = gl & 2;
            if gl & 1 != 0 {
                glneu += 4;
            }
            if gl & 4 != 0 {
                glneu += 1;
            }
            gl = glneu;
        }

        if p2.b.x > p3.b.x {
            let h = p3;
            p3 = p2;
            p2 = h;
            let mut glneu = gl & 1;
            if gl & 2 != 0 {
                glneu += 4;
            }
            if gl & 4 != 0 {
                glneu += 2;
            }
            gl = glneu;
        }

        let cols = originalTriangle.delegate.cols;

        polygon {
            delegate: dreiecktyp {
                p1: p1,
                p2: p2,
                p3: p3,
                l1: Line::new(&p2, &p3),
                l2: Line::new(&p3, &p1),
                l3: Line::new(&p1, &p2),
                gl,
                cols,
            },
            farbe: cols,
            originalTriangle,
            ymin: p1.b.y.min(p2.b.y).min(p3.b.y),
            ymax: p1.b.y.max(p2.b.y).max(p3.b.y),
        }
    }
    pub fn newpoly(aOriginalTriangle: Rc<dreieck>) -> polygon {
        polygon::initpoly(
            &aOriginalTriangle.delegate.p1,
            &aOriginalTriangle.delegate.p2,
            &aOriginalTriangle.delegate.p3,
            aOriginalTriangle.delegate.gl,
            aOriginalTriangle.clone(), //TODO find a way to prevent cloning
        )
    }

    pub fn yscan(&self, xscan: Float) -> Float {
        let dreiecktyp { p1, p2, p3, .. } = self.delegate;
        let mut x1 = p1.b.x;
        let mut y1 = p1.b.y;
        let mut x2 = p3.b.x;
        let mut y2 = p3.b.y;

        let miny = y1.min(y2).min(p2.b.y);
        let maxy = y1.max(y2).max(p2.b.y);

        let mut h;
        if (x2 - x1).abs() < epsilon1 {
            h = y1 + y2
        } else {
            h = y1 + (y2 - y1) * (xscan - x1) / (x2 - x1);
            if xscan < p2.b.x {
                x2 = p2.b.x;
                y2 = p2.b.y;
            } else {
                x1 = p2.b.x;
                y1 = p2.b.y;
            }
            if (x2 - x1).abs() < epsilon1 {
                h = y1 + y2
            } else {
                h = h + y1 + (y2 - y1) * (xscan - x1) / (x2 - x1);
            }
        }

        if h < 2.0 * miny || h > 2.0 * maxy {
            h = miny + maxy;
        }

        h / 2.0
    }

    fn draw4(&self, ctx: &mut dyn DrawContext) {
        //TODO use min and max to get my1, my2

        let p1 = self.delegate.p1;
        let p2 = self.delegate.p2;
        let p3 = self.delegate.p3;
        let mut my1 = p1.b.y;
        let mut my2 = my1;
        if p2.b.y > my1 {
            my2 = p2.b.y
        } else {
            my1 = p2.b.y;
        }

        if p3.b.y > my2 {
            my2 = p3.b.y;
        } else if p3.b.y < my1 {
            my1 = p3.b.y;
        }

        let ym1 = my1.round() as i32;
        let ym2 = my2.round() as i32;
        let xm1 = p1.b.x.round() as i32;
        let xm2 = p3.b.x.round() as i32;

        for wx in xm1..xm2 {
            for wy in ym1..ym2 {
                let h = Vector2::new(wx as Float, wy as Float);
                if self.delegate.punkttest(&h, "draw", false) == 0 {
                    let col = 0.5 + 0.5 * self.yscan(wx as Float);
                    ctx.putpixel(wx, wy, col);
                }
            }
        }
    }

    fn draw5(&self, ctx: &mut dyn DrawContext, appCtx: &AppContext) {
        //TODO use min and max to get my1, my2
        let p1 = self.delegate.p1;
        let p2 = self.delegate.p2;
        let p3 = self.delegate.p3;
        let mut my1 = p1.b.y;
        let mut my2 = my1;
        if p2.b.y > my1 {
            my2 = p2.b.y
        } else {
            my1 = p2.b.y;
        }

        if p3.b.y > my2 {
            my2 = p3.b.y;
        } else if p3.b.y < my1 {
            my1 = p3.b.y;
        }

        let ym1 = my1.round() as i32;
        let ym2 = my2.round() as i32;
        let xm1 = p1.b.x.round() as i32;
        let xm2 = p3.b.x.round() as i32;

        for wx in xm1..xm2 {
            for wy in ym1..ym2 {
                let h = Vector2::new(wx as Float, wy as Float);
                if self.delegate.punkttest(&h, "draw2", false) == 0 {
                    let my1 = 1.0 + 7.0 * self.yscan(wx as Float);

                    let col = calcColor(
                        appCtx
                            .tiefePerspektive
                            .relative(self.originalTriangle.tiefe(appCtx, &h)),
                    );

                    ctx.putpixel(wx, wy, col);
                }
            }
        }
    }

    pub fn drawpoly(&self, ctx: &mut DrawContext, appCtx: &AppContext) {
        let cols = if appCtx.colmode {
            rand::thread_rng().gen()
        } else {
            self.delegate.cols
        };

        match appCtx.drawmode {
            1 => self.delegate.draw1(ctx, cols),
            2 => self.delegate.draw2(ctx, cols),
            3 => self.delegate.draw3(ctx, cols),
            4 => self.draw4(ctx),
            5 => self.draw5(ctx, appCtx),
            6 => self.delegate.draw1(ctx, cols),
            7 => self
                .delegate
                .draw2(ctx, self.originalTriangle.delegate.cols),
            8 => self.delegate.draw1(ctx, self.farbe),
            9 => self
                .delegate
                .draw1(ctx, self.originalTriangle.delegate.cols),
            _ => panic!("unexpected drawmode"),
        }
    }
}

pub fn polytest(
    ctx: &AppContext,
    xscan: Float,
    p1: &polygon,
    p2: &polygon,
    schnitttest: bool,
    ausgabe: bool,
) -> u8 {
    // Inc(zaehl.ptest);

    if p1.ymin - 1.0 > p2.ymax {
        return 1;
    }

    if p2.ymin - 1.0 > p1.ymax {
        return 2;
    }

    let mut schnitt = false;

    let mut h: Vector2 = Vector2::new(0.0, 0.0);

    let mut v2 = false;
    let mut v1 = false;
    let mut test = move |p: &Vector2| {
        if p1.delegate.punkttest(&h, "polytest.test1", ausgabe) == 0
            && p2.delegate.punkttest(&h, "polytest.test2", ausgabe) == 0
        {
            let d1 = p1.originalTriangle.tiefe(ctx, p);
            let d2 = p2.originalTriangle.tiefe(ctx, p);

            schnitt = (d1 - d2).abs() > epsilon1;

            if Rc::ptr_eq(&p1.originalTriangle, &p2.originalTriangle) {
                v2 = true;
            } else {
                v1 = d1 < d2;
            }

            if ausgabe {
                println!("v1: {} v2: {} schnitt: {}", v1, v2, schnitt);
                println!("p {} d1 {} d2 {} d1-d2 {}", p, d1, d2, d1 - d2);
            }
        }
    };

    if schnitttest && !Rc::ptr_eq(&p1.originalTriangle, &p2.originalTriangle) {
        let mut k = 0;

        let mut i = 1;
        let mut j = 1;
        while (k < 6) && (j <= 3) {
            let p1lj = p1.delegate.l(j);
            let p2li = p2.delegate.l(i);
            let interset = intersect(p1lj, p2li);
            if interset.matched == 1 {
                h = h.add2d(
                    &p1lj
                        .a
                        .b
                        .add2d(&p2li.a.b)
                        .mul2d(1.0 - interset.lambda)
                        .add2d(&p1lj.e.b.mul2d(interset.lambda)),
                );
                k += 1;
            }

            i += 1;
            if i == 4 {
                j += 1;
                i = 1;
            }
        }

        h = h.div2d(2.0);

        i = 1;
        while
        /* k < 6 && */
        i <= 3 {
            let p1pi = p1.delegate.p(i);
            let pip = p2.delegate.punkttest(&p1pi.b, "polytest1", ausgabe);
            if ausgabe {
                println!("pip3 {}", pip);
            }

            match pip {
                0 => {
                    h = h.add2d(&p1pi.b);
                    k += 1;
                }
                // eckpunkte des oberen, die nur im(nicht auf)unteren sind
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
                    //TODO check epsilon?
                    if h1 == 1.0 || h2 == 1.0 {
                        h = h.add2d(&p1pi.b);
                        k += 1;
                    }
                }
                11 | 12 | 13 => {
                    let h1 = gleicheseite(
                        p1pi,
                        p1.delegate.p(i % 3 + 1),
                        p2.delegate.p(((pip - 1) % 3) + 1),
                        p1.delegate.p(((i + 1) % 3) + 1),
                    );
                    let h2 = gleicheseite(
                        p1pi,
                        p1.delegate.p(((i + 1) % 3) + 1),
                        p2.delegate.p(((pip - 1) % 3) + 1),
                        p1.delegate.p(i % 3 + 1),
                    );
                    let h3 = gleicheseite(
                        p1pi,
                        p1.delegate.p(i % 3 + 1),
                        p2.delegate.p(pip % 3 + 1),
                        p1.delegate.p(((i + 1) % 3) + 1),
                    );
                    let h4 = gleicheseite(
                        p1pi,
                        p1.delegate.p(((i + 1) % 3) + 1),
                        p2.delegate.p(pip % 3 + 1),
                        p1.delegate.p(i % 3 + 1),
                    );
                    //TODO check epsilon?
                    if h1 == -1.0 || h2 == -1.0 || h3 == -1.0 || h4 == -1.0 {
                        h = h.add2d(&p1pi.b);
                        k += 1;
                    }
                }
                _ => {}
            }

            i += 1;
        }
        i = 1;
        while
        /* k < 6 && */
        i <= 3 {
            let p2pi = p2.delegate.p(i);
            let pip = p1.delegate.punkttest(&p2pi.b, "polytest2", ausgabe);
            if ausgabe {
                println!("pip4 {}", pip);
            }
            match pip {
                0 => {
                    h = h.add2d(&p2pi.b);
                    k += 1;
                }
                //eckpunkte des oberen, die nur im(nicht auf)unteren sind
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
                        p2.delegate.p(((i + 1) % 3) + 1),
                    );
                    if h1 == 1.0 || h2 == 1.0 {
                        h = h.add2d(&p2pi.b);
                        k += 1;
                    }
                }
                11 | 12 | 13 => {
                    let h1 = gleicheseite(
                        p2pi,
                        p2.delegate.p(i % 3 + 1),
                        p1.delegate.p(((pip - 1) % 3) + 1),
                        p2.delegate.p(((i + 1) % 3) + 1),
                    );
                    let h2 = gleicheseite(
                        p2pi,
                        p2.delegate.p(((i + 1) % 3) + 1),
                        p1.delegate.p(((pip - 1) % 3) + 1),
                        p2.delegate.p(i % 3 + 1),
                    );
                    let h3 = gleicheseite(
                        p2pi,
                        p2.delegate.p(i % 3 + 1),
                        p1.delegate.p(pip % 3 + 1),
                        p2.delegate.p(((i + 1) % 3) + 1),
                    );
                    let h4 = gleicheseite(
                        p2pi,
                        p2.delegate.p(((i + 1) % 3) + 1),
                        p1.delegate.p(pip % 3 + 1),
                        p2.delegate.p(i % 3 + 1),
                    );
                    if h1 == -1.0 || h2 == -1.0 || h3 == -1.0 || h4 == -1.0 {
                        h = h.add2d(&p2pi.b);
                        k += 1;
                    }
                }
                _ => {}
            }
            i += 1;
        }

        // {    while (k<6)and(i<=3)do begin
        //       if p1^.punkttest(p2^.p[i]^.b)=0 then begin
        //         inc(k,2);
        //         h[x]:=h[x]+p2^.p[i]^.b[x];
        //         h[y]:=h[y]+p2^.p[i]^.b[y];
        //       end;
        //       inc(i);
        //     end;}

        if k > 0 {
            h = h.div2d(k as Float);
            test(&h);
        }
    }

    let p1p1b = &p1.delegate.p1.b;
    let p1p2b = &p1.delegate.p2.b;
    let p1p3b = &p1.delegate.p3.b;

    if !schnitt {
        test(&p1p1b.add2d(p1p2b).add2d(p1p3b).div2d(3.0));
    }

    let p2p1b = &p2.delegate.p1.b;
    let p2p2b = &p2.delegate.p2.b;
    let p2p3b = &p2.delegate.p3.b;

    if !schnitt {
        test(&p2p1b.add2d(&p2p2b).add2d(p2p3b).div2d(3.0));
    }

    if !schnitt {
        test(
            &p1p1b
                .add2d(&p1p2b)
                .add2d(&p1p3b)
                .add2d(&p2p1b)
                .add2d(&p2p2b)
                .add2d(p2p3b)
                .div2d(6.0),
        );
    }

    let result = if v2 {
        5
    } else if schnitt {
        if v1 {
            3
        } else {
            4
        }
    } else {
        let xscanHelp = (p1p1b.x.max(p2p1b.x) + p1p3b.x.min(p2p3b.x)) / 2.0;

        //   if drawmode = 6 then
        //   begin
        //     marke(round(bmx + xscanHelp), round(bmy - p1^.yscan(xscanHelp)),
        //       yellow, 'p1^.yscan');
        //     marke(round(bmx + xscanHelp), round(bmy - p2^.yscan(xscanHelp)),
        //       lightmagenta, 'p2^.yscan');
        //   end;

        if p1.yscan(xscanHelp) > p2.yscan(xscanHelp) {
            1
        } else {
            2
        }
    };

    if ausgabe {
        //     begin
        //       outint('polytest: ', Result);
        //       outvector2d('p1^.p[1]^.b', p1^.p[1]^.b);
        //       outvector2d('p1^.p[2]^.b', p1^.p[2]^.b);
        //       outvector2d('p1^.p[3]^.b', p1^.p[3]^.b);
        //       outvector2d('p1^.originalTriangle^.p[1]^.b', p1^.originalTriangle^.p[1]^.b);
        //       outvector2d('p1^.originalTriangle^.p[2]^.b', p1^.originalTriangle^.p[2]^.b);
        //       outvector2d('p1^.originalTriangle^.p[3]^.b', p1^.originalTriangle^.p[3]^.b);
        //       outvector2d('p2^.p[1]^.b', p2^.p[1]^.b);
        //       outvector2d('p2^.p[2]^.b', p2^.p[2]^.b);
        //       outvector2d('p2^.p[3]^.b', p2^.p[3]^.b);
        //       outvector2d('p2^.originalTriangle^.p[1]^.b', p2^.originalTriangle^.p[1]^.b);
        //       outvector2d('p2^.originalTriangle^.p[2]^.b', p2^.originalTriangle^.p[2]^.b);
        //       outvector2d('p2^.originalTriangle^.p[3]^.b', p2^.originalTriangle^.p[3]^.b);
        //       p1^.draw3(1);
        //       p2^.draw3(2);

        //       readkey;
    }

    result
}

// type
//   punr = 1..3;

//   pppoly = ^ppoly;
//   ppoly = ^poly;

//   poly = object(dreiecktyp)
//     li, re, ne, pr: ppoly;
//     so, su, pu, po: ppoly;
//     ss, ps: pppoly;
//     originalTriangle: pdreieck;
//     drx, dry: integer;
//     farbe: color;
//     Count: int;
//     ymin, ymax: float;

//     destructor done;

//   end;

// var
//   swurzel: ppoly;
//   wurzel, First: array [punr] of ppoly;
//   colmode: boolean;
//   drawmode: integer;
//   rand: boolean;
//   tiefePerspektive: minmax;

// procedure push(p: ppoly; pnr: punr; c: int);
// function pop(pnr: punr): ppoly;
// function del(p: ppoly; pnr: punr): ppoly;
//
// procedure verbindeso(v, s: ppoly);
// procedure verbindesu(v, s: ppoly);
// procedure verbindeli(v, s: ppoly);
// procedure verbindere(v, s: ppoly);
// procedure verbindepo(v, s: ppoly);
// procedure verbindepu(v, s: ppoly);
// procedure verbindepr(v, s: ppoly);
// procedure verbindene(v, s: ppoly);

// implementation

// destructor poly.done;                   {hier druff guggn!}
// var
//   i: integer;
// begin
//   {  outstring(aufr,false);}
//   for i := 1 to 3 do
//     if p[i] <> nil then
//       dispose(p[i], done)
//     else
//       outstring('p[i]=nil');
//   zaehl.polygons.del;
// end;

// procedure push;
// var
//   a: ppoly;
//   fertig: boolean;

// begin
//   p^.Count := c;
//   if pnr <> 3 then
//     p^.farbe := pnr;
//   zaehl.q[pnr].ins;
//   p^.ne := nil;
//   p^.li := nil;
//   p^.re := nil;
//   p^.ps := nil;
//   p^.pr := nil;
//   if wurzel[pnr] = nil then
//   begin
//     wurzel[pnr] := p;
//     p^.ps := @wurzel[pnr];
//     First[pnr] := p;
//   end
//   else
//   begin
//     a := wurzel[pnr];
//     fertig := False;
//     repeat
//       if p^.p[pnr]^.b.x <= a^.p[pnr]^.b.x then
//       begin
//         if a^.li <> nil then
//         begin
//           a := a^.li;
//         end
//         else
//         begin
//           verbindeli(a, p);
//           verbindene(a^.pr, p);
//           verbindene(p, a);
//           fertig := True;
//         end;
//       end
//       else
//       begin
//         if a^.re <> nil then
//         begin
//           a := a^.re;
//         end
//         else
//         begin
//           verbindere(a, p);
//           verbindene(p, a^.ne);
//           verbindene(a, p);
//           fertig := True;
//         end;
//       end
//     until fertig;
//   end;
// {  if first[pnr]=nil then begin
//     first[pnr]:=wurzel[pnr];
//     outstring('first[pnr]is nil',false);
//   end;}
//   while First[pnr]^.li <> nil do
//     First[pnr] := First[pnr]^.li;
// end;

// function pop;
// begin
//   exit(del(First[pnr], pnr));
// end;

// function del;
// var
//   h: ppoly;
// begin
//   if p = nil then
//   begin
//     exit(nil);
//   end;
//   zaehl.q[pnr].del;
//   if p = First[pnr] then
//     First[pnr] := First[pnr]^.ne;
//   h := nil;
//   if p^.re = nil then
//     h := p^.li
//   else if p^.li = nil then
//     h := p^.re
//   else
//   begin
//     if p^.ne = nil then
//     begin
//       outstring('p^.ne=nil');
//     end;
//     if p^.pr = nil then
//     begin
//       outstring('p^.pr=nil');
//     end;
//     rand := not rand;
//     if rand then
//     begin
//       h := p^.pr;
//       if h^.re <> nil then
//         outstring('h^.re<>nil');
//       if h = p^.li then
//       begin
//         verbindere(h, p^.re);
//       end
//       else
//       begin
//         h^.ps^ := h^.li;
//         if h^.li <> nil then
//         begin
//           h^.li^.ps := h^.ps;
//         end;
//         verbindere(h, p^.re);
//         verbindeli(h, p^.li);
//       end;
//     end
//     else
//     begin
//       h := p^.ne;
//       if h^.li <> nil then
//         outstring('h^.li<>nil');
//       if h = p^.re then
//       begin
//         verbindeli(h, p^.li);
//       end
//       else
//       begin
//         h^.ps^ := h^.re;
//         if h^.re <> nil then
//         begin
//           h^.re^.ps := h^.ps;
//         end;
//         verbindere(h, p^.re);
//         verbindeli(h, p^.li);
//       end;
//     end;
//   end;
//   p^.ps^ := h;
//   if h <> nil then
//     h^.ps := p^.ps;
//   verbindepr(p^.ne, p^.pr);
//   p^.ps := nil;
//   p^.li := nil;
//   p^.re := nil;
//   p^.ne := nil;
//   p^.pr := nil;

//   exit(p);
// end;

// procedure verbindeso;
// begin
//   if v <> nil then
//     v^.so := s;
//   if s <> nil then
//   begin
//     if v <> nil then
//       s^.ss := @v^.so
//     else
//       s^.ss := nil;
//   end;
// end;

// procedure verbindesu;
// begin
//   if v <> nil then
//     v^.su := s;
//   if s <> nil then
//   begin
//     if v <> nil then
//       s^.ss := @v^.su
//     else
//       s^.ss := nil;
//   end;
// end;

// procedure verbindeli;
// begin
//   if v <> nil then
//     v^.li := s;
//   if s <> nil then
//   begin
//     if v <> nil then
//       s^.ps := @v^.li
//     else
//       s^.ps := nil;
//   end;
// end;

// procedure verbindere;
// begin
//   if v <> nil then
//     v^.re := s;
//   if s <> nil then
//   begin
//     if v <> nil then
//       s^.ps := @v^.re
//     else
//       s^.ps := nil;
//   end;
// end;

// procedure verbindepo;
// begin
//   if v <> nil then
//     v^.po := s;
//   if s <> nil then
//     s^.pu := v;
// end;

// procedure verbindepu;
// begin
//   if v <> nil then
//     v^.pu := s;
//   if s <> nil then
//     s^.po := v;
// end;

// procedure verbindepr;
// begin
//   if v <> nil then
//     v^.pr := s;
//   if s <> nil then
//     s^.ne := v;
// end;

// procedure verbindene;
// begin
//   if v <> nil then
//     v^.ne := s;
//   if s <> nil then
//     s^.pr := v;
// end;

// begin
//   colmode := False;
//   drawmode := 1;
//   randomize;
//   wurzel[1] := nil;
//   First[1] := nil;
//   wurzel[3] := nil;
//   First[3] := nil;
//   swurzel := nil;
//   rand := False;
// end.
