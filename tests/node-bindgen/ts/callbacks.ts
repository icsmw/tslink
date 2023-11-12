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
