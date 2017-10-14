unit vector;
interface
uses ptcgraph,crt;
const
  epsilon1=0.000001;
  epsilon2=epsilon1*epsilon1;
  epsilon3=epsilon1*epsilon1*epsilon1;
type
  float=real;
  int=longint;
  vector3d=object
    x, y, z: float;
    constructor init(x,y,z: float);
    function add3d(v: vector3d): vector3d;
    function sub3d(v: vector3d): vector3d;
    function mul3d(f: float): vector3d;
    function div3d(d: float): vector3d;

    function neg3d: vector3d;
    function betrag3d: float;
    function kreuz(v:vector3d): vector3d;
    function skalar(v:vector3d): float;

  end;
  matrix3d=object
    x, y, z: vector3d;
    constructor init(x,y,z: vector3d);
  end;
  vector2d=object
    x, y: float;
    constructor init(x,y: float);

    function sub2d(v: vector2d): vector2d;
    function add2d(v: vector2d): vector2d;
    function mul2d(f: float): vector2d;
    function div2d(d: float): vector2d;
    function betrag2d: float;
  end;
  matrix2d=object
    x, y: vector2d;
    constructor init(x,y: vector2d);
  end;


function det3d(A:matrix3d):float;
function det2d(A:matrix2d):float;
PROCEDURE RotVec(VAR ToRot1,ToRot2:Vector3d;t:float);
PROCEDURE MoveVec(VAR ToMove:Vector3d;Direction:Vector3d;Polarisation:float);
procedure outstring(s:string);
procedure outfloat(name:string;f:float);
procedure outvector2d(name:string;f:vector2d);
procedure outvector3d(name:string;f:vector3d);
procedure outint(name:string;c:int);
procedure marke(x,y,c:int;s:string);
procedure cls;
function sgn(x:float):int;
var
  Auge,BlickR,iv,jv:vector3d;
  bmx,bmy:float;
  palleiste:pointer;
  tausgabe:boolean;

implementation
const
  MoveSpeed:float=1;
var
  th,tx: integer;

constructor vector3d.init;
begin
  self.x:=x;
  self.y:=y;
  self.z:=z;
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
  mul3d.init(x*f, y*f, z*f);
end;

function vector3d.div3d;
begin
  if d=0 then outstring('d=0');
  div3d.init(x/d, y/d, z/d);
end;

function vector3d.neg3d;
begin
  neg3d.init( -x, -y, -z );
end;

function vector3d.kreuz;
begin
  kreuz.init(
    y*v.z - z*v.y,
    z*v.x - x*v.z,
    x*v.y - y*v.x
  );
end;

function vector3d.skalar;
begin
  skalar := x*v.x + y*v.y + z*v.z;
end;

function vector3d.betrag3d;
begin
  betrag3d:=sqrt(sqr(x)+sqr(y)+sqr(z));
end;

constructor vector2d.init;
begin
  self.x:=x;
  self.y:=y;
end;

function vector2d.mul2d;
begin
  mul2d.init(x*f, y*f);
end;

function vector2d.div2d;
begin
  div2d.init(x/d, y/d);
end;

function vector2d.sub2d;
begin
  sub2d.init( x-v.x, y-v.y );
end;

function vector2d.add2d;
begin
  add2d.init( x+v.x, y+v.y );
end;

function vector2d.betrag2d;
begin
  betrag2d:=sqrt(sqr(x)+sqr(y));
end;


constructor matrix3d.init;
begin
  self.x := x;
  self.y := y;
  self.z := z;
end;

function det3d;
begin
  det3d:=A.x.x*A.y.y*A.z.z+A.y.x*A.z.y*A.x.z+A.z.x*A.x.y*A.y.z-
         A.x.x*A.z.y*A.y.z-A.y.x*A.x.y*A.z.z-A.z.x*A.y.y*A.x.z
end;

constructor matrix2d.init;
begin
  self.x := x;
  self.y := y;
end;

function det2d;
begin
  det2d:=A.x.x*A.y.y-A.x.y*A.y.x;
end;


PROCEDURE MoveVec;
VAR
  DirLength:Real;
BEGIN
  DirLength:=Direction.betrag3d;
  if DirLength=0 then outstring('DirLength=0');
  ToMove.x:=ToMove.X+Polarisation*MoveSpeed*Direction.X/DirLength;
  ToMove.y:=ToMove.Y+Polarisation*MoveSpeed*Direction.Y/DirLength;
  ToMove.z:=ToMove.Z+Polarisation*MoveSpeed*Direction.Z/DirLength;
END;
PROCEDURE RotVec;
VAR
  Length1,Length2:Real;
  Copy1,Copy2:Vector3d;
  RotVecLength,RotInc:float;
BEGIN
  RotInc:=cos(t*Pi/180)/sin(t*pi/180);
  RotVecLength:=sqrt(sqr(RotInc)+1);

  Length1:=ToRot1.betrag3d;
  Length2:=ToRot2.Betrag3d;

  Copy1:=ToRot1;
  Copy2:=ToRot2;

  if RotVecLength=0 then writeln('RotVecLength=0');
  if Length1=0 then writeln('Length1=0');
  if Length2=0 then writeln('Length2=0');
  ToRot1.X:=(Length1/RotVecLength)*(RotInc*Copy1.X/Length1+Copy2.X/Length2);
  ToRot1.Y:=(Length1/RotVecLength)*(RotInc*Copy1.Y/Length1+Copy2.Y/Length2);
  ToRot1.Z:=(Length1/RotVecLength)*(RotInc*Copy1.Z/Length1+Copy2.Z/Length2);

  ToRot2.X:=(Length2/RotVecLength)*(-Copy1.X/Length1+RotInc*Copy2.X/Length2);
  ToRot2.Y:=(Length2/RotVecLength)*(-Copy1.Y/Length1+RotInc*Copy2.Y/Length2);
  ToRot2.Z:=(Length2/RotVecLength)*(-Copy1.Z/Length1+RotInc*Copy2.Z/Length2)
END;

procedure outstring;
begin
  if tausgabe then begin
    setcolor(white);
    outtextxy(tx,th,s);
    inc(th,10);
    while th>2*bmy-20 do begin
      th:=th-round(2*bmy-20);
      tx:=tx+240;
    end
  end;
end;

procedure outfloat;
var
  s:string;
begin
  setcolor(red);
  str(f:6:2,s);
  outstring(name+s);
end;

procedure outvector2d;
var
  s,s1:string;
begin
  setcolor(red);
  str(f.x:6:2,s);
  str(f.y:6:2,s1);
  outstring(name+'('+s+','+s1+')');
end;

procedure outvector3d;
var
  s,s1,s2:string;
begin
  setcolor(red);
  str(f.x:6:2,s);
  str(f.y:6:2,s1);
  str(f.z:6:2,s2);
  outstring(name+'('+s+','+s1+','+s2+')');
end;

procedure outint;
var
  s:string;
begin
  str(c,s);
  outstring(name+s);
end;

procedure cls;
begin
  cleardevice;
  if tausgabe then putimage(0,0,palleiste^,normalput);
  th:=10;
  tx:=20;
end;
function sgn;
begin
  if x<-epsilon1 then
    sgn:=-1
  else if x>epsilon1 then
    sgn:=1
  else
    sgn:=0;
end;
procedure marke;
begin
  setcolor(c);
  line(x-4,y-4,x+4,y+4);
  line(x-4,y+4,x+4,y-4);
  outtextxy(x+5,y-4,s);
end;
Begin
  tx:=20;
  th:=10;
end.
