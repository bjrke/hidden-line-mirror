unit zeit;

interface

uses ptccrt, dos, vector;

type
  dtyp = object
    insert, Delete, maximum, minimum, aktuell: longint;
    constructor init;
    procedure ins;
    procedure del;
    procedure ausgabe(Name: string);
  end;

  ctyp = object
    q: array[1..3] of dtyp;
    suchbaum, polygons, points2d, points3d: dtyp;
    ptest, Count, pp: longint;
    constructor init;
    procedure ausgabe;
  end;

  minmax = object
    MinValue, MaxValue: float;
    constructor init;
    procedure update(f: float);
    function toString: string;
    function relative(f: float): float;
  end;

  cset = set of char;

function gettime2: longint;
procedure starttime;
procedure outtime;
function readkey2(include: cset): char;

var
  zaehl: ctyp;

implementation

var
  time: longint;

constructor dtyp.init;
begin
  insert := 0;
  Delete := 0;
  maximum := 0;
  minimum := 0;
  aktuell := 0;
end;

procedure dtyp.ins;
begin
  Inc(insert);
  Inc(aktuell);
  if aktuell > maximum then
    maximum := aktuell;
end;

procedure dtyp.del;
begin
  Inc(Delete);
  Dec(aktuell);
  if aktuell < minimum then
    minimum := aktuell;
end;

procedure dtyp.ausgabe;
begin
  outstring(Name);
  if (insert = Delete) and (insert <> 0) then
    outint('  Einfügungen = Löschungen:', insert)
  else
  begin
    if insert <> 0 then
      outint('  Einfügungen:', insert);
    if Delete <> 0 then
      outint('  Löschungen:', Delete);
  end;
  if (maximum <> 0) and (maximum <> aktuell) then
    outint('  Höchststand:', maximum);
  if (minimum <> 0) and (minimum <> aktuell) then
    outint('  Tiefststand:', minimum);
  if (aktuell <> insert) then
  begin
    if (aktuell <> 0) and (minimum <> aktuell) and (maximum <> aktuell) then
      outint('  aktueller Stand:', aktuell);
    if (aktuell = maximum) and (aktuell <> 0) then
      outint('  aktuell(Höchst)Stand:', aktuell);
    if (aktuell = minimum) and (aktuell <> 0) then
      outint('  aktuell(Tiefst)Stand:', aktuell);
  end
  else
  begin
    if (aktuell <> 0) and (minimum <> aktuell) and (maximum <> aktuell) then
      outint('  aktueller Stand = Einfügungen:', aktuell);
    if (aktuell = maximum) and (aktuell <> 0) then
      outint('  aktuell(Höchst)Stand = Einfügungen:', aktuell);
    if (aktuell = minimum) and (aktuell <> 0) then
      outint('  aktuell(Tiefst)Stand = Einfügungen:', aktuell);
  end;
  outstring('--------------------------');
end;

constructor ctyp.init;
begin
  q[1].init;
  q[2].init;
  q[3].init;
  suchbaum.init;
  polygons.init;
  points2d.init;
  Count := 0;
  ptest := 0;
  pp := 0;
end;

procedure ctyp.ausgabe;
begin
  outint('# insert aufrufe : ', Count);
  outint('# Polytests: ', ptest);
  outint('# Triangulationen: ', pp);
  q[1].ausgabe('Warteschlange 1');
  q[2].ausgabe('Warteschlange 2');
  q[3].ausgabe('Warteschlange 3');
  suchbaum.ausgabe('Suchbaum');
  polygons.ausgabe('Polygone insgesamt');
  points2d.ausgabe('Punkte in Bildeben');
  points3d.ausgabe('Punkte im Raum');
end;

constructor minmax.init;
begin
  MinValue := 1e20;
  MaxValue := -1e20;
end;

procedure minmax.update;
begin
  if (f < MinValue) then
    MinValue := f;
  if (f > MaxValue) then
    MaxValue := f;
end;

function minmax.toString;
begin
  exit('min: ' + floatToString(MinValue) + ' max: ' + floatToString(MaxValue));
end;

function minmax.relative;
begin
  exit((f - MinValue) / (MaxValue - MinValue));
end;

procedure starttime;
begin
  time := gettime2;
end;

function gettime2;
var
  h, m, s, s100: word;
begin
  gettime(h, m, s, s100);
  gettime2 := 360000 * h + 6000 * m + s * 100 + s100;
end;

procedure outtime;
var
  t: longint;
  h, m: word;
  s: float;
  s1, s2: string;
begin
  s1 := '';
  t := time;
  starttime;
  t := time - t;
  h := t div 360000;
  m := (t - h * 360000) div 6000;
  s := (t - h * 360000 - m * 6000) / 100;
  if h > 0 then
  begin
    str(h, s1);
    s1 := s1 + 'h';
  end;
  if (h > 0) or (m > 0) then
  begin
    str(m, s2);
    if m = 0 then
      s1 := s1 + '0';
    if m < 10 then
      s1 := s1 + '0';
    s1 := s1 + s2 + 'm';
    if s = 0 then
      s1 := s1 + '0';
    if s < 10 then
      s1 := s1 + '0';
  end;
  str(s: 0: 2, s2);
  s1 := s1 + s2 + 's';
  outstring('Zeit: ' + s1);
end;

function readkey2;
var
  start: longint;
  ch, res: char;
begin
  repeat
    res := ReadKey;
    if (res = #0) then
      ReadKey
  until (Upcase(res) in include);

  if KeyPressed then
  begin
    start := gettime2;
    repeat
      ch := ReadKey;
      if (ch = #0) then
        ReadKey
    until ((not KeyPressed) or (ch <> res) or ((gettime2 - start) > 500));
  end;

  if (Upcase(ch) in include) then
    exit(ch)
  else
    exit(res);
end;

begin
  zaehl.points3d.init;
end.
