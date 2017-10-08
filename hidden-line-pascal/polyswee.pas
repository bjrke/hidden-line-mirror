unit polyswee;
interface
uses crt,ptcgraph,polygon,vector,punkte,linien,dreiecke,zeit;
var
  zumalen:set of byte;
  mtf:longint;


procedure sweep;
procedure polypoly(p1:pdreieck;p2:ppoly);
procedure drawtree;
procedure abflachen;

implementation

function sdelete(p:ppoly):ppoly;
var
  h,h1,h2:ppoly;
begin
  zaehl.s.del;
  sdelete:=p;
  if p<>nil then begin
    h:=nil;
    if p^.so=nil then
      h:=p^.su
    else if p^.su=nil then
      h:=p^.so
    else begin
      if p^.po=nil then begin
        drawtree;
        outstring('p^.po=nil');
      end;
      if p^.pu=nil then begin
        drawtree;
        outstring('p^.pu=nil');
      end;
      rand:=not rand;
      if rand then begin
        h:=p^.pu;
        if h^.so<>nil then outstring('h^.so<>nil');
        if h=p^.su then begin
          verbinde(h,p^.so,so);
        end else begin
          h^.ss^:=h^.su;
          if h^.su<>nil then begin
            h^.su^.ss:=h^.ss;
          end;
          verbinde(h,p^.so,so);
          verbinde(h,p^.su,su)
        end
      end else begin
        h:=p^.po;
        if h^.su<>nil then outstring('h^.su<>nil');
        if h=p^.so then begin
          verbinde(h,p^.su,su);
        end else begin
          h^.ss^:=h^.so;
          if h^.so<>nil then begin
            h^.so^.ss:=h^.ss;
          end;
          verbinde(h,p^.so,so);
          verbinde(h,p^.su,su)
        end
      end;
    end;
    p^.ss^:=h;
    if h<>nil then
      h^.ss:=p^.ss;
    verbinde(p^.po,p^.pu,pu);
    p^.ss:=nil;
    p^.po:=nil;
    p^.pu:=nil;
    p^.so:=nil;
    p^.su:=nil;
  end else outstring('sdelete(nil)');
end;

function loesche(p:ppoly):ppoly;
var
  o,u,h,h1:ppoly;
  fertig:boolean;
begin
  loesche:=p;
  if p<>nil then begin
    o:=p^.po;
    u:=p^.pu;
    sdelete(p);
    del(p,3);
    fertig:=false;
    while (not fertig) and (o<>nil)and(u<>nil) do begin
      case polytest(o,u,true,false) of
        1:begin
          h:=o;
          o:=o^.po;
          sdelete(h);
          del(h,3);
          push(h,2,h^.count);
          fertig:=false;
        end;
        2:fertig:=true;
        4:begin
          h:=o;
          o:=o^.po;
          sdelete(h);
          del(h,3);
          if h<>nil then begin
            polypoly(u^.ur,h);
            killppoly(h,'lofall3')
          end else
            outstring('h is nil (falls3)');
        end;
        3:begin
          h:=u;
          u:=u^.pu;
          sdelete(h);
          del(h,3);
          if h<>nil then begin
            polypoly(o^.ur,h);
            killppoly(h,'lofall4')
          end else
            outstring('h is nil (falls4)');
        end;
        5:begin
          h:=o;
          o:=o^.po;
          sdelete(h);
          del(h,3);
          outint('l5 oz‰hler ',h^.count);
          killppoly(h,'l5');

          h:=u;
          u:=u^.pu;
          sdelete(h);
          del(h,3);
          killppoly(h,'l52');
          outint('l5 uz‰hler',h^.count);
        end;
      end;
    end;
  end else outstring('p ist nil');
end;

procedure insert(p:ppoly;schnitttest:boolean);
var
  h,o,u,a:ppoly;
  ak:pppoly;
  typ:richtung;
  ch:char;
  tf:int;
