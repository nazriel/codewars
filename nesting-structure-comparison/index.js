Array.prototype.sameStructureAs = function (other) {
    if (!Array.isArray(other)) {
        return false;
    }

    for (let i = 0; i < this.length; i++) {
        if (Array.isArray(this[i]) && !Array.isArray(other[i])) {
            return false;
        } else if (!Array.isArray(this[i]) && Array.isArray(other[i])) {
            return false;
        } else if (Array.isArray(this[i]) && Array.isArray(other[i])) {
            if (this[i].length !== other[i].length) {
                return false;
            }

            if (!this[i].sameStructureAs(other[i])) {
                return false;
            }
        }
    }
    return true;
};

const chai = require("chai");
const assert = chai.assert;

describe("Tests", () => {
  it("test", () => {
    assert.isTrue([1,1,1].sameStructureAs([2,2,2]), "[1,1,1] same as [2,2,2]");

    assert.isTrue([1,[1,1]].sameStructureAs([2,[2,2]]), "[1,[1,1]] same as [2,[2,2]]");
    assert.isNotTrue([1,[1,1]].sameStructureAs([[2,2],2]), "[1,[1,1]] not same as [[2,2],2]");
    assert.isNotTrue([1,[1,1]].sameStructureAs([2,[2]]), "[1,[1,1]] not same as [2,[2]]");

    assert.isTrue([[[],[]]].sameStructureAs([[[],[]]]), "[[[],[]]] same as [[[],[]]]");
    assert.isNotTrue([[[],[]]].sameStructureAs([[1,1]]), "[[[],[]]] not same as [[1,1]]");

    assert.isTrue([1,[[[1]]]].sameStructureAs([2,[[[2]]]]), "[1,[[[1]]]] same as [2,[[[2]]]]");

    assert.isNotTrue([].sameStructureAs(1), "[] not same as 1");
    assert.isNotTrue([].sameStructureAs({}), "[] not same as {}");

    assert.isTrue([1,'[',']'].sameStructureAs(['[',']',1]), "[1,'[',']'] same as ['[',']',1]");

    assert.isNotTrue( [1,2].sameStructureAs([[3],3]), "[1,2] not same as [[3],3]" );
  });
});
