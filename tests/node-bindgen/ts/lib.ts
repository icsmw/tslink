import { assert } from "./common";
import "./custom_data";
import "./error_handeling";
import "./callbacks";
import "./consturctors";

import { Struct, Data, getDataFunc } from "binding";

{
    // Test 001
    const A = "Hello, World!";
    const B = "A";
    const struct = new Struct(A, B);
    const data: Data = {
        a: 1,
        b: 2,
        s: "test",
    };
    assert(struct.getA() === A)
        .msg("method get_a() gives invalid output")
        .beTrue();
    assert(struct.getB() === B)
        .msg("method get_b() gives invalid output")
        .beTrue();
    const recieved = struct.getData(data);
    assert(recieved)
        .msg("method getData() gives invalid output")
        .typeNot("string");
    if (recieved instanceof Error) {
        throw new Error(`method getData() returns error: ${recieved.message}`);
    }
    assert(recieved.a).msg("Value of struct.a invalid").equal(2);
    assert(recieved.b).msg("Value of struct.b invalid").equal(3);
    assert(recieved.s).msg("Value of struct.s invalid").equal("testtest");
}
{
    // Test 002
    const data: Data = {
        a: 1,
        b: 2,
        s: "test",
    };
    const recieved = getDataFunc(data, 10, 10);
    assert(recieved)
        .msg("method getDataFunc() gives invalid output")
        .typeNot("string");
    if (recieved instanceof Error) {
        throw new Error(
            `method getDataFunc() returns error: ${recieved.message}`
        );
    }
    assert(recieved.a).msg("Value of struct.a invalid").equal(11);
    assert(recieved.b).msg("Value of struct.b invalid").equal(12);
    assert(recieved.s).msg("Value of struct.s invalid").equal("testtest");
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
