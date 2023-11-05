"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
const binding_1 = require("binding");
{
    // Test 001
    const A = "Hello, World!";
    const B = "A";
    const struct = new binding_1.Struct(A, B);
    const data = {
        a: 1,
        b: 2,
        s: "test",
    };
    console.assert(struct.getA() === A, "method get_a() gives invalid output");
    console.assert(struct.getB() === B, "method get_b() gives invalid output");
    const recieved = struct.getData(data);
    console.assert(typeof recieved !== "string", "method getData() gives invalid output");
    if (typeof recieved === "string") {
        throw new Error("method getData() gives invalid output");
    }
    console.assert(recieved.a === 2, "Value of struct.a invalid");
    console.assert(recieved.b === 3, "Value of struct.b invalid");
    console.assert(recieved.s === "testtest", "Value of struct.s invalid");
}
{
    // Test 002
    const data = {
        a: 1,
        b: 2,
        s: "test",
    };
    const recieved = (0, binding_1.getDataFunc)(data, 10, 10);
    console.assert(typeof recieved !== "string", "method getDataFunc() gives invalid output");
    if (typeof recieved === "string") {
        throw new Error("method getDataFunc() gives invalid output");
    }
    console.assert(recieved.a === 11, "Value of struct.a invalid");
    console.assert(recieved.b === 12, "Value of struct.b invalid");
    console.assert(recieved.s === "testtest", "Value of struct.s invalid");
}
try {
    (0, binding_1.testError)();
}
catch (e) {
    console.log(e);
    console.log(typeof e);
}
// {
//     // Test 003
//     const invalid = {
//         a: 1,
//         b: 2,
//         s: 666,
//     };
//     try {
//         const recieved = getDataFunc(invalid as unknown as Data, 10, 10);
//         console.assert(
//             typeof recieved !== "string",
//             "method getDataFunc() gives invalid output"
//         );
//         if (typeof recieved === "string") {
//             throw new Error("method getDataFunc() gives invalid output");
//         }
//         console.assert(recieved.a === 11, "Value of struct.a invalid");
//         console.assert(recieved.b === 12, "Value of struct.b invalid");
//         console.assert(recieved.s === "testtest", "Value of struct.s invalid");
//     } catch (e) {
//         console.log(e);
//     }
// }
//# sourceMappingURL=lib.js.map