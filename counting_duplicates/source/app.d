
import std.string : format;
import std.stdio : writefln, writeln;
import std.algorithm;
import std.conv;
import std.string;
import std.array;
import std.ascii;

export uint duplicateCount(string text)
{
    uint[char] m;
	text.strip().toLower().each!((char ch) {
		if (ch.isAlphaNum) {
			if (ch in m)
				m[ch]++;
			else
				m[ch] = 1;
		}
	});

	return m.byKeyValue().filter!(v => v.value > 1).count.to!uint;
}

void main() {
	writeln(duplicateCount("Ee9hkz21c2EKgO1yKZidaoHJWheW39gbSBffhWTtYuCAp"));
}

version(unittest) {
	import fluent.asserts;

	void dotest(string text, uint expected)
	{
	//    writefln("text : %s\nExpected: %d", text, expected);
		duplicateCount(text)
			.should.equal(expected)
			.because("text = \"%s\"".format(text));
	}

	@("Basic tests")
	unittest {
		dotest("abcde", 0);
		dotest("abcdea", 1);
		dotest("abcdeaB11", 3);
		dotest("indivisibility", 1);
		dotest("aA11", 2);
		dotest("ABBA", 2);
		dotest("Indivisibilities", 2);
		dotest("aA11", 2);

		dotest("zjcWVJl9qZkXJjAR7y61NE89G4k6l3wwU9BATrAMNDbulGez", 15);
	}

}
