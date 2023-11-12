import { Group } from "./common";
import { StructConstructorA, StructConstructorB } from "binding";

const tests = new Group("Constructors Tests");

{
    const test = tests.test("StructConstructorA");
    try {
        const _ = new StructConstructorA();
        test.success();
    } catch (e) {
        test.fail(`Fail to construct StructConstructorA`);
    }
}

{
    const test = tests.test("StructConstructorB");
    try {
        const _ = new StructConstructorB(666, 666);
        test.success();
    } catch (e) {
        test.fail(`Fail to construct StructConstructorB`);
    }
}
