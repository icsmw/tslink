import { A, B } from "binding";

console.log(A);
console.log(B);
if (A !== "Hello" || B !== 42) {
    process.exit(1);
}
