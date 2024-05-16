var BUGNUMBER = 1391519;
var summary = "for-await-of outside of async function should provide better error";

print(BUGNUMBER + ": " + summary);

assertThrownErrorContains(
    () => eval("for await (let x of []) {}"),
    "for await (... of ...) is only valid in"
);

// Extra `await` shouldn't throw that error.
assertThrownErrorContains(
    () => eval("async function f() { for await await (let x of []) {} }"),
    "missing ( after for"
);

if (typeof reportCompare === "function")
    reportCompare(true, true);
