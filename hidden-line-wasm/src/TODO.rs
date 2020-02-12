/*



dreidplot.pas




type
  int = longint;
  pvector3d = ^vector3d;
  pvector2d = ^vector2d;





function floatToString(f: float): string;


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


//var
//  th,tx: integer;





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
  exit(s);
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
    exit(-1)
  else if x > epsilon1 then
    exit(1)
  else
    exit(0);
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
*/
