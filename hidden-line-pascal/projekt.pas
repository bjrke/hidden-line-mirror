unit projekt;
interface
uses crt,ptcgraph,vector,polygon,dreiecke,punkte,linien;

procedure neukamera;
procedure rechnung;

var backface:boolean;
  spal,bpal:palettetype;

implementation

procedure neukamera;
begin
  cls;
  iv := BlickR.kreuz(jv);
  if iv.betrag3d=0 then outstring('i=0');
  iv := iv.mul3d(0.4*BlickR.betrag3d/(bmx*iv.betrag3d));
  jv := iv.kreuz(BlickR);
  if jv.betrag3d=0 then outstring('j=0');
  jv := jv.mul3d(0.4*BlickR.betrag3d/(bmx*jv.betrag3d));
end;

procedure perspektive(p:punkt3d);
var
  K:matrix3d;
  kd:float;
begin
  K.init(iv, jv, Auge.sub3d(p.o));
  kd:=K.det3d;
  if abs(kd)>epsilon3 then begin
    K.x:=BlickR.neg3d;
    p.b^.b.x:=K.det3d/kd;
    K.y:=K.x; K.x:=iv;
    p.b^.b.y:=K.det3d/kd
  end;
end;

procedure rechnung;
var
  i:ppunkt3d;
  j:pdreieck;
  ED:float;
  h:ppoly;
begin
  i:=points^.first;
  while i<>nil do begin
    perspektive(i^);
    i:=points^.next;
  end;

  ED:=BlickR.skalar(Auge)-epsilon1;
  j:=dreiecks.first;
  while j<>nil do begin
    if (BlickR.skalar(j^.o[1]^.o)>ED) AND (BlickR.skalar(j^.o[2]^.o)>ED) AND (BlickR.skalar(j^.o[3]^.o)>ED) AND // test if not behind view plane
       (not backface or ((j^.p[3]^.b.x-j^.p[1]^.b.x)*(j^.p[2]^.b.y-j^.p[1]^.b.y)+epsilon1<
                         (j^.p[3]^.b.y-j^.p[1]^.b.y)*(j^.p[2]^.b.x-j^.p[1]^.b.x))) then begin
         new(h, newpoly(j));
{         if h^.flaechentest then}
           polygon.push(h,1,-2)
{         else
           dispose(h,kill('flächentest'));}
       end;
    j:=dreiecks.next
  end;
{  xscan:=-1e20;}
end;
var
  tr,md:integer;
begin
  tr:=9;{installuserdriver('bgi256',nil);}
  md:=2;
  initgraph(tr,md,'');
  bmx:=getmaxx div 2;
  bmy:=getmaxy div 2;
  getdefaultpalette(spal);
  setrgbpalette(32,0,0,0);
  setpalette(32,32);

  for tr:=1 to 15 do begin
{    setrgbpalette(tr+32,32+trunc(2.05*tr),trunc(2.05*tr),trunc(0*tr));}
    setrgbpalette(tr+32,trunc(4.02*tr),trunc(4.02*tr),trunc(4.02*tr));
    setpalette(tr,tr+32);
  end;
  getpalette(bpal);
  setallpalette(spal);
  for tr:=0 to 15 do begin
    setcolor(tr);
    setfillstyle(1,tr);
    bar(0,tr*30,10,tr*30+29)
  end;
  getmem(palleiste,imagesize(0,0,10,479));
  getimage(0,0,10,479,palleiste^);
  setcolor(15);
  delay(300);
end.
