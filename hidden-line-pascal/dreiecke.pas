unit dreiecke;

interface

uses ptcgraph, vector, punkte, linien;

type
  dreiecktyp = object
    p: array[1..3] of ppunkt;
    l: array[1..3] of linie;
    gl: lset;
    cols: color;
    procedure draw1(c: color);
    procedure draw2(c: color);
    procedure draw3(c: color);
    function linientest(li: linie; ausgabe: boolean): boolean;
    function punkttest(t: vector2d; caller: string; ausgabe: boolean): int;
    function flaechentest: boolean;
  end;
  pdreieck = ^dreieck;

  dreieck = object(dreiecktyp)
  public
    origPoints: array[1..3] of ppunkt3d;
    planeNorm: vector3d;
    planeDist: float;
    function tiefe(k: vector2d): float;
    constructor init(p1, p2, p3: ppunkt3d; ls: lset);
  private
    Next: pdreieck;
  end;

  dliste = object
  public
    function add(p1, p2, p3: ppunkt3d; ls: lset): pdreieck;
    function First: pdreieck;
    function Next: pdreieck;
    constructor init;
  private
    Anker, aktuell, Last: pdreieck;
  end;

var
  dreiecks: dliste;

function calcColor(f: float): color;

implementation

function calcColor;
begin
  if (f < 0) then
    exit(1);
  if (f > 1) then
    exit(15);
  exit(Round(f * 14.0));
end;

constructor dreieck.init;
begin
  origPoints[1] := p1;
  origPoints[2] := p2;
  origPoints[3] := p3;
  p[1] := p1^.b;
  p[2] := p2^.b;
  p[3] := p3^.b;
  gl := ls;
  l[1].init(p[2], p[3]);
  l[2].init(p[3], p[1]);
  l[3].init(p[1], p[2]);
  Next := nil;
  planeNorm := origPoints[2]^.o.sub3d(origPoints[1]^.o)
    .kreuz(origPoints[3]^.o.sub3d(origPoints[1]^.o));
  planeNorm := planeNorm.mul3d(planeNorm.invBetrag3d);
  planeDist := planeNorm.skalar(origPoints[1]^.o);
end;

procedure dreiecktyp.draw1;
begin
  if (gl * [1]) <> [] then
    l[1].draw(c);
  if (gl * [2]) <> [] then
    l[2].draw(c);
  if (gl * [3]) <> [] then
    l[3].draw(c);
end;

procedure dreiecktyp.draw2;
var
  tri: array[1..3] of pointtype;
begin
  setfillstyle(1, c);
  setcolor(c);
  tri[1].x := round(bmx + p[1]^.b.x);
  tri[1].y := round(bmy - p[1]^.b.y);
  tri[2].x := round(bmx + p[2]^.b.x);
  tri[2].y := round(bmy - p[2]^.b.y);
  tri[3].x := round(bmx + p[3]^.b.x);
  tri[3].y := round(bmy - p[3]^.b.y);
  fillpoly(3, tri);
end;

procedure dreiecktyp.draw3;
begin
  l[1].draw(c);
  l[2].draw(c);
  l[3].draw(c);
end;

function dreiecktyp.punkttest;
var
  K: matrix3d;
  h, b1: vector3d;
  kd: float;
  la1, la2, la3: float;
  l1, l2, l3: int;

  function testl(l: float): int;
  begin
    if abs(l) < epsilon0 then
      exit(0); //=0
    if abs(l - 1) < epsilon0 then
      exit(1); //=1
    if l < 0 then
      exit(2);
    if l > 1 then
      exit(3);
    exit(4); // 0<l<1
  end;

begin
  K.x.x := p[1]^.b.x;
  K.y.x := p[2]^.b.x;
  K.z.x := p[3]^.b.x;
  K.x.y := p[1]^.b.y;
  K.y.y := p[2]^.b.y;
  K.z.y := p[3]^.b.y;
  K.x.z := 1;
  K.y.z := 1;
  K.z.z := 1;
  b1.x := t.x;
  b1.y := t.y;
  b1.z := 1;
  kd := K.det3d;
  if abs(kd) < epsilon1 then
  begin
    outstring('nullerdiv ' + K.toString);
    exit(0);
  end;

  h := K.x;
  K.x := b1;
  la1 := K.det3d / kd;
  K.x := h;
  l1 := testl(la1);
  h := K.y;
  K.y := b1;
  la2 := K.det3d / kd;
  K.y := h;
  l2 := testl(la2);
  h := K.z;
  K.z := b1;
  la3 := K.det3d / kd;
  K.z := h;
  l3 := testl(la3);
  if (ausgabe) then
    outstring('punkttest ' + caller + ' ' + floatToString(la1) +
      ' ' + floatToString(la2) + ' ' + floatToString(la3));
  case l1 * 25 + l2 * 5 + l3 of
    124: exit(0);          // drin

    24: exit(1);            // kanten
    104: exit(2);
    120: exit(3);

    25: exit(11);          // eckpunkte
    5: exit(12);
    1: exit(13);
    else
      exit(20);         // draußen
  end;

end;

function dreiecktyp.linientest;
var
  pa, pe: int;
  la, m: float;
  li1, li2, li3: int;
begin
  pa := punkttest(li.a^.b, 'linientest1', ausgabe);
  if (pa = 0) then
    exit(True);

  pe := punkttest(li.e^.b, 'linientest2', ausgabe);
  if (pe = 0) then
    exit(True);

  if (pa < 20) and (pe < 20) then
    exit(punkttest((li.a^.b).add2d(li.e^.b).div2d(2), 'linientest3', ausgabe) = 0);

  li1 := linien.intersect(li, l[1], la, m);
  if (li1 = 1) then
    exit(True);

  li2 := linien.intersect(li, l[2], la, m);
  if ((li2 = 1) or ((li1 = 2) and (li2 = 2))) then
    exit(True);

  li3 := linien.intersect(li, l[3], la, m);
  exit((li3 = 1) or ((li3 = 2) and ((li2 = 2) or (li1 = 2))));
end;

function dreiecktyp.flaechentest;
begin
  exit(not colinear(@p[1]^.b, @p[2]^.b, @p[3]^.b));
end;

function dreieck.tiefe;
var
  bv: vector3d;
  t: float;
begin
  bv := blickr.add3d(iv.mul3d(k.x)).add3d(jv.mul3d(k.y));
  t := planeNorm.skalar(bv);
  if abs(t) < epsilon2 then
    exit(100000000);
  exit((planeDist - planeNorm.skalar(auge)) / (t * bv.invBetrag3d));
end;

constructor dliste.init;
begin
  Anker := nil;
  Last := nil;
end;

function dliste.add;
var
  h: pdreieck;
begin
  New(h, init(p1, p2, p3, ls));
  if anker = nil then
    anker := h;
  if last = nil then
    last := h
  else
  begin
    last^.Next := h;
    last := h;
  end;
  exit(h);
end;

function dliste.First;
begin
  aktuell := anker;
  exit(aktuell);
end;

function dliste.Next;
begin
  if aktuell <> nil then
    aktuell := aktuell^.Next;
  exit(aktuell);
end;

begin
end.
