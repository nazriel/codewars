function solve(grid) {

}


const assert = require("chai").assert;

describe("Tests", () => {
    it("test", () => {
    assert.deepEqual(solve([[0, 0, 2, 0],
                               [0, 3, 0, 4],
                               [3, 0, 4, 0],
                               [0, 2, 0, 0]]),
                              [[1, 4, 2, 3],
                               [2, 3, 1, 4],
                               [3, 1, 4, 2],
                               [4, 2, 3, 1]]);
    });
  });
