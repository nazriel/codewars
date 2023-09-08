import std.stdio;
import std.string;
import std.algorithm;
import std.array;

void main()
{
    writeln("Edit source/app.d to start your project.");
}

export string[] inArray(const string[] arr1, const string[] arr2)
{
    return arr1.dup()
        .filter!(
            w1 => arr2.canFind!(
                w2 => w2.canFind(w1)))
        .array()
        .sort().uniq().array();
}

version (unittest) import fluent.asserts;

void testing(const string[] arr1, const string[] arr2, string[] exp)
{
    string[] actual = inArray(arr1, arr2);
    expect(actual).to.equal(exp).because("inArray(%s, %s)".format(arr1, arr2));
}

@("fixed tests")
unittest
{
    testing(["live", "arp", "strong", "der", "ong", "arp", "arp", "live"],
    ["lively", "alive", "harp", "sharp", "armstrong"], [
        "arp", "live", "ong", "strong"
    ]);
    testing(["arp", "mice", "bull"], [
        "lively", "alive", "harp", "sharp", "armstrong"
    ], ["arp"]);
    testing(["cod", "code", "wars", "ewar"], [
        "lively", "alive", "harp", "sharp", "armstrong", "codewars"
    ], ["cod", "code", "ewar", "wars"]);
    testing(["cod", "code", "wars", "ewar", "ar"], [], []);

}
