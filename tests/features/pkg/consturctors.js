"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
const common_1 = require("./common");
const binding_1 = require("binding");
const tests = new common_1.Group("Constructors Tests");
{
    const test = tests.test("StructConstructorA");
    try {
        const _ = new binding_1.StructConstructorA();
        test.success();
    }
    catch (e) {
        test.fail(`Fail to construct StructConstructorA`);
    }
}
{
    const test = tests.test("StructConstructorB");
    try {
        const _ = new binding_1.StructConstructorB(666, 666);
        test.success();
    }
    catch (e) {
        test.fail(`Fail to construct StructConstructorB`);
    }
}
//# sourceMappingURL=consturctors.js.map