unit projekt;
interface
uses crt,ptcgraph,vector,polygon,dreiecke,punkte,linien;

procedure neukamera;
procedure rechnung;

var backface:boolean;
  spal,bpal:palettetype;

implementation

procedure neukamera;
var tr:int;
begin
  cls;
  kreuz(iv,BlickR,jv);
  if betrag3d(iv)=0 then outstring('i=0');
  mul3d(iv,0.4*betrag3d(BlickR)/(bmx*betrag3d(iv)));
  kreuz(jv,iv,BlickR);
  if betrag3d(jv)=0 then outstring('j=0');
  mul3d(jv,0.4*betrag3d(BlickR)/(bmx*betrag3d(jv)));
end;

procedure perspektive(p:punkt3d);
var
  help:vector3d;
  K:matrix3d;
  kd:float;
begin
  sub3d(help,Auge,p.o);
  K[x]:=iv; K[y]:=jv; K[z]:=help;
  kd:=det3d(K);
  if abs(kd)>epsilon3 then begin
    K[x]:=BlickR;
    neg3d(K[x]);
    p.b^.b[x]:=det3d(k)/kd;
    K[y]:=K[x]; K[x]:=iv;
    p.b^.b[y]:=det3d(k)/kd
  end;
end;

procedure rechnung;
var
  i:ppunkt3d;
  j:pdreieck;
  EA,EB,EC,ED:float;
  w:boolean;
  h:ppoly;

begin
  i:=points^.first;
  while i<>nil do begin
    perspektive(i^);
    i:=points^.next;
  end;

  EA:=BlickR[x];
  EB:=BlickR[y];
  EC:=BlickR[z];
  ED:=-EA*auge[x]-EB*auge[y]-EC*auge[z];
  j:=dreiecks^.first;
  while j<>nil do begin
    if (EA*j^.o[1]^.o[x]+EB*j^.o[1]^.o[y]+EC*j^.o[1]^.o[z]+ED+epsilon1>0) AND
       (EA*j^.o[2]^.o[x]+EB*j^.o[2]^.o[y]+EC*j^.o[2]^.o[z]+ED+epsilon1>0) AND
       (EA*j^.o[3]^.o[x]+EB*j^.o[3]^.o[y]+EC*j^.o[3]^.o[z]+ED+epsilon1>0) AND
       (not backface or ((j^.p[3]^.b[x]-j^.p[1]^.b[x])*(j^.p[2]^.b[y]-j^.p[1]^.b[y])+epsilon1<
       (j^.p[3]^.b[y]-j^.p[1]^.b[y])*(j^.p[2]^.b[x]-j^.p[1]^.b[x]))) then begin
         h:=newpoly(j);
{         if h^.flaechentest then}
           polygon.push(h,1,-2)
{         else
           dispose(h,kill('flächentest'));}
       end;
    j:=dreiecks^.next
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
