import std.stdio;

void main()
{
	writeln("Edit source/app.d to start your project.");
}

string solution(string input)
{
	import std.conv : to;
	import std.range : retro;

	return input.retro().to!string;
}

version (unittest) import fluent.asserts;

void dotest(string str, string expected)
{
	import std.format : format;

	solution(str)
		.should.equal(expected)
		.because("str = \"%s\"".format(str));
}

@("Basic tests")
unittest
{
	dotest("world", "dlrow");
	dotest("hello", "olleh");
	dotest("", "");
	dotest("h", "h");
}
