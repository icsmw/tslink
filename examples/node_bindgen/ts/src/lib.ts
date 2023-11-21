import { MyScruct } from "binding";

const myStruct = new MyScruct();

if (myStruct.incMyNumber(1) === 2) {
    console.log(`Success`);
} else {
    console.error(`Fail to test native module`);
    process.exit(1);
}
