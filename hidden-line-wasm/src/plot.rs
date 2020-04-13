use crate::appcontext::*;
use crate::calcctontext::*;
use crate::drawcontext::*;
use crate::dreidext::*;
use crate::float::*;

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
        while yy <= ad {
            qs.add(
                Vector3(xx, yy, f(xx, yy)),
                Vector3(xx + sw, yy, f(xx + sw, yy)),
            );
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
    let eye = Vector3(1.5, 2.0, 2.5);
    let view = eye / -2.0;

    AppContext {
        scene_builder: SceneBuilder::new(),
        eye,
        view,
        iv: Vector3(1.0, 0.0, 0.0),
        jv: Vector3(0.0, 0.0, 1.0),
        backface: false,
    }
}

pub fn darstellung(dctx: &mut dyn DrawContext, actx: &mut AppContext) {
    actx.neukamera();
    let polys = actx.rechnung();

    println!("Anzahl Polygone: {}", polys.len());
    println!("Auge: {}", actx.eye);
    println!("BlickR: {}", actx.view);
    println!("Oben x 10000: {}", actx.jv * 10000.0);

    dctx.circle(0.99, 0.99, 0.01, 1.0);
    dctx.circle(0.99, -0.99, 0.01, 1.0);
    dctx.circle(-0.99, 0.99, 0.01, 1.0);
    dctx.circle(-0.99, -0.99, 0.01, 1.0);

    let mut ctx = CalcContext::new();
    ctx.polygons = polys;

    ctx.test(dctx, actx);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::vec2::Vector2;

    struct TestDrawContext {
        lines: Vec<(Float, Float, Float, Float, Color)>,
    }

    impl DrawContext for TestDrawContext {
        fn circle(&mut self, _x: Float, _y: Float, _r: Float, _c: Color) {}

        fn line(&mut self, xa: Float, ya: Float, xe: Float, ye: Float, c: Color) {
            self.lines.push((xa, ya, xe, ye, c));
        }

        fn poly(&mut self, _coordinates: &[Vector2], _c: Color) {}

        fn putpixel(&mut self, _x: i32, _y: i32, _c: Color) {}

        fn cls(&mut self) {}
    }

    impl TestDrawContext {
        fn new() -> TestDrawContext {
            TestDrawContext { lines: vec![] }
        }
    }
    #[test]
    fn integration() {
        let mut actx = init();

        actx.scene_builder = init_scene(|x, y| {
            let mut h = 0.0;
            let step = PI / 36.0;
            let mut a = 0.0;
            while a < PI {
                let c = a.cos();
                let s = a.sin();
                let xd = x * c - y * s;
                let yd = x * s + y * c;
                h += (a * (xd * xd * 25.0 + yd * yd * 100.0).sqrt()).cos() / (PI + a);
                a += step;
            }
            h / 10.0
        });
        actx.backface = false;

        let mut dctx = TestDrawContext::new();
        darstellung(&mut dctx, &mut actx);

        println!("{:?}", dctx.lines);
        println!("{:?}", dctx.lines.len());
    }
}
