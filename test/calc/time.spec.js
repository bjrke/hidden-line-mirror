/* eslint-env mocha */

import {dtyp} from '../../js/calc/time';

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
