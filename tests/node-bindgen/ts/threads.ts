import { Group } from "./common";
import { StructThreads } from "binding";

const tests = new Group("Threads Tests");
const struct = new StructThreads();

{
    const test = tests.test("rt");
    const timeout = setTimeout(() => {
        test.fail(`Function rt() didn't call callback`);
    }, 50);
    struct
        .rt((a: number) => {
            if (a !== 101 && a !== -1) {
                test.fail("Callback should return 101 or -1");
            } else if (a === 101) {
                struct.shutdown();
            } else if (a === -1) {
                test.success();
                clearTimeout(timeout);
            }
        })
        .then(() => {
            // Thread is created
            const test = tests.test("incValue");
            if (struct.incValue(100) instanceof Error) {
                test.fail("Fail to call incValue");
            } else {
                test.success();
            }
        })
        .catch(() => {
            test.fail("Fail to create thread");
            clearTimeout(timeout);
        });
}
