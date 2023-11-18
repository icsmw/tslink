"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
const common_1 = require("./common");
const binding_1 = require("binding");
const tests = new common_1.Group("Async Methods Tests");
const struct = new binding_1.StructAsyncMethods();
{
    const test = tests.test("getData");
    const timeout = setTimeout(() => {
        test.fail(`Function getData() didn't call callback`);
    }, 50);
    let res = struct.getData({ a: 1, b: 2, c: "test" });
    struct
        .getData({ a: 1, b: 2, c: "test" })
        .then((result) => {
        test.assert(result.a).msg("Value of result.a invalid").equal(2);
        test.assert(result.b).msg("Value of result.b invalid").equal(3);
        test.assert(result.c)
            .msg("Value of result.s invalid")
            .equal("testtest");
        test.success();
    })
        .catch((err) => {
        test.fail(`Function getData() of StructAsyncMethods returns error: ${err.message}`);
    })
        .finally(() => {
        clearTimeout(timeout);
    });
}
{
    const test = tests.test("getDataA");
    const timeout = setTimeout(() => {
        test.fail(`Function getData() didn't call callback`);
    }, 50);
    struct
        .getDataA({ a: 1, b: 2, c: "test" })
        .then((result) => {
        test.assert(result.a.a).msg("Value of result.a.a invalid").equal(2);
        test.assert(result.a.b).msg("Value of result.a.b invalid").equal(3);
        test.assert(result.a.c)
            .msg("Value of result.a.c invalid")
            .equal("testtest");
        test.assert(result.b instanceof Array)
            .msg("Value of result.b invalid")
            .beTrue();
        test.assert(result.b.length)
            .msg("Value of result.b invalid")
            .equal(2);
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
        test.assert(result.c)
            .msg("Value of result.c invalid")
            .type("object");
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
        test.assert(result.d)
            .msg("Value of result.d invalid")
            .equal("test");
        test.success();
    })
        .catch((err) => {
        test.fail(`Function getDataA() of StructAsyncMethods returns error: ${err.message}`);
    })
        .finally(() => {
        clearTimeout(timeout);
    });
}
{
    const test = tests.test("testOfErrorSupportOk");
    const timeout = setTimeout(() => {
        test.fail(`Function getData() didn't call callback`);
    }, 50);
    struct
        .testOfErrorSupportOk()
        .then((result) => {
        test.assert(result).msg("Value of result invalid").type("number");
        test.assert(result).msg("Value of result invalid").equal(666);
        test.success();
    })
        .catch((err) => {
        test.fail(`Function testOfErrorSupportOk() of StructAsyncMethods returns error: ${err.message}`);
    })
        .finally(() => {
        clearTimeout(timeout);
    });
}
{
    const test = tests.test("testOfErrorSupportErr");
    const timeout = setTimeout(() => {
        test.fail(`Function getData() didn't call callback`);
    }, 50);
    struct
        .testOfErrorSupportErr()
        .then((_) => {
        test.fail("Value of result invalid");
    })
        .catch((err) => {
        if (err instanceof Error) {
            test.assert(err.err)
                .msg("Value of result.err invalid")
                .type("object");
            test.assert(err.err?.code)
                .msg("Value of result.err?.code invalid")
                .equal(666);
            test.assert(err.err?.msg)
                .msg("Value of result.err?.msg invalid")
                .equal("test");
            test.success();
        }
        else {
            test.fail("Value of result invalid");
        }
    })
        .finally(() => {
        clearTimeout(timeout);
    });
}
//# sourceMappingURL=async_methods.js.map