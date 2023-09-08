function add(a, b) {
    let result = "";
    let current = 0;
    a = a.split("");
    b = b.split("");

    while (a.length > 0 || b.length > 0 || current !== 0) {
        let lhs = a.pop();
        let rhs = b.pop();

        if (lhs === undefined)
            lhs = 0;
        if (rhs === undefined)
            rhs = 0;
        lhs = Number(lhs);
        rhs = Number(rhs);

        current += (lhs + rhs);

        result = (current % 10) + result;
        current = current > 9 ? 1 : 0;
    }
    return result;
}

console.debug(add("101", "100") === "201");
console.debug(
    add("63829983432984289347293874", "90938498237058927340892374089") ==
    "91002328220491911630239667963"
);
