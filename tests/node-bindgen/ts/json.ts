import { Group } from "./common";
import { StructCustomData, EnumA } from "binding";

const tests = new Group("Custom Data Tests");
const struct = new StructCustomData();

{
    const test = tests.test("getData");
    const result = struct.getData({ a: 1, b: 2, c: "test" });
    if (result instanceof Error) {
        test.fail(
            `Function getData() of StructCustomData returns error: ${result.message}`
        );
    } else {
        test.assert(result.a).msg("Value of result.a invalid").equal(2);
        test.assert(result.b).msg("Value of result.b invalid").equal(3);
        test.assert(result.c)
            .msg("Value of result.s invalid")
            .equal("testtest");
        test.success();
    }
}

{
    const test = tests.test("getDataA");
    const result = struct.getDataA({ a: 1, b: 2, c: "test" });
    if (result instanceof Error) {
        test.fail(
            `Function getDataA() of StructCustomData returns error: ${result.message}`
        );
    } else {
        test.assert(result.a.a).msg("Value of result.a.a invalid").equal(2);
        test.assert(result.a.b).msg("Value of result.a.b invalid").equal(3);
        test.assert(result.a.c)
            .msg("Value of result.a.c invalid")
            .equal("testtest");
        test.assert(result.b instanceof Array)
            .msg("Value of result.b invalid")
            .beTrue();
        test.assert(result.b.length).msg("Value of result.b invalid").equal(2);
        test.assert(result.b[0].a)
            .msg("Value of result.b[0].a invalid")
            .equal(2);
        test.assert(result.b[0].b)
            .msg("Value of result.b[0].b invalid")
            .equal(3);
        test.assert(result.b[0].c)
            .msg("Value of result.b[0].c invalid")
            .equal("testtest");
        test.assert(result.b[1].a)
            .msg("Value of result.b[1].a invalid")
            .equal(2);
        test.assert(result.b[1].b)
            .msg("Value of result.b[1].b invalid")
            .equal(3);
        test.assert(result.b[1].c)
            .msg("Value of result.b[1].c invalid")
            .equal("testtest");
        test.assert(result.c).msg("Value of result.c invalid").type("object");
        test.assert(result.c["first"])
            .msg("Value of result.c invalid")
            .typeNot("undefined");
        test.assert(result.c["first"].a)
            .msg("Value of result.b[1].a invalid")
            .equal(2);
        test.assert(result.c["first"].b)
            .msg("Value of result.b[1].b invalid")
            .equal(3);
        test.assert(result.c["first"].c)
            .msg("Value of result.b[1].c invalid")
            .equal("testtest");
        test.assert(result.c["second"])
            .msg("Value of result.c invalid")
            .typeNot("undefined");
        test.assert(result.c["second"].a)
            .msg("Value of result.b[1].a invalid")
            .equal(2);
        test.assert(result.c["second"].b)
            .msg("Value of result.b[1].b invalid")
            .equal(3);
        test.assert(result.c["second"].c)
            .msg("Value of result.b[1].c invalid")
            .equal("testtest");
        test.assert(result.d).msg("Value of result.d invalid").equal("test");
        test.success();
    }
}

{
    const test = tests.test("getMultipleData");
    const result = struct.getMultipleData(
        { a: 1, b: 2, c: "test" },
        { a: 2, b: 3 }
    );
    if (result instanceof Error) {
        test.fail(
            `Function getMultipleData() of StructCustomData returns error: ${result.message}`
        );
    } else {
        test.assert(result[0]).msg("Value of result.a invalid").equal(3);
        test.assert(result[1]).msg("Value of result.b invalid").equal(5);
        test.success();
    }
}

{
    const test = tests.test("getEnumA");
    console.log(EnumA.One);
    const one = struct.getEnumA(EnumA.One);
    const two = struct.getEnumA(EnumA.Two);
    const three = struct.getEnumA(EnumA.Three);
    if (
        one instanceof Error ||
        two instanceof Error ||
        three instanceof Error
    ) {
        test.fail(`Function getEnumA() of StructCustomData returns error`);
    } else {
        test.assert(one).msg("Value of one invalid").equal(EnumA.One);
        test.assert(two).msg("Value of two invalid").equal(EnumA.Two);
        test.assert(three).msg("Value of three invalid").equal(EnumA.Three);
        test.success();
    }
}

{
    const test = tests.test("getEnumB");
    const one = struct.getEnumB({
        One: "test",
    });
    const two = struct.getEnumB({
        Two: [1, 2],
    });
    const three = struct.getEnumB({
        Three: 1,
    });
    const fourSome = struct.getEnumB({
        Four: 1,
    });
    const fourNone = struct.getEnumB({
        Four: null,
    });
    if (
        one instanceof Error ||
        two instanceof Error ||
        three instanceof Error ||
        fourSome instanceof Error ||
        fourNone instanceof Error
    ) {
        test.fail(`Function getEnumB() of StructCustomData returns error`);
    } else {
        test.assert(one.One).msg("Value of one invalid").equal("test");
        test.assert(Object.keys(one).length)
            .msg("Value of one invalid")
            .equal(1);
        test.assert((two.Two as any)[0])
            .msg("Value of two invalid")
            .equal(1);
        test.assert((two.Two as any)[1])
            .msg("Value of two invalid")
            .equal(2);
        test.assert(Object.keys(two).length)
            .msg("Value of one invalid")
            .equal(1);
        test.assert(three.Three).msg("Value of three invalid").equal(1);
        test.assert(Object.keys(three).length)
            .msg("Value of three invalid")
            .equal(1);
        test.assert(fourSome.Four).msg("Value of fourSome invalid").equal(1);
        test.assert(Object.keys(fourSome).length)
            .msg("Value of fourSome invalid")
            .equal(1);
        test.assert(fourNone.Four).msg("Value of fourNone invalid").equal(null);
        test.assert(Object.keys(fourNone).length)
            .msg("Value of fourNone invalid")
            .equal(1);
        test.success();
    }
}

{
    const test = tests.test("testOfErrorSupportOk");
    const result = struct.testOfErrorSupportOk();
    test.assert(result).msg("Value of result invalid").type("number");
    test.assert(result).msg("Value of result invalid").equal(666);
    test.success();
}

{
    const test = tests.test("testOfErrorSupportErr");
    const result = struct.testOfErrorSupportErr();
    if (result instanceof Error) {
        test.assert(result.err)
            .msg("Value of result.err invalid")
            .type("object");
        test.assert(result.err?.code)
            .msg("Value of result.err?.code invalid")
            .equal(666);
        test.assert(result.err?.msg)
            .msg("Value of result.err?.msg invalid")
            .equal("test");
        test.assert(result.err?.err.code)
            .msg("Value of result.err?.err.code invalid")
            .equal(666);
        test.assert(result.err?.err.msg)
            .msg("Value of result.err?.err.code invalid")
            .equal("Error");
    } else {
        test.fail("Value of result invalid");
    }
    test.success();
}
