"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
const common_1 = require("./common");
const binding_1 = require("binding");
const tests = new common_1.Group("Error Handeling Tests");
const struct = new binding_1.StructErrorHandeling();
{
    const test = tests.test("testOfException");
    try {
        const _ = struct.testOfException();
        test.fail("function should throw exception");
    }
    catch (err) {
        test.assert(err).msg("Value of err invalid").type("string");
        test.success();
    }
}
{
    const test = tests.test("testOfExceptionSuppression");
    const err = struct.testOfExceptionSuppression();
    if (err instanceof Error) {
        test.assert(err.message).msg("Value of err invalid").equal("test");
    }
    else {
        test.fail("Value of err invalid");
    }
    test.success();
}
{
    const test = tests.test("testOfExceptionWithCustomError");
    try {
        const _ = struct.testOfExceptionWithCustomError();
        test.fail("function should throw exception");
    }
    catch (err) {
        if (err instanceof Error) {
            test.assert(err.err)
                .msg("Value of err invalid")
                .type("object");
        }
        else {
            test.fail("Value of err invalid");
        }
        test.success();
    }
}
{
    const test = tests.test("testOfExceptionSuppressionWithCustomError");
    try {
        const err = struct.testOfExceptionSuppressionWithCustomError();
        if (err instanceof Error) {
            test.assert(err.err).msg("Value of err invalid").type("object");
        }
        else {
            test.fail("Value of err invalid");
        }
        test.success();
    }
    catch (err) {
        test.fail("function should not throw exception");
    }
}
//# sourceMappingURL=error_handeling.js.map