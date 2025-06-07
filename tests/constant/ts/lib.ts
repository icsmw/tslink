import { A, B, C, D } from "../output/module";

if (A !== "Hello" || B !== 42 || C !== 42 || D !== 42) {
    process.exit(1);
}
