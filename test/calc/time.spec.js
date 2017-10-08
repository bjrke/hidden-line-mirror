import {ctyp, dtyp} from '../../app/calc/time';

import assert from 'assert';

describe('time', () => {

    describe('dtyp', () => {
        it ('should work', () => {
            const d = new dtyp();
            d.ins();
            d.del();
            d.ausgabe();
        });
    });
  
});
