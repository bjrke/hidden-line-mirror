unit vector;
interface
uses ggigraph,crt;
const
  x=0;
  y=1;
  z=2;
  epsilon1=0.000001;
  epsilon2=epsilon1*epsilon1;
  epsilon3=epsilon1*epsilon1*epsilon1;
type
  float=real;
  int=longint;
  vector3d=array[x..z]of float;
  matrix3d=array[x..z]of vector3d;
  vector2d=array[x..y]of float;
  matrix2d=array[x..y]of vector2d;
procedure sub3d(var c:vector3d;a,b:vector3d);
procedure sub2d(var c:vector2d;a,b:vector2d);
procedure add3d(var c:vector3d;a,b:vector3d);
procedure add2d(var c:vector3d;a,b:vector3d);
procedure mul3d(var v:vector3d;f:float);
procedure div3d(var v:vector3d;d:float);
procedure kreuz(var c:vector3d;a,b:vector3d);
function skalar(a,b:vector3d):float;
procedure neg3d(var v:vector3d);
function det3d(A:matrix3d):float;
function det2d(A:matrix2d):float;
function betrag3d(v:vector3d):float;
function betrag2d(v:vector2d):float;
PROCEDURE RotVec(VAR ToRot1,ToRot2:Vector3d;t:float);
PROCEDURE MoveVec(VAR ToMove:Vector3d;Direction:Vector3d;Polarisation:float);
procedure outstring(s:string);
procedure outfloat(name:string;f:float);
procedure outvector2d(name:string;f:vector2d);
procedure outvector3d(name:string;f:vector3d);
procedure outint(name:string;c:int);
procedure outschleife(s:string);
procedure marke(x,y,c:int;s:string);
procedure cls;
function sgn(x:float):int;
var
  Auge,BlickR,iv,jv:vector3d;
  bmx,bmy:float;
  drawmode,th,tx:integer;
  colmode:boolean;
  spal,bpal:palettetype;
  palleiste:pointer;
  sc:int;
  tausgabe,rand:boolean;

implementation
const
  MoveSpeed:float=1;
var
  RotVecLength,RotInc:float;

procedure sub3d;
begin
  c[x]:=a[x]-b[x];
  c[y]:=a[y]-b[y];
  c[z]:=a[z]-b[z]
end;
procedure add3d;
begin
  c[x]:=a[x]+b[x];
  c[y]:=a[y]+b[y];
  c[z]:=a[z]+b[z]
end;
procedure sub2d;
begin
  c[x]:=a[x]-b[x];
  c[y]:=a[y]-b[y];
end;
procedure add2d;
begin
  c[x]:=a[x]+b[x];
  c[y]:=a[y]+b[y];
end;
function det3d;
begin
  det3d:=A[x,x]*A[y,y]*A[z,z]+A[y,x]*A[z,y]*A[x,z]+A[z,x]*A[x,y]*A[y,z]-
         A[x,x]*A[z,y]*A[y,z]-A[y,x]*A[x,y]*A[z,z]-A[z,x]*A[y,y]*A[x,z]
end;
function det2d;
begin
  det2d:=A[x,x]*A[y,y]-A[x,y]*A[y,x];
end;
procedure neg3d;
begin
  v[x]:=-v[x];
  v[y]:=-v[y];
  v[z]:=-v[z];
end;

procedure kreuz;
begin
  c[x]:=a[y]*b[z]-a[z]*b[y];
  c[y]:=a[z]*b[x]-a[x]*b[z];
  c[z]:=a[x]*b[y]-a[y]*b[x]
end;

function skalar;
begin
  skalar:=a[x]*b[x]+a[y]*b[y]+a[z]*b[z];
end;

function betrag3d;
begin
  betrag3d:=sqrt(sqr(v[x])+sqr(v[y])+sqr(v[z]))
end;
function betrag2d;
begin
  betrag2d:=sqrt(sqr(v[x])+sqr(v[y]))
end;
procedure mul3d;
begin
  v[x]:=v[x]*f;
  v[y]:=v[y]*f;
  v[z]:=v[z]*f
end;
procedure div3d;
begin
  if d=0 then outstring('d=0');
  v[x]:=v[x]/d;
  v[y]:=v[y]/d;
  v[z]:=v[z]/d
end;
PROCEDURE MoveVec;
VAR
  DirLength:Real;
BEGIN
  DirLength:=Betrag3d(Direction);
  if DirLength=0 then outstring('DirLength=0');
  ToMove[X]:=ToMove[X]+Polarisation*MoveSpeed*Direction[X]/DirLength;
  ToMove[Y]:=ToMove[Y]+Polarisation*MoveSpeed*Direction[Y]/DirLength;
  ToMove[Z]:=ToMove[Z]+Polarisation*MoveSpeed*Direction[Z]/DirLength
END;
PROCEDURE RotVec;
VAR
  Length1,Length2:Real;
  Copy1,Copy2:Vector3d;
BEGIN
  RotInc:=cos(t*Pi/180)/sin(t*pi/180);
  RotVecLength:=sqrt(sqr(RotInc)+1);

  Length1:=Betrag3d(ToRot1);
  Length2:=Betrag3d(ToRot2);

  Copy1:=ToRot1;
  Copy2:=ToRot2;

  if RotVecLength=0 then writeln('RotVecLength=0');
  if Length1=0 then writeln('Length1=0');
  if Length2=0 then writeln('Length2=0');
  ToRot1[X]:=(Length1/RotVecLength)*(RotInc*Copy1[X]/Length1+Copy2[X]/Length2);
  ToRot1[Y]:=(Length1/RotVecLength)*(RotInc*Copy1[Y]/Length1+Copy2[Y]/Length2);
  ToRot1[Z]:=(Length1/RotVecLength)*(RotInc*Copy1[Z]/Length1+Copy2[Z]/Length2);

  ToRot2[X]:=(Length2/RotVecLength)*(-Copy1[X]/Length1+RotInc*Copy2[X]/Length2);
  ToRot2[Y]:=(Length2/RotVecLength)*(-Copy1[Y]/Length1+RotInc*Copy2[Y]/Length2);
  ToRot2[Z]:=(Length2/RotVecLength)*(-Copy1[Z]/Length1+RotInc*Copy2[Z]/Length2)
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
  str(f[x]:6:2,s);
  str(f[y]:6:2,s1);
  outstring(name+'('+s+','+s1+')');
end;

procedure outvector3d;
var
  s,s1,s2:string;
begin
  setcolor(red);
  str(f[x]:6:2,s);
  str(f[y]:6:2,s1);
  str(f[z]:6:2,s2);
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
procedure outschleife;
begin
  inc(sc);
  if sc=80 then begin
    sc:=0;
    setfillstyle(1,0);
    bar(0,0,639,31);
  end;
  outtextxy((sc mod 20)*32,(sc div 20)*8,s);
end;
Begin
  th:=10;
  colmode:=false;
  drawmode:=1;
  sc:=-1;
  rand:=false;
end.
