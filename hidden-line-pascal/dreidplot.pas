program dreidplot;
uses ptccrt,ptcgraph,vector,dreidext,projekt,dreiecke,polyswee,polygon,punkte,zeit;
var
  ch:char;
  p:ppoly;
  palettePos: Integer;

procedure init;
var {a,b,c:int;}
    {h:^triStrip;}
    qs:^quadstrip;
    xx,yy:float;

  function fkt(x,y:float):float;
  var
    h:float;
  begin
    h:=sqrt(sqr(x)+sqr(y));
    fkt:=30*cos(h)/(2+h);
  {  fkt:=random;}
  {  fkt:=sin(y)*x/10;}
  end;

const
//  xs=4;      ys=4;      zs=4;
//  xo=3;      yo=3;      zo=3;
  sw=0.5;
  ad=13;
//  w34=0.43301270189221932338186158537647;

{var
  r1,r2,r3:float;}

begin
  tausgabe:=true;

  auge.x:=30;  auge.y:=40;  auge.z:=50;
  blickR.x:=-auge.x/10;  blickR.y:=-auge.y/10;  blickR.z:=-auge.z/10;
{ blickR.x:=-3;  blickR.y:=-6;  blickR.z:=-12;}
  jv.x:=0;  jv.y:=0;  jv.z:=1;

//  setallpalette(bpal);
  backface:=true;

  drawmode := 1;

{ tetraeder(0,0,0, -1,0,-2, 1,1,-2 ,1,-1,-2);

  cube(-4,2,-2, 3,0,0, 0,3,0, 0,0,3);

  kegel(1,3,-2, 1,0,0, 0,1,0, 0,0,2, 8);

  kugel(0,8,0, 3, 32, 16); }

{  kegel(0,0,0, 2,0,0, 0,2,0, 0,0,-4, 10);  }

{  cube (0,0,0, 1,0,0, 0,1,0, 0,0,1);}

  begin
    xx:=-ad;
    while xx<ad do begin
      new(qs,init(xx,-ad,fkt(xx,-ad),xx+sw,-ad,fkt(xx+sw,-ad),xx,-ad+sw,fkt(xx,-ad+sw),xx+sw,-ad+sw,fkt(xx+sw,-ad+sw)));
      yy:=-ad+2*sw;
      while yy<ad do begin
        qs^.add(xx,yy,fkt(xx,yy),xx+sw,yy,fkt(xx+sw,yy));
        yy:=yy+sw;
      end;
      xx:=xx+sw;
      dispose(qs, done);
    end;
  end;

{  for a:=1 to 20 do begin
    triangle(-a/2,a,-w34*a, a/2,a,-w34*a, 0,a,w34*a);
  end;}




{  kugel(0,0,20,10,12,12);
  kugel(0,0,0,10,12,12);}

{ kugel(11,11,-10,10,20,20);
  kugel(11,-11,-10,10,20,20);
  kugel(-11,11,-10,10,20,20);
  kugel(-11,-11,-10,10,20,20);
  kugel(0,0,5,10,20,20);}




