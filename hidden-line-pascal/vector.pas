unit vector;

interface

uses ptcgraph;

const
  epsilon0 = 0.0001;
  epsilon1 = epsilon0 * epsilon0;
  epsilon2 = epsilon1 * epsilon1;

type
  float = double;
  int = longint;
  pvector3d = ^vector3d;

  vector3d = object
    x, y, z: float;
    constructor init(x, y, z: float);
    function add3d(v: vector3d): vector3d;
    function sub3d(v: vector3d): vector3d;
    function mul3d(f: float): vector3d;
    function div3d(d: float): vector3d;

    function neg3d: vector3d;
    function invBetrag3d: float;
    function kreuz(v: vector3d): vector3d;
    function skalar(v: vector3d): float;

    function move3d(direction: Vector3d; polarisation: float): vector3d;
    function toString: string;
  end;

  matrix3d = object
    x, y, z: vector3d;
    constructor init(x, y, z: vector3d);
    function det3d: float;
    function toString: string;
  end;
  pvector2d = ^vector2d;

  vector2d = object
    x, y: float;
    constructor init(x, y: float);

    function sub2d(v: vector2d): vector2d;
    function add2d(v: vector2d): vector2d;
    function mul2d(f: float): vector2d;
    function div2d(d: float): vector2d;
    function betrag2d: float;
    function sqrbetrag2d: float;
    function toString: string;
  end;

  matrix2d = object
    x, y: vector2d;
    constructor init(x, y: vector2d);

    function withX(v: vector2d): matrix2d;
    function withY(v: vector2d): matrix2d;

    function det2d: float;
    function toString: string;
  end;

function floatToString(f: float): string;
procedure RotVec(ToRot1, ToRot2: pvector3d; t: float);

procedure outstring(s: string);
procedure outfloat(Name: string; f: float);
procedure outvector2d(Name: string; f: vector2d);
procedure outvector3d(Name: string; f: vector3d);
procedure outint(Name: string; c: int);
procedure marke(x, y, c: int; s: string);
procedure cls;
function sgn(x: float): int;

var
  Auge, BlickR, iv, jv: vector3d;
  bmx, bmy: integer;
  palleiste: pointer;
  tausgabe: boolean;

implementation

const
  MoveSpeed: float = 1;
//var
//  th,tx: integer;

function invsqrt(number: double): double;
var
  y: double;
var
  i: int64;
begin
  i := $5fe6eb50c7b537a9 - ((int64(number)) div 2);
  y := double(i);
  y := y * (1.5 - (number * 0.5 * y * y));   // 1st iteration
  //  y := y * (1.5 - (number * 0.5 * y * y));   // 2nd iteration, this can be removed
  invsqrt := y;
end;

constructor vector3d.init;
begin
  self.x := x;
  self.y := y;
  self.z := z;
end;

function vector3d.sub3d;
begin
  sub3d.init(x - v.x, y - v.y, z - v.z);
end;

function vector3d.add3d;
begin
  add3d.init(x + v.x, y + v.y, z + v.z);
end;

function vector3d.mul3d;
begin
  mul3d.init(x * f, y * f, z * f);
end;

function vector3d.div3d;
begin
  if d = 0 then
    outstring('d=0');
  div3d.init(x / d, y / d, z / d);
end;

function vector3d.neg3d;
begin
  neg3d.init(-x, -y, -z);
end;

function vector3d.kreuz;
begin
  kreuz.init(
    y * v.z - z * v.y,
    z * v.x - x * v.z,
    x * v.y - y * v.x
    );
end;

function vector3d.skalar;
begin
  skalar := x * v.x + y * v.y + z * v.z;
end;

function vector3d.invBetrag3d;
begin
  invBetrag3d := invsqrt(sqr(x) + sqr(y) + sqr(z));
end;

function vector3d.move3d;
begin
  move3d := add3d(direction.mul3d(polarisation * MoveSpeed).mul3d(
    direction.invBetrag3d));
end;

function vector3d.toString;
begin
  toString := '(' + floatToString(x) + ',' + floatToString(y) + ',' +
    floatToString(z) + ')';
end;

constructor vector2d.init;
begin
  self.x := x;
  self.y := y;
end;

function vector2d.mul2d;
begin
  mul2d.init(x * f, y * f);
end;

function vector2d.div2d;
begin
  div2d.init(x / d, y / d);
end;

function vector2d.sub2d;
begin
  sub2d.init(x - v.x, y - v.y);
end;

