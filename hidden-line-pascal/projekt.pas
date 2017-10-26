unit projekt;
interface
uses ptccrt,ptcgraph,vector,polygon,dreiecke,punkte,linien,zeit;

procedure neukamera;
procedure rechnung;

var
  backface:boolean;
  palette: array [0..2] of palettetype;
implementation

procedure neukamera;
begin
  cls;
  iv := BlickR.kreuz(jv);
//  if iv.betrag3d=0 then outstring('i=0');
  iv := iv.mul3d(0.4*iv.invBetrag3d/(bmx*BlickR.invBetrag3d));
  jv := iv.kreuz(BlickR);
//  if jv.betrag3d=0 then outstring('j=0');
  jv := jv.mul3d(0.4*jv.invBetrag3d/(bmx*BlickR.invBetrag3d));
end;

procedure perspektive(p:ppunkt3d);
var
  K:matrix3d;
  kd:float;
begin
  K.init(iv, jv, Auge.sub3d(p^.o));
  kd:=K.det3d;
  if abs(kd)>epsilon2 then begin
    K.x:=BlickR.neg3d;
    p^.b^.b.x:=K.det3d/kd;
    K.y:=K.x; K.x:=iv;
    p^.b^.b.y:=K.det3d/kd;

    if ( drawmode = 5 ) then begin
      tiefePerspektive.update( 1.0 / p^.o.sub3d( Auge ).invBetrag3d );
    end;
  end;
end;

procedure rechnung;
var
  i:ppunkt3d;
  j:pdreieck;
  ED:float;
  h:ppoly;
begin
  tiefePerspektive.init;
  i:=points^.first;
  while i<>nil do begin
    perspektive(i);
    i:=points^.next;
  end;

  ED:=BlickR.skalar(Auge) + epsilon1;
  j:=dreiecks.first;
  while j<>nil do begin
    if (BlickR.skalar(j^.o[1]^.o)>ED) AND (BlickR.skalar(j^.o[2]^.o)>ED) AND (BlickR.skalar(j^.o[3]^.o)>ED) AND // test if not behind view plane
       // evtl kann man das mit der Lichtberechnung beim Initialisieren des Polygons kombinieren
       (not backface or ((j^.p[3]^.b.x-j^.p[1]^.b.x)*(j^.p[2]^.b.y-j^.p[1]^.b.y)+epsilon1<
                         (j^.p[3]^.b.y-j^.p[1]^.b.y)*(j^.p[2]^.b.x-j^.p[1]^.b.x))) then begin
        new(h, newpoly(j));
        if h^.flaechentest then
          polygon.push(h,1,-2)
        else
          dispose(h,done('flächentest'));
       end;
    j:=dreiecks.next
  end;
{  xscan:=-1e20;}
end;

procedure initGraphic;
var
  tr,md: Integer;
begin
  tr:=D8bit;
  md:=m1024x768;
  initgraph(tr,md,'');
end;

const colors = 16;
var p, i: Integer;
begin
  initGraphic;
  bmx:=getmaxx div 2;
  bmy:=getmaxy div 2;

  for p := 0 to Length(palette) - 1 do begin
    for i:=1 to (colors - 1) do begin
      setpalette(i,i+p*colors);
    end;
    getpalette(palette[p]);
  end;

  SetAllPalette(palette[0]);
  for i:=0 to (colors - 1) do begin
    setcolor(i);
    setfillstyle(1,i);
    bar(0,i*480 div colors,10,((i+1)*480 div colors) -1 )
  end;
  getmem(palleiste,imagesize(0,0,10,479));
  getimage(0,0,10,479,palleiste^);
end.
