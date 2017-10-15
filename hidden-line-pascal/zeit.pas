unit zeit;
interface
uses crt, dos, vector;
type
  dtyp=object
    insert,delete,maximum,minimum,aktuell:longint;
    constructor init;
    procedure ins;
    procedure del;
    procedure ausgabe(name:string);
  end;

  ctyp=object
    q:array[1..3]of dtyp;
    s,p,p2,p3:dtyp;
    ptest,count,pp:longint;
    constructor init;
    procedure ausgabe;
  end;

function gettime2: LongInt;
procedure starttime;
procedure outtime;

type cset = set of char;
function readkey2( include: cset): char;

var
  zaehl:ctyp;
implementation
var time:longint;

constructor dtyp.init;
begin
  insert:=0;
  delete:=0;
  maximum:=0;
  minimum:=0;
  aktuell:=0;
end;

procedure dtyp.ins;
begin
  inc(insert);
  inc(aktuell);
  if aktuell>maximum then maximum:=aktuell;
end;

procedure dtyp.del;
begin
  inc(delete);
  dec(aktuell);
  if aktuell<minimum then minimum:=aktuell;
end;

procedure dtyp.ausgabe;
begin
  outstring(name);
  if (insert=delete)and(insert<>0)then
    outint('  Einfügungen = Löschungen:',insert)
  else begin
    if insert<>0 then outint('  Einfügungen:',insert);
    if delete<>0 then outint('  Löschungen:',delete);
  end;
  if (maximum<>0) and (maximum<>aktuell) then outint('  Höchststand:',maximum);
  if (minimum<>0) and (minimum<>aktuell) then outint('  Tiefststand:',minimum);
  if (aktuell<>insert)then begin
    if (aktuell<>0) and (minimum<>aktuell) and (maximum<>aktuell) then outint('  aktueller Stand:',aktuell);
    if (aktuell=maximum)and (aktuell<>0)then outint('  aktuell(Höchst)Stand:',aktuell);
    if (aktuell=minimum)and (aktuell<>0)then outint('  aktuell(Tiefst)Stand:',aktuell);
  end else begin
    if (aktuell<>0) and (minimum<>aktuell) and (maximum<>aktuell) then outint('  aktueller Stand = Einfügungen:',aktuell);
    if (aktuell=maximum)and (aktuell<>0)then outint('  aktuell(Höchst)Stand = Einfügungen:',aktuell);
    if (aktuell=minimum)and (aktuell<>0)then outint('  aktuell(Tiefst)Stand = Einfügungen:',aktuell);
  end;
  outstring('--------------------------');
end;

constructor ctyp.init;
begin
  q[1].init;
  q[2].init;
  q[3].init;
  s.init;
  p.init;
  p2.init;
  count:=0;
  ptest:=0;
  pp:=0;
end;

procedure ctyp.ausgabe;
begin
  outint('# insert aufrufe : ',count);
  outint('# Polytests: ',ptest);
  outint('# Triangulationen: ',pp);
  q[1].ausgabe('Warteschlange 1');
  q[2].ausgabe('Warteschlange 2');
  q[3].ausgabe('Warteschlange 3');
  s.ausgabe('Suchbaum');
  p.ausgabe('Polygone insgesamt');
  p2.ausgabe('Punkte in Bildeben');
  p3.ausgabe('Punkte im Raum');
end;

procedure starttime;
begin
  time := gettime2;
end;

function gettime2;
var
  h,m,s,s100:word;
begin
  gettime(h,m,s,s100);
  gettime2:=360000*h+6000*m+s*100+s100;
end;

procedure outtime;
var
  t:longint;
  h,m:word;
  s:float;
  s1,s2:string;
begin
  s1:='';
  t:=time;
  starttime;
  t:=time-t;
  h:=t div 360000;
  m:=(t-h*360000) div 6000;
  s:=(t-h*360000-m*6000)/100;
  if h>0 then begin
    str(h,s1);
    s1:=s1+'h';
  end;
  if (h>0) or (m>0) then begin
    str(m,s2);
    if m=0 then s1:=s1+'0';
    if m<10 then s1:=s1+'0';
    s1:=s1+s2+'m';
    if s=0 then s1:=s1+'0';
    if s<10 then s1:=s1+'0';
  end;
  str(s:0:2,s2);
  s1:=s1+s2+'s';
  outstring('Zeit: '+s1);
end;

function readkey2;
var
  start: LongInt;
  ch, res: Char;
begin
  repeat
    res := ReadKey;
    if ( res = #0 ) then
      ReadKey
  until ( Upcase(res) in include);

  if KeyPressed then
  begin
    start := gettime2;
    repeat
      ch := ReadKey;
      if ( ch = #0 ) then
        ReadKey
    until ((not KeyPressed) or (ch <> res) or ((gettime2 - start) > 500))
  end;

  if ( Upcase(ch) in include ) then
    exit(ch)
  else
    exit(res);
end;

begin
  zaehl.p3.init;
end.
