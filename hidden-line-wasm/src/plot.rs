use crate::appcontext::*;
use crate::drawcontext::*;
use crate::dreidext::*;
use crate::float::*;
use crate::time::*;
use crate::vec3::*;
// use std::time::SystemTime;

// program dreidplot;
// uses ptccrt, ptcgraph, vector, dreidext, projekt, dreiecke, polyswee, polygon,
//   punkte, zeit, linien;
// var
//   ch:char;
//   p:ppoly;
//   palettePos: Integer;

pub fn fkt(x: Float, y: Float) -> Float {
    let h = (x.sqr() + y.sqr()).sqrt();
    30.0 * h.cos() / (2.0 + h)
    // y.sin() * x / 10.0
}

pub fn init() -> AppContext {
    // var {a,b,c:int;}
    //     {h:^triStrip;}
    //     qs:^quadstrip;
    //     xx,yy:float;

    // const
    // //  xs=4;      ys=4;      zs=4;
    // //  xo=3;      yo=3;      zo=3;

    // //  w34=0.43301270189221932338186158537647;

    // {var
    //   r1,r2,r3:float;}

    // begin
    //   tausgabe:=true;

    let Auge = Vector3::new(30.0, 40.0, 50.0);
    let BlickR = Auge.div3d(-10.0);
    //   BlickR = Vector3::new(-3.0, -6.0, -12.0);

    // { tetraeder(0,0,0, -1,0,-2, 1,1,-2 ,1,-1,-2);

    //   cube(-4,2,-2, 3,0,0, 0,3,0, 0,0,3);

    //   kegel(1,3,-2, 1,0,0, 0,1,0, 0,0,2, 8);

    //   kugel(0,8,0, 3, 32, 16); }

    // {  kegel(0,0,0, 2,0,0, 0,2,0, 0,0,-4, 10);  }

    // {  cube (0,0,0, 1,0,0, 0,1,0, 0,0,1);}

    let ad = 13.0;
    let sw = 0.5;
    let mut xx = -ad;

    let mut sceneBuilder = SceneBuilder::new();
    while xx < ad {
        let mut qs = QuadStrip::init(
            sceneBuilder,
            xx,
            -ad,
            fkt(xx, -ad),
            xx + sw,
            -ad,
            fkt(xx + sw, -ad),
        );

        let mut yy = -ad + sw;
        while yy < ad {
            qs.add(xx, yy, fkt(xx, yy), xx + sw, yy, fkt(xx + sw, yy));
            yy = yy + sw;
        }
        xx = xx + sw;
        sceneBuilder = qs.build();
    }

    // {  for a:=1 to 20 do begin
    //     triangle(-a/2,a,-w34*a, a/2,a,-w34*a, 0,a,w34*a);
    //   end;}

    // {  kugel(0,0,20,10,12,12);
    //   kugel(0,0,0,10,12,12);}

    // { kugel(11,11,-10,10,20,20);
    //   kugel(11,-11,-10,10,20,20);
    //   kugel(-11,11,-10,10,20,20);
    //   kugel(-11,-11,-10,10,20,20);
    //   kugel(0,0,5,10,20,20);}

    // {  for a:=-xo to xo do
    //     for b:=-yo to yo do
    //       for c:=-zo to zo do
    //         tetraeder(a*xs,b*ys,c*zs-2,a*xs,b*ys+1,c*zs,a*xs+1,b*ys-1,c*zs,a*xs-1,b*ys-1,c*zs);

    // {        case random(3) of
    //     0:kugel(a*xs,b*ys,c*zs,(xs+ys+zs)*random/8,8,8);
    //     1:begin
    //           r1:=random;
    //           r2:=random;
    //           r3:=random;
    //         cube((a-0.4*r1)*xs,(b-0.4*r2)*ys,(c-0.4*r3)*zs,0.4*(random+r1)*xs,0,0, 0,0.4*(random+r2)*ys,0, 0,0,0.4*(random+r3)*zs);
    //       end;
    //     2:begin
    //           r1:=random;
    //           r2:=random;
    //           kegel(a*xs,b*ys,c*zs-r1,      xs*r2*0.4,0,0,    0,ys*r2*0.4,0, 0,0,zs*(random+r1)*0.4, 8)
    //       end
    //   end}

    AppContext {
        sceneBuilder,
        Auge,
        BlickR,
        iv: Vector3::new(1.0, 0.0, 0.0),
        jv: Vector3::new(0.0, 0.0, 1.0),
        backface: true,
        drawmode: 9,
        ausgabeInsert: false,
        colmode: false,
        zaehl: ctyp::init(),
        tiefePerspektive: minmax::new(),
    }
}

pub fn darstellung(ctx: &mut dyn DrawContext, appCtx: &mut AppContext) {
    appCtx.zaehl = ctyp::init();
    appCtx.neukamera();
    let polys = appCtx.rechnung();

    println!("Anzahl Polygone: {}", appCtx.zaehl.q1.maximum);
    println!("Auge: {}", appCtx.Auge);
    println!("BlickR: {}", appCtx.BlickR);
    println!("Oben x 10000: {}", appCtx.jv.mul3d(10000.0));

    // let start = SystemTime::now();
    //   sweep;
    // println!("Zeit: {}", start.elapsed().unwrap().as_secs());
    appCtx.zaehl.ausgabe();

    //TODO

    for poly in polys {
        poly.drawpoly(ctx, appCtx)
    }
    //   OutInt('höchste Tiefe Suchbaum:',mtf);
    //   if wurzel[1]<>nil then outstring('wurzel[1]');
    //   if wurzel[2]<>nil then outstring('wurzel[2]');
    //   if wurzel[3]<>nil then outstring('wurzel[3]');
    //   if first[1]<>nil then outstring('first[1]');
    //   if first[2]<>nil then outstring('first[2]');
    //   if first[3]<>nil then outstring('first[3]');
    //   while first[1]<>nil do begin
    //     p:=pop(1);
    //     p^.drawpoly;
    //     dispose(p, done);
    //     outstring('f1');
    //   end;
    //   while first[2]<>nil do begin
    //     p:=pop(2);
    //     p^.drawpoly;
    //     dispose(p, done);
    //     outstring('f2');
    //   end;
    //   while first[3]<>nil do begin
    //     p:=pop(3);
    //     p^.drawpoly;
    //     dispose(p, done);
    //     outstring('f3');
    //   end;
    // end;
}

// begin
//   init;
//   repeat
//     darstellung;
//     tastatur;
//   until ch=#27;
//   ende;
// end.
