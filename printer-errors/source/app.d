import std.stdio;

void main()
{
	writeln("Edit source/app.d to start your project.");
}

import std.algorithm;

export string printerError(const string s)
{
    return format("%s/%s", s.count!(c => c > 'm'), s.length);
}

import std.stdio;
import std.string : format;

version(unittest) import fluent.asserts;

void dotest(const string s, string exp)
{
    string actual = printerError(s);
    expect(actual).to.equal(exp).because("printerError(%s)".format(s));
}

@("fixed tests")
unittest {
    dotest("aaabbbbhaijjjm", "0/14");
    dotest("aaabbbbhaijjjmuvwx", "4/18");
    dotest("aaaxbbbbyyhwawiwjjjwwm", "8/22");
    dotest("uvwx", "4/4");

}

