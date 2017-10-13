unit punkte;
interface
uses ptcgraph,vector,zeit;

type
  color=byte;
  lset=set of 1..3;

  ppunkt=^punkt;
  punkt=object
    b:vector2d;
    gz:lset;
    procedure draw(c:color);
  end;

  ppunkt3d=^punkt3d;
  punkt3d=object
    b:ppunkt;
    o:vector3d;
    next:ppunkt3d;
  end;

  punktliste=^pliste;
  pliste=object
    aktuell,Anker,Last:ppunkt3d;
    function addo(ax,ay,az:float):ppunkt3d;
    procedure add(p:ppunkt3d);
    function first:ppunkt3d;
    function next:ppunkt3d;
    constructor init;
  end;

procedure disposeppunkt(var p:ppunkt);
procedure initbppunkt(var p:ppunkt;bv:vector2d;ls:lset);
procedure copyppunkt(var p:ppunkt;q:punkt);
procedure initppunkt3d(var p:ppunkt3d;ax,ay,az:float);

function colinear(p1,p2,p3:punkt):boolean;
function gleicheseite(s,p2,p3,p4:punkt):int;


var
  points:punktliste;

implementation

procedure newppunkt(var p:ppunkt);
begin
  new(p);
  zaehl.p2.ins;
end;

procedure initppunkt3d;
begin
  new(p);
  zaehl.p3.ins;
  p^.o[x]:=ax;
  p^.o[y]:=ay;
  p^.o[z]:=az;
  p^.next:=nil;
  newppunkt(p^.b);
end;

procedure disposeppunkt;
begin
  dispose(p);
  zaehl.p2.del;
end;

procedure initbppunkt;
begin
  newppunkt(p);
  p^.b:=bv;
  p^.gz:=ls
end;

procedure copyppunkt;
begin
  newppunkt(p);
  p^.b:=q.b;
  p^.gz:=q.gz
end;

procedure punkt.draw;
begin
  setcolor(c);
  circle(round(bmx+b[x]),round(bmy-b[y]),2);
end;

function colinear;
begin
  colinear:=abs((p1.b[y]-p2.b[y])*(p3.b[x]-p2.b[x])-(p1.b[x]-p2.b[x])*(p3.b[y]-p2.b[y]))<epsilon1
end;

function gleicheseite;
begin
  gleicheseite:=sgn(((p3.b[y]-s.b[y])*(p2.b[x]-s.b[x])-(p3.b[x]-s.b[x])*(p2.b[y]-s.b[y]))
               *((p4.b[y]-s.b[y])*(p2.b[x]-s.b[x])-(p4.b[x]-s.b[x])*(p2.b[y]-s.b[y])))
end;


procedure pliste.add;
begin
  if anker=nil then
    anker:=p;
  if last=nil then
    last:=p
  else begin
    last^.next:=p;
    last:=p
  end
end;

function pliste.addo;
var h:ppunkt3d;
begin
  initppunkt3d(h,ax,ay,az);
  add(h);
  addo:=h
end;

function pliste.first;
begin
  aktuell:=anker;
  first:=aktuell;
end;

function pliste.next;
begin
  if aktuell<>nil then
    aktuell:=aktuell^.next;
  next:=aktuell
end;

constructor pliste.init;
begin
  Last:=nil;
  Anker:=nil;
end;

begin
end.
