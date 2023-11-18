"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
const common_1 = require("./common");
const binding_1 = require("binding");
const tests = new common_1.Group("Primitive Types Tests");
{
    const test = tests.test("typesA");
    const result = (0, binding_1.typesA)(1, 2);
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
    const result = (0, binding_1.typesB)(1);
    test.assert(result).msg("Value of result invalid").type("number");
    test.assert(result).msg("Value of result invalid").equal(1);
    test.success();
}
{
    const test = tests.test("typesB: none");
    const result = binding_1.typesB();
    test.assert(result).msg("Value of result invalid").equal(null);
    test.success();
}
{
    const test = tests.test("typesC: none");
    const result = (0, binding_1.typesC)(null, null);
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
    const result = (0, binding_1.typesC)(null, 1);
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
    const result = (0, binding_1.typesC)(1, 1);
    test.assert(result instanceof Array)
        .msg("Value of result invalid")
        .beTrue();
    test.assert(result.length).msg("Value of result invalid").equal(2);
    test.assert(result[0]).msg("Value of result invalid").equal(1);
    test.assert(result[1]).msg("Value of result invalid").equal(1);
    test.success();
}
//# sourceMappingURL=types.js.map