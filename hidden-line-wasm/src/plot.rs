use crate::appcontext::*;
use crate::float::*;
use crate::mat3::*;
use crate::point::*;



// var
//   backface: boolean;
//   palette: array [0..2] of palettetype;

// implementation


// procedure initGraphic;
// var
//   tr, md: integer;
// begin
//   tr := D8bit;
//   md := m1024x768;
//   initgraph(tr, md, '');
// end;

// const
//   colors = 16;

// var
//   p, i: integer;
// begin
//   initGraphic;
//   bmx := getmaxx div 2;
//   bmy := getmaxy div 2;

//   for p := 0 to Length(palette) - 1 do
//   begin
//     for i := 1 to (colors - 1) do
//     begin
//       setpalette(i, i + p * colors);
//     end;
//     getpalette(palette[p]);
//   end;

//   SetAllPalette(palette[0]);
//   for i := 0 to (colors - 1) do
//   begin
//     setcolor(i);
//     setfillstyle(1, i);
//     bar(0, i * 480 div colors, 10, ((i + 1) * 480 div colors) - 1);
//   end;
//   getmem(palleiste, imagesize(0, 0, 10, 479));
//   getimage(0, 0, 10, 479, palleiste^);
// end.