label ende;
begin
{  schnitttest:=true;}
{  if p^.flaechentest then begin}
  inc(zaehl.count);
    if drawmode=6 then begin
      p^.cols:=15;
      p^.draw2;
      outint('z‰hler',zaehl.count);
    end;
    tf:=0;
    a:=nil;
    ak:=@swurzel;
    while ak^<>nil do begin
      case polytest(ak^,p,schnitttest,false) of
        1:begin
            a:=ak^;
            ak:=@ak^^.so;
            typ:=so;
            inc(tf);
{            outstring('f1so');}
          end;
        2:begin
            a:=ak^;
            ak:=@ak^^.su;
            typ:=su;
            inc(tf);
{            outstring('f2su');}
          end;
        3:begin
            if p<>nil then begin
              polypoly(ak^^.ur,p);
              killppoly(p,'insert fall3');
{              outstring('f3u');}
            end else
              outstring('p is nil (fall3)');
            goto ende
          end;
        4:begin
{            outstring('f4o');}
            h:=loesche(ak^);
            if h<>nil then begin
              polypoly(p^.ur,h);
              killppoly(h,'insert fall4')
            end else
              outstring('h is nil (fall4)');
          end;
        5:begin
          outint('i5 pz‰hler',p^.count);
          killppoly(p,'i5');
          p:=loesche(ak^);
          outint('i5 akz‰hler',p^.count);
          killppoly(p,'i52');
          goto ende;
        end;
      end
    end;
    if a<>nil then begin
      verbinde(a,p,typ);
      if typ=so then begin
        o:=a^.po;
        u:=a
      end else begin
        o:=a;
        u:=a^.pu
      end;
      verbinde(o,p,pu);
      verbinde(p,u,pu);
      p^.so:=nil;
      p^.su:=nil;
      push(p,3,p^.count);
    end else begin
      swurzel:=p;
      p^.po:=nil;
      p^.pu:=nil;
      p^.so:=nil;
      p^.su:=nil;
      p^.pr:=nil;
      p^.ss:=@swurzel;
      push(swurzel,3,p^.count);
    end;
    zaehl.s.ins;
  ende:
    if tf>mtf then begin
      abflachen;
      inc(mtf);
    end;
    if drawmode=6 then begin
      repeat
        drawtree;
        repeat
          ch:=readkey
        until ch in [#32, #27, '1'..'9','a'];
        if ch in ['1'..'9'] then begin
          cls;
          if (ord(ch)-ord('0'))in zumalen then
            zumalen:=zumalen-[(ord(ch)-ord('0'))]
          else
            zumalen:=zumalen+[(ord(ch)-ord('0'))]
        end;
        if ch='a' then abflachen;
        if ch=#27 then drawmode:=1;
      until ch in [#32, #27];
      cls
    end;
{  end;}
end;

procedure polypoly;
var
  pl:array[1..20]of ppunkt;
  ll:array[1..20]of linie;
  am:array[1..20,1..20]of boolean;
  s:lset;
  h1,h2,h3,h4,plpos,llpos,i,j,k,pip:int;
  l,m:float;
  h:punkt;
  li:linie;
  w:boolean;

procedure addpl(p:punkt;ls:lset);
var h:ppunkt;
begin
  inc(plpos);
  initbppunkt(h,p.b,ls);
  pl[plpos]:=h;
end;

procedure addll(i,j:integer);
begin
  inc(llpos);
  ll[llpos].a:=pl[i];
  ll[llpos].e:=pl[j];
  am[i,j]:=true;
  am[j,i]:=true;
end;

procedure addppl(pu1,pu2,pu3:ppunkt);
var
  h:vector2d;
  ls:lset;
  ph:ppoly;
  pp:pppoly;

begin
  inc(zaehl.pp);
  h[x]:=(pu1^.b[x]+pu2^.b[x]+pu3^.b[x])/3;
  h[y]:=(pu1^.b[y]+pu2^.b[y]+pu3^.b[y])/3;
  if (p1^.punkttest(h)=20)and not colinear(pu1^,pu2^,pu3^) then begin
    ls:=[];
    if pu2^.gz*pu3^.gz<>[] then      ls:=ls+[1];
    if pu3^.gz*pu1^.gz<>[] then      ls:=ls+[2];
    if pu1^.gz*pu2^.gz<>[] then      ls:=ls+[3];
    initppoly(ph,pu1,pu2,pu3,ls,p2^.ur);
    h[x]:=(p1^.p[1]^.b[x]+p1^.p[2]^.b[x]+p1^.p[3]^.b[x])/3;
    h[y]:=(p1^.p[1]^.b[y]+p1^.p[2]^.b[y]+p1^.p[3]^.b[y])/3;
    if (ph^.punkttest(h)=20) {and ((ph^.gl<>[]) or (ph^.flaechentest))} then begin
      if ph^.p[1]^.b[x]>=xscan then begin
        ph^.cols:=2; {15}
        push(ph,1,zaehl.count)
      end else begin
        ph^.cols:=3;  {4}
        push(ph,2,zaehl.count)
      end
    end else
      killppoly(ph,'addppl 2');
  end;
end;

begin
  plpos:=0;  llpos:=0;
  for i:=1 to 20 do
    for j:=1 to 20 do
      am[i,j]:=false;
  for i:=1 to 3 do
    for j:=1 to 3 do
      if linien.intersect(p1^.l[j],p2^.l[i],l,m)=1 then begin
        h.b[x]:=(p1^.l[j].a^.b[x]+l*(p1^.l[j].e^.b[x]-p1^.l[j].a^.b[x])+
                 p2^.l[i].a^.b[x]+m*(p2^.l[i].e^.b[x]-p2^.l[i].a^.b[x]))/2;
        h.b[y]:=(p1^.l[j].a^.b[y]+l*(p1^.l[j].e^.b[y]-p1^.l[j].a^.b[y])+
                 p2^.l[i].a^.b[y]+m*(p2^.l[i].e^.b[y]-p2^.l[i].a^.b[y]))/2;
        addpl(h,p2^.gl*[i]);
      end;
  for i:=1 to 3 do begin
    pip:=p2^.punkttest(p1^.p[i]^.b);
    case pip of
      0:addpl(p1^.p[i]^,[]); {eckpunkte des oberen, die nur im(nicht auf)unteren sind}
      1..3:begin
            h1:=gleicheseite(p1^.p[i]^,p2^.l[pip].a^,p2^.p[pip]^,p1^.p[(i mod 3)+1]^);
            h2:=gleicheseite(p1^.p[i]^,p2^.l[pip].a^,p2^.p[pip]^,p1^.p[((i+1) mod 3)+1]^);
            if (h1=1) or (h2=1) then
              addpl(p1^.p[i]^,p2^.gl*[pip]);
          end;
      11..13:begin
             h1:=gleicheseite(p1^.p[i]^,p1^.p[(i mod 3)+1]^,p2^.p[((pip-1) mod 3)+1]^,p1^.p[((i+1) mod 3)+1]^);
             h2:=gleicheseite(p1^.p[i]^,p1^.p[((i+1) mod 3)+1]^,p2^.p[((pip-1) mod 3)+1]^,p1^.p[(i mod 3)+1]^);
             h3:=gleicheseite(p1^.p[i]^,p1^.p[(i mod 3)+1]^,p2^.p[(pip mod 3)+1]^,p1^.p[((i+1) mod 3)+1]^);
             h4:=gleicheseite(p1^.p[i]^,p1^.p[((i+1) mod 3)+1]^,p2^.p[(pip mod 3)+1]^,p1^.p[(i mod 3)+1]^);
             if (h1=-1)or(h2=-1)or(h3=-1)or(h4=-1) then addpl(p1^.p[i]^,p2^.gl*([1,2,3]-[pip-10]));
      end;
    end;  {3 drauﬂen brauchen wir nich}
  end;
  for i:=1 to 3 do begin
    pip:=p1^.punkttest(p2^.p[i]^.b);
    case pip of{eckpunkt unteres dreieck}
      1..3:begin
         h1:=gleicheseite(p2^.p[i]^,p1^.l[pip].a^,p1^.p[pip]^,p2^.p[(i mod 3)+1]^);
         h2:=gleicheseite(p2^.p[i]^,p1^.l[pip].a^,p1^.p[pip]^,p2^.p[((i+1) mod 3)+1]^);
         if (h1=-1)or(h2=-1) then addpl(p2^.p[i]^,p2^.gl*([1,2,3]-[i]));
      end;
      20:addpl(p2^.p[i]^,p2^.gl*([1,2,3]-[i])); {eckpunkte des unteren, die alle drauﬂen sind}
    end;
  end;
  begin
    for i:=1 to plpos do
      for j:=1 to i-1 do
        if j<>i then begin
          initlinie(li,pl[i],pl[j]);
          if not p1^.linientest(li) then begin
            w:=false;
            for k:=1 to llpos do
              w:=w or (linien.intersect(li,ll[k],l,m) in [1,2,3]);
            if not w then
              addll(i,j);
          end
        end;
    for i:=1 to plpos do
      for j:=i+1 to plpos do
        for k:=j+1 to plpos do
          if am[i,j] and am[j,k] and am[k,i] then
            addppl(pl[i],pl[j],pl[k]);
  end;
  for i:=1 to plpos do
    disposeppunkt(pl[i]);
end;

procedure sweep;
var
  p:ppoly;
  ende:boolean;
begin
  mtf:=0;
  swurzel:=polygon.pop(1);
  zaehl.s.ins;
  if swurzel<>nil then begin
    swurzel^.ss:=@swurzel;
    swurzel^.so:=nil;
    swurzel^.su:=nil;
    swurzel^.po:=nil;
    swurzel^.pu:=nil;
    push(swurzel,3,swurzel^.count);
    xscan:=swurzel^.p[1]^.b[x]
  end;
  ende:=false;
  while not ende do begin
    if first[1]=nil then begin
      if first[3]=nil then
        ende:=true
      else begin
        p:=loesche(first[3]);
        p^.cols:=15; {5}
        p^.draw;
        killppoly(p,'sweep ende');
      end;
    end else begin
      p:=polygon.pop(1);
      xscan:=p^.p[1]^.b[x];
      insert(p,true);

      if first[1]<>nil then xscan:=first[1]^.p[1]^.b[x];

      while(first[3]<>nil)and(first[3]^.p[3]^.b[x]<=xscan+epsilon1)do begin

        while first[2]<>nil do begin
          p:=polygon.pop(2);
          insert(p,false);
        end;
        p:=loesche(first[3]);
        p^.cols:=15; {6}
        p^.draw;
        killppoly(p,'insert pop3');
      end;
    end;
    if not ende then begin
      p:=polygon.pop(2);
      while p<>nil do begin
        insert(p,false);
        p:=polygon.pop(2);
      end;
    end;
  end;
end;

procedure drawtree;
var
  f:color;
procedure dp(p:ppoly;dx,dy:integer);
begin
  if (dy<480) and (p<>nil) then begin
    inc(f);
    p^.cols:=f;
    p^.drx:=dx;
    p^.dry:=dy;
    dp(p^.so,dx+320 shr (dy div 10),dy+10);
    dp(p^.su,dx-320 shr (dy div 10),dy+10);
    dec(f);
  end;
end;

procedure verbindung(s,z:ppoly;c:color);
begin
  if (z<>nil)and(z^.dry<480) then begin
    setcolor(c);
    line(s^.drx,s^.dry,z^.drx,z^.dry);
    line(200-4*s^.dry+round(xscan),240-round(s^.yscan),200-4*z^.dry+round(xscan),240-round(z^.yscan));
  end;
end;

procedure zeichne(p:ppoly);
var
  s:string;
  ys:int;
begin
  if p<>nil then begin
    setcolor(p^.cols);
    circle(p^.drx,p^.dry,3);
    verbindung(p,p^.so,1);
    verbindung(p,p^.su,2);
{    verbindung(p,p^.pu,4);
    verbindung(p,p^.po,4);}
    zeichne(p^.so);
    zeichne(p^.su);
    if p^.cols in zumalen then
      p^.draw3(p^.cols)
    else
      setcolor(p^.cols);
    ys:=round(p^.yscan);
    str(ys,s);
    outtextxy(200-4*p^.dry+round(xscan)-4*length(s),240-ys,s);
    outtextxy(p^.drx-12,p^.dry,s)
  end;
end;

begin
  setcolor(white);
  line(round(xscan+bmx),0,round(xscan+bmx),479);
  f:=0;
  dp(swurzel,320,10);
  zeichne(swurzel);
end;

procedure abflachen;
type
  fettesfeld=array[0..16382]of ppoly;
var
  pa:^fettesfeld;
  c:int;
  p,q:ppoly;
  t:int;

function newtree(a,e:int):ppoly;
var
  h,h1,h2:ppoly;
  x:int;
begin
  x:=a+((e-a)div 2);
  h:=pa^[x];
  h^.so:=nil;
  h^.su:=nil;
  inc(t);
  if a<=(x-1) then
    h1:=newtree(a,x-1)
  else
    h1:=nil;
  if (x+1)<=e then
    h2:=newtree(x+1,e)
  else
    h2:=nil;
  dec(t);
  verbinde(h,h1,so);
  verbinde(h,h2,su);
  newtree:=h;
end;

begin
  if swurzel<>nil then begin
    new(pa);
    p:=swurzel;
    while p<>nil do begin
      q:=p;
      p:=p^.so;
    end;
    c:=0;
    while q<>nil do begin
      pa^[c]:=q;
      inc(c);
      q:=q^.pu;
    end;
    t:=1;
    swurzel:=newtree(0,c-1);
    swurzel^.ss:=@swurzel;
    dispose(pa);
  end;
end;

end.
