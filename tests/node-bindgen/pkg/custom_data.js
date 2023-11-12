"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
const common_1 = require("./common");
const binding_1 = require("binding");
const tests = new common_1.Group("Custom Data Tests");
const struct = new binding_1.StructCustomData();
{
    const test = tests.test("getData");
    const result = struct.getData({ a: 1, b: 2, c: "test" });
    if (result instanceof Error) {
        test.fail(`Function getData() of StructCustomData returns error: ${result.message}`);
    }
    else {
        test.assert(result.a).msg("Value of result.a invalid").equal(2);
        test.assert(result.b).msg("Value of result.b invalid").equal(3);
        test.assert(result.c)
            .msg("Value of result.s invalid")
            .equal("testtest");
        test.success();
    }
}
{
    const test = tests.test("getDataA");
    const result = struct.getDataA({ a: 1, b: 2, c: "test" });
    if (result instanceof Error) {
        test.fail(`Function getDataA() of StructCustomData returns error: ${result.message}`);
    }
    else {
        test.assert(result.a.a).msg("Value of result.a.a invalid").equal(2);
        test.assert(result.a.b).msg("Value of result.a.b invalid").equal(3);
        test.assert(result.a.c)
            .msg("Value of result.a.c invalid")
            .equal("testtest");
        test.assert(result.b instanceof Array)
            .msg("Value of result.b invalid")
            .beTrue();
        test.assert(result.b.length).msg("Value of result.b invalid").equal(2);
        test.assert(result.b[0].a)
            .msg("Value of result.b[0].a invalid")
            .equal(2);
        test.assert(result.b[0].b)
            .msg("Value of result.b[0].b invalid")
            .equal(3);
        test.assert(result.b[0].c)
            .msg("Value of result.b[0].c invalid")
            .equal("testtest");
        test.assert(result.b[1].a)
            .msg("Value of result.b[1].a invalid")
            .equal(2);
        test.assert(result.b[1].b)
            .msg("Value of result.b[1].b invalid")
            .equal(3);
        test.assert(result.b[1].c)
            .msg("Value of result.b[1].c invalid")
            .equal("testtest");
        test.assert(result.c).msg("Value of result.c invalid").type("object");
        test.assert(result.c["first"])
            .msg("Value of result.c invalid")
            .typeNot("undefined");
        test.assert(result.c["first"].a)
            .msg("Value of result.b[1].a invalid")
            .equal(2);
        test.assert(result.c["first"].b)
            .msg("Value of result.b[1].b invalid")
            .equal(3);
        test.assert(result.c["first"].c)
            .msg("Value of result.b[1].c invalid")
            .equal("testtest");
        test.assert(result.c["second"])
            .msg("Value of result.c invalid")
            .typeNot("undefined");
        test.assert(result.c["second"].a)
            .msg("Value of result.b[1].a invalid")
            .equal(2);
        test.assert(result.c["second"].b)
            .msg("Value of result.b[1].b invalid")
            .equal(3);
        test.assert(result.c["second"].c)
            .msg("Value of result.b[1].c invalid")
            .equal("testtest");
        test.success();
    }
}
{
    const test = tests.test("testOfErrorSupportOk");
    const result = struct.testOfErrorSupportOk();
    test.assert(result).msg("Value of result invalid").type("number");
    test.assert(result).msg("Value of result invalid").equal(666);
    test.success();
}
{
    const test = tests.test("testOfErrorSupportErr");
    const result = struct.testOfErrorSupportErr();
    if (result instanceof Error) {
        test.assert(result.err)
            .msg("Value of result.err invalid")
            .type("object");
        test.assert(result.err?.code)
            .msg("Value of result.err?.code invalid")
            .equal(666);
        test.assert(result.err?.msg)
            .msg("Value of result.err?.msg invalid")
            .equal("test");
        test.assert(result.err?.err.code)
            .msg("Value of result.err?.err.code invalid")
            .equal(666);
        test.assert(result.err?.err.msg)
            .msg("Value of result.err?.err.code invalid")
            .equal("Error");
    }
    else {
        test.fail("Value of result invalid");
    }
    test.success();
}
//# sourceMappingURL=custom_data.js.map