import { Struct } from "binding";

const A = "Hello, World!";
const B = "A";
const struct = new Struct(A, B);

console.assert(struct.getA() === A, "method get_a() gives invalid output");
console.assert(struct.getB() === B, "method get_b() gives invalid output");
console.log(struct.getA());
