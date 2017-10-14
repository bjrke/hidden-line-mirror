unit polygon;
interface
uses crt,vector,punkte,linien,dreiecke,ptcgraph,zeit;
type
  punr=1..3;
  richtung=(li,re,so,su,po,pu,pr,ne);
  pppoly=^ppoly;
  ppoly=^poly;
  poly=object(dreiecktyp)
    li,re,ne,pr:ppoly;
    so,su,pu,po:ppoly;
    ss,ps:pppoly;
    ur:pdreieck;
    drx,dry:integer;
    farbe:color;
    count:int;
    ymin,ymax:float;

    constructor init(p1,p2,p3:ppunkt;ls:lset;u:pdreieck);
    constructor newpoly(d:pdreieck);

    destructor done(aufr: string);

    function yscan:float;
    procedure draw;
  end;
var
  swurzel:ppoly;
  wurzel,first:array [punr] of ppoly;
  xscan:float;
  colmode:boolean;
  drawmode:integer;
  rand:boolean;

procedure push(p:ppoly;pnr:punr;c:int);
function pop(pnr:punr):ppoly;
function del(p:ppoly;pnr:punr):ppoly;
function polytest(p1,p2:ppoly;schnitttest,ausgabe:boolean):byte;
procedure verbinde(v,s:ppoly;r:richtung);

implementation

destructor poly.done;                   {hier druff guggn!}
var i:integer;
begin
{  outstring(aufr,false);}
  for i:=1 to 3 do
    if p[i]<>nil then
      dispose(p[i], done)
    else
      outstring('p[i]=nil');
  zaehl.p.del;
end;

procedure poly.draw;
var my1,my2:float;
    wx,wy,xm1,xm2,ym1,ym2,bx,by:int;
    h:vector2d;
