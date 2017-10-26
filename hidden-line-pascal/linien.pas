unit linien;
interface
uses ptcgraph,vector,punkte;
type
  linie=object
    a,e:ppunkt;
    constructor init(p1,p2: ppunkt);
    procedure draw(c:color);
  end;

function intersect(l1,l2:linie;var lambda,mue:float):int;

implementation
constructor linie.init;
begin
  if p1^.b.x<p2^.b.x then begin
    a:=p1;
    e:=p2
  end else begin
    a:=p2;
    e:=p1
  end;
end;

procedure linie.draw;
begin
  setcolor(c);
  if a=nil then
    outstring('a=nil')
  else if a=nil then
    outstring('e=nil')
  else begin
    line(round(bmx+a^.b.x),round(bmy-a^.b.y),round(bmx+e^.b.x),round(bmy-e^.b.y));
  end;
end;

function intersect;
var
  k:matrix2d;
  h:vector2d;
  dk:float;
begin
  k.init(l1.e^.b.sub2d(l1.a^.b), l2.a^.b.sub2d(l2.e^.b));

  dk:=k.det2d;
  if (abs(dk) <= epsilon1) then
    intersect := 0 // schneiden sich nicht
  else begin
    h := l2.a^.b.sub2d(l1.a^.b);
    lambda:=k.withX(h).det2d/dk;
    mue:=k.withY(h).det2d/dk;
    if (lambda>epsilon1) and (lambda<1-epsilon1) and (mue>epsilon1) and (mue<1-epsilon1) then
      intersect := 1 // schneiden sich ordentlich
    else if (lambda>epsilon1) and (lambda<1-epsilon1) and ((abs(mue-1)<=epsilon1) or (abs(mue)<=epsilon1)) then
      intersect := 2 // min 1 endpunkt2 auf linie 1
    else if ((abs(lambda-1)<=epsilon1) or (abs(lambda)<=epsilon1)) and (mue>epsilon1) and (mue<1-epsilon1) then
      intersect := 3 // min 1 endpunkt1 auf linie 2
    else if ((abs(lambda-1)<=epsilon1) or (abs(lambda)<=epsilon1)) and ((abs(mue-1)<=epsilon1) or (abs(mue)<=epsilon1)) then
      intersect := 4 // 1 gemeinsamer endpunkt
    else
      intersect := 0 // schneiden sich nicht
  end;
end;

end.
