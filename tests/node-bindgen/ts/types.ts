import { Group } from "./common";
import { typesA, typesB, typesC, typesD } from "binding";

const tests = new Group("Primitive Types Tests");

{
    const test = tests.test("typesA");
    const result = typesA(1, 2);
    test.assert(result instanceof Array)
        .msg("Value of result invalid")
        .beTrue();
    test.assert(result.length).msg("Value of result invalid").equal(2);
    test.assert(result[0]).msg("Value of result invalid").equal(1);
    test.assert(result[1]).msg("Value of result invalid").equal(2);
    test.success();
}

{
    const test = tests.test("typesB: some");
    const result = typesB(1);
    test.assert(result).msg("Value of result invalid").type("number");
    test.assert(result).msg("Value of result invalid").equal(1);
    test.success();
}

{
    const test = tests.test("typesB: none");
    const result = (typesB as any)();
    test.assert(result).msg("Value of result invalid").equal(null);
    test.success();
}

{
    const test = tests.test("typesC: none");
    const result = typesC(null, null);
    test.assert(result instanceof Array)
        .msg("Value of result invalid")
        .beTrue();
    test.assert(result.length).msg("Value of result invalid").equal(2);
    test.assert(result[0]).msg("Value of result invalid").equal(null);
    test.assert(result[1]).msg("Value of result invalid").equal(null);
    test.success();
}

{
    const test = tests.test("typesC: none & value");
    const result = typesC(null, 1);
    test.assert(result instanceof Array)
        .msg("Value of result invalid")
        .beTrue();
    test.assert(result.length).msg("Value of result invalid").equal(2);
    test.assert(result[0]).msg("Value of result invalid").equal(null);
    test.assert(result[1]).msg("Value of result invalid").equal(1);
    test.success();
}

{
    const test = tests.test("typesC: value & value");
    const result = typesC(1, 1);
    test.assert(result instanceof Array)
        .msg("Value of result invalid")
        .beTrue();
    test.assert(result.length).msg("Value of result invalid").equal(2);
    test.assert(result[0]).msg("Value of result invalid").equal(1);
    test.assert(result[1]).msg("Value of result invalid").equal(1);
    test.success();
}

{
    const test = tests.test("typesD: value & value");
    const result = typesD([1, 2, 3]);
    test.assert(result instanceof Array)
        .msg("Value of result invalid")
        .beTrue();
    test.assert(result.length).msg("Value of result invalid").equal(3);
    test.assert(result[0]).msg("Value of result invalid").equal(1);
    test.assert(result[1]).msg("Value of result invalid").equal(2);
    test.assert(result[2]).msg("Value of result invalid").equal(3);
    test.success();
}
