/*eslint-disable no-console */

export const x=0;
export const y=1;
export const z=2;
export const X=x;
export const Y=y;
export const Z=z;
export const epsilon1=0.000001;
export const epsilon2=epsilon1*epsilon1;
export const epsilon3=epsilon1*epsilon1*epsilon1;

export class vector3d {
    constructor(x = 0,y = 0,z = 0) {
        this[X] = x;
        this[Y] = y;
        this[Z] = z;
    }

    clone() {
        return new vector3d(this[x], this[y], this[z]);
    }
}

export class vector2d {
    constructor(x = 0,y = 0) {
        this[X] = x;
        this[Y] = y;
    }

    clone() {
        return new vector3d(this[x], this[y]);
    }
}

export class matrix2d {
    constructor(x = new vector2d(), y = new vector2d()) {
        this[X] = x.clone();
        this[Y] = y.clone();
    }

    clone() {
        return new matrix2d(this[x], this[y]);
    }
}

export class matrix3d {
    constructor(x = new vector3d(), y = new vector3d(), z = new vector3d()) {
        this[X] = x.clone();
        this[Y] = y.clone();
        this[Z] = z.clone();
    }

    clone() {
        return new matrix3d(this[x], this[y], this[z]);
    }
}

export function add3d(/* vector3d */ c,/* vector3d */ a,/* vector3d */ b) {
    c[x]=a[x]+b[x];
    c[y]=a[y]+b[y];
    c[z]=a[z]+b[z];
}

export function add2d(/* vector3d */ c,/* vector3d */ a,/* vector3d */ b) {
    c[x]=a[x]+b[x];
    c[y]=a[y]+b[y];
}

export function sub3d(/* vector3d */ c,/* vector3d */ a,/* vector3d */ b) {
    c[x]=a[x]-b[x];
    c[y]=a[y]-b[y];
    c[z]=a[z]-b[z];
}

export function sub2d(/*vector3d*/ c,/*vector3d*/ a,/*vector3d*/ b) {
    c[x]=a[x]-b[x];
    c[y]=a[y]-b[y];
}

export function det3d(/*matrix3d*/A) /*float*/ {
    return A[x][x]*A[y][y]*A[z][z]+A[y][x]*A[z][y]*A[x][z]+A[z][x]*A[x][y]*A[y][z]-
         A[x][x]*A[z][y]*A[y][z]-A[y][x]*A[x][y]*A[z][z]-A[z][x]*A[y][y]*A[x][z];
}

export function det2d(/*matrix2d*/A) /*float*/ {
    return A[x][x]*A[y][y]-A[x][y]*A[y][x];
}

export function neg3d(/*vector3d*/v) {
    v[x]=-v[x];
    v[y]=-v[y];
    v[z]=-v[z];
}

export function kreuz(/*vector3d*/c,/*vector3d*/a,/*vector3d*/b) {
    c[x]=a[y]*b[z]-a[z]*b[y];
    c[y]=a[z]*b[x]-a[x]*b[z];
    c[z]=a[x]*b[y]-a[y]*b[x];
}

export function skalar(/*vector3d*/a,/*vector3d*/b) /*float*/ {
    return a[x]*b[x]+a[y]*b[y]+a[z]*b[z];
}

export function betrag3d(/*vector3d*/v) /*float*/ {
    return Math.sqrt(v[x]*v[x]+v[y]*v[y]+v[z]*v[z]);
}

export function betrag2d(/*vector2d*/v) /*float*/ {
    return Math.sqrt(v[x]*v[x]+v[y]*v[y]);
}

export function mul3d(/*vector3d*/v,/*float*/f) {
    v[x]=v[x]*f;
    v[y]=v[y]*f;
    v[z]=v[z]*f;
}

export function div3d(/*vector3d*/v,/*float*/d) {
    if (d===0) {
        console.error('d=0');
    }
    v[x]=v[x]/d;
    v[y]=v[y]/d;
    v[z]=v[z]/d;
}

const MoveSpeed = 1;

export function MoveVec(/*vector3d*/ toMove, /*vector3d*/direction,/*float*/polarisation) {
    const DirLength=betrag3d(direction);
    if (DirLength===0) {
        console.error('DirLength=0');
    }
    toMove[X]=toMove[X]+polarisation*MoveSpeed*direction[X]/DirLength;
    toMove[Y]=toMove[Y]+polarisation*MoveSpeed*direction[Y]/DirLength;
    toMove[Z]=toMove[Z]+polarisation*MoveSpeed*direction[Z]/DirLength;
}

export function RotVec(/*vector3d*/ToRot1,/*vector3d*/ToRot2,/*float*/t) {
    const RotInc = Math.cos(t*Math.PI/180)/Math.sin(t*Math.PI/180);
    const RotVecLength=Math.sqrt(RotInc * RotInc + 1);

    const Length1=betrag3d(ToRot1);
    const Length2=betrag3d(ToRot2);

    const Copy1=ToRot1.clone();
    const Copy2=ToRot2.clone();

    if (RotVecLength==0) { console.error('RotVecLength=0'); }
    if (Length1==0) { console.error('Length1=0'); }
    if (Length2==0) { console.error('Length2=0'); }
  
    ToRot1[x]=(Length1/RotVecLength)*(RotInc*Copy1[X]/Length1+Copy2[X]/Length2);
    ToRot1[y]=(Length1/RotVecLength)*(RotInc*Copy1[Y]/Length1+Copy2[Y]/Length2);
    ToRot1[z]=(Length1/RotVecLength)*(RotInc*Copy1[Z]/Length1+Copy2[Z]/Length2);

    ToRot2[x]=(Length2/RotVecLength)*(-Copy1[X]/Length1+RotInc*Copy2[X]/Length2);
    ToRot2[y]=(Length2/RotVecLength)*(-Copy1[Y]/Length1+RotInc*Copy2[Y]/Length2);
    ToRot2[z]=(Length2/RotVecLength)*(-Copy1[Z]/Length1+RotInc*Copy2[Z]/Length2);
}

export function sgn(x /*float*/) /* int */ {
    if (x<-epsilon1) {
        return -1;
    } else if ( x>epsilon1 ) {
        return 1;
    } else {
        return 0;
    }
}

export function outstring(s /*string*/) {
    tausgabe && console.log(s);
}

export function outfloat(name /*string*/, f /*float*/) {
    tausgabe && console.log(name, f);
}

export function outint(name /*string*/, f /*int*/) {
    tausgabe && console.log(name, f);
}

export function outvector2d(name /*string*/, f /*vector2d*/) {
    tausgabe && console.log(name, f);
}

export function outvector3d(name /*string*/, f /*vector3d*/) {
    tausgabe && console.log(name, f);
}

export function marke() {
    //x,y,c/*int*/,s/*strin*/) {
    /*setcolor(c);
    line(x-4,y-4,x+4,y+4);
    line(x-4,y+4,x+4,y-4);
    outtextxy(x+5,y-4,s);*/
}

export function cls() {
    /*
      cleardevice;
  if tausgabe then putimage(0,0,palleiste^,normalput);
  th:=10;
  tx:=20;
  */
}

export let Auge = new vector3d();
export let BlickR = new vector3d();
export let iv = new vector3d();
export let jv = new vector3d();
export let tausgabe = true;

//bmx,bmy:float;
//palleiste:pointer;

/*
begin
  tx:=20;
  th:=10;
end.
*/
