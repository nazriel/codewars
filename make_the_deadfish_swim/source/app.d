module solution;
import std.string : format;

void main()
{
}

import std.algorithm : each, filter;
import std.array : split;

export int[] parse(string data)
{
    int count = 0;
    int[] ret = [];

    data
        .each!((char c) {
            if (c == 'i')
                count++;
            else if (c == 'd')
                count--;
            else if (c == 'o')
                ret ~= count;
            else if (c == 's')
                count = count ^^ 2;
        });
    return ret;
}

version (unittest) import fluent.asserts;

void dotest(string data, int[] expected)
{
    parse(data)
        .should.equal(expected)
        .because("data = \"%s\"".format(data));
}

@("Basic tests")
unittest
{
    dotest("ooo", [0, 0, 0]);
    dotest("ioioio", [1, 2, 3]);
    dotest("idoiido", [0, 1]);
    dotest("isoisoiso", [1, 4, 25]);
    dotest("codewars", [0]);
}
