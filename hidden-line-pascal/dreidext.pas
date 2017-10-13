unit dreidext;
interface
uses dreiecke,vector,punkte;
type
  TriFan=Object
    Center,Last,First:ppunkt3d;
    constructor Init(cx,cy,cz,ax,ay,az,bx,by,bz:float);
    destructor done;
    function add(ax,ay,az:float):ppunkt3d;
  end;
  TriStrip=Object
    l1,l2:ppunkt3d;
    w:boolean;
    constructor Init(cx,cy,cz,ax,ay,az,bx,by,bz:float);
    function add(ax,ay,az:float):ppunkt3d;
  end;
  QuadStrip=Object
    l1,l2:ppunkt3d;
    constructor Init(ax,ay,az,bx,by,bz,dx,dy,dz,cx,cy,cz:float);
    procedure add(bx,by,bz,ax,ay,az:float);
    destructor done;
  end;
procedure tetraeder(ax,ay,az,bx,by,bz,cx,cy,cz,dx,dy,dz:float);
procedure kugel(mx,my,mz,r:float;l,b:int);
procedure kegel(mx,my,mz,r1x,r1y,r1z,r2x,r2y,r2z,hx,hy,hz:float;b:int);
procedure cube(ex,ey,ez,ax,ay,az,bx,by,bz,cx,cy,cz:float);
procedure triangle(ax,ay,az,bx,by,bz,cx,cy,cz:float);

implementation

Constructor TriFan.Init;
begin
  Center:=points^.addo(cx,cy,cz);
  First:=points^.addo(ax,ay,az);
  Last:=points^.addo(bx,by,bz);
  dreiecks^.add(Center,First,Last,[1,2,3])
end;

function Trifan.add;
var help:ppunkt3d;
begin
  help:=Points^.addo(ax,ay,az);
  dreiecks^.add(Center,Last,help,[1,2,3]);
  Last:=help;
  add:=Last
end;

Destructor Trifan.Done;
begin
  dreiecks^.add(Center,Last,First,[1,2,3])
end;

Constructor TriStrip.Init;
begin
  l1:=points^.addo(ax,ay,az);
  l2:=points^.addo(bx,by,bz);
  dreiecks^.add(points^.addo(cx,cy,cz),l1,l2,[1,2,3]);
  w:=true;
end;

function TriStrip.add;
var help:ppunkt3d;
begin
  help:=points^.addo(ax,ay,az);
  if w then dreiecks^.add(l1,help,l2,[1,2,3]) else dreiecks^.add(l1,l2,help,[1,2,3]);
  w:=not w;
  l1:=l2;
  l2:=help;
  add:=l2
end;

Constructor QuadStrip.Init;
var h:ppunkt3d;
begin
  h:=points^.addo(ax,ay,az);
  l1:=points^.addo(cx,cy,cz);
  l2:=points^.addo(dx,dy,dz);
  dreiecks^.add(h,points^.addo(bx,by,bz),l1,[1,3]);
  dreiecks^.add(h,l1,l2,[1,2]);
end;

Destructor QuadStrip.done;
begin
end;

Procedure QuadStrip.add;
var h1,h2:ppunkt3d;
begin
  h1:=points^.addo(ax,ay,az);
  h2:=points^.addo(bx,by,bz);

  dreiecks^.add(l2,l1,h1,[1,3]);
  dreiecks^.add(l2,h1,h2,[1,2]);

  l1:=h1;
  l2:=h2;
end;

procedure tetraeder;
var h:^triStrip;
begin
  new(h,init(ax,ay,az,cx,cy,cz,bx,by,bz));
  h^.add(dx,dy,dz);
  h^.add(ax,ay,az);
  h^.add(cx,cy,cz);
  dispose(h);
end;

procedure kugel;
var
  i,j:int;
  wl,wb:float;
  npol,spol:^trifan;
  land:^quadstrip;
