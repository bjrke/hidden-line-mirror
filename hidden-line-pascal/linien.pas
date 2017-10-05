unit linien;
interface
uses ptcgraph,vector,punkte;
type
  linie=object
    a,e:ppunkt;
    procedure draw(c:color);
    procedure draw1(c:color);
    procedure draw2(l1,l2:float;c:color);
  end;
function intersect(l1,l2:linie;var lambda,mue:float):int;
procedure initlinie(var p:linie;p1,p2:ppunkt);
implementation
procedure initlinie;
begin
  if p1^.b[x]<p2^.b[x] then begin
    p.a:=p1;
    p.e:=p2
  end else begin
    p.a:=p2;
    p.e:=p1
  end;
end;

procedure linie.draw1;
var
  x1,y1,x2,y2:int;
  dx,dy:float;

begin
  setcolor(c);
  if a^.gz*e^.gz<>[]  then
    line(round(bmx+a^.b[x]),round(bmy-a^.b[y]),round(bmx+e^.b[x]),round(bmy-e^.b[y]));
end;

procedure linie.draw;
var
  w:boolean;
begin
  w:=true;
  setcolor(c);
  if a=nil then
    outstring('a=nil')
  else if a=nil then
    outstring('e=nil')
  else begin
{    with a^ do begin
      if (b[x]<-bmx) or (b[x]>bmx) then begin
        outstring('a.x:',false);
        outfloat(b[x],true);
        w:=false;
      end;
      if (b[y]<-bmy) or (b[y]>bmy) then begin
        outstring('a.y:',false);
        outfloat(b[y],false);
        w:=false;
      end;
    end;
    with e^ do begin
      if (b[x]<-bmx) or (b[x]>bmx) then begin
        outstring('e.x:',false);
        outfloat(b[x],false);
        w:=false;
      end;
      if (b[y]<-bmy) or (b[y]>bmy) then begin
        outstring('e.y:',false);
        outfloat(b[y],false);
        w:=false;
      end;
    end;
    if w then}
      line(round(bmx+a^.b[x]),round(bmy-a^.b[y]),round(bmx+e^.b[x]),round(bmy-e^.b[y]));
  end;
end;

procedure linie.draw2;
var
  x1,y1,x2,y2:int;
  dx,dy:float;

begin
  setcolor(c);
  dx:=e^.b[x]-a^.b[x];
  dy:=e^.b[y]-a^.b[y];
  line(round(bmx+a^.b[x]+l1*dx),round(bmy-a^.b[y]-l1*dy),round(bmx+a^.b[x]+l2*dx),round(bmy-a^.b[y]-l2*dy));
end;

function intersect;
var
  k:matrix2d;
  h:vector2d;
  dk:float;
  i:int;
begin
  i:=0;
  k[x,x]:=l1.e^.b[x]-l1.a^.b[x];  k[x,y]:=l1.e^.b[y]-l1.a^.b[y];
  k[y,x]:=l2.a^.b[x]-l2.e^.b[x];  k[y,y]:=l2.a^.b[y]-l2.e^.b[y];
  dk:=det2d(k);
  if abs(dk)>epsilon1 then begin
    h:=k[x];
    k[x,x]:=l2.a^.b[x]-l1.a^.b[x];    k[x,y]:=l2.a^.b[y]-l1.a^.b[y];
    lambda:=det2d(k)/dk;
    k[y]:=k[x];
    k[x]:=h;
    mue:=det2d(k)/dk;
    if (lambda>epsilon1)and(lambda<1-epsilon1)and(mue>epsilon1)and(mue<1-epsilon1) then i:=1 else {schneiden sich ordentlich}
    if (lambda>epsilon1)and(lambda<1-epsilon1)and((abs(mue-1)<=epsilon1) or (abs(mue)<=epsilon1))then i:=2 else
       {min 1 endpunkt2 auf linie 1}
    if ((abs(lambda-1)<=epsilon1) or (abs(lambda)<=epsilon1))and(mue>epsilon1)and(mue<1-epsilon1)then i:=3 else
       {min 1 endpunkt1 auf linie 2}
    if ((abs(lambda-1)<=epsilon1) or (abs(lambda)<=epsilon1))and((abs(mue-1)<=epsilon1) or (abs(mue)<=epsilon1)) then i:=4;
       {1 gemeinsamer endpunkt}
  end;
  intersect:=i;
end;
end.