begin
  if colmode then cols:=random(15)+1;
  case drawmode of
       1:draw1;
       2:draw2;
       3:draw3(cols);
       4:begin
           my1:=p[1]^.b.y;
           my2:=my1;
           if p[2]^.b.y>my1 then
             my2:=p[2]^.b.y
           else
             my1:=p[2]^.b.y;
           if p[3]^.b.y>my2 then
             my2:=p[3]^.b.y
           else if p[3]^.b.y<my1 then
             my1:=p[3]^.b.y;
           ym1:=round(my1);
           ym2:=round(my2);
           xm1:=round(p[1]^.b.x);
           xm2:=round(p[3]^.b.x);
           bx:=round(bmx);
           by:=round(bmy);
           my2:=xscan;
           if (not keypressed) or (readkey<>#13) then
             for wx:=xm1 to xm2 do begin
               xscan:=wx;
               for wy:=ym1 to ym2 do begin
                 h.x:=wx;
                 h.y:=wy;
                 if punkttest(h)=0 then begin
                   my1:=(bmy-yscan)/30;
                   if my1>15 then putpixel(wx+bx,by-wy,15) else
                     if my1<0 then putpixel(wx+bx,by-wy,0) else
                       putpixel(bx+wx,by-wy,round(my1))
                 end
               end
             end;
           xscan:=my2;
         end;
       5:begin
           my1:=p[1]^.b.y;
           my2:=my1;
           if p[2]^.b.y>my1 then
             my2:=p[2]^.b.y
           else
             my1:=p[2]^.b.y;
           if p[3]^.b.y>my2 then
             my2:=p[3]^.b.y
           else if p[3]^.b.y<my1 then
             my1:=p[3]^.b.y;
           ym1:=round(my1);
           ym2:=round(my2);
           xm1:=round(p[1]^.b.x);
           xm2:=round(p[3]^.b.x);
           bx:=round(bmx);
           by:=round(bmy);
           if (not keypressed) or (readkey<>#13) then
             for wx:=xm1 to xm2 do
               for wy:=ym1 to ym2 do begin
                 h.x:=wx;
                 h.y:=wy;
                 if punkttest(h)=0 then begin
                   my1:=mt*ur^.tiefe(h,false)+nt;
                   if my1>15 then putpixel(wx+bx,by-wy,15) else
                     if my1<0 then putpixel(wx+bx,by-wy,0) else
                       putpixel(bx+wx,by-wy,round(my1))
                 end;
               end;
       end;
       6:draw1;
       7:begin
         cols:=ur^.cols;
         draw2
       end;
       8:begin
         cols:=farbe;
         draw1;
       end;
  end;
end;

constructor poly.init;
var
  h:ppunkt;
  lsneu:lset;

begin
  zaehl.p.ins;

  if p1=nil then outstring('p1=nil');
  if p2=nil then outstring('p2=nil');
  if p3=nil then outstring('p3=nil');

  p[1] := p1^.copy;
  p[2] := p2^.copy;
  p[3] := p3^.copy;

  if p[1]^.b.x>p[2]^.b.x then begin
    h:=p[2];      p[2]:=p[1];      p[1]:=h;
    lsneu:=[];
    if 1 in ls then lsneu:=lsneu+[2];
    if 2 in ls then lsneu:=lsneu+[1];
    if 3 in ls then lsneu:=lsneu+[3];
    ls:=lsneu
  end;

  if p[1]^.b.x>p[3]^.b.x then begin
    h:=p[3];      p[3]:=p[1];      p[1]:=h;
    lsneu:=[];
    if 1 in ls then lsneu:=lsneu+[3];
    if 2 in ls then lsneu:=lsneu+[2];
    if 3 in ls then lsneu:=lsneu+[1];
    ls:=lsneu
  end;

  if p[2]^.b.x>p[3]^.b.x then begin
    h:=p[3];      p[3]:=p[2];      p[2]:=h;
    lsneu:=[];
    if 1 in ls then lsneu:=lsneu+[1];
    if 2 in ls then lsneu:=lsneu+[3];
    if 3 in ls then lsneu:=lsneu+[2];
    ls:=lsneu
  end;

  gl:=ls;

  l[1].init(p[2],p[3]);
  l[2].init(p[3],p[1]);
  l[3].init(p[1],p[2]);

  ur:=u;
  cols:=green;

  ymin:=p[1]^.b.y;
  ymax:=p[1]^.b.y;

  if p[2]^.b.y<ymin then
    ymin:=p[2]^.b.y
  else if p[2]^.b.y>ymax then
    ymax:=p[2]^.b.y;

  if p[3]^.b.y<ymin then
    ymin:=p[3]^.b.y
  else if p[3]^.b.y>ymax then
    ymax:=p[3]^.b.y
end;

constructor poly.newpoly;
var
{  A1,A2:float;
  t:vector3d;}
  c:vector3d;
begin
  c :=   d^.o[1]^.o.sub3d(d^.o[2]^.o)
  .kreuz(d^.o[3]^.o.sub3d(d^.o[2]^.o));
  d^.cols:=round(1+14*BlickR.skalar(c)/(BlickR.betrag3d*c.betrag3d));

{  add3d(t,a,b);
  add3d(t,t,c);

  A1:=betrag3d(c)*betrag3d(t);
  A2:=abs((d^.p[1]^.b[x]-d^.p[2]^.b[x])*(d^.p[3]^.b[y]-d^.p[2]^.b[y])
         -(d^.p[1]^.b[y]-d^.p[2]^.b[y])*(d^.p[3]^.b[x]-d^.p[2]^.b[x]));}
{  d^.cols:=round(1+A2/(10*A1));}
  if d^.cols>15 then d^.cols:=15;
  if d^.cols<1 then d^.cols:=1;

  init(d^.p[1],d^.p[2],d^.p[3],d^.gl,d);
end;

procedure push;
var a:ppoly;
    fertig:boolean;

begin
  p^.count:=c;
  if pnr<>3 then p^.farbe:=pnr;
  zaehl.q[pnr].ins;
  p^.ne:=nil;
  p^.li:=nil;
  p^.re:=nil;
  p^.ps:=nil;
  p^.pr:=nil;
  if wurzel[pnr]=nil then begin
    wurzel[pnr]:=p;
    p^.ps:=@wurzel[pnr];
    first[pnr]:=p;
  end else begin
    a:=wurzel[pnr];
    fertig:=false;
    repeat
      if p^.p[pnr]^.b.x<=a^.p[pnr]^.b.x then begin
        if a^.li<>nil then begin
          a:=a^.li
        end else begin
          verbinde(a,p,li);
          verbinde(a^.pr,p,ne);
          verbinde(p,a,ne);
          fertig:=true
        end
      end else begin
        if a^.re<>nil then begin
          a:=a^.re
        end else begin
          verbinde(a,p,re);
          verbinde(p,a^.ne,ne);
          verbinde(a,p,ne);
          fertig:=true
        end
      end
    until fertig
  end;
{  if first[pnr]=nil then begin
    first[pnr]:=wurzel[pnr];
    outstring('first[pnr]is nil',false);
  end;}
  while first[pnr]^.li<>nil do
    first[pnr]:=first[pnr]^.li;
end;

function pop;
begin
  pop:=del(first[pnr],pnr);
end;

function del;
var
  h:ppoly;
begin
  del:=p;
  if p<>nil then begin
    zaehl.q[pnr].del;
    if p=first[pnr] then
      first[pnr]:=first[pnr]^.ne;
    h:=nil;
    if p^.re=nil then
      h:=p^.li
    else if p^.li=nil then
      h:=p^.re
    else begin
      if p^.ne=nil then begin
        outstring('p^.ne=nil');
      end;
      if p^.pr=nil then begin
        outstring('p^.pr=nil');
      end;
      rand:=not rand;
      if rand then begin
        h:=p^.pr;
        if h^.re<>nil then outstring('h^.re<>nil');
        if h=p^.li then begin
          verbinde(h,p^.re,re);
        end else begin
          h^.ps^:=h^.li;
          if h^.li<>nil then begin
            h^.li^.ps:=h^.ps;
          end;
          verbinde(h,p^.re,re);
          verbinde(h,p^.li,li)
        end
      end else begin
        h:=p^.ne;
        if h^.li<>nil then outstring('h^.li<>nil');
        if h=p^.re then begin
          verbinde(h,p^.li,li);
        end else begin
          h^.ps^:=h^.re;
          if h^.re<>nil then begin
            h^.re^.ps:=h^.ps;
          end;
          verbinde(h,p^.re,re);
          verbinde(h,p^.li,li)
        end
      end;
    end;
    p^.ps^:=h;
    if h<>nil then
      h^.ps:=p^.ps;
    verbinde(p^.ne,p^.pr,pr);
    p^.ps:=nil;
    p^.li:=nil;
    p^.re:=nil;
    p^.ne:=nil;
    p^.pr:=nil;
  end;
end;

function poly.yscan;
var
  h:float;
  miny,maxy,x1,y1,x2,y2:float;
begin
  x1:=p[1]^.b.x;
  y1:=p[1]^.b.y;
  x2:=p[3]^.b.x;
  y2:=p[3]^.b.y;
  if y1<y2 then begin
    miny:=y1;
    maxy:=y2;
  end else begin
    miny:=y2;
    maxy:=y1;
  end;
  if maxy<p[2]^.b.y then
    maxy:=p[2]^.b.y
  else if p[2]^.b.y<miny then
    miny:=p[2]^.b.y;
  if abs(x2-x1)<epsilon1 then
    h:=(y1+y2)
  else begin
    h:=y1+(y2-y1)*(xscan-x1)/(x2-x1);
    if xscan<p[2]^.b.x then begin
      x2:=p[2]^.b.x;
      y2:=p[2]^.b.y;
    end else begin
      x1:=p[2]^.b.x;
      y1:=p[2]^.b.y;
    end;
    if abs(x2-x1)<epsilon1 then
      h:=y1+y2
    else
      h:=h+y1+(y2-y1)*(xscan-x1)/(x2-x1);
  end;
  if (h<2*miny) or (h>2*maxy) then begin
    h:=miny+maxy;
  end;
  yscan:=h/2
end;

function polytest;
var
  i,j,k:integer;
  pip,h1,h2,h3,h4:shortint;
  l,m:float;
  h:vector2d;
  schnitt,v1,v2:boolean;

  procedure test(p:vector2d);
  var
    d1,d2:float;
  begin
    if (p1^.punkttest(h)=0)and(p2^.punkttest(h)=0)then begin
      d1:=p1^.ur^.tiefe(p,ausgabe);
      d2:=p2^.ur^.tiefe(p,ausgabe);
      schnitt:=d1<>d2;
      if p1^.ur=p2^.ur then begin
        v2:=true;
      end else
        v1:=d1<d2;
      if ausgabe then begin
        outvector2d('p',p);
        outfloat('d1',d1);
        outfloat('d2',d2);
      end;
    end
  end;

  procedure addpl(v:vector2d);
  begin
    h := h.add2d(v);
    inc(k);
  end;

begin
  inc(zaehl.ptest);
  if p1^.ymin-1>p2^.ymax then
    polytest:=1
  else if p2^.ymin-1>p1^.ymax then
    polytest:=2
  else begin
    schnitt:=false;
    v2:=false;
    if schnitttest and (p1^.ur<>p2^.ur) then begin
      k:=0;
      h.init(0,0);
      i:=1;
      j:=1;
      while (k<6)and(j<=3)do begin
        if linien.intersect(p1^.l[j],p2^.l[i],l,m)=1 then begin
          h.x:=h.x+p1^.l[j].a^.b.x+l*(p1^.l[j].e^.b.x-p1^.l[j].a^.b.x)+
                     p2^.l[i].a^.b.x+m*(p2^.l[i].e^.b.x-p2^.l[i].a^.b.x);
          h.y:=h.y+p1^.l[j].a^.b.y+l*(p1^.l[j].e^.b.y-p1^.l[j].a^.b.y)+
                     p2^.l[i].a^.b.y+m*(p2^.l[i].e^.b.y-p2^.l[i].a^.b.y);
          inc(k)
        end;
        inc(i);
        if i=4 then begin
          inc(j);
          i:=1;
        end;
      end;

      h := h.div2d(2);

      i:=1;
      while {(k<6)and}(i<=3)do begin
        pip:=p2^.punkttest(p1^.p[i]^.b);
        case pip of
          0:addpl(p1^.p[i]^.b); {eckpunkte des oberen, die nur im(nicht auf)unteren sind}
          1..3:begin
                h1:=gleicheseite(p1^.p[i]^,p2^.l[pip].a^,p2^.p[pip]^,p1^.p[(i mod 3)+1]^);
                h2:=gleicheseite(p1^.p[i]^,p2^.l[pip].a^,p2^.p[pip]^,p1^.p[((i+1) mod 3)+1]^);
                if (h1=1) or (h2=1) then
                  addpl(p1^.p[i]^.b);
              end;
          11..13:begin
                 h1:=gleicheseite(p1^.p[i]^,p1^.p[(i mod 3)+1]^,p2^.p[((pip-1) mod 3)+1]^,p1^.p[((i+1) mod 3)+1]^);
                 h2:=gleicheseite(p1^.p[i]^,p1^.p[((i+1) mod 3)+1]^,p2^.p[((pip-1) mod 3)+1]^,p1^.p[(i mod 3)+1]^);
                 h3:=gleicheseite(p1^.p[i]^,p1^.p[(i mod 3)+1]^,p2^.p[(pip mod 3)+1]^,p1^.p[((i+1) mod 3)+1]^);
                 h4:=gleicheseite(p1^.p[i]^,p1^.p[((i+1) mod 3)+1]^,p2^.p[(pip mod 3)+1]^,p1^.p[(i mod 3)+1]^);
                 if (h1=-1)or(h2=-1)or(h3=-1)or(h4=-1) then
                   addpl(p1^.p[i]^.b);
          end;
        end;
        inc(i);
      end;
      i:=1;
      while {(k<6)and}(i<=3)do begin
        pip:=p1^.punkttest(p2^.p[i]^.b);
        case pip of
          0:addpl(p2^.p[i]^.b); {eckpunkte des oberen, die nur im(nicht auf)unteren sind}
          1..3:begin
                h1:=gleicheseite(p2^.p[i]^,p1^.l[pip].a^,p1^.p[pip]^,p2^.p[(i mod 3)+1]^);
                h2:=gleicheseite(p2^.p[i]^,p1^.l[pip].a^,p1^.p[pip]^,p2^.p[((i+1) mod 3)+1]^);
                if (h1=1) or (h2=1) then
                  addpl(p2^.p[i]^.b);
              end;
          11..13:begin
                 h1:=gleicheseite(p2^.p[i]^,p2^.p[(i mod 3)+1]^,p1^.p[((pip-1) mod 3)+1]^,p2^.p[((i+1) mod 3)+1]^);
                 h2:=gleicheseite(p2^.p[i]^,p2^.p[((i+1) mod 3)+1]^,p1^.p[((pip-1) mod 3)+1]^,p2^.p[(i mod 3)+1]^);
                 h3:=gleicheseite(p2^.p[i]^,p2^.p[(i mod 3)+1]^,p1^.p[(pip mod 3)+1]^,p2^.p[((i+1) mod 3)+1]^);
                 h4:=gleicheseite(p2^.p[i]^,p2^.p[((i+1) mod 3)+1]^,p1^.p[(pip mod 3)+1]^,p2^.p[(i mod 3)+1]^);
                 if (h1=-1)or(h2=-1)or(h3=-1)or(h4=-1) then
                   addpl(p2^.p[i]^.b);
          end;
        end;
        inc(i);
      end;
  {    while (k<6)and(i<=3)do begin
        if p1^.punkttest(p2^.p[i]^.b)=0 then begin
          inc(k,2);
          h[x]:=h[x]+p2^.p[i]^.b[x];
          h[y]:=h[y]+p2^.p[i]^.b[y];
        end;
        inc(i);
      end;}
      if k>0 then begin
        h := h.div2d(k);

        test(h);

      end else begin

      end;
    end;

    if not schnitt then begin
      h.x:=(p1^.p[1]^.b.x+p1^.p[2]^.b.x+p1^.p[3]^.b.x)/3;
      h.y:=(p1^.p[1]^.b.y+p1^.p[2]^.b.y+p1^.p[3]^.b.y)/3;
      test(h);
    end;

    if not schnitt then begin
      h.x:=(p2^.p[1]^.b.x+p2^.p[2]^.b.x+p2^.p[3]^.b.x)/3;
      h.y:=(p2^.p[1]^.b.y+p2^.p[2]^.b.y+p2^.p[3]^.b.y)/3;
      test(h);
    end;

    if not schnitt then begin
      h.x:=(p1^.p[1]^.b.x+p1^.p[2]^.b.x+p1^.p[3]^.b.x+p2^.p[1]^.b.x+p2^.p[2]^.b.x+p2^.p[3]^.b.x)/6;
      h.y:=(p1^.p[1]^.b.y+p1^.p[2]^.b.y+p1^.p[3]^.b.y+p2^.p[1]^.b.y+p2^.p[2]^.b.y+p2^.p[3]^.b.y)/6;
      test(h);
    end;

    if v2 then
      polytest:=5
    else if not schnitt then begin
      l:=xscan;
      if p1^.p[1]^.b.x<p2^.p[1]^.b.x then
        xscan:=p2^.p[1]^.b.x
      else
        xscan:=p1^.p[1]^.b.x;
      if p1^.p[3]^.b.x>p2^.p[3]^.b.x then
        xscan:=(xscan+p2^.p[3]^.b.x)/2
      else
        xscan:=(xscan+p1^.p[3]^.b.x)/2;

      if drawmode = 6 then begin
        marke(round(320+xscan),round(240-p1^.yscan),yellow,'p1^.yscan');
        marke(round(320+xscan),round(240-p2^.yscan),lightmagenta,'p2^.yscan');
      end;
      if p1^.yscan>p2^.yscan then
        polytest:=1
      else
        polytest:=2;
      xscan:=l;
    end else begin
      if v1 then
        polytest:=3
      else
        polytest:=4
    end;
    if ausgabe then begin
      outvector2d('p1^.p[1]^.b',p1^.p[1]^.b);
      outvector2d('p1^.p[2]^.b',p1^.p[2]^.b);
      outvector2d('p1^.p[3]^.b',p1^.p[3]^.b);
      outvector2d('p2^.p[1]^.b',p2^.p[1]^.b);
      outvector2d('p2^.p[2]^.b',p2^.p[2]^.b);
      outvector2d('p2^.p[3]^.b',p2^.p[3]^.b);
      outvector2d('ur^.p[1]^.b',p2^.ur^.p[1]^.b);
      outvector2d('ur^.p[2]^.b',p2^.ur^.p[2]^.b);
      outvector2d('ur^.p[3]^.b',p2^.ur^.p[3]^.b);
      p1^.draw3(1);
      p2^.draw3(2);

      readkey;
    end;
  end;
end;

procedure verbinde;
begin
    case r of
        so:begin
          if v<>nil then
            v^.so:=s;
          if s<>nil then begin
            if v<>nil then
              s^.ss:=@v^.so
            else
              s^.ss:=nil
          end
        end;
      su:begin
        if v<>nil then
          v^.su:=s;
        if s<>nil then begin
          if v<>nil then
            s^.ss:=@v^.su
          else s^.ss:=nil
        end
      end;
      li:begin
        if v<>nil then
          v^.li:=s;
        if s<>nil then begin
          if v<>nil then
            s^.ps:=@v^.li
          else
            s^.ps:=nil
        end
      end;
      re:begin
        if v<>nil then
          v^.re:=s;
        if s<>nil then begin
          if v<>nil then
            s^.ps:=@v^.re
          else
            s^.ps:=nil
        end
      end;
      po:begin
        if v<>nil then
          v^.po:=s;
        if s<>nil then
          s^.pu:=v
      end;
      pu:begin
        if v<>nil then
          v^.pu:=s;
        if s<>nil then
          s^.po:=v
      end;
      pr:begin
        if v<>nil then
          v^.pr:=s;
        if s<>nil then
          s^.ne:=v
      end;
      ne:begin
        if v<>nil then
          v^.ne:=s;
        if s<>nil then
          s^.pr:=v
      end;
    else
      outstring('gehtnicht');
    end
end;

begin
  colmode:=false;
  drawmode:=1;
  randomize;
  wurzel[1]:=nil;
  first[1]:=nil;
  wurzel[3]:=nil;
  first[3]:=nil;
  swurzel:=nil;
  rand:=false;
end.