begin
  if l<3 then l:=3;
  if b<3 then b:=3;
  wl:=2*pi/l;
  wb:=pi/b;

  for i:=1 to l do begin
    if i=1 then begin
      new(npol,init(mx,my,mz+r,
        mx+r*sin(wb),my,mz+r*cos(wb),
        mx+r*sin(wb)*cos(wl),my+r*sin(wb)*sin(wl),mz+r*cos(wb)));
      new(spol,init(mx,my,mz-r,
        mx+r*sin(wb),my,mz-r*cos(wb),
        mx+r*sin(wb)*cos(wl),my-r*sin(wb)*sin(wl),mz-r*cos(wb)));
    end else begin
      npol^.add(mx+r*sin(wb)*cos(i*wl),my+r*sin(wb)*sin(i*wl),mz+r*cos(wb));
      spol^.add(mx+r*sin(wb)*cos(i*wl),my-r*sin(wb)*sin(i*wl),mz-r*cos(wb));
    end;
    new(land,init(
      mx+r*sin(wb)*cos(i*wl),my+r*sin(wb)*sin(i*wl),mz+r*cos(wb),
      mx+r*sin(wb)*cos((i-1)*wl),my+r*sin(wb)*sin((i-1)*wl),mz+r*cos(wb),
      mx+r*sin(2*wb)*cos(i*wl),my+r*sin(2*wb)*sin(i*wl),mz+r*cos(2*wb),
      mx+r*sin(2*wb)*cos((i-1)*wl),my+r*sin(2*wb)*sin((i-1)*wl),mz+r*cos(2*wb)));
    for j:=3 to b-1 do begin
      land^.add(
        mx+r*sin(j*wb)*cos(i*wl),my+r*sin(j*wb)*sin(i*wl),mz+r*cos(j*wb),
        mx+r*sin(j*wb)*cos((i-1)*wl),my+r*sin(j*wb)*sin((i-1)*wl),mz+r*cos(j*wb));
    end;
    dispose(land);
  end;
  dispose(npol);
  dispose(spol);
end;

procedure kegel;
var
  tr,th:^trifan;
  i:int;
  w,c,s:float;
begin
  w:=2*pi/b;
  for i:=1 to b do begin
    c:=cos(w*i);
    s:=sin(w*i);
    if i=1 then begin
      new(tr,init(mx,my,mz,          mx-r1x, my-r1y, mz-r1z, mx-r1x*c+r2x*s, my-r1y*c+r2y*s, mz-r1z*c+r2z*s));
      new(th,init(mx+hx,my+hy,mz+hz, mx+r1x, my+r1y, mz+r1z, mx+r1x*c+r2x*s, my+r1y*c+r2y*s, mz+r1z*c+r2z*s));
    end else begin
      tr^.add(mx-r1x*c+r2x*s, my-r1y*c+r2y*s, mz-r1z*c+r2z*s);
      th^.add(mx+r1x*c+r2x*s, my+r1y*c+r2y*s, mz+r1z*c+r2z*s);
    end
  end;
  dispose(tr, done);
  dispose(th, done);
end;

procedure cube;
var q:^quadstrip;
begin
  new(q,init(ex,ey,ez, ex+bx, ey+by, ez+bz, ex+ax,ey+ay,ez+az, ex+ax+bx,ey+ay+by,ez+az+bz));
  q^.add(ex+ax+cx, ey+ay+cy, ez+az+cz, ex+ax+bx+cx, ey+ay+by+cy, ez+az+bz+cz);
  q^.add(ex+cx, ey+cy, ez+cz, ex+bx+cx, ey+by+cy, ez+bz+cz);
  dispose(q, done);
  new(q,init(ex+ax,ey+ay,ez+az,ex+ax+cx, ey+ay+cy, ez+az+cz,ex,ey,ez,ex+cx, ey+cy, ez+cz));
  q^.add(ex+bx, ey+by, ez+bz,ex+bx+cx, ey+by+cy, ez+bz+cz);
  q^.add(ex+ax+bx,ey+ay+by,ez+az+bz,ex+ax+bx+cx, ey+ay+by+cy, ez+az+bz+cz);
  dispose(q, done);
end;

procedure triangle;
var
  a,b,c:ppunkt3d;
begin
  c:=points^.addo(cx,cy,cz);
  a:=points^.addo(ax,ay,az);
  b:=points^.addo(bx,by,bz);
  dreiecks^.add(c,a,b,[1,2,3])
end;

begin
  new(points,init);
  initdliste(dreiecks);
end.
