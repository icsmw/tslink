import { Group } from "./common";
import { StructCallback } from "binding";

const tests = new Group("Callbacks Tests");
const struct = new StructCallback();

{
    const test = tests.test("testA");
    const timeout = setTimeout(() => {
        test.fail(`Function testA() didn't call callback`);
    }, 50);
    struct.testA((a: number, b: number) => {
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
    struct.testB((a: number | null, b: number | null) => {
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
    struct.testC((a: number | null, b: number | null) => {
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
    struct.testD((a: number | null, b: number | null) => {
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
    struct.testE((a: number | null, b: number | null) => {
        clearTimeout(timeout);
        test.assert(a).msg("Value of a invalid").equal(null);
        test.assert(b).msg("Value of b invalid").equal(null);
        test.success();
    });
}

{
    const test = tests.test("testF");
    const timeout = setTimeout(() => {
        test.fail(`Function testF() didn't call callback`);
    }, 50);
    const results = { a: false, b: false };
    const finish = () => {
        if (results.a && results.b) {
            test.success();
        }
    };
    struct.testF(
        (a: number) => {
            clearTimeout(timeout);
            test.assert(a).msg("Value of a invalid").equal(666);
            results.a = true;
            finish();
        },
        (b: string) => {
            test.assert(b).msg("Value of a invalid").equal("test");
            results.b = true;
            finish();
        }
    );
}
