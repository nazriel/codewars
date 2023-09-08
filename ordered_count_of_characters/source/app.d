import std.stdio;

import std.typecons;
import std.string : format;
import std.algorithm;
import std.array;

void main()
{
    writeln("Edit source/app.d to start your project.");
}

export Tuple!(char, uint)[] orderedCount(string text)
{
    Tuple!(uint, uint)[char] cnt;
    uint pos = 0;
    text.each!((char c) {
        cnt.update(c,
            () => tuple(1U, pos++),
            (ref Tuple!(uint, uint) t) {
                t[0]++;
                pos++;
            }
        );
    });
    return cnt.byPair().array().sort!((a, b) => a.value[1] < b.value[1])
        .map!(e => tuple(e.key, e.value[0]))
        .array();
}

version (unittest) import fluent.asserts;

void dotest(string text, Tuple!(char, uint)[] expected)
{
    orderedCount(text).should.equal(expected).because("text = \"%s\"".format(text));
}

@("Basic tests")
unittest
{
    dotest("abracadabra", [
        tuple('a', 5u), tuple('b', 2u), tuple('r', 2u), tuple('c', 1u),
        tuple('d', 1u)
    ]);
    dotest("banana", [tuple('b', 1u), tuple('a', 3u), tuple('n', 2u)]);
    dotest("i am a master kata solver",
        [
            tuple('i', 1u),
            tuple(' ', 5u),
            tuple('a', 5u),
            tuple('m', 2u),
            tuple('s', 2u),
            tuple('t', 2u),
            tuple('e', 2u),
            tuple('r', 2u),
            tuple('k', 1u),
            tuple('o', 1u),
            tuple('l', 1u),
            tuple('v', 1u),
        ]);
}
