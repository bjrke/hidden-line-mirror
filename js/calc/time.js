import {outstring, outint} from './vector';

export class dtyp {
    constructor() {
        this.init();
    }

    init() {
        this.insert = 0;
        this.delete = 0;
        this.maximum = 0;
        this.minimum = 0;
        this.aktuell = 0;
    }
 
    ins() {
        this.insert++;
        this.aktuell++;
        if (this.aktuell > this.maximum ) {
            this.maximum = this.aktuell;
        }
    }

    del() {
        this.delete++;
        this.aktuell--;
        if (this.aktuell < this.minimum) {
            this.minimum = this.aktuell;
        }
    }

    ausgabe(name /*string*/) {
        outstring(name);
        if ((this.insert==this.delete)&&(this.insert != 0)){
            outint('  Einfügungen = Löschungen:',this.insert);
        } else {
            if (this.insert != 0) {
                outint('  Einfügungen:',this.insert);
            }
            if (this.delete != 0) {
                outint('  Löschungen:',this.delete); 
            }
        }
        if ((this.maximum != 0) && (this.maximum != this.aktuell)) {
            outint('  Höchststand:',this.maximum);
        }
        if ((this.minimum!=0) && (this.minimum != this.aktuell))  {
            outint('  Tiefststand:',this.minimum);
        }
        if (this.aktuell != 0) {
            if ((this.aktuell != this.insert)) {
                if ((this.minimum!=this.aktuell) && (this.maximum!=this.aktuell)) { 
                    outint('  aktueller Stand:',this.aktuell);}
                if ((this.aktuell==this.maximum)){
                    outint('  aktuell(Höchst)Stand:',this.aktuell);}
                if ((this.aktuell==this.minimum)) { 
                    outint('  aktuell(Tiefst)Stand:',this.aktuell); }
            } else {
                if ((this.minimum!=this.aktuell) && (this.maximum!=this.aktuell)) { 
                    outint('  aktueller Stand = Einfügungen:',this.aktuell); 
                }
                if ((this.aktuell==this.maximum)) { 
                    outint('  aktuell(Höchst)Stand = Einfügungen:',this.aktuell);
                }
                if ((this.aktuell==this.minimum)) { 
                    outint('  aktuell(Tiefst)Stand = Einfügungen:',this.aktuell);
                }
            }
        }
        outstring('--------------------------');
    }
}

export class ctyp {
    constructor() {
        this.init();
    }

    init() {
        this.q = [new dtyp(), new dtyp(), new dtyp()];
        this.s = new dtyp();
        this.p = new dtyp();
        this.p2 = new dtyp();
        this.p3 = new dtyp();
        this.ptest = 0;
        this.count = 0;
        this.pp = 0;
    }

    ausgabe() {
        outint('# insert aufrufe : ', this.count);
        outint('# Polytests: ', this.ptest);
        outint('# Triangulationen: ',this.pp);
        this.q[1].ausgabe('Warteschlange 1');
        this.q[2].ausgabe('Warteschlange 2');
        this.q[3].ausgabe('Warteschlange 3');
        this.s.ausgabe('Suchbaum');
        this.p.ausgabe('Polygone insgesamt');
        this.p2.ausgabe('Punkte in Bildeben');
        this.p3.ausgabe('Punkte im Raum');
    }
}

export function starttime() {
    time = Date.now();
}

export function outtime() {
    let s1='';
    const oldtime = time;
    starttime();
    const t=new Date(time-oldtime);
    const h=t.getHours();
    const m=t.getMinutes();
    const s=t.getSeconds();
    if (h>0) {
        s1 += h + 'h';
    }
    if (h>0 || m>0) {
        if (m<10) {
            s1 += '0';
        }
        s1 += m + 'm';
        if (s<10) {
            s1 += '0';
        }
    }
    s1 += s + 's';
    outstring('Zeit: '+s1);
}

export const zaehl = new ctyp();

let time = 0;
