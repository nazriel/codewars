function validateEAN(ean) {
    // iterate over chars in string

    let sum = ean.split("")
        .filter((_, i) => i < ean.length - 1)
        .map((char, i) => i % 2 === 0 ? +char : +char * 3)
        .reduce((a, b) => a + b);

    let checksum = 0;
    if (sum % 10 !== 0) {
        checksum = 10 - (sum % 10);
    }

    return checksum === +ean[ean.length - 1];
}

const chai = require("chai");
const assert = chai.assert;

describe("Tests", () => {
  it("test", () => {
    assert.isTrue(validateEAN("9783815820865"));
  });
});
