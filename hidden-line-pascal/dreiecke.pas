unit dreiecke;
interface
uses ptcgraph,vector,punkte,linien;
type
  dreiecktyp=object
    p:array[1..3] of ppunkt;
    l:array[1..3] of linie;
    gl:lset;
    cols:color;
    procedure draw1;
    procedure draw2;
    procedure draw3(c:color);
    function linientest(li:linie):boolean;
    function punkttest(t:vector2d):int;
    function flaechentest:boolean;
  end;
  pdreieck=^dreieck;
  dreieck=object(dreiecktyp)
    public
      o:array[1..3] of ppunkt3d;
      planeNorm: vector3d;
      planeDist: float;
      function tiefe(k:vector2d;ausgabe:boolean):float;
      constructor init(p1,p2,p3:ppunkt3d;ls:lset);
    private
      next:pdreieck;
  end;

  dliste=object
    public
      function add(p1,p2,p3:ppunkt3d;ls:lset):pdreieck;
      function first:pdreieck;
      function next:pdreieck;
      constructor init;
      destructor done;
    private
      Anker,aktuell,Last:pdreieck;
  end;

var
  dreiecks:dliste;
implementation

constructor dreieck.init;
begin
  o[1]:=p1;
  o[2]:=p2;
  o[3]:=p3;
  p[1]:=p1^.b;
  p[2]:=p2^.b;
  p[3]:=p3^.b;
  gl:=ls;
  l[1].init(p[2],p[3]);
  l[2].init(p[3],p[1]);
  l[3].init(p[1],p[2]);
  next:=nil;
  planeNorm := o[2]^.o.sub3d(o[1]^.o)
        .kreuz(o[3]^.o.sub3d(o[1]^.o));
  planeNorm := planeNorm.div3d(planeNorm.betrag3d);
  planeDist := planeNorm.skalar(o[1]^.o);
end;

procedure dreiecktyp.draw1;
begin
  if (gl*[1])<>[] then l[1].draw(cols);
  if (gl*[2])<>[] then l[2].draw(cols);
  if (gl*[3])<>[] then l[3].draw(cols);
end;

procedure dreiecktyp.draw2;
var tri:array[1..3]of pointtype;
begin
  setfillstyle(1,cols);
  setcolor(cols);
  tri[1].x:=round(bmx+p[1]^.b.x);
  tri[1].y:=round(bmy-p[1]^.b.y);
  tri[2].x:=round(bmx+p[2]^.b.x);
  tri[2].y:=round(bmy-p[2]^.b.y);
  tri[3].x:=round(bmx+p[3]^.b.x);
  tri[3].y:=round(bmy-p[3]^.b.y);
  fillpoly(3,tri)
end;

procedure dreiecktyp.draw3;
begin
  l[1].draw(c);
  l[2].draw(c);
  l[3].draw(c);
end;

function dreiecktyp.punkttest;
var
  K:matrix3d;
  h,b1:vector3d;
  kd,la:float;
  l1,l2,l3:int;

  function testl(l:float):int;
  begin
    if abs(l)<epsilon1 then begin  {=0}
      testl:=0
    end else if abs(l-1)<epsilon1 then begin {=1}
      testl:=1
    end else if l<0 then begin {<0}
      testl:=2
    end else if l>1 then begin {>1}
      testl:=3
    end else begin  {0<l<1}
      testl:=4
    end
  end;

begin
  K.x.x:=p[1]^.b.x;    K.y.x:=p[2]^.b.x;    K.z.x:=p[3]^.b.x;
  K.x.y:=p[1]^.b.y;    K.y.y:=p[2]^.b.y;    K.z.y:=p[3]^.b.y;
  K.x.z:=1;            K.y.z:=1;            K.z.z:=1;
  b1.x:=t.x;    b1.y:=t.y;        b1.z:=1;
  kd:=K.det3d;
  if abs(kd)>epsilon1 then begin
    h:=K.x;    K.x:=b1;    la:=K.det3d/kd;  K.x:=h;        l1:=testl(la);
    h:=K.y;    K.y:=b1;    la:=K.det3d/kd;  K.y:=h;        l2:=testl(la);
    h:=K.z;    K.z:=b1;    la:=K.det3d/kd;  K.z:=h;        l3:=testl(la);
    case l1*25+l2*5+l3 of
      124:punkttest:=0;          {drin}

      24:punkttest:=1;            {kanten}
      104:punkttest:=2;
      120:punkttest:=3;

      25:punkttest:=11;          {eckpunkte}
      5:punkttest:=12;
      1:punkttest:=13;
    else
      punkttest:=20;         {draußen}
    end;
  end else begin
    outstring('nullerdiv ' + K.toString);
    punkttest:=0;
  end;
end;

function dreiecktyp.linientest;
var
  pa,pe:int;
  h:vector2d;
  la,m:float;
  li1,li2,li3:int;
begin
  pa:=punkttest(li.a^.b);
  pe:=punkttest(li.e^.b);
  if (pa=0)or(pe=0)then
    linientest:=true
  else if (pa<20)and(pe<20)then begin
    h.x:=(li.a^.b.x+li.e^.b.x)/2;
    h.y:=(li.a^.b.y+li.e^.b.y)/2;
    linientest:=(punkttest(h)=0);
  end else begin
    li1:=linien.intersect(li,l[1],la,m);
    li2:=linien.intersect(li,l[2],la,m);
    li3:=linien.intersect(li,l[3],la,m);
    linientest:=(li1=1)or(li2=1)or(li3=1)or((li1=2)and(li2=2))or((li2=2)and(li3=2))or((li3=2)and(li1=2));
  end
end;

function dreiecktyp.flaechentest;
begin
  flaechentest:=p[1]^.b.sub2d(p[3]^.b).betrag2d * 1.01 <
                p[1]^.b.sub2d(p[2]^.b).betrag2d +
                p[2]^.b.sub2d(p[3]^.b).betrag2d
end;

function dreieck.tiefe;
var
  bv:vector3d;
  t:float;
begin
  bv := blickr.add3d(iv.mul3d(k.x))
              .add3d(jv.mul3d(k.y));
  t := planeNorm.skalar(bv);
  if abs(t)>epsilon3 then
    tiefe := bv.betrag3d * ( planeDist - planeNorm.skalar(auge) ) / t
  else
    tiefe := 100000000;
{  cols:=darkgray;}
end;

constructor dliste.init;
begin
  Anker := nil;
  Last := nil;
end;

destructor dliste.done;
begin

end;

function dliste.add;
var h:pdreieck;
begin
  New(h,init(p1,p2,p3,ls));
  if anker=nil then
    anker:=h;
  if last=nil then
    last:=h
  else begin
    last^.next:=h;
    last:=h
  end;
  add:=h
end;

function dliste.first;
begin
  aktuell:=anker;
  first:=aktuell;
end;

function dliste.next;
begin
  if aktuell<>nil then
    aktuell:=aktuell^.next;
  next:=aktuell;
end;

begin
end.
