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
    constructor init(bv:vector2d;ls:lset);
    constructor init0;
    destructor done;
    function copy:ppunkt;
  end;

  ppunkt3d=^punkt3d;
  punkt3d=object
    public
      b:ppunkt;
      o:vector3d;
      constructor init(ax,ay,az:float);
    private
      next:ppunkt3d;
  end;

  punktliste=^pliste;
  pliste=object
    private
      aktuell,Anker,Last:ppunkt3d;
    public
      function addo(ax,ay,az:float):ppunkt3d;
      procedure add(p:ppunkt3d);
      function first:ppunkt3d;
      function next:ppunkt3d;
      constructor init;
  end;

function colinear(p1,p2,p3:pvector2d):boolean;
function gleicheseite(s,p2,p3,p4:punkt):int;


var
  points:punktliste;

implementation

constructor punkt.init;
begin
  self.init0;
  b:=bv;
  gz:=ls;
end;

constructor punkt.init0;
begin
  zaehl.points2d.ins;
end;

constructor punkt3d.init;
begin
  zaehl.points3d.ins;
  o.init(ax,ay,az);
  next:=nil;
  new(b, init0);
end;

destructor punkt.done;
begin
  zaehl.points2d.del;
end;

function punkt.copy;
var result: ^punkt;
begin
  new(result, init(b, gz));
  copy := result;
end;

procedure punkt.draw;
begin
  setcolor(c);
  circle(round(bmx+b.x),round(bmy-b.y),2);
end;

function colinear;
begin
  colinear:=abs((p1^.y-p2^.y)*(p3^.x-p2^.x)-(p1^.x-p2^.x)*(p3^.y-p2^.y))<epsilon0
end;

function gleicheseite;
begin
  gleicheseite := sgn(((p3.b.y-s.b.y)*(p2.b.x-s.b.x)-(p3.b.x-s.b.x)*(p2.b.y-s.b.y))
                    *((p4.b.y-s.b.y)*(p2.b.x-s.b.x)-(p4.b.x-s.b.x)*(p2.b.y-s.b.y)));
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
  new(h, init(ax,ay,az));
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
