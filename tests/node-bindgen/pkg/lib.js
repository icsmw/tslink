"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
const binding_1 = require("binding");
const A = "Hello, World!";
const B = "A";
const struct = new binding_1.Struct(A, B);
console.assert(struct.getA() === A, "method get_a() gives invalid output");
console.assert(struct.getB() === B, "method get_b() gives invalid output");
console.log(struct.getA());
//# sourceMappingURL=lib.js.map