function vector2d.add2d;
begin
  add2d.init(x + v.x, y + v.y);
end;

function vector2d.sqrbetrag2d;
begin
  sqrbetrag2d := sqr(x) + sqr(y);
end;

function vector2d.betrag2d;
begin
  betrag2d := 1.0 / invsqrt(self.sqrbetrag2d);
end;

function vector2d.toString;
begin
  toString := '(' + floatToString(x) + ',' + floatToString(y) + ')';
end;

constructor matrix3d.init;
begin
  self.x := x;
  self.y := y;
  self.z := z;
end;

function matrix3d.det3d;
begin
  det3d := x.x * y.y * z.z + y.x * z.y * x.z + z.x * x.y * y.z - x.x * z.y * y.z -
    y.x * x.y * z.z - z.x * y.y * x.z;
end;

function matrix3d.toString;
begin
  toString := '(' + x.toString + ',' + y.toString + ',' + z.toString + ')';
end;

constructor matrix2d.init;
begin
  self.x := x;
  self.y := y;
end;

function matrix2d.withX;
begin
  withX.init(v, y);
end;

function matrix2d.withY;
begin
  withY.init(x, v);
end;

function matrix2d.det2d;
begin
  det2d := x.x * y.y - x.y * y.x;
end;

function matrix2d.toString;
begin
  toString := '(' + x.toString + ',' + y.toString + ')';
end;

procedure RotVec;
var
  InvLength1, InvLength2, InvRotVecLength, RotInc: float;
  Copy1, Copy2: Vector3d;

begin
  RotInc := cos(t * Pi / 180) / sin(t * pi / 180);
  InvRotVecLength := invsqrt(sqr(RotInc) + 1);

  Copy1 := ToRot1^;
  Copy2 := ToRot2^;

  InvLength1 := Copy1.invBetrag3d;
  InvLength2 := Copy2.invBetrag3d;

  if InvRotVecLength = 0 then
    writeln('InvRotVecLength=0');
  if InvLength1 = 0 then
    writeln('InvLength1=0');
  if InvLength2 = 0 then
    writeln('InvLength2=0');
  ToRot1^.X := (InvRotVecLength / InvLength1) * (RotInc * Copy1.X * InvLength1 + Copy2.X * InvLength2);
  ToRot1^.Y := (InvRotVecLength / InvLength1) * (RotInc * Copy1.Y * InvLength1 + Copy2.Y * InvLength2);
  ToRot1^.Z := (InvRotVecLength / InvLength1) * (RotInc * Copy1.Z * InvLength1 + Copy2.Z * InvLength2);

  ToRot2^.X := (InvRotVecLength / InvLength2) *
    (-Copy1.X * InvLength1 + RotInc * Copy2.X * InvLength2);
  ToRot2^.Y := (InvRotVecLength / InvLength2) *
    (-Copy1.Y * InvLength1 + RotInc * Copy2.Y * InvLength2);
  ToRot2^.Z := (InvRotVecLength / InvLength2) *
    (-Copy1.Z * InvLength1 + RotInc * Copy2.Z * InvLength2);
end;

procedure outstring;
begin
  if tausgabe then
  begin
    WriteLn(s);
{    setcolor(white);
    outtextxy(tx,th,s);
    inc(th,10);
    while th>2*bmy-20 do begin
      th:=th-round(2*bmy-20);
      tx:=tx+240;
    end}
  end;
end;

function floatToString;
var
  s: string;
begin
  Str(f: 15: 10, s);
  floatToString := s;
end;

procedure outfloat;
begin
  setcolor(red);
  outstring(Name + floatToString(f));
end;

procedure outvector2d;
begin
  setcolor(red);
  outstring(Name + f.toString);
end;

procedure outvector3d;
begin
  setcolor(red);
  outstring(Name + f.toString);
end;

procedure outint;
var
  s: string;
begin
  str(c, s);
  outstring(Name + s);
end;

procedure cls;
begin
  cleardevice;
  //  if tausgabe then putimage(0,0,palleiste^,normalput);
  //  th:=10;
  //  tx:=20;
end;

function sgn;
begin
  if x < -epsilon1 then
    sgn := -1
  else if x > epsilon1 then
    sgn := 1
  else
    sgn := 0;
end;

procedure marke;
begin
  setcolor(c);
  line(x - 4, y - 4, x + 4, y + 4);
  line(x - 4, y + 4, x + 4, y - 4);
  outtextxy(x + 5, y - 4, s);
end;
//Begin
//  tx:=20;
//  th:=10;
end.
