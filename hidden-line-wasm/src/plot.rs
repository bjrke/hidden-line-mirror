use crate::appcontext::*;
use crate::calcctontext::*;
use crate::drawcontext::*;
use crate::dreidext::*;
use crate::float::*;
use crate::time::*;
use crate::vec3::*;

// program dreidplot;
// uses ptccrt, ptcgraph, vector, dreidext, projekt, dreiecke, polyswee, polygon,
//   punkte, zeit, linien;
// var
//   ch:char;
//   p:ppoly;
//   palettePos: Integer;

pub fn init_scene<F: Fn(Float, Float) -> Float>(f: F) -> SceneBuilder {
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

    // { tetraeder(0,0,0, -1,0,-2, 1,1,-2 ,1,-1,-2);

    //   cube(-4,2,-2, 3,0,0, 0,3,0, 0,0,3);

    //   kegel(1,3,-2, 1,0,0, 0,1,0, 0,0,2, 8);

    //   kugel(0,8,0, 3, 32, 16); }

    // {  kegel(0,0,0, 2,0,0, 0,2,0, 0,0,-4, 10);  }

    // {  cube (0,0,0, 1,0,0, 0,1,0, 0,0,1);}

    let ad = 1.0;
    let sw = 0.04;
    let mut xx = -ad;

    let mut scene = SceneBuilder::new();
    while xx < ad {
        let mut qs = QuadStrip::init(scene, xx, -ad, f(xx, -ad), xx + sw, -ad, f(xx + sw, -ad));

        let mut yy = -ad + sw;
        while yy < ad {
            qs.add(xx, yy, f(xx, yy), xx + sw, yy, f(xx + sw, yy));
            yy = yy + sw;
        }
        xx = xx + sw;
        scene = qs.build();
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
    scene
}

pub fn init() -> AppContext {
    let Auge = Vector3::new(1.5, 2.0, 2.5);
    let BlickR = Auge.div3d(-2.0);

    AppContext {
        sceneBuilder: SceneBuilder::new(),
        Auge,
        BlickR,
        iv: Vector3::new(1.0, 0.0, 0.0),
        jv: Vector3::new(0.0, 0.0, 1.0),
        backface: true,
        drawmode: 0,
        ausgabeInsert: false,
        colmode: false,
        zaehl: ctyp::init(),
    }
}

pub fn darstellung(dctx: &mut dyn DrawContext, actx: &mut AppContext) {
    actx.zaehl = ctyp::init();
    actx.neukamera();
    let polys = actx.rechnung();

    println!("Anzahl Polygone: {}", actx.zaehl.q1.maximum);
    println!("Auge: {}", actx.Auge);
    println!("BlickR: {}", actx.BlickR);
    println!("Oben x 10000: {}", actx.jv.mul3d(10000.0));

    // let start = SystemTime::now();
    //   sweep;
    // println!("Zeit: {}", start.elapsed().unwrap().as_secs());
    actx.zaehl.ausgabe();
    //TODO

    dctx.circle(0.99, 0.99, 0.01, 1.0);
    dctx.circle(0.99, -0.99, 0.01, 1.0);
    dctx.circle(-0.99, 0.99, 0.01, 1.0);
    dctx.circle(-0.99, -0.99, 0.01, 1.0);

    let mut ctx = CalcContext::new();
    ctx.minxQueue = polys;

    ctx.test(dctx, actx);

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
