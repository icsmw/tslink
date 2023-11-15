"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.assert = exports.Test = exports.Group = void 0;
class Group {
    group;
    constructor(group) {
        this.group = group;
        console.log(`Starting tests for: ${group}`);
    }
    test(name) {
        return new Test(name);
    }
}
exports.Group = Group;
class Test {
    name;
    started = Date.now();
    constructor(name) {
        this.name = name;
    }
    fail(msg) {
        fail(`[FAIL] ${this.name}: ${msg}`);
    }
    success() {
        console.log(`[OK in ${Date.now() - this.started}ms] ${this.name}`);
    }
    assert(smth) {
        return assert(smth, this);
    }
}
exports.Test = Test;
function fail(msg) {
    console.error(msg);
    try {
        throw new Error("Stack");
    }
    catch (e) {
        console.error(e.stack);
    }
    process.exit(1);
}
function assert(smth, test) {
    let errorMessage;
    const failCB = test !== undefined ? test.fail.bind(test) : fail;
    const out = {
        equal: (value) => {
            if (value !== smth) {
                if (errorMessage === undefined) {
                    failCB(`No error message. Value: ${smth} (type: ${typeof smth}) not equal to: ${value} (type: ${typeof value})`);
                }
                else {
                    failCB(`${errorMessage}. Value: ${smth} (type: ${typeof smth}) not equal to: ${value} (type: ${typeof value})`);
                }
            }
            return out;
        },
        beTrue: () => {
            if (smth !== true) {
                if (errorMessage === undefined) {
                    failCB(`No error message. Condition isn't true`);
                }
                else {
                    failCB(`${errorMessage}. Condition isn't true`);
                }
            }
            return out;
        },
        type: (typeName) => {
            if (typeof smth !== typeName) {
                if (errorMessage === undefined) {
                    failCB(`No error message. Value: ${smth} (type: ${typeof smth}) has different type to: ${typeName})`);
                }
                else {
                    failCB(`${errorMessage}. Value: ${smth} (type: ${typeof smth}) has different type to: ${typeName})`);
                }
            }
            return out;
        },
        typeNot: (typeName) => {
            if (typeof smth === typeName) {
                if (errorMessage === undefined) {
                    failCB(`No error message. Value: ${smth} (type: ${typeof smth}) has prohibited type: ${typeName})`);
                }
                else {
                    failCB(`${errorMessage}. Value: ${smth} (type: ${typeof smth}) has prohibited type: ${typeName})`);
                }
            }
            return out;
        },
        msg: (msg) => {
            errorMessage = msg;
            return out;
        },
    };
    return out;
}
exports.assert = assert;
//# sourceMappingURL=common.js.map