{  for a:=-xo to xo do
    for b:=-yo to yo do
      for c:=-zo to zo do
        tetraeder(a*xs,b*ys,c*zs-2,a*xs,b*ys+1,c*zs,a*xs+1,b*ys-1,c*zs,a*xs-1,b*ys-1,c*zs);

{        case random(3) of
    0:kugel(a*xs,b*ys,c*zs,(xs+ys+zs)*random/8,8,8);
    1:begin
          r1:=random;
          r2:=random;
          r3:=random;
        cube((a-0.4*r1)*xs,(b-0.4*r2)*ys,(c-0.4*r3)*zs,0.4*(random+r1)*xs,0,0, 0,0.4*(random+r2)*ys,0, 0,0,0.4*(random+r3)*zs);
      end;
    2:begin
          r1:=random;
          r2:=random;
          kegel(a*xs,b*ys,c*zs-r1,      xs*r2*0.4,0,0,    0,ys*r2*0.4,0, 0,0,zs*(random+r1)*0.4, 8)
      end
  end}
end;

procedure ende;
begin
  freemem(palleiste,imagesize(0,0,10,479));
  closegraph;
end;

procedure darstellung;
begin
  zaehl.init;
  neukamera;
  rechnung;

  outint(' Anzahl Polygone: ',zaehl.q[1].maximum);
  OutVector3d('Auge ',auge);
  OutVector3d('BlickR ',blickr);
  OutVector3d('Oben x 10000 ', jv.mul3d(10000));

  starttime;
  sweep;
  outtime;
  zaehl.ausgabe;

  OutInt('höchste Tiefe Suchbaum:',mtf);
  if wurzel[1]<>nil then outstring('wurzel[1]');
  if wurzel[2]<>nil then outstring('wurzel[2]');
  if wurzel[3]<>nil then outstring('wurzel[3]');
  if first[1]<>nil then outstring('first[1]');
  if first[2]<>nil then outstring('first[2]');
  if first[3]<>nil then outstring('first[3]');
  while first[1]<>nil do begin
    p:=pop(1);
    p^.drawpoly;
    dispose(p, done);
    outstring('f1');
  end;
  while first[2]<>nil do begin
    p:=pop(2);
    p^.drawpoly;
    dispose(p, done);
    outstring('f2');
  end;
  while first[3]<>nil do begin
    p:=pop(3);
    p^.drawpoly;
    dispose(p, done);
    outstring('f3');
  end;
end;

procedure tastatur;
begin
  repeat
    ch:=readkey2([#27,'A','B','Y','Z','K','L','S','X','D','C','F','T','P',',',';','<','>','.',':','O','I','0'..'9']);
    case ch of
      'a':auge := auge.move3d(BlickR,1);       'A':auge := auge.move3d(BlickR,10);
      'y','z':auge := auge.move3d(BlickR,-1);  'Y','Z':auge := auge.move3d(BlickR,-10);
      'k':auge := auge.move3d(iv,-1);          'K':auge := auge.move3d(iv,-10);
      'l':auge := auge.move3d(iv,1);           'L':auge := auge.move3d(iv,10);
      's':auge := auge.move3d(jv,1);           'S':auge := auge.move3d(jv,10);
      'x':auge := auge.move3d(jv,-1);          'X':auge := auge.move3d(jv,-10);
      'd':RotVec(@BlickR, @jv,1);              'D':RotVec(@BlickR, @jv, 10);
      'c':RotVec(@jv, @BlickR, 1);             'C':RotVec(@jv, @BlickR, 10);
      ',':RotVec(@iv, @BlickR, 1);             ';','<':RotVec(@iv, @BlickR, 10);
      '.':RotVec(@BlickR, @iv, 1);             ':','>':RotVec(@BlickR, @iv, 10);
      'o':RotVec(@jv, @iv, 1);                 'O':RotVec(@jv, @iv, 10);
      'i':RotVec(@iv, @jv, 1);                 'I':RotVec(@iv, @jv, 10);
      'f','F':colmode:=not colmode;
      't','T':tausgabe:=not tausgabe;
      'b','B':backface:=not backface;
      'p':begin
        palettePos := (palettePos + 1) MOD Length(palette);
        SetAllPalette(palette[palettePos]);
      end;
      'P':begin
        palettePos := (palettePos + Length(palette) - 1) MOD Length(palette);
        SetAllPalette(palette[palettePos]);
      end;
      '0':drawmode:=1;
      '1':drawmode:=1;
      '2':drawmode:=2;
      '3':drawmode:=3;
      '4':drawmode:=4;
      '5':drawmode:=5;
      '6':begin drawmode:=6;zumalen:=[0..255];tausgabe:=true;end;
      '7':drawmode:=7;
      '8':drawmode:=8;
      '9':drawmode:=9;
    end;
  until (not (Upcase(ch) in ['P']));
end;

begin
  init;
  repeat
    darstellung;
    tastatur;
  until ch=#27;
  ende;
end.
