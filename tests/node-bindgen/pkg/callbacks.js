"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
const common_1 = require("./common");
const binding_1 = require("binding");
const tests = new common_1.Group("Callbacks Tests");
const struct = new binding_1.StructCallback();
{
    const test = tests.test("testA");
    const timeout = setTimeout(() => {
        test.fail(`Function testA() didn't call callback`);
    }, 50);
    struct.testA((a, b) => {
        clearTimeout(timeout);
        test.assert(a).msg("Value of a invalid").equal(666);
        test.assert(b).msg("Value of b invalid").equal(666);
        test.success();
    });
}
{
    const test = tests.test("testB");
    const timeout = setTimeout(() => {
        test.fail(`Function testB() didn't call callback`);
    }, 50);
    struct.testB((a, b) => {
        clearTimeout(timeout);
        test.assert(a).msg("Value of a invalid").equal(666);
        test.assert(b).msg("Value of b invalid").equal(666);
        test.success();
    });
}
{
    const test = tests.test("testC");
    const timeout = setTimeout(() => {
        test.fail(`Function testC() didn't call callback`);
    }, 50);
    struct.testC((a, b) => {
        clearTimeout(timeout);
        test.assert(a).msg("Value of a invalid").equal(null);
        test.assert(b).msg("Value of b invalid").equal(666);
        test.success();
    });
}
{
    const test = tests.test("testD");
    const timeout = setTimeout(() => {
        test.fail(`Function testD() didn't call callback`);
    }, 50);
    struct.testD((a, b) => {
        clearTimeout(timeout);
        test.assert(a).msg("Value of a invalid").equal(666);
        test.assert(b).msg("Value of b invalid").equal(null);
        test.success();
    });
}
{
    const test = tests.test("testE");
    const timeout = setTimeout(() => {
        test.fail(`Function testE() didn't call callback`);
    }, 50);
    struct.testE((a, b) => {
        clearTimeout(timeout);
        test.assert(a).msg("Value of a invalid").equal(null);
        test.assert(b).msg("Value of b invalid").equal(null);
        test.success();
    });
}
//# sourceMappingURL=callbacks.js.map