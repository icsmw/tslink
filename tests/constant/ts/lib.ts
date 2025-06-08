import { A, B, C, D, F, G } from "../output/module";

if (
    A !== "Hello" ||
    B !== 42 ||
    C !== 42 ||
    D !== 42 ||
    F.join(",") !== "1,2,3,4" ||
    G.join(",") !== "1,2,3,4"
) {
    process.exit(1);
}
