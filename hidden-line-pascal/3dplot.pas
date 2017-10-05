program dreidplot;
uses crt,graph,vector,dreidext,projekt,dreiecke,polyswee,polygon,punkte,zeit;
var
  bmx,bmy,m:int;
  ch:char;
  h:vector3d;
  p:ppoly;

procedure init;
var a,b,c:int;
    h:^triStrip;
    tr,md:integer;
    qs:^quadstrip;
    xx,yy:float;

function fkt(x,y:float):float;
var
  h:float;
begin
  h:=sqrt(sqr(x)+sqr(y));
{  fkt:=30*cos(h)/(2+h)}
{  fkt:=random;}
  fkt:=sin(y)*x/10;
end;

const
  xs=4;      ys=4;      zs=4;
  xo=3;      yo=3;      zo=3;
  sw=0.5;
  ad=13;
  w34=0.43301270189221932338186158537647;

var
  r1,r2,r3:float;

begin
  delay(1000);

  tausgabe:=false;

  auge[x]:=3;  auge[y]:=-15;  auge[z]:=3;
  blickR[x]:=-auge[x]/10;  blickR[y]:=-auge[y]/10;  blickR[z]:=-auge[z]/10;
{  blickR[x]:=-3;  blickR[y]:=-6;  blickR[z]:=-12;}
  jv[x]:=0;  jv[y]:=0;  jv[z]:=1;

  setallpalette(bpal);
  backface:=true;

  drawmode:=1;

  tetraeder(0,0,0, -1,0,-2, 1,1,-2 ,1,-1,-2);

  cube(-4,2,-2, 3,0,0, 0,3,0, 0,0,3);

  kegel(1,3,-2, 1,0,0, 0,1,0, 0,0,2, 8);

  kugel(0,8,0, 3, 32, 16);

{  kegel(0,0,0, 2,0,0, 0,2,0, 0,0,-4, 10);  }

{  cube (0,0,0, 1,0,0, 0,1,0, 0,0,1);}

{  begin
    xx:=-ad;
    while xx<ad do begin
      new(qs,init(xx,-ad,fkt(xx,-ad),xx+sw,-ad,fkt(xx+sw,-ad),xx,-ad+sw,fkt(xx,-ad+sw),xx+sw,-ad+sw,fkt(xx+sw,-ad+sw)));
      yy:=-ad+2*sw;
      while yy<ad do begin
        qs^.add(xx,yy,fkt(xx,yy),xx+sw,yy,fkt(xx+sw,yy));
        yy:=yy+sw;
      end;
      xx:=xx+sw;
      dispose(qs);
    end;
  end;}

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

begin
  init;
  m:=memavail;
  repeat
    zaehl.init;
    neukamera;
    rechnung;
    outint(' Anzahl Polygone: ',zaehl.q[1].maximum);
    OutVector3d('Auge ',auge);
    OutVector3d('BlickR ',blickr);

    h:=jv;
    mul3d(h,10000);
    OutVector3d('Oben x 10000 ',h);
    maxtiefe:=-1e20;
    mintiefe:=1e20;
    starttime;
    sweep;
    outtime;
    zaehl.ausgabe;
    outint('memdiff ',m-memavail);
    m:=memavail;

    OutInt('höchste Tiefe Suchbaum:',mtf);
    if wurzel[1]<>nil then outstring('wurzel[1]');
    if wurzel[2]<>nil then outstring('wurzel[2]');
    if wurzel[3]<>nil then outstring('wurzel[3]');
    if first[1]<>nil then outstring('first[1]');
    if first[2]<>nil then outstring('first[2]');
    if first[3]<>nil then outstring('first[3]');
    while first[1]<>nil do begin
      p:=pop(1);
      p^.draw;
      killppoly(p,'a');
      outstring('f1');
    end;
    while first[2]<>nil do begin
      p:=pop(2);
      p^.draw;
      killppoly(p,'a');
      outstring('f2');
    end;
    while first[3]<>nil do begin
      p:=pop(3);
      p^.draw;
      killppoly(p,'a');
      outstring('f3');
    end;
    repeat
      ch:=readkey;
    until (upcase(ch) in [#27,'A','Y','K','L','S','X','D','C','F','T',',',';','.',':','O','I','0'..'9']);

    case ch of
      'a':MoveVec(Auge,BlickR,1);       'A':MoveVec(Auge,BlickR,10);
      'y':MoveVec(Auge,BlickR,-1);      'Y':MoveVec(Auge,BlickR,-10);
      'k':MoveVec(Auge,iv,-1);          'K':MoveVec(Auge,iv,-10);
      'l':MoveVec(Auge,iv,1);           'L':MoveVec(Auge,iv,10);
      's':MoveVec(Auge,jv,1);           'S':MoveVec(Auge,jv,10);
      'x':MoveVec(Auge,jv,-1);          'X':MoveVec(Auge,jv,-10);
      'd':RotVec(BlickR,jv,1);          'D':RotVec(BlickR,jv,10);
      'c':RotVec(jv,BlickR,1);          'C':RotVec(jv,BlickR,10);
      ',':RotVec(iv,BlickR,1);          ';':RotVec(iv,BlickR,10);
      '.':RotVec(BlickR,iv,1);          ':':RotVec(BlickR,iv,10);
      'o':RotVec(jv,iv,1);              'O':RotVec(jv,iv,10);
      'i':RotVec(iv,jv,1);              'I':RotVec(iv,jv,10);
      'f','F':colmode:=not colmode;
      't','T':tausgabe:=not tausgabe;
      '0':drawmode:=1;
      '1':begin drawmode:=1;setallpalette(spal);end;
      '2':begin drawmode:=2;setallpalette(spal);end;
      '3':begin drawmode:=3;setallpalette(spal);end;
      '4':begin drawmode:=4;setallpalette(bpal);end;
      '5':begin drawmode:=5;setallpalette(bpal);end;
      '6':begin zumalen:=[0..255];drawmode:=6;setallpalette(spal);tausgabe:=true;end;
      '7':begin drawmode:=7;setallpalette(bpal);end;
      '8':begin drawmode:=8;setallpalette(spal);end;
      '9':drawmode:=1;
    end;
  until ch=#27;
  ende;
end.