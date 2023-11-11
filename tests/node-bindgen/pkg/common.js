"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.assert = void 0;
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
function assert(smth) {
    let errorMessage;
    const out = {
        equal: (value) => {
            if (value !== smth) {
                if (errorMessage === undefined) {
                    fail(`No error message. Value: ${smth} (type: ${typeof smth}) not equal to: ${value} (type: ${typeof value})`);
                }
                else {
                    fail(`${errorMessage}. Value: ${smth} (type: ${typeof smth}) not equal to: ${value} (type: ${typeof value})`);
                }
            }
            return out;
        },
        beTrue: () => {
            if (smth !== true) {
                if (errorMessage === undefined) {
                    fail(`No error message. Condition isn't true`);
                }
                else {
                    fail(`${errorMessage}. Condition isn't true`);
                }
            }
            return out;
        },
        type: (typeName) => {
            if (typeof smth !== typeName) {
                if (errorMessage === undefined) {
                    fail(`No error message. Value: ${smth} (type: ${typeof smth}) has different type to: ${typeName})`);
                }
                else {
                    fail(`${errorMessage}. Value: ${smth} (type: ${typeof smth}) has different type to: ${typeName})`);
                }
            }
            return out;
        },
        typeNot: (typeName) => {
            if (typeof smth === typeName) {
                if (errorMessage === undefined) {
                    fail(`No error message. Value: ${smth} (type: ${typeof smth}) has prohibited type: ${typeName})`);
                }
                else {
                    fail(`${errorMessage}. Value: ${smth} (type: ${typeof smth}) has prohibited type: ${typeName})`);
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