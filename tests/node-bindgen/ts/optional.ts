import { Group } from "./common";
import { ObjectWithOptions, ErrorWithOption, StructWithOptions } from "binding";

const tests = new Group("Options Tests");
const struct = new StructWithOptions();

{
    const test = tests.test("getErrWithOptionNone");
    const err = struct.getErrWithOptionNone();
    if (err instanceof Error) {
        test.assert(err.err).msg("Value of err invalid").type("object");
        test.assert(err.err?.msg).msg("Value of err.msg invalid").equal(null);
        test.success();
    } else {
        test.fail(
            `Function getErrWithOptionNone() of StructWithOptions should return Error`
        );
    }
}

{
    const test = tests.test("getErrWithOptionSome");
    const err = struct.getErrWithOptionSome();
    if (err instanceof Error) {
        test.assert(err.err).msg("Value of err invalid").type("object");
        test.assert(err.err?.msg).msg("Value of err.msg invalid").equal("test");
        test.success();
    } else {
        test.fail(
            `Function getErrWithOptionSome() of StructWithOptions should return Error`
        );
    }
}

{
    const test = tests.test("parsingOptions");
    const result = struct.parsingOptions({ a: 1, b: "test", c: [1, 2] });
    if (result instanceof Error) {
        test.fail(
            `Function parsingOptions() of StructWithOptions should return Error`
        );
    } else {
        test.assert(result).msg("Value of result invalid").equal(666);
        test.success();
    }
}

{
    const test = tests.test("parsingOptions (mixed options)");
    const result = struct.parsingOptions({
        a: null,
        b: "test",
        c: undefined,
    } as any);
    if (result instanceof Error) {
        test.fail(
            `Function parsingOptions() of StructWithOptions should return Error: ${result.err?.msg}`
        );
    } else {
        test.assert(result).msg("Value of result invalid").equal(666);
        test.success();
    }
}

{
    const test = tests.test("parsingOptions (using null)");
    const result = struct.parsingOptions({
        a: null,
        b: null,
        c: null,
    });
    if (result instanceof Error) {
        test.fail(
            `Function parsingOptions() of StructWithOptions should return Error: ${result.err?.msg}`
        );
    } else {
        test.assert(result).msg("Value of result invalid").equal(666);
        test.success();
    }
}

{
    const test = tests.test("parsingOptions (using undefined)");
    const result = struct.parsingOptions({
        a: undefined,
        b: undefined,
        c: undefined,
    } as any);
    if (result instanceof Error) {
        test.fail(
            `Function parsingOptions() of StructWithOptions should return Error: ${result.err?.msg}`
        );
    } else {
        test.assert(result).msg("Value of result invalid").equal(666);
        test.success();
    }
}
