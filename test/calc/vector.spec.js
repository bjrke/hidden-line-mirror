import assert from 'assert';

import {x, y, z, vector3d, vector2d, matrix3d, matrix2d, add2d, add3d, RotVec, epsilon1} from '../../app/calc/vector';

function assertEpsilonEquals(a,b) {
    if ( Math.abs(a-b) > epsilon1) {
        assert.equal(a,b);
    }
}


describe('vector', () => {

    describe('vector3d', () => {
        describe('constructor', () => {
            it('should accept parameters', () => {
                const v = new vector3d(1,2,3);
                assert.equal(v[x],1);
                assert.equal(v[y],2);
                assert.equal(v[z],3);
            });

            it('should work without parameters', () => {
                const v = new vector3d();
                assert.equal(v[x],0);
                assert.equal(v[y],0);
                assert.equal(v[z],0);
            });
        });

        describe('clone', () => {
            it('should copy', () => {
                const v = new vector3d(1,2,3);
                const v2 = v.clone();
                assert.equal(v2[x],1);
                assert.equal(v2[y],2);
                assert.equal(v2[z],3);
            });
        });
    });

    describe('vector2d', () => {
        describe('constructor', () => {
            it('should accept parameters', () => {
                const v = new vector2d(1,2);
                assert.equal(v[x],1);
                assert.equal(v[y],2);
            });

            it('should work without parameters', () => {
                const v = new vector2d();
                assert.equal(v[x],0);
                assert.equal(v[y],0);
            });
        });
    });

    describe('matrix3d', () => {
        describe('constructor', () => {
            it('should accept parameters', () => {
                const m = new matrix3d(new vector3d(1,2,3), new vector3d(4,5,6), new vector3d(7,8,9));
                assert.equal(m[x][x], 1);
                assert.equal(m[x][y], 2);
                assert.equal(m[x][z], 3);
                assert.equal(m[y][x], 4);
                assert.equal(m[y][y], 5);
                assert.equal(m[y][z], 6);
                assert.equal(m[z][x], 7);
                assert.equal(m[z][y], 8);
                assert.equal(m[z][z], 9);
            });

            it('should work without parameters', () => {
                const m = new matrix3d();
                assert.equal(m[x][x], 0);
                assert.equal(m[x][y], 0);
                assert.equal(m[x][z], 0);
                assert.equal(m[y][x], 0);
                assert.equal(m[y][y], 0);
                assert.equal(m[y][z], 0);
                assert.equal(m[z][x], 0);
                assert.equal(m[z][y], 0);
                assert.equal(m[z][z], 0);
            });
        });
    });

    describe('matrix2d', () => {
        describe('constructor', () => {
            it('should accept parameters', () => {
                const m = new matrix2d(new vector2d(1,2), new vector2d(3,4));
                assert.equal(m[x][x], 1);
                assert.equal(m[x][y], 2);
                assert.equal(m[y][x], 3);
                assert.equal(m[y][y], 4);
            });

            it('should work without parameters', () => {
                const m = new matrix2d();
                assert.equal(m[x][x], 0);
                assert.equal(m[x][y], 0);
                assert.equal(m[y][x], 0);
                assert.equal(m[y][y], 0);
            });
        });
    });

    describe('add3d', () => {
        it('should work', () => {
            const v = new vector3d();
            add3d(v, new vector3d(1,2,3), new vector3d(10,20,30));
            assert.equal(v[x],11);
            assert.equal(v[y],22);
            assert.equal(v[z],33);
        });
    });

    describe('add2d', () => {
        it('should work', () => {
            const v = new vector2d();
            add2d(v, new vector2d(1,2), new vector2d(10,20));
            assert.equal(v[x],11);
            assert.equal(v[y],22);
        });
    });

    describe('RotVec', () => {
        it('should work', () => {
            const v1 = new vector3d(1,0,0);
            const v2 = new vector3d(0,1,0);
            RotVec(v1,v2, 90);

            assertEpsilonEquals(v1[x],0);
            assertEpsilonEquals(v1[y],1);
            assertEpsilonEquals(v1[z],0); 

            assertEpsilonEquals(v2[x],-1);
            assertEpsilonEquals(v2[y],0);
            assertEpsilonEquals(v2[z],0); 
        });
    });

